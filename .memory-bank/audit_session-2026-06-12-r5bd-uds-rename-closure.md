# R5BD UDS Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-uds` -> `ontocode-uds`.
- Accepted `codex_uds` -> `ontocode_uds`.
- Scope remained identity-only: package, library, Bazel target, Cargo lock, dependent imports, and README crate-name reference.

## Guardrails Preserved

- UnixStream and UnixListener API behavior.
- Private socket directory creation and permission behavior.
- Stale socket detection.
- App-server control socket path and startup lock behavior.
- App-server client and daemon remote-control UDS behavior.
- Stdio-to-uds bridge behavior.
- Windows `uds_windows` backing behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `uds` directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-uds --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport unix_socket`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-client --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-daemon --tests`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-stdio-to-uds --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Scoped stale-reference search for `codex_uds|codex-uds`: clean in `ontocode-rs`.
- Cargo metadata residual count: 17 `codex-*` packages.
- `git diff --check`: clean.
- OntoIndex `detect-changes --repo codex`: completed with the known broad high-risk dirty-tree envelope.

## Result

- R5BD accepted.
- Work completed on `gpt-5.4-mini` after Spark usage-limit fallback.
