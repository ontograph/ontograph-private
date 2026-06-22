# R5BD UDS Rename Worker Verification

Date: 2026-06-12

## Summary

- Renamed `codex-uds` to `ontocode-uds` and `codex_uds` to `ontocode_uds` as an identity-only slice.
- Preserved UDS socket/runtime behavior, app-server transport/client/daemon behavior, stdio-to-uds bridge behavior, Windows `uds_windows` backing behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `uds` directory path.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-uds --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport unix_socket`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-client --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-daemon --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-stdio-to-uds --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Scoped stale-reference search for `codex_uds|codex-uds`
- `cargo metadata --format-version 1 --no-deps` residual count: 17 remaining `codex-*` packages
- `git diff --check`
- `OntoIndex detect-changes --repo codex`
