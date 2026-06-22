# R5C Client Rename Risk Review

Date: 2026-06-11

## Candidate

- Approved slice: `codex-client` -> `ontocode-client`.
- Approved crate import rename: `codex_client` -> `ontocode_client`.
- Keep directory path: `ontocode-rs/codex-client`.

## Why This Slice

- Remaining R5 direct scope counts:
- `codex-client`: 24 package refs, 116 crate refs, 46 files.
- `codex-api`: 38 package refs, 334 crate refs, 105 files.
- `codex-core`: 144 package refs, 775 crate refs, 281 files.
- `codex-client` is the smallest remaining core/shared slice, but it is network/auth sensitive.

## OntoIndex Evidence

- `HttpTransport`: MEDIUM risk, 11 impacted nodes, 8 direct, 1 module affected.
- `ReqwestTransport` struct: LOW risk, 0 impacted nodes.
- `build_reqwest_client_with_custom_ca`: CRITICAL risk, 35 impacted nodes, 10 direct, 13 modules affected.
- Direct affected areas include login auth client construction, exec-server HTTP client construction, `codex-api` file upload, cloud-tasks environment detection, backend-client construction, app-server account flows, and core MCP file upload.

## Allowed Changes

- Rename Cargo package identity from `codex-client` to `ontocode-client`.
- Rename Rust crate identity/imports from `codex_client` to `ontocode_client`.
- Update Bazel crate identity, workspace dependency keys, lockfiles, and direct dependent imports/manifests.
- Update README/internal package-identity references only when they describe the crate identity.

## Preserved Surfaces

- HTTP transport behavior.
- Custom CA, TLS root, proxy, and cookie behavior.
- Retry/backoff behavior.
- SSE and streaming behavior.
- Request/response telemetry semantics.
- Auth/login/client construction behavior.
- Env/config/wire names and persisted state.
- Test-only `custom_ca_probe` behavior.
- Existing `ontocode-rs/codex-client` directory path.

## Required Verification

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-client --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-api --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-backend-client --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-cloud-tasks --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-login --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Stale search for active `codex_client` and `codex-client` references under `ontocode-rs`.
- `git diff --check`.
- OntoIndex scoped diff verification if MCP is usable, otherwise CLI `detect-changes --repo codex`.

## Decision

- Dispatch one worker for R5C only.
- Do not dispatch `codex-api` or `codex-core` until R5C is accepted.
