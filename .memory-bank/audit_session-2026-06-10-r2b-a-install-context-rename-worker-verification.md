# R2B-A Install Context Rename Worker Verification

Date: 2026-06-10

Scope:
- Implemented only `codex-install-context` -> `ontocode-install-context` package/lib/Bazel/import identity rename.
- Preserved package-layout behavior, `codex-package.json`, `codex-path`, `codex-resources`, bundled resource names, npm/bun/brew detection, update-action behavior, doctor output semantics, `rg` lookup behavior, SDK/package-builder names, env/config semantics, telemetry, runtime layout, and persisted state.

Changed surfaces:
- Cargo package and lib crate identity.
- Root workspace dependency key.
- Direct dependent manifest keys and Rust imports in arg0, CLI, core, thread-store, TUI, and Linux sandbox.
- Bazel `crate_name`.
- Cargo and Bazel lockfiles after regeneration.

Verification completed:
- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-install-context`
- `CARGO_BUILD_JOBS=8 just test -p codex-arg0`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p codex-thread-store`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox`
- Focused package-layout, `rg` fallback, doctor update/report, thread list/search, update-action, and bundled-bwrap filters.
- `python3 scripts/codex_package/test_cargo.py`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active source and lockfile stale-reference search for `codex-install-context` and `codex_install_context`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`

Notes:
- `codex-core` passed with one nextest flaky retry that passed.
- Historical memory-bank records still mention old names intentionally.
- Status is worker verification complete; manager acceptance remains the next gate before any follow-on R2B slice.
