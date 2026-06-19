# Alpha Release Readiness Baseline

Date: 2026-06-13

## Scope

- Reviewed release/version surfaces for the first alpha cut.
- Reused existing release staging architecture instead of forcing a branch-wide manifest bump.
- Patched the native Copilot runtime so shipped header metadata no longer hardcodes `0.0.0`.

## Findings

- Source manifests intentionally use placeholder versions for dev/source builds.
- Release tooling already supports explicit version injection during staging.
- The highest-signal runtime defect was `Copilot` sending hardcoded `0.0.0` header versions.
- `Claude OAuth live validation` remains the only known non-version release blocker in the current tracked plan.

## Actions Taken

- Updated `ontocode-rs/core/src/native_provider/copilot.rs` to derive release headers from `env!("CARGO_PKG_VERSION")`.
- Added header coverage in `ontocode-rs/core/src/native_provider/copilot_tests.rs`.
- Added `ALPHA_RELEASE_READINESS.md` documenting accepted prep decisions, owners, blockers, and cut checklist.
