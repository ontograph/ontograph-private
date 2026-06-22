# R5BA Backend Client Rename Risk Review

Date: 2026-06-12

Decision:
- Dispatch `codex-backend-client` -> `ontocode-backend-client`.
- Dispatch `codex_backend_client` -> `ontocode_backend_client`.
- Scope is identity-only package/lib/Bazel/import rename.

OntoIndex:
- `Struct:ontocode-rs/backend-client/src/client.rs:Client`: LOW, 1 impacted node, no affected processes.
- `Enum:ontocode-rs/backend-client/src/client.rs:RequestError`: LOW, 0 impacted nodes, no affected processes.
- Backend API method UIDs such as `get_config_bundle` and `list_tasks` are not indexed individually; direct inventory is the source of truth for dependent refs.

Direct Inventory:
- Root workspace dependency metadata.
- `backend-client` manifest and Bazel crate identity.
- App-server request processor imports.
- Cloud-config backend/service-test imports.
- Cloud-tasks-client HTTP imports.
- Memories-write guard imports.
- Cargo lock entries.

Guardrails:
- Preserve backend HTTP request paths, auth/header/user-agent/account/FedRAMP/path-style behavior, task list/details/create behavior, config bundle response behavior, rate-limit mapping, and add-credits nudge behavior.
- Preserve cloud-config backend behavior, cloud-tasks-client HTTP behavior, and memories-write guard behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `backend-client` directory path.

Required Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-backend-client --no-tests=pass`
- Focused app-server backend/request-processor tests if discoverable, otherwise `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cloud-tasks-client --tests` or current package target if not renamed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-write --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_backend_client|codex-backend-client`.
- Cargo metadata residual count.
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

Model:
- Dispatch on `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.
