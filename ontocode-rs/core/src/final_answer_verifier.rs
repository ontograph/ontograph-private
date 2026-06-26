use ontocode_protocol::read_evidence::FileReadEvidence;

const MAX_WARNING_CHARS: usize = 512;

pub(crate) fn final_answer_verification_warning(
    answer: &str,
    evidence: &FileReadEvidence,
) -> Option<String> {
    let answer = answer.trim();
    if answer.is_empty() {
        return None;
    }

    let normalized = normalize_answer(answer);
    let mut gaps = Vec::new();

    if claims_tests_ran(&normalized) && evidence.tests_run.is_empty() {
        gaps.push("tests");
    }
    if claims_policy_check_ran(&normalized) && evidence.policy_checks.is_empty() {
        gaps.push("policy checks");
    }
    if claims_source_change(&normalized) && evidence.source_references.is_empty() {
        gaps.push("source references");
    }

    if gaps.is_empty() {
        return None;
    }

    let mut warning = format!(
        "Final answer verifier could not match claimed {} against recorded turn evidence.",
        gaps.join(", ")
    );
    if warning.chars().count() > MAX_WARNING_CHARS {
        warning = truncate_to_chars(&warning, MAX_WARNING_CHARS);
    }
    Some(warning)
}

fn normalize_answer(answer: &str) -> String {
    answer
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn claims_tests_ran(answer: &str) -> bool {
    !contains_any(
        answer,
        &[
            "tests not run",
            "test not run",
            "did not run tests",
            "didn't run tests",
            "not able to run tests",
            "unable to run tests",
        ],
    ) && contains_any(
        answer,
        &[
            "tests passed",
            "test passed",
            "tests pass",
            "test pass",
            "ran tests",
            "ran the tests",
            "verification passed",
        ],
    )
}

fn claims_policy_check_ran(answer: &str) -> bool {
    !contains_any(
        answer,
        &[
            "fmt not run",
            "formatting not run",
            "did not run fmt",
            "didn't run fmt",
            "did not run formatting",
            "didn't run formatting",
        ],
    ) && contains_any(
        answer,
        &[
            "fmt passed",
            "formatting passed",
            "ran fmt",
            "ran formatting",
            "just fmt",
            "cargo fmt",
        ],
    )
}

fn claims_source_change(answer: &str) -> bool {
    !contains_any(
        answer,
        &[
            "no source changes",
            "no file changes",
            "did not change files",
            "didn't change files",
        ],
    ) && contains_any(
        answer,
        &[
            "changed file",
            "changed files",
            "updated file",
            "updated files",
            "modified file",
            "modified files",
        ],
    )
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn truncate_to_chars(value: &str, max_chars: usize) -> String {
    let mut chars = value.chars();
    let mut truncated = chars.by_ref().take(max_chars).collect::<String>();
    if chars.next().is_some() {
        truncated.push_str("...");
    }
    truncated
}

#[cfg(test)]
#[path = "final_answer_verifier_tests.rs"]
mod tests;
