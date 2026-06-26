use super::*;
use anyhow::Context;
use anyhow::Result;
use ontocode_utils_output_truncation::approx_token_count;
use tempfile::tempdir;

#[tokio::test]
async fn small_hook_output_remains_inline() -> Result<()> {
    let dir = tempdir()?;
    let output_dir = AbsolutePathBuf::from_absolute_path(dir.path())?.join(HOOK_OUTPUTS_DIR);
    let thread_id = ThreadId::new();
    let spiller = HookOutputSpiller {
        output_dir: output_dir.clone(),
    };

    let output = spiller
        .maybe_spill_text(thread_id, "short".to_string())
        .await;

    assert_eq!(output, "short");
    assert!(!output_dir.exists());
    Ok(())
}

#[tokio::test]
async fn large_hook_output_spills_to_file() -> Result<()> {
    let dir = tempdir()?;
    let text = "hook output ".repeat(1_000);
    let output_dir = AbsolutePathBuf::from_absolute_path(dir.path())?.join(HOOK_OUTPUTS_DIR);
    let spiller = HookOutputSpiller { output_dir };

    let output = spiller
        .maybe_spill_text(ThreadId::new(), text.clone())
        .await;

    assert!(output.contains("tokens truncated"));
    let path = output
        .lines()
        .find_map(|line| line.strip_prefix("Full hook output saved to: "))
        .context("spill path")?;
    assert_eq!(fs::read_to_string(path).await?, text);
    Ok(())
}

/// Repeated large hook text must each stay within the model-visible budget.
///
/// This guards against duplicate hook output bypassing the spill path (e.g.
/// if a caller loops and re-spills the same text).
#[tokio::test]
async fn repeated_large_hook_text_stays_bounded() -> Result<()> {
    let dir = tempdir()?;
    let text = "repeated hook line\n".repeat(2_000);
    let output_dir = AbsolutePathBuf::from_absolute_path(dir.path())?.join(HOOK_OUTPUTS_DIR);
    let spiller = HookOutputSpiller {
        output_dir: output_dir.clone(),
    };
    let thread_id = ThreadId::new();

    // Spill the same large text twice, simulating duplicate hook output.
    let results = spiller
        .maybe_spill_texts(thread_id, vec![text.clone(), text.clone()])
        .await;

    assert_eq!(results.len(), 2);
    for result in &results {
        let token_count = approx_token_count(result);
        assert!(
            token_count <= HOOK_OUTPUT_TOKEN_LIMIT,
            "spilled hook output exceeded token limit: {token_count} > {HOOK_OUTPUT_TOKEN_LIMIT}"
        );
    }
    Ok(())
}

/// A spilled hook output must carry exactly one recovery-path attribution line.
///
/// A preview that contained zero paths would leave the model unable to retrieve
/// the full text; a preview with two paths would be misleading.
#[tokio::test]
async fn spilled_output_has_exactly_one_recovery_path() -> Result<()> {
    let dir = tempdir()?;
    let text = "hook line content\n".repeat(2_000);
    let output_dir = AbsolutePathBuf::from_absolute_path(dir.path())?.join(HOOK_OUTPUTS_DIR);
    let spiller = HookOutputSpiller { output_dir };

    let output = spiller.maybe_spill_text(ThreadId::new(), text).await;

    let attribution_count = output
        .lines()
        .filter(|line| line.starts_with("Full hook output saved to: "))
        .count();
    assert_eq!(
        attribution_count, 1,
        "expected exactly one recovery-path line, got {attribution_count}"
    );
    Ok(())
}

/// Each spill file must preserve the complete original hook text, even when
/// the same large text is spilled more than once for the same thread.
#[tokio::test]
async fn duplicate_hook_text_spill_files_contain_full_text() -> Result<()> {
    let dir = tempdir()?;
    let text = "full content line\n".repeat(2_000);
    let output_dir = AbsolutePathBuf::from_absolute_path(dir.path())?.join(HOOK_OUTPUTS_DIR);
    let spiller = HookOutputSpiller {
        output_dir: output_dir.clone(),
    };
    let thread_id = ThreadId::new();

    let results = spiller
        .maybe_spill_texts(thread_id, vec![text.clone(), text.clone()])
        .await;

    assert_eq!(results.len(), 2);
    for (i, result) in results.iter().enumerate() {
        let path = result
            .lines()
            .find_map(|line| line.strip_prefix("Full hook output saved to: "))
            .with_context(|| format!("spill path missing in result {i}"))?;
        let saved = fs::read_to_string(path)
            .await
            .with_context(|| format!("could not read spill file for result {i}"))?;
        assert_eq!(
            saved, text,
            "spill file {i} did not contain the full original text"
        );
    }
    Ok(())
}
