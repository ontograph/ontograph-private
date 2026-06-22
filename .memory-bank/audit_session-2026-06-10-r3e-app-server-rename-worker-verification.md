# R3E App Server Rename Worker Verification

Date: 2026-06-10

## Scope

- Renamed Cargo package identity `ontocode-app-server` to `ontocode-app-server`.
- Renamed lib crate identity/imports `codex_app_server` to `ontocode_app_server` where they refer to the app-server package/library identity.
- Updated workspace metadata, app-server manifest/lib/Bazel crate name, direct CLI/app-server-client/test imports, `Cargo.lock`, and nextest package selector.

## Preserved

- `ontocode-app-server` binary name.
- `ontocode-app-server-test-notify-capture` binary name.
- `ontocode-app-server-protocol`, `ontocode-app-server-client`, `ontocode-app-server-daemon`, `ontocode-app-server-transport`, and `ontocode-app-server-test-client` package names.
- JWT audience strings, socket/runtime behavior, remote-control behavior, config warning behavior, skills warning behavior, CLI/TUI behavior, telemetry semantics, env/config semantics, and persisted state.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server` passed: 810 passed, 1 skipped; bench smoke completed.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server` passed: 25 passed, 236 skipped; bench smoke completed.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli remote_control_cmd` passed: 8 passed, 253 skipped; bench smoke completed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass` passed: 2772 passed, 4 skipped; bench smoke completed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- Active stale-reference search found only intentional binary/JWT/telemetry/docs/comment/test-client local variable references.
- `git diff --check` passed.
- Scoped OntoIndex `gn_verify_diff` passed for the R3E changed files and required tests.

## Notes

- `ctx_shell` could not run `just` because the local lean-ctx shell allowlist blocks it; verification ran through `lean-ctx -c`.
- Bazel lock update emitted existing well-known crate annotation warnings only.
