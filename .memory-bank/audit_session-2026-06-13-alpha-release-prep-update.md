---
name: Alpha Release Prep Update
description: Narrow the alpha-release work to final version choice, staged artifact verification, and the remaining Claude OAuth evidence gate
type: audit
date: 2026-06-13
status: active
---

# Alpha Release Prep Update

## Outcome

- Confirmed the existing staging scripts already support explicit version injection for npm and Python artifacts.
- Kept the source-build sentinel policy intact: workspace/package source manifests remain on `0.0.0` or dev placeholders until release staging injects a concrete version.
- Recorded `0.1.0-alpha.1` as the default first-alpha candidate, pending final release-manager confirmation.
- Narrowed alpha-release execution to artifact verification plus the existing `Claude OAuth` live-validation blocker.

## Verified Release Version Injection Owners

- npm and staged CLI packages
  - `scripts/stage_npm_packages.py --release-version <version>`
  - `codex-cli/scripts/build_npm_package.py --release-version <version>`
- Python SDK and runtime
  - `sdk/python/scripts/update_sdk_artifacts.py stage-sdk --sdk-version <version>`
  - `sdk/python/scripts/update_sdk_artifacts.py stage-runtime --codex-version <version>`

## Remaining Evidence Gates

- Final alpha version confirmation.
- Local/staged `ontocode` artifact verification under release build.
- Real redacted `CLAUDE_OAUTH_REDACTED_SAMPLE` for live validation.
