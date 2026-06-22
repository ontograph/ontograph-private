# Claude Parked Row 190 Review

Date: 2026-06-20

## Decision

Row 190 stays parked.

## Source

- ADR row 190: `Partial | Non-core | DEFER | Dependency audit workflow is useful but not donor core.`
- Donor row 190: `Add CI build script wrapper. | CI | Keeps local/CI commands aligned. | CI dry-run.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `scripts/build_codex_package.py` is already a stable executable wrapper over the package builder.
- `scripts/stage_npm_packages.py` is already the CI-facing npm staging wrapper and reuses the same npm build module instead of creating a second build path.
- `.github/scripts/run-bazel-ci.sh` already wraps Bazel CI invocation, local fallback behavior, platform config, failure summaries, and test-log reporting.
- `.github/workflows/bazel.yml` and `.github/workflows/ci.yml` already call those wrappers in CI.
- `justfile` already exposes local recipes for GitHub script tests and release binary builds.
- Dependency-audit checks already live in the cargo-deny and cargo-shear workflows.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
