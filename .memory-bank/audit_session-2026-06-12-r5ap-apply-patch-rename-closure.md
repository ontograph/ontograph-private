# R5AP Apply Patch Rename Closure

Date: 2026-06-12

## Closure

Accepted R5AP: `codex-apply-patch` -> `ontocode-apply-patch` and `codex_apply_patch` -> `ontocode_apply_patch`.

The rename stayed identity-only across Cargo package metadata, Rust lib crate name, Bazel crate identity, workspace dependencies, and direct imports.

## Preserved Surfaces

- Public `apply_patch` binary name.
- Public `apply_patch` tool/protocol/schema/prompt names.
- `CODEX_CORE_APPLY_PATCH_ARG1` constant name and `--codex-run-as-apply-patch` compatibility value.
- Patch parsing, verification, application, runtime routing, sandboxing, shell interception, env/config/wire/generated names, telemetry/product strings, persisted state, and `ontocode-rs/apply-patch` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-apply-patch --no-tests=pass`: passed, 84 tests.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0 --no-tests=pass`: passed, 8 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec --no-tests=pass`: passed, 122 tests.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed, no effective lock drift.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active-source stale-reference search for `codex_apply_patch|codex-apply-patch`: clean.
- Cargo metadata residual `codex-*` package count: 31.
- `git diff --check`: clean.
- OntoIndex `detect-changes --repo codex`: high risk from the known broad dirty tree, not a new apply-patch-specific blocker.

## Model Fallback

Worker and manager recorded fallback use of `gpt-5.4-mini` because `gpt-5.3-codex-spark` was unavailable or usage-limited.
