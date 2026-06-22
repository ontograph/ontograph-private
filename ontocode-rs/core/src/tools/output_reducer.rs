use std::borrow::Cow;

const MIN_SUCCESS_LINES: usize = 6;
const MAX_WARNING_LINES: usize = 6;
const MAX_ERROR_LINES: usize = 40;
const MAX_TEST_FAILURE_LINES: usize = 24;
const MAX_RG_MATCH_LINES: usize = 8;
const MIN_GIT_STATUS_LINES: usize = 10;
const MIN_PYTHON_UNITTEST_LINES: usize = 4;
const MIN_PYTHON_DOT_SUMMARY_COUNT: usize = 12;
const MIN_GENERIC_LOG_LINES: usize = 8;
const GENERIC_LOG_EDGE_LINES: usize = 2;

pub(crate) fn reduce_exec_output(content: &str) -> Cow<'_, str> {
    let lines: Vec<&str> = content.lines().collect();

    if let Some(reduced) = reduce_failed_rust_tests(&lines)
        .or_else(|| reduce_rust_compiler_errors(&lines))
        .or_else(|| reduce_successful_rust_output(&lines))
        .or_else(|| reduce_python_unittest_output(&lines))
        .or_else(|| reduce_rg_output(&lines))
        .or_else(|| reduce_git_status_output(&lines))
        .or_else(|| reduce_generic_log_output(&lines))
        && reduced.len() < content.len()
    {
        return Cow::Owned(reduced);
    }

    Cow::Borrowed(content)
}

fn reduce_successful_rust_output(lines: &[&str]) -> Option<String> {
    if lines.len() < MIN_SUCCESS_LINES {
        return None;
    }

    let status_line = lines
        .iter()
        .rev()
        .copied()
        .find(|line| is_success_status_line(line))?;
    let has_verbose_chatter = lines.iter().any(|line| {
        line.starts_with("Compiling ")
            || line.starts_with("Checking ")
            || line.starts_with("Running ")
            || line.starts_with("running ")
    });

    let warning_lines = warning_summary_lines(lines);
    if warning_lines.is_empty() && !has_verbose_chatter && lines.len() < MIN_SUCCESS_LINES * 2 {
        return None;
    }

    let mut reduced = Vec::new();

    if !warning_lines.is_empty() {
        reduced.extend(warning_lines);
    }

    if reduced.len() > MAX_WARNING_LINES {
        reduced.truncate(MAX_WARNING_LINES);
    }

    if reduced.last().map(std::string::String::as_str) != Some(status_line) {
        reduced.push(status_line.to_owned());
    }

    Some(reduced.join("\n"))
}

fn reduce_rust_compiler_errors(lines: &[&str]) -> Option<String> {
    if !lines.iter().any(|line| is_compiler_error_line(line)) {
        return None;
    }

    if !lines.iter().any(|line| is_file_reference_line(line)) {
        return None;
    }

    let mut keep = vec![false; lines.len()];

    for (index, line) in lines.iter().enumerate() {
        if is_compiler_error_line(line)
            || is_file_reference_line(line)
            || is_error_followup_line(line)
        {
            mark_window(&mut keep, index, 1, 2);
        }
    }

    collect_selected_lines(lines, &keep, MAX_ERROR_LINES)
}

fn reduce_failed_rust_tests(lines: &[&str]) -> Option<String> {
    if !lines.iter().any(|line| is_failed_test_marker(line)) {
        return None;
    }

    if !lines.iter().any(|line| is_test_failure_context(line)) {
        return None;
    }

    let mut keep = vec![false; lines.len()];

    for (index, line) in lines.iter().enumerate() {
        if is_failed_test_marker(line) || is_test_failure_context(line) {
            mark_window(&mut keep, index, 1, 1);
        }
    }

    collect_selected_lines(lines, &keep, MAX_TEST_FAILURE_LINES)
}

fn reduce_python_unittest_output(lines: &[&str]) -> Option<String> {
    if !is_confident_python_unittest_output(lines) {
        return None;
    }

    let summarize_dots = should_summarize_python_dots(lines);
    let mut keep = vec![false; lines.len()];

    for (index, line) in lines.iter().enumerate() {
        if is_python_unittest_run_summary_line(line)
            || is_python_unittest_final_status_line(line)
            || is_python_unittest_failure_marker_line(line)
            || is_python_traceback_header_line(line)
            || is_python_traceback_context_line(line)
        {
            keep[index] = true;
        } else if is_python_traceback_target_line(line) {
            mark_window(&mut keep, index, 1, 2);
        }
    }

    let mut reduced = Vec::new();
    let mut pending_dots = Vec::new();
    let mut pending_dot_count = 0usize;

    for (index, line) in lines.iter().enumerate() {
        if keep[index] {
            flush_python_dot_run(
                &mut reduced,
                &mut pending_dots,
                pending_dot_count,
                summarize_dots,
            );
            pending_dot_count = 0;
            reduced.push((*line).to_owned());
        } else if is_python_unittest_dot_line(line) {
            pending_dots.push(*line);
            pending_dot_count += line.trim().chars().filter(|ch| *ch == '.').count();
        }
    }

    flush_python_dot_run(
        &mut reduced,
        &mut pending_dots,
        pending_dot_count,
        summarize_dots,
    );

    if reduced.is_empty() {
        return None;
    }

    Some(reduced.join("\n"))
}

fn reduce_rg_output(lines: &[&str]) -> Option<String> {
    if lines.len() <= MAX_RG_MATCH_LINES {
        return None;
    }

    if !lines.iter().all(|line| is_rg_match_line(line)) {
        return None;
    }

    let omitted_count = lines.len() - MAX_RG_MATCH_LINES;
    let mut reduced = lines
        .iter()
        .take(MAX_RG_MATCH_LINES)
        .copied()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    reduced.push(format!("... ({omitted_count} matches omitted)"));
    Some(reduced.join("\n"))
}

fn reduce_git_status_output(lines: &[&str]) -> Option<String> {
    if lines.len() < MIN_GIT_STATUS_LINES {
        return None;
    }

    let mut counts = GitStatusCounts::default();
    for line in lines {
        let category = git_status_category(line)?;
        counts.increment(category);
    }

    if counts.total() < MIN_GIT_STATUS_LINES {
        return None;
    }

    let mut reduced = vec![format!("git status summary ({} paths):", counts.total())];
    counts.push_lines(&mut reduced);
    Some(reduced.join("\n"))
}

fn reduce_generic_log_output(lines: &[&str]) -> Option<String> {
    if lines.len() < MIN_GENERIC_LOG_LINES {
        return None;
    }

    let mut keep = vec![false; lines.len()];
    let edge_count = GENERIC_LOG_EDGE_LINES.min(lines.len());

    for index in 0..edge_count {
        keep[index] = true;
    }

    for index in lines.len().saturating_sub(edge_count)..lines.len() {
        keep[index] = true;
    }

    for (index, line) in lines.iter().enumerate() {
        if is_generic_log_error_line(line) {
            keep[index] = true;
        }
    }

    let kept_count = keep.iter().filter(|slot| **slot).count();
    if kept_count == 0 || kept_count >= lines.len() {
        return None;
    }

    let omitted_count = lines.len() - kept_count;
    let mut reduced = Vec::new();
    let mut inserted_summary = false;

    for (index, line) in lines.iter().enumerate() {
        if keep[index] {
            if !inserted_summary && !reduced.is_empty() {
                reduced.push(format!("... ({omitted_count} lines omitted)"));
                inserted_summary = true;
            }
            reduced.push((*line).to_owned());
        }
    }

    if reduced.len() >= lines.len() {
        return None;
    }

    Some(reduced.join("\n"))
}

fn warning_summary_lines(lines: &[&str]) -> Vec<String> {
    let mut warnings = Vec::new();
    let mut warning_count = 0usize;

    for line in lines {
        if is_warning_line(line) {
            warning_count += 1;

            if warnings.len() < MAX_WARNING_LINES {
                warnings.push((*line).to_owned());
            }
        }
    }

    if warning_count > warnings.len() && warning_count > 1 {
        warnings.insert(0, format!("warnings: {warning_count}"));
    }

    warnings
}

fn collect_selected_lines(lines: &[&str], keep: &[bool], max_lines: usize) -> Option<String> {
    let mut selected = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        if keep[index] {
            selected.push((*line).to_owned());
        }
    }

    if selected.is_empty() {
        return None;
    }

    if selected.len() > max_lines {
        selected.truncate(max_lines);
    }

    Some(selected.join("\n"))
}

fn mark_window(keep: &mut [bool], index: usize, before: usize, after: usize) {
    let start = index.saturating_sub(before);
    let end = index
        .saturating_add(after)
        .min(keep.len().saturating_sub(1));

    for slot in &mut keep[start..=end] {
        *slot = true;
    }
}

fn is_success_status_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("Finished ")
        || trimmed.starts_with("test result: ok.")
        || (trimmed.starts_with("Doc-tests ") && trimmed.ends_with(" ok"))
}

fn is_warning_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("warning:")
        || trimmed.contains("warning: ")
        || trimmed.ends_with("warnings emitted")
}

fn is_compiler_error_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("error[")
        || trimmed.starts_with("error:")
        || trimmed.starts_with("error: could not compile")
        || trimmed.starts_with("error: aborting")
}

fn is_file_reference_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("--> ") || trimmed.contains(".rs:") || trimmed.contains(".rs)")
}

fn is_error_followup_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with('|')
        || trimmed.starts_with("= note:")
        || trimmed.starts_with("= help:")
        || trimmed.starts_with("note:")
        || trimmed.starts_with("help:")
        || trimmed.starts_with("For more information about this error")
        || trimmed.starts_with("error: could not compile")
}

fn is_failed_test_marker(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.contains("FAILED")
        || trimmed.starts_with("failures:")
        || trimmed.starts_with("test result: FAILED")
}

fn is_test_failure_context(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("---- ")
        || trimmed.starts_with("thread '")
        || trimmed.contains("panicked at")
        || trimmed.starts_with("assertion failed:")
}

fn is_confident_python_unittest_output(lines: &[&str]) -> bool {
    if lines.len() < MIN_PYTHON_UNITTEST_LINES {
        return false;
    }

    let has_summary = lines
        .iter()
        .any(|line| is_python_unittest_final_status_line(line));
    let has_run_summary = lines
        .iter()
        .any(|line| is_python_unittest_run_summary_line(line));
    let has_failure_marker = lines
        .iter()
        .any(|line| is_python_unittest_failure_marker_line(line));
    let has_traceback = lines
        .iter()
        .any(|line| is_python_traceback_header_line(line) || is_python_traceback_target_line(line));
    let has_test_activity = lines.iter().any(|line| {
        is_python_unittest_dot_line(line)
            || line.trim_start().starts_with("test ")
            || line.trim_start().starts_with("FAIL:")
            || line.trim_start().starts_with("ERROR:")
    });

    has_summary && has_test_activity && (has_run_summary || has_failure_marker || has_traceback)
}

fn should_summarize_python_dots(lines: &[&str]) -> bool {
    let dot_line_count = lines
        .iter()
        .filter(|line| is_python_unittest_dot_line(line))
        .count();
    let dot_count = lines
        .iter()
        .filter(|line| is_python_unittest_dot_line(line))
        .map(|line| line.trim().chars().filter(|ch| *ch == '.').count())
        .sum::<usize>();

    dot_line_count > 1 || dot_count >= MIN_PYTHON_DOT_SUMMARY_COUNT || lines.len() > 20
}

fn flush_python_dot_run(
    reduced: &mut Vec<String>,
    pending_dots: &mut Vec<&str>,
    pending_dot_count: usize,
    summarize_dots: bool,
) {
    if pending_dots.is_empty() {
        return;
    }

    if summarize_dots {
        reduced.push(format!("... ({pending_dot_count} dots)"));
    } else {
        reduced.extend(pending_dots.iter().copied().map(str::to_owned));
    }

    pending_dots.clear();
}

fn is_python_unittest_run_summary_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("Ran ") && trimmed.contains(" tests in ")
}

fn is_python_unittest_final_status_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed == "OK" || trimmed == "FAILED" || trimmed.starts_with("FAILED (")
}

fn is_python_unittest_failure_marker_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("FAIL:")
        || trimmed.starts_with("ERROR:")
        || trimmed.contains(" ... FAIL")
        || trimmed.contains(" ... ERROR")
}

fn is_python_unittest_dot_line(line: &str) -> bool {
    let trimmed = line.trim();

    !trimmed.is_empty() && trimmed.chars().all(|ch| ch == '.')
}

fn is_python_traceback_header_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("Traceback (most recent call last):")
}

fn is_python_traceback_target_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("File \"") || trimmed.starts_with("File '")
}

fn is_python_traceback_context_line(line: &str) -> bool {
    let trimmed = line.trim_start();

    trimmed.starts_with("During handling of the above exception")
        || trimmed
            .starts_with("The above exception was the direct cause of the following exception")
}

fn is_rg_match_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }

    let (path, line_number_and_rest) = match trimmed.split_once(':') {
        Some(parts) => parts,
        None => return false,
    };
    if path.is_empty() {
        return false;
    }

    let (line_number, rest) = match line_number_and_rest.split_once(':') {
        Some(parts) => parts,
        None => return false,
    };
    if line_number.parse::<usize>().is_err() {
        return false;
    }
    if rest.is_empty() {
        return false;
    }

    if let Some((column, match_text)) = rest.split_once(':') {
        if column.parse::<usize>().is_err() {
            return false;
        }
        !match_text.is_empty()
    } else {
        true
    }
}

fn is_generic_log_error_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    let lowered = trimmed.to_ascii_lowercase();

    lowered.starts_with("error")
        || lowered.contains(" error:")
        || lowered.starts_with("panic:")
        || lowered.starts_with("fatal:")
        || lowered.starts_with("exception:")
        || lowered.starts_with("traceback")
        || lowered.starts_with("caused by:")
        || lowered.starts_with("failed:")
        || lowered.starts_with("failure:")
}

#[derive(Clone, Copy, Default)]
struct GitStatusCounts {
    modified: usize,
    untracked: usize,
    deleted: usize,
    renamed: usize,
    added: usize,
    copied: usize,
    typechanged: usize,
    unmerged: usize,
    ignored: usize,
    other: usize,
}

impl GitStatusCounts {
    fn increment(&mut self, category: GitStatusCategory) {
        match category {
            GitStatusCategory::Modified => self.modified += 1,
            GitStatusCategory::Untracked => self.untracked += 1,
            GitStatusCategory::Deleted => self.deleted += 1,
            GitStatusCategory::Renamed => self.renamed += 1,
            GitStatusCategory::Added => self.added += 1,
            GitStatusCategory::Copied => self.copied += 1,
            GitStatusCategory::Typechanged => self.typechanged += 1,
            GitStatusCategory::Unmerged => self.unmerged += 1,
            GitStatusCategory::Ignored => self.ignored += 1,
            GitStatusCategory::Other => self.other += 1,
        }
    }

    fn total(self) -> usize {
        self.modified
            + self.untracked
            + self.deleted
            + self.renamed
            + self.added
            + self.copied
            + self.typechanged
            + self.unmerged
            + self.ignored
            + self.other
    }

    fn push_lines(self, reduced: &mut Vec<String>) {
        push_status_count(reduced, "modified", self.modified);
        push_status_count(reduced, "untracked", self.untracked);
        push_status_count(reduced, "deleted", self.deleted);
        push_status_count(reduced, "renamed", self.renamed);
        push_status_count(reduced, "added", self.added);
        push_status_count(reduced, "copied", self.copied);
        push_status_count(reduced, "typechanged", self.typechanged);
        push_status_count(reduced, "unmerged", self.unmerged);
        push_status_count(reduced, "ignored", self.ignored);
        push_status_count(reduced, "other", self.other);
    }
}

#[derive(Clone, Copy)]
enum GitStatusCategory {
    Modified,
    Untracked,
    Deleted,
    Renamed,
    Added,
    Copied,
    Typechanged,
    Unmerged,
    Ignored,
    Other,
}

fn push_status_count(reduced: &mut Vec<String>, label: &str, count: usize) {
    if count > 0 {
        reduced.push(format!("{label}: {count}"));
    }
}

fn git_status_category(line: &str) -> Option<GitStatusCategory> {
    let bytes = line.as_bytes();
    if bytes.len() < 3 || bytes[2] != b' ' {
        return None;
    }

    let first = bytes[0] as char;
    let second = bytes[1] as char;

    if first == '?' && second == '?' {
        return Some(GitStatusCategory::Untracked);
    }
    if first == '!' && second == '!' {
        return Some(GitStatusCategory::Ignored);
    }
    if first == 'R' || second == 'R' {
        return Some(GitStatusCategory::Renamed);
    }
    if first == 'D' || second == 'D' {
        return Some(GitStatusCategory::Deleted);
    }
    if first == 'A' || second == 'A' {
        return Some(GitStatusCategory::Added);
    }
    if first == 'C' || second == 'C' {
        return Some(GitStatusCategory::Copied);
    }
    if first == 'T' || second == 'T' {
        return Some(GitStatusCategory::Typechanged);
    }
    if first == 'U' || second == 'U' {
        return Some(GitStatusCategory::Unmerged);
    }
    if first == 'M' || second == 'M' {
        return Some(GitStatusCategory::Modified);
    }

    Some(GitStatusCategory::Other)
}

#[cfg(test)]
#[path = "output_reducer_tests.rs"]
mod tests;
