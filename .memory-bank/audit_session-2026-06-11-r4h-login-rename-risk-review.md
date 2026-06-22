# R4H Login Rename Risk Review

Date: 2026-06-11

Candidate:
- `codex-login` -> `ontocode-login`
- `codex_login` -> `ontocode_login`

OntoIndex evidence:
- Tool: OntoIndex CLI against repo `codex`.
- Indexed repo path: `/opt/demodb/_workfolder/ontocode`.
- `Struct:ontocode-rs/login/src/auth/manager.rs:AuthManager`: LOW impact, 0 upstream impacted nodes.

Direct inventory:
- `codex-login` package refs under `ontocode-rs`: 61.
- `codex_login` crate refs under `ontocode-rs`: 323.
- Scope is auth-sensitive despite LOW graph impact because dependents include provider auth, OAuth/token/keyring, CLI/app-server/core/RMCP/MCP, and config surfaces.

Approved scope:
- Identity-only Cargo package/lib/Bazel/import rename.
- Preserve existing `login` folder path.
- Preserve OAuth login/logout/status behavior.
- Preserve token parsing/refresh behavior.
- Preserve keyring/file auth-store behavior.
- Preserve provider auth header behavior.
- Preserve Claude/Gemini/Copilot/OpenAI/native-provider auth behavior.
- Preserve account-state behavior, telemetry/product strings, env/config semantics, wire/generated names, and persisted state.

Rejected scope:
- No auth runtime behavior changes.
- No token format, keyring key, config key, persisted-state, public command, or generated-wire rename.
- No broad find-and-replace outside active package/lib/Bazel/import references.

Required verification:
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp auth --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core auth --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for active `codex_login` / `codex-login` refs under `ontocode-rs`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.
