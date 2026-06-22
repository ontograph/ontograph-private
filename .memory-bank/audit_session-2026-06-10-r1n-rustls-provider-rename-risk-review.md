# R1N Rustls Provider Internal Crate Rename Risk Review

Date: 2026-06-10

Status: approved for one exact identity-only worker slice.

Decision:
- Approve `codex-utils-rustls-provider` -> `ontocode-utils-rustls-provider`.
- Scope is package name, crate/lib name, Bazel crate name, Rust imports, workspace dependency refs, lockfiles, and active direct references only.
- Do not rename or change `ensure_rustls_crypto_provider`, TLS provider behavior, auth behavior, CA loading, websocket behavior, proxy/MITM behavior, runtime package layout, public command names, telemetry, or persisted data.

Risk evidence:
- Direct Cargo dependents: `codex-api`, `ontocode-app-server-client`, `ontocode-app-server-transport`, `codex-client`, `ontocode-exec-server`, and `codex-network-proxy`.
- OntoIndex `ensure_rustls_crypto_provider`: CRITICAL, 73 impacted nodes, 13 direct callers, 18 modules.
- Impact includes auth, realtime websocket, remote control websocket, custom CA, exec-server remote transport, and network-proxy MITM/upstream flows.

Required verification:
- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-rustls-provider`
- `CARGO_BUILD_JOBS=8 just test -p codex-api`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport`
- `CARGO_BUILD_JOBS=8 just test -p codex-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-network-proxy`
- focused custom CA, websocket, proxy, and MITM tests if available.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- stale-reference search for `codex-utils-rustls-provider|codex_utils_rustls_provider` under active `ontocode-rs` sources.
- `git diff --check`
- OntoIndex `gn_verify_diff` covering changed files and executed tests.
