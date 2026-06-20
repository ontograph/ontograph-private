# Claude Parked Row 189 Review

Date: 2026-06-20

## Decision

Row 189 stays parked.

## Source

- ADR row 189: `Partial | Non-core | DEFER | Version checks are release engineering.`
- Donor row 189: `Add package-npm script with artifact checks. | npm packaging | Prevents broken packages. | Dry-run package test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-cli/scripts/build_npm_package.py` already stages package contents, runs `npm pack --json`, parses pack output, verifies the tarball exists, and moves it to the requested `--pack-output`.
- `scripts/stage_npm_packages.py` already expands packages, installs native artifacts, and invokes `build_npm_package.py` with `--pack-output` for each staged package.
- `.github/workflows/ci.yml` already runs npm package staging and uploads the staged tarball artifact.
- `.github/workflows/rust-release.yml` already stages npm packages during release and checks for missing required platform/root tarballs before publishing.
- `ontocode-cli/scripts/README.md` documents `stage_npm_packages.py` as the release path and direct `build_npm_package.py` use as package-specific debugging.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
