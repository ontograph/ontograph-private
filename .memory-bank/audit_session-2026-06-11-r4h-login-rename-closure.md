# R4H Login Rename Closure

Date: 2026-06-11

Scope:
- Accepted identity-only rename `codex-login` -> `ontocode-login`.
- Accepted crate/lib import rename `codex_login` -> `ontocode_login`.
- Preserved existing `login` folder path, OAuth login/logout/status behavior, token parsing/refresh behavior, keyring/file auth-store behavior, provider auth header behavior, Claude/Gemini/Copilot/OpenAI/native-provider auth behavior, account-state behavior, telemetry/product strings, env/config semantics, wire/generated names, persisted state, and `codex-login.log`.

Manager verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core auth --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n "\\bcodex_login\\b|\\bcodex-login\\b" ontocode-rs --glob "!target" || true`
- `git diff --check`
- OntoIndex CLI `detect-changes --repo codex` completed; broad dirty-tree report is medium risk. Worker also ran scoped staged OntoIndex CLI `detect-changes` on the R4H identity file set and it passed with expected auth-flow reach.

Notes:
- Worker verification already covered fmt, model-provider, RMCP client, MCP auth, CLI login, TUI login, Bazel lock update/check, stale-reference classification, and scoped OntoIndex CLI detection.
- No active `codex_login` refs remain.
- Remaining `codex-login` refs intentionally preserve `codex-login.log`.
- Remaining R4 crate is `codex-config`, which requires fresh one-slice senior risk review before dispatch.
