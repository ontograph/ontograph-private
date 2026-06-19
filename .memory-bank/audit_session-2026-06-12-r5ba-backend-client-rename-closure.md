# R5BA Backend Client Rename Closure

Date: 2026-06-12

Scope:
- Accepted `codex-backend-client` -> `ontocode-backend-client`.
- Accepted `codex_backend_client` -> `ontocode_backend_client`.
- Identity-only package/lib/Bazel/import rename; existing `backend-client` directory path is preserved.

Guardrails:
- Preserved backend HTTP request paths, auth/header/user-agent/account/FedRAMP/path-style behavior, task list/details/create behavior, config bundle response behavior, rate-limit mapping, and add-credits nudge behavior.
- Preserved cloud-config backend behavior, cloud-tasks-client HTTP behavior, app-server request processor behavior, and memories-write guard behavior.
- Preserved env/config/wire/generated names, telemetry/product strings, and persisted state.

Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-backend-client --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cloud-tasks-client --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-write --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_backend_client|codex-backend-client`: clean.
- Cargo metadata residual `codex-*` package count: 20.
- `git diff --check`: clean.
- OntoIndex `detect-changes --repo codex`: known broad dirty-tree high-risk report remains; no new R5BA-specific blocker found.

Notes:
- Work completed on fallback `gpt-5.4-mini` after `gpt-5.3-codex-spark` usage-limit fallback.
