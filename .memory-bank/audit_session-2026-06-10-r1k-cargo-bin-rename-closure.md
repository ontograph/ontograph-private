# R1K Cargo Bin Utility Rename Closure

Date: 2026-06-10

Scope:
- Renamed `codex-utils-cargo-bin` to `ontocode-utils-cargo-bin`.
- Updated Cargo package identity, Rust lib crate identity, Bazel crate name, workspace dependency names, direct dependent manifests/imports, crate-local README guidance, and lockfile identity.
- Preserved binary names, runfile resource names, public command names, runtime package layout, protocol/schema identities, telemetry names, `.codex`, persisted state, and `CODEX_*` compatibility.

Risk:
- OntoIndex impact for `cargo_bin`: CRITICAL.
- OntoIndex impact for `resolve_bazel_runfile`: LOW in worker review.
- OntoIndex impact for `resolve_cargo_runfile`: LOW in worker review.
- OntoIndex impact for `repo_root`: LOW in worker review.

Unblock:
- `ontocode-app-server-protocol` initially failed on stale generated schema fixtures for existing `MCP_OAUTH_CREDENTIALS`.
- R1K-U1 regenerated focused app-server protocol schema fixtures; `ontocode-app-server-protocol` then passed.

Verification:
- `CARGO_BUILD_JOBS=8 cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-cargo-bin --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-apply-patch`
- `CARGO_BUILD_JOBS=8 just test -p codex-chatgpt`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-client`
- `CARGO_BUILD_JOBS=8 just test -p codex-core` after one flaky retry timeout
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-rmcp-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-stdio-to-uds`
- `CARGO_BUILD_JOBS=8 just test -p codex-tools`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH just bazel-lock-check`
- Stale-reference search found zero `codex-utils-cargo-bin` or `codex_utils_cargo_bin` matches under `ontocode-rs`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff` passed.

Next:
- Remaining R1 candidates are blocked until a fresh exact-slice inventory and senior risk review selects one candidate.
