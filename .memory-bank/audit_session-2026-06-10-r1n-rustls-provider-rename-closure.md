# R1N Rustls Provider Internal Crate Rename Closure

Date: 2026-06-10

Status: accepted.

Scope:
- Renamed `codex-utils-rustls-provider` to `ontocode-utils-rustls-provider`.
- Renamed Rust crate imports from `codex_utils_rustls_provider` to `ontocode_utils_rustls_provider`.
- Updated workspace dependency refs, direct dependent manifests/imports, Cargo lockfile, Bazel crate name, and Bazel lockfile.
- Preserved `ensure_rustls_crypto_provider`, TLS provider behavior, auth behavior, CA loading, websocket behavior, proxy/MITM behavior, runtime package layout, public command names, telemetry, env/config semantics, and persisted data.

Verification:
- `cargo metadata --format-version 1 --no-deps`: passed; metadata exposes `ontocode-utils-rustls-provider` and no `codex-utils-rustls-provider`.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-rustls-provider`: compiled then exited 4 because the package has no tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-rustls-provider --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-api`: passed, 124 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-client`: passed, 26 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport`: passed, 105 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-client`: passed, 27 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server`: passed, 196 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-network-proxy`: passed, 165 passed.
- Focused custom CA tests: passed, 10 passed.
- Focused realtime websocket tests: passed, 6 passed.
- Focused remote-control websocket tests: passed, 6 passed.
- Focused proxy/MITM/SOCKS tests: passed, 41 passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Stale-reference search for `codex-utils-rustls-provider|codex_utils_rustls_provider` under active `ontocode-rs` sources: passed, 0 matches.
- `git diff --check`: passed.
- Scoped OntoIndex `gn_verify_diff`: passed.
