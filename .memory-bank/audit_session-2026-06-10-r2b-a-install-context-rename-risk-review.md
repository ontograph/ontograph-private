# R2B-A Install Context Rename Risk Review

Date: 2026-06-10

Scope:
- Approve only `codex-install-context` -> `ontocode-install-context` package/lib/Bazel/import identity rename.
- Preserve package-layout behavior, `codex-package.json`, `codex-path`, `codex-resources`, bundled resource names, standalone package detection, npm/bun/brew detection, update-action behavior, doctor output semantics, `rg` lookup behavior, SDK/package-builder names, env/config semantics, telemetry, and persisted state.

OntoIndex impact:
- `InstallMethod`: LOW but traversal partial due read-only pool adapter warning.
- `InstallContext.bundled_resource`: LOW but traversal partial.
- `InstallContext.rg_command`: LOW but traversal partial.
- `InstallContext` and `CodexPackageLayout` require UID disambiguation in the index.
- Direct inventory overrides weak graph impact because this crate owns package layout and bundled resource discovery.

Direct Cargo dependents:
- `codex-arg0`
- `codex-cli`
- `codex-core`
- `codex-thread-store`
- `ontocode-tui`
- `ontocode-linux-sandbox`

Required verification:
- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-install-context`
- `CARGO_BUILD_JOBS=8 just test -p codex-arg0`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p codex-thread-store`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox`
- Focused install-context, doctor runtime/update, package-layout, bundled-bwrap, thread search, and update-action tests where available.
- `python3 scripts/codex_package/test_cargo.py`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale package/lib reference search for `codex-install-context` and `codex_install_context`, excluding intentional persisted/public package-layout names if any are explicitly justified.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.

Decision:
- Approved as R2B-A, identity-only.
- Do not proceed to `arg0`, `exec`, `exec-server`, or `sandboxing` until R2B-A is accepted.
