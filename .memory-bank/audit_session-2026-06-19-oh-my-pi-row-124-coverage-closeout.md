# Oh My Pi Row 124 Coverage Closeout

Date: 2026-06-19

Outcome:
- ADR row 124 was closed as already covered by existing tests in `ontocode-rs/codex-mcp/src/connection_manager_tests.rs`.
- No Rust source changes were required.

Evidence:
- `no_local_runtime_fails_local_stdio_but_keeps_local_http_server` asserts `wait_for_server_ready("stdio", ...)` is false for the failed local stdio server.
- The same test asserts `required_startup_failures(&["stdio".to_string()])` returns the expected local stdio error string.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp` passed: 71 tests run, 71 passed.

OntoIndex:
- Context and impact requests for `McpConnectionManager::required_startup_failures` could not resolve the local symbol store in this session, so no additional blast radius was recorded.
