# Alpha 0.1.0-alpha.5 Release Prep

Date: 2026-06-26
Status: prepared, not published

## Decision

Use `0.1.0-alpha.5` as the next private alpha candidate.

Private release state checked during prep:

- `gh release list --repo ontograph/ontograph-private --limit 10` shows published private releases through `0.1.0-alpha.3`.
- `git ls-remote --tags ontograph 'rust-v0.1.0-alpha.*'` shows private tags through `rust-v0.1.0-alpha.3`.
- Local tag names `rust-v0.1.0-alpha.3` and `rust-v0.1.0-alpha.4` are occupied by fetched upstream/OpenAI tag objects in this mixed-remote checkout, so `0.1.0-alpha.5` is the safe next private candidate.

## Changes

- Added `release-notes-v0.1.0-alpha.5.md`.
- Updated `.github/workflows/private-alpha-release.yml` to use `release-notes-v<version>.md` as GitHub prerelease notes when the matching file exists.
- Updated `.memory-bank/ALPHA_RELEASE_READINESS.md`, `.memory-bank/project_plan-current.md`, and `.memory-bank/project_pending-tasks.md` to point at alpha.5 and preserve the no-source-version-bump policy.

## Verification

Passed:

- `git diff --check -- .github/workflows/private-alpha-release.yml .memory-bank/ALPHA_RELEASE_READINESS.md release-notes-v0.1.0-alpha.5.md README.md scripts/install/install.sh`
- `sh -n scripts/install/install.sh`
- `bash -n -c '<private-alpha publish step shell>'`
- workflow version-resolution shell for tag `rust-v0.1.0-alpha.5`
- workflow version-resolution shell for manual input `rust-v0.1.0-alpha.5`
- `python3 scripts/stage_npm_packages.py --help`
- `python3 sdk/python/scripts/update_sdk_artifacts.py --help`
- `CARGO_BUILD_JOBS=8 cargo metadata --manifest-path ontocode-rs/Cargo.toml --no-deps --format-version 1`

## Not Done

- No GitHub release was created.
- No tag was created or pushed.
- No full release build was run in this prep slice.
- The checkout remains heavily dirty with unrelated work; stage only release-prep files if cutting the alpha.
