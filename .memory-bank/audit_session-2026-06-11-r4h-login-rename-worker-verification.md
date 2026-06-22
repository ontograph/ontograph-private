# R4H Login Rename Worker Verification

Date: 2026-06-11

Scope:
- Implemented identity-only rename `codex-login` -> `ontocode-login`.
- Implemented Rust crate/import identity rename `codex_login` -> `ontocode_login`.
- Preserved existing directory path `ontocode-rs/login`.
- Preserved `codex-login.log` as an intentional support artifact compatibility string.

Preserved behavior:
- OAuth login/logout/status behavior.
- Token parsing/refresh behavior.
- Keyring/file auth-store behavior.
- Provider auth header behavior.
- Claude/Gemini/Copilot/OpenAI/native-provider auth behavior.
- Account-state behavior.
- Telemetry/product strings, env/config semantics, wire/generated names, and persisted state.

Verification:
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-login --no-tests=pass` passed: 118 passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass` passed: 50 passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client --no-tests=pass` passed: 64 passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-mcp auth --no-tests=pass` passed: 11 passed, 60 skipped.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-core auth --no-tests=pass` passed: 37 passed, 2625 skipped.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-core login --no-tests=pass` passed: 15 passed, 2647 skipped.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-app-server login --no-tests=pass` passed: 16 passed, 795 skipped.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-cli login --no-tests=pass` passed: 4 passed, 257 skipped.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-tui login --no-tests=pass` passed: 7 passed, 2769 skipped.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-update` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- `rg -n "\bcodex_login\b|\bcodex-login\b" ontocode-rs --glob "!target" || true` returned only `ontocode-rs/cli/src/login.rs` `codex-login.log` compatibility refs.
- `git diff --check` passed.
- OntoIndex CLI `detect-changes --repo codex --scope staged` passed on the temporarily staged 183-file R4H identity set; it reported CRITICAL risk due broad auth-flow import reach, with no behavior edits made.

Notes:
- MCP `gn_verify_diff` was not used because the MCP repo registry accepted only `OntoIndex` and did not resolve this repo's `AuthManager`; CLI verification used the indexed `/opt/demodb/_workfolder/ontocode` repo instead.
- The working tree had unrelated dirty files before this slice; temporary staging was cleared after scoped OntoIndex verification.
