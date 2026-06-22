# R5BD UDS Rename Risk Review

Date: 2026-06-12

## Decision

- Approve exactly one residual slice: `codex-uds` -> `ontocode-uds`.
- Approve crate import rename: `codex_uds` -> `ontocode_uds`.
- Scope is identity-only: package metadata, library crate name, Bazel crate name, Cargo lock, dependent imports, and README crate-name reference.

## OntoIndex Impact

- Exact `prepare_private_socket_directory`: HIGH, 11 impacted nodes, 4 direct, 3 affected modules, 2 affected processes.
- Exact `is_stale_socket_path`: HIGH, 7 impacted nodes, 1 direct, 3 affected modules, 2 affected processes.
- `UnixStream` and `UnixListener` resolved with unknown graph risk but direct inventory shows bounded import-only usage.

## Direct Active References

- Root workspace dependency metadata.
- `uds` manifest and Bazel identity.
- `app-server-client` imports.
- `app-server-daemon` imports.
- `app-server-transport` imports and unix-socket tests.
- `stdio-to-uds` imports, tests, and README crate-name reference.
- Cargo lock entries.

## Guardrails

- Preserve UnixStream and UnixListener API behavior.
- Preserve private socket directory creation and permission behavior.
- Preserve stale socket detection.
- Preserve app-server control socket path and startup lock behavior.
- Preserve app-server client and daemon remote-control UDS behavior.
- Preserve stdio-to-uds bridge behavior.
- Preserve Windows `uds_windows` backing behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `uds` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-uds --no-tests=pass`.
- Focused `ontocode-app-server-transport` unix-socket tests.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-client --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-daemon --tests`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-stdio-to-uds --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Scoped stale-reference search for `codex_uds|codex-uds`.
- Metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
