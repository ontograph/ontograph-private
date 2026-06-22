# Oh My Pi Row 81 Coverage Closeout

Date: 2026-06-19

Scope: ADR row 81, context overflow retry tests around `ontocode-rs/core/src/session/turn.rs` and existing compact/session tests.

Outcome: covered

Evidence:
- `ontocode-rs/core/tests/suite/compact.rs::manual_compact_retries_after_context_window_error` mounts a `context_length_exceeded` compact failure followed by a successful retry.
- The test asserts the retry drops exactly one history item and that the compact warning is emitted once.

Verification:
- `cd ontocode-rs && TMPDIR="$(mktemp -d /var/tmp/ontocode-core.XXXXXX)" CARGO_BUILD_JOBS=8 just test -p ontocode-core manual_compact_retries_after_context_window_error`

Notes:
- No Rust source changes were needed for this row.
