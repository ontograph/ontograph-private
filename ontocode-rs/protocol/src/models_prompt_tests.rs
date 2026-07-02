//! Static prompt duplication guard for base_instructions/default.md (JSC-33).
//!
//! Guards against accidentally duplicating headings or significant section
//! blocks in the base-instructions prompt. This is intentionally scoped to
//! the one static prompt file included by this crate; it is not a repo-wide
//! markdown clone scanner.

use super::BASE_INSTRUCTIONS_DEFAULT;

/// Minimum number of non-empty lines a block must have to be considered
/// "high-signal" for the duplicate-block check.
const MIN_BLOCK_LINES: usize = 4;

/// Collect all ATX heading lines from a markdown string, normalising
/// whitespace so that `# Foo  ` and `# Foo` are treated identically.
fn extract_headings(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with('#'))
        .map(|line| line.trim().to_string())
        .collect()
}

/// Split text into non-empty paragraph-level blocks (runs of consecutive
/// non-blank lines), then return those that meet the minimum line count.
fn extract_significant_blocks(text: &str) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    let mut current: Vec<&str> = Vec::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            if current.len() >= MIN_BLOCK_LINES {
                blocks.push(current.join("\n"));
            }
            current.clear();
        } else {
            current.push(line);
        }
    }
    if current.len() >= MIN_BLOCK_LINES {
        blocks.push(current.join("\n"));
    }

    blocks
}

#[test]
fn base_instructions_has_no_duplicate_headings() {
    let headings = extract_headings(BASE_INSTRUCTIONS_DEFAULT);
    let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut duplicates: Vec<String> = Vec::new();

    for (idx, heading) in headings.iter().enumerate() {
        if let Some(first_idx) = seen.get(heading) {
            duplicates.push(format!(
                "heading {:?} first at line-group {first_idx}, duplicated at line-group {idx}",
                heading
            ));
        } else {
            seen.insert(heading.clone(), idx);
        }
    }

    assert!(
        duplicates.is_empty(),
        "base_instructions/default.md contains duplicate headings:\n{}",
        duplicates.join("\n")
    );
}

#[test]
fn base_instructions_has_no_duplicate_section_blocks() {
    let blocks = extract_significant_blocks(BASE_INSTRUCTIONS_DEFAULT);
    let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut duplicates: Vec<String> = Vec::new();

    for (idx, block) in blocks.iter().enumerate() {
        // Normalise internal whitespace to make the check content-focused
        // rather than spacing-sensitive.
        let key: String = block.lines().map(str::trim).collect::<Vec<_>>().join("\n");

        if let Some(first_idx) = seen.get(&key) {
            // Show a short excerpt so a failure is easy to diagnose.
            let excerpt: String = block.lines().take(3).collect::<Vec<_>>().join(" / ");
            duplicates.push(format!(
                "block #{idx} is identical to block #{first_idx}: \"{excerpt}...\""
            ));
        } else {
            seen.insert(key, idx);
        }
    }

    assert!(
        duplicates.is_empty(),
        "base_instructions/default.md contains duplicate section blocks:\n{}",
        duplicates.join("\n")
    );
}

#[test]
fn base_instructions_preserve_manager_loop_preflight_rules() {
    for required in [
        "classify the current tracking state as `active`, `no-dispatch`, `blocked`, `docs/design-only`, `proof-only`, or `implementation-ready`",
        "close with the recorded reopen gate instead of rewriting tracking or inventing a next task",
        "lacks both `active_next_task` and dependency-ready `OPEN` task rows, do not promote it into a tracker during the loop",
        "Do not dispatch explorer, reviewer, worker, or other helper agents for loop shaping, tracker rewriting, or implementation prep",
        "write down the expected changed files before editing",
        "rerun the smallest targeted command that can prove the slice before widening scope",
        "do not retry the same role/model pair again in the same session unless the tool surface changed",
        "do not substitute a nearby generic role such as `explorer`, `worker`, or `reviewer`",
        "Cache the failure for the current session and move to closeout",
        "If a `spawn_agent` call sets `agent_type`, `model`, or `reasoning_effort`, omit `fork_context` or set it to false",
        "Before declaring a required role/model dispatch unavailable, review the user's role spec for prompt-shape mistakes",
        "Report the corrected structured call shape, for example `model=\"gpt-5.5\", reasoning_effort=\"medium\"`",
        "fail closed with `prompt-shape error` and the corrected field split",
        "Do not use `requested role/model unavailable` for a value that can be parsed as an available model plus a separate option",
    ] {
        assert!(
            BASE_INSTRUCTIONS_DEFAULT.contains(required),
            "base_instructions/default.md is missing manager-loop guidance: {required}"
        );
    }
}

#[test]
fn base_instructions_preserve_session_isolation_rules() {
    for required in [
        "Treat the current session as one bounded workstream",
        "Work only on the newest user-confirmed slice unless the user explicitly retargets the session",
        "Do not mix unrelated asks into the active workstream",
        "Preserve them unless the current workstream explicitly owns them",
        "Do not do \"while I am here\" edits during a bounded workstream",
    ] {
        assert!(
            BASE_INSTRUCTIONS_DEFAULT.contains(required),
            "base_instructions/default.md is missing session-isolation guidance: {required}"
        );
    }
}

#[test]
fn base_instructions_preserve_read_only_write_loop_rules() {
    for required in [
        "prefer one bounded write phase per turn",
        "If the write path is read-only, preserve the intended patch or write set as the artifact",
        "do not keep probing or partially applying edits",
        "If a goal-continuation loop hits the same external write blocker repeatedly",
        "call the goal-status update tool to mark the goal `blocked` before the final response",
        "Final-answer prose saying the work is blocked is not enough when the persisted goal is still active",
    ] {
        assert!(
            BASE_INSTRUCTIONS_DEFAULT.contains(required),
            "base_instructions/default.md is missing read-only write-loop guidance: {required}"
        );
    }
}
