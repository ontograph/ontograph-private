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
