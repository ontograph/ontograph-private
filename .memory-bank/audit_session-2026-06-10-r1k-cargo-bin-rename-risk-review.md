# R1K Cargo Bin Utility Rename Risk Review

Date: 2026-06-10

Decision:
- Approve `codex-utils-cargo-bin` -> `ontocode-utils-cargo-bin` as the next exact R1 identity-only slice.
- Scope is limited to Cargo package identity, Rust lib crate identity, Bazel crate name, workspace dependency keys, dependent manifests/imports, active README refs, lockfiles, and verification metadata.
- No binary names, runfile resource names, public command names, runtime package layout, protocol/schema identities, telemetry, persisted-state, `.codex`, or `CODEX_*` rename authority is granted.

Evidence:
- Current metadata shows `codex-utils-cargo-bin` has 15 direct reverse dependency packages, mostly test harnesses and packages that spawn first-party binaries or locate runfiles/resources.
- Direct search found references in app-server protocol/schema fixtures, app-server tests/support, apply-patch tests, chatgpt tests, CLI tests, client CA tests, core test support/integration tests, exec tests, MCP server test support, RMCP client tests, stdio-to-uds tests, tools tests, and TUI tests.
- OntoIndex impact for `cargo_bin`: CRITICAL, 408 impacted nodes, 52 direct, primarily `Suite`, `Tests`, `V2`, `Config`, and `Unified_exec`.

Rejected alternatives:
- `codex-utils-path`: CRITICAL through cwd normalization, config writes, rollout/thread filters, shell runtime path prepends, and plugin config edits.
- `codex-utils-home-dir`: owns `ONTOCODE_HOME`/`CODEX_HOME` resolution and must not be automatic.
- `codex-utils-rustls-provider`: CRITICAL through TLS/auth/websocket/proxy/exec-server paths.
- `codex-utils-pty`: central to app-server command exec, core exec/unified-exec, exec-server, rmcp-client stdio, tools, and Windows sandbox.
- `codex-utils-string`: HIGH/CRITICAL through telemetry, metrics, tool previews, Windows sandbox, and TUI/app-server/exec process paths.
- `codex-utils-output-truncation`: CRITICAL through shell/tool-output/context/hook output paths.

Required R1K verification:
- `CARGO_BUILD_JOBS=8 cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-cargo-bin`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-apply-patch`
- `CARGO_BUILD_JOBS=8 just test -p codex-chatgpt`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-client`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-rmcp-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-stdio-to-uds`
- `CARGO_BUILD_JOBS=8 just test -p codex-tools`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH just bazel-lock-check`
- Stale-reference search for `codex-utils-cargo-bin` and `codex_utils_cargo_bin`.
- `git diff --check`
- OntoIndex `gn_verify_diff` scoped to R1K files and tests.
