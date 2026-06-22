# R5D API Rename Risk Review

Date: 2026-06-11

## Candidate

- Approved slice: `codex-api` -> `ontocode-api`.
- Approved crate import rename: `codex_api` -> `ontocode_api`.
- Keep directory path: `ontocode-rs/codex-api`.

## Why This Slice

- Remaining R5 direct scope:
- `codex-api`: 38 package refs, 334 crate refs.
- `codex-core`: 143 package refs, 775 crate refs.
- `codex-api` is the smaller remaining R5 slice, but it is model request/streaming sensitive.

## OntoIndex Evidence

- `ResponsesApiRequest`: HIGH risk, 11 impacted nodes, 5 direct, 4 modules affected.
- `SharedAuthProvider`: LOW risk, 0 impacted nodes.
- `ResponsesClient` struct: LOW risk, 0 impacted nodes.
- `ResponsesWebsocketClient`: ambiguous between struct and impl; treat as streaming-sensitive and require WebSocket/SSE/core coverage.

## Allowed Changes

- Rename Cargo package identity from `codex-api` to `ontocode-api`.
- Rename Rust crate identity/imports from `codex_api` to `ontocode_api`.
- Update Bazel crate identity, workspace dependency keys, lockfiles, and direct dependent imports/manifests.
- Update README/internal package-identity references only when they describe the crate identity.

## Preserved Surfaces

- Request/response schema semantics.
- Auth header behavior.
- Responses, Realtime, WebSocket, and SSE streaming behavior.
- Model/list/search/image/file upload behavior.
- Telemetry, retry, and error mapping behavior.
- Protocol/wire/generated names.
- Env/config names and persisted state.
- Existing `ontocode-rs/codex-api` directory path.

## Required Verification

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-api --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-core client --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-core responses --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-cli debug --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-backend-client --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-cloud-tasks --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Stale search for active `codex_api` and `codex-api` references under `ontocode-rs`.
- `git diff --check`.
- OntoIndex scoped diff verification if MCP is usable, otherwise CLI `detect-changes --repo codex`.

## Decision

- Dispatch one worker for R5D only.
- Do not dispatch `codex-core` until R5D is accepted.
