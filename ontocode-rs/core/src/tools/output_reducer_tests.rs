use super::reduce_exec_output;
use crate::tools::format_exec_output_for_model;
use crate::tools::format_exec_output_str;
use ontocode_protocol::exec_output::ExecToolCallOutput;
use ontocode_protocol::exec_output::StreamOutput;
use ontocode_utils_output_truncation::TruncationPolicy;
use std::borrow::Cow;
use std::time::Duration;

fn count_occurrences(text: &str, needle: &str) -> usize {
    text.match_indices(needle).count()
}

fn exec_output(content: &str) -> ExecToolCallOutput {
    ExecToolCallOutput {
        exit_code: 0,
        stdout: StreamOutput::new(content.to_string()),
        stderr: StreamOutput::new(String::new()),
        aggregated_output: StreamOutput::new(content.to_string()),
        duration: Duration::from_millis(420),
        timed_out: false,
    }
}

#[test]
fn returns_borrowed_input_unchanged() {
    let content = "line one\nline two";

    assert!(matches!(
        reduce_exec_output(content),
        Cow::Borrowed(val) if val == content
    ));
}

#[test]
fn shortens_successful_cargo_build_output() {
    let content = r#"Compiling demo v0.1.0 (/tmp/demo)
Compiling demo-tools v0.1.0 (/tmp/demo-tools)
Compiling demo-core v0.1.0 (/tmp/demo-core)
warning: unused import: `std::fmt::Debug`
warning: 1 warning emitted
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
"#;

    let reduced = reduce_exec_output(content);

    assert!(matches!(&reduced, Cow::Owned(_)));
    let reduced = reduced.as_ref();
    assert!(reduced.len() < content.len());
    assert!(reduced.contains("warning: 1 warning emitted"));
    assert!(
        reduced.contains("Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s")
    );
}

#[test]
fn preserves_compiler_error_file_path_and_message() {
    let content = r#"Compiling demo v0.1.0 (/tmp/demo)
error[E0425]: cannot find value `missing` in this scope
  --> src/main.rs:3:5
   |
3  |     missing();
   |     ^^^^^^^ not found in this scope
   |
   = help: a variable with a similar name exists: `message`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `demo` (bin "demo") due to 1 previous error
"#;

    let reduced = reduce_exec_output(content);

    assert!(matches!(&reduced, Cow::Owned(_)));
    let reduced = reduced.as_ref();
    assert!(reduced.len() < content.len());
    assert!(reduced.contains("error[E0425]: cannot find value `missing` in this scope"));
    assert!(reduced.contains("src/main.rs:3:5"));
    assert!(reduced.contains("missing();"));
    assert!(reduced.contains("could not compile `demo`"));
}

#[test]
fn preserves_failed_test_name_and_summary() {
    let content = r#"running 3 tests
test tests::adds_items ... ok
test tests::fails_with_missing_value ... FAILED

failures:

---- tests::fails_with_missing_value stdout ----
thread 'tests::fails_with_missing_value' panicked at src/lib.rs:42:9:
assertion failed: value.is_some()

failures:
    tests::fails_with_missing_value

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
"#;

    let reduced = reduce_exec_output(content);

    assert!(matches!(&reduced, Cow::Owned(_)));
    let reduced = reduced.as_ref();
    assert!(reduced.len() < content.len());
    assert!(reduced.contains("tests::fails_with_missing_value"));
    assert!(reduced.contains("test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s"));
}

#[test]
fn preserves_python_unittest_failure_traceback_and_summary() {
    let content = r#"test_example_failure (tests.test_demo.DemoTests) ... FAIL
FAIL: test_example_failure (tests.test_demo.DemoTests)
Traceback (most recent call last):
  File "/tmp/demo/tests/test_demo.py", line 17, in test_example_failure
    self.assertEqual(value, 3)
AssertionError: 2 != 3

Ran 1 test in 0.01s

FAILED (failures=1)
"#;

    let reduced = reduce_exec_output(content);

    assert!(matches!(&reduced, Cow::Owned(_)));
    let reduced = reduced.as_ref();
    assert!(reduced.len() < content.len());
    assert!(reduced.contains("test_example_failure"));
    assert!(reduced.contains("Traceback (most recent call last):"));
    assert!(
        reduced
            .contains(r#"File "/tmp/demo/tests/test_demo.py", line 17, in test_example_failure"#)
    );
    assert!(reduced.contains("FAILED (failures=1)"));
}

#[test]
fn shortens_python_unittest_dots_and_keeps_ok_line() {
    let content = r#"........................................................................
........................................................................
Ran 84 tests in 0.42s

OK
"#;

    let reduced = reduce_exec_output(content);

    assert!(matches!(&reduced, Cow::Owned(_)));
    let reduced = reduced.as_ref();
    assert!(reduced.len() < content.len());
    assert!(reduced.contains("Ran 84 tests in 0.42s"));
    assert!(reduced.contains("OK"));
    assert!(reduced.contains("dots"));
}

#[test]
fn shortens_long_rg_output_and_reports_omitted_matches() {
    let content = r#"/tmp/project/src/main.rs:10:first hit
/tmp/project/src/main.rs:11:second hit
/tmp/project/src/main.rs:12:3:third hit
/tmp/project/src/main.rs:13:fourth hit
/tmp/project/src/main.rs:14:fifth hit
/tmp/project/src/main.rs:15:sixth hit
/tmp/project/src/main.rs:16:seventh hit
/tmp/project/src/main.rs:17:eighth hit
/tmp/project/src/main.rs:18:ninth hit
/tmp/project/src/main.rs:19:tenth hit
"#;

    let reduced = reduce_exec_output(content);

    assert!(matches!(&reduced, Cow::Owned(_)));
    let reduced = reduced.as_ref();
    assert!(reduced.len() < content.len());
    assert!(reduced.contains("/tmp/project/src/main.rs:10:first hit"));
    assert!(reduced.contains("/tmp/project/src/main.rs:12:3:third hit"));
    assert!(reduced.contains("... (2 matches omitted)"));
    assert!(!reduced.contains("/tmp/project/src/main.rs:18:ninth hit"));
}

#[test]
fn shortens_large_git_status_output_and_reports_counts() {
    let content = r#" M src/lib.rs
M  src/main.rs
?? Cargo.lock
?? new_file.txt
D  old_file.txt
 D deleted_in_worktree.txt
R  old_name.rs -> new_name.rs
 R moved_in_worktree.rs -> moved.rs
A  src/new.rs
C  copied.rs -> copied_new.rs
T  src/changed_type.rs
"#;

    let reduced = reduce_exec_output(content);

    assert!(matches!(&reduced, Cow::Owned(_)));
    let reduced = reduced.as_ref();
    assert!(reduced.len() < content.len());
    assert!(reduced.contains("git status summary (11 paths):"));
    assert!(reduced.contains("modified: 2"));
    assert!(reduced.contains("untracked: 2"));
    assert!(reduced.contains("deleted: 2"));
    assert!(reduced.contains("renamed: 2"));
    assert!(reduced.contains("added: 1"));
    assert!(reduced.contains("copied: 1"));
    assert!(reduced.contains("typechanged: 1"));
}

#[test]
fn leaves_small_git_status_output_unchanged() {
    let content = r#" M src/lib.rs
?? Cargo.lock
D  old_file.txt
R  old_name.rs -> new_name.rs
"#;

    assert!(matches!(
        reduce_exec_output(content),
        Cow::Borrowed(val) if val == content
    ));
}

#[test]
fn leaves_non_status_text_unchanged() {
    let content = "status: ready\nnote: see also: details";

    assert!(matches!(
        reduce_exec_output(content),
        Cow::Borrowed(val) if val == content
    ));
}

#[test]
fn shortens_generic_long_log_and_keeps_edges_and_error_line() {
    let content = r#"boot: start
loading config
warming cache
ERROR: database connection lost
retrying connection
waiting for backoff
shutdown initiated
cleanup complete
boot: end
"#;

    let reduced = reduce_exec_output(content);

    assert!(matches!(&reduced, Cow::Owned(_)));
    let reduced = reduced.as_ref();
    assert!(reduced.len() < content.len());
    assert!(reduced.contains("boot: start"));
    assert!(reduced.contains("ERROR: database connection lost"));
    assert!(reduced.contains("boot: end"));
    assert!(reduced.contains("... (4 lines omitted)"));
}

#[test]
fn leaves_short_generic_log_unchanged() {
    let content = r#"boot: start
ERROR: database connection lost
boot: end
"#;

    assert!(matches!(
        reduce_exec_output(content),
        Cow::Borrowed(val) if val == content
    ));
}

#[test]
fn reducers_do_not_duplicate_token_like_text() {
    let fixtures = [
        (
            "generic log",
            r#"boot token-abc123: start
loading config
warming cache
ERROR: token-abc123 database connection lost
retrying connection
waiting for backoff
shutdown token-abc123 initiated
cleanup complete
boot token-abc123: end
"#,
        ),
        (
            "git status",
            r#" M src/token-abc123.rs
?? token-abc123.lock
D  old_token-abc123.rs
R  token-abc123_old.rs -> token-abc123_new.rs
 A src/other.rs
?? Cargo.lock
 D deleted.rs
M  src/lib.rs
A  src/main.rs
C  copied.rs -> copied_new.rs
T  src/changed_type.rs
"#,
        ),
        (
            "rg",
            r#"src/lib.rs:1:token-abc123
src/lib.rs:2:token-abc123
src/lib.rs:3:token-abc123
src/lib.rs:4:token-abc123
src/lib.rs:5:token-abc123
src/lib.rs:6:token-abc123
src/lib.rs:7:token-abc123
src/lib.rs:8:token-abc123
src/lib.rs:9:token-abc123
src/lib.rs:10:token-abc123
"#,
        ),
        (
            "python unittest",
            r#"test_token-abc123_feature (tests.test_demo.DemoTests) ... FAIL
FAIL: test_token-abc123_feature (tests.test_demo.DemoTests)
Traceback (most recent call last):
  File "/tmp/token-abc123/tests/test_demo.py", line 17, in test_token-abc123_feature
    self.assertEqual(value, 3)
AssertionError: 2 != 3

................
Ran 16 tests in 0.01s

FAILED (failures=1)
"#,
        ),
        (
            "rust success",
            r#"Compiling token-abc123-core v0.1.0 (/tmp/token-abc123-core)
Compiling token-abc123-cli v0.1.0 (/tmp/token-abc123-cli)
Compiling token-abc123-tools v0.1.0 (/tmp/token-abc123-tools)
warning: token-abc123 unused import: `std::fmt::Debug`
warning: 1 warning emitted
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
"#,
        ),
    ];

    for (name, content) in fixtures {
        let reduced = reduce_exec_output(content);
        assert!(matches!(&reduced, Cow::Owned(_)), "{name} did not reduce");
        let reduced = reduced.as_ref();
        assert!(reduced.len() < content.len(), "{name} did not shorten");
        let input_count = count_occurrences(content, "token-abc123");
        let output_count = count_occurrences(reduced, "token-abc123");

        assert!(
            output_count <= input_count,
            "{name} duplicated token-like text: input={input_count}, output={output_count}\n{reduced}"
        );
    }
}

#[test]
fn format_exec_output_truncates_after_reduction() {
    let content = "boot token-abc123: start\nloading config\nwarming cache\nERROR: token-abc123 database connection lost\nretrying connection\nwaiting for backoff\nshutdown token-abc123 initiated\ncleanup complete\nboot token-abc123: end\n";

    let exec_output = exec_output(content);
    let formatted = format_exec_output_str(&exec_output, TruncationPolicy::Tokens(10_000));
    let model_formatted =
        format_exec_output_for_model(&exec_output, TruncationPolicy::Tokens(10_000));

    assert!(formatted.len() < content.len());
    assert!(formatted.contains("... (4 lines omitted)"));
    assert!(formatted.contains("ERROR: token-abc123 database connection lost"));
    assert!(model_formatted.contains("Exit code: 0"));
    assert!(model_formatted.contains("Wall time: 0.4 seconds"));
    assert!(model_formatted.contains("... (4 lines omitted)"));
}
