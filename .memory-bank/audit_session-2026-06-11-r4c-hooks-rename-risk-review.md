# R4C Hooks Rename Risk Review

Date: 2026-06-11

## Decision

- Approve exactly one identity-only slice: `codex-hooks` -> `ontocode-hooks` and `codex_hooks` -> `ontocode_hooks`.
- Treat as HIGH risk because hook key generation, hook runtime dispatch, plugin hook declarations, app-server hook catalog, and core session/tool flows are behavior-sensitive.

## OntoIndex Evidence

- CLI index status: `/opt/demodb/_workfolder/ontocode` is indexed and up to date at commit `73ba304`.
- MCP facade defaulted to repository id `OntoIndex`, so R4C impact evidence uses the CLI command with `--repo codex`.
- `hook_key`: HIGH impact, 16 upstream impacted nodes, direct callers include hook discovery and plugin hook declarations.
- `execute_handlers`: MEDIUM impact, 12 upstream impacted nodes, direct callers include pre/post tool, permission, compact, session-start, user-prompt-submit, and stop hook run paths.
- `Hooks::dispatch`: LOW impact, 3 upstream impacted nodes through legacy after-agent hook dispatch and session turn runtime.

## Direct Inventory

- `ontocode-rs/hooks/Cargo.toml`: package and lib crate identity.
- `ontocode-rs/hooks/BUILD.bazel`: Bazel crate identity.
- `ontocode-rs/Cargo.toml`: workspace dependency key.
- Direct dependent manifests: `ontocode-rs/core/Cargo.toml`, `ontocode-rs/core-plugins/Cargo.toml`, `ontocode-rs/app-server/Cargo.toml`, `ontocode-rs/external-agent-migration/Cargo.toml`.
- Active Rust imports/usages are concentrated in core hook runtime/session/tool paths, app-server hook catalog/config tests, core-plugins plugin hook declarations, and external-agent-migration hook event names.

## Allowed Changes

- Cargo package name, Rust crate name, Bazel crate name, workspace/dependent manifest keys, Cargo/Bazel lockfiles, and active Rust imports/selectors.

## Forbidden Changes

- Hook JSON/config schema semantics.
- Persisted hook-state key strings produced by `hook_key`.
- Deprecated `[features].codex_hooks` compatibility behavior or diagnostics.
- Hook matcher semantics.
- Trust/hash behavior.
- Hook command execution behavior.
- Stop-loop behavior.
- Plugin hook declaration behavior.
- App-server hook catalog behavior.
- Telemetry/product strings.
- Env/config semantics.
- Wire/generated names.
- Persisted state.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-hooks --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core hooks --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core hook --no-tests=pass` if useful for wider hook-runtime coverage.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server hooks --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-plugins --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-external-agent-migration --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale-reference search for `codex-hooks` and `codex_hooks`; classify intentional deprecated feature-key/test/docs compatibility refs.
- `git diff --check`
- OntoIndex scoped `gn_verify_diff` or CLI `detect-changes`.
