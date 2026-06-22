# R2B-B Exec Rename Risk Review

Date: 2026-06-10

Scope:
- Approve only `ontocode-exec` -> `ontocode-exec` package/lib/Bazel/import identity rename.
- Preserve the existing `ontocode-exec` binary name, CLI behavior, app-server in-process behavior, exec session behavior, config loading, resume/review behavior, output schema behavior, telemetry, env/config semantics, runtime layout, and persisted state.

OntoIndex impact:
- `run_main` in `ontocode-rs/exec/src/lib.rs`: LOW, 0 impacted nodes.
- Direct inventory overrides weak graph impact because this crate owns the headless exec CLI/runtime path used by `codex-cli`.

Direct Cargo dependents:
- `codex-cli`

Required verification:
- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- Focused exec/resume/review/output-schema/config tests where available.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale package/lib reference search for `ontocode-exec` and `codex_exec`, excluding `ontocode-exec-server`, `ontocode-execpolicy`, and intentional historical docs.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.

Decision:
- Approved as R2B-B, identity-only.
- Do not proceed to `arg0`, `exec-server`, or `sandboxing` until R2B-B is accepted.
