# R5AP Apply Patch Rename Risk Review

Date: 2026-06-12

## Decision

Dispatch R5AP as an identity-only residual package rename:

- `codex-apply-patch` -> `ontocode-apply-patch`
- `codex_apply_patch` -> `ontocode_apply_patch`

## OntoIndex Impact

- `Function:ontocode-rs/apply-patch/src/lib.rs:apply_patch`: LOW, 1 impacted caller, no affected processes.
- `Function:ontocode-rs/apply-patch/src/invocation.rs:maybe_parse_apply_patch_verified`: LOW, 5 impacted callers, no affected processes.
- `Function:ontocode-rs/apply-patch/src/invocation.rs:verify_apply_patch_args`: LOW, 5 impacted callers, 1 affected process in the core apply-patch handler path.
- `CODEX_CORE_APPLY_PATCH_ARG1`: searched by symbol name; LOW impact by name and preserved unchanged.

## Scope

Allowed:

- Rename Cargo package metadata, Rust lib crate name, Bazel crate identity, dependency entries, and Rust imports from old crate identity to Ontocode identity.
- Update lockfiles and generated Bazel lock metadata as required.

Forbidden:

- Do not rename the public `apply_patch` binary.
- Do not rename model/tool/schema/protocol strings named `apply_patch`.
- Do not rename the compatibility argument `--codex-run-as-apply-patch`.
- Do not change patch parsing, verification, application, runtime routing, sandboxing, shell interception, or core handler behavior.
- Do not move the existing `ontocode-rs/apply-patch` directory.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-apply-patch --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0 --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_apply_patch|codex-apply-patch`
- Cargo metadata residual package count
- `git diff --check`
- OntoIndex diff verification through `detect-changes --repo codex`

## Model Fallback

Dispatch uses `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.
