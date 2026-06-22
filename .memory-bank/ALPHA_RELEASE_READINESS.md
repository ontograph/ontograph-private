---
name: Alpha Release Readiness
description: Release-prep baseline for the first Ontocode/Codex alpha cut without breaking source-build sentinels
type: release_plan
date: 2026-06-13
status: active
---

# Alpha Release Readiness

## Goal

Prepare the repository for an alpha release cut while preserving the existing `0.0.0` source-build sentinel used by local-dev and non-release workflows.

## Current Baseline

- Release staging already supports explicit versions through release scripts instead of requiring permanent source-manifest bumps.
- Installer validation already accepts prerelease inputs such as `x.y.z-alpha` and `x.y.z-alpha.N`.
- The main runtime leak found during review was the native Copilot provider sending hardcoded `0.0.0` header metadata.
- Python staging already supports explicit version injection through `sdk/python/scripts/update_sdk_artifacts.py`.

## Accepted Prep Decisions

- Keep `[workspace.package].version = "0.0.0"` in source until an actual release branch/tag cut.
- Keep npm/Python source manifests on their dev placeholders unless the release process explicitly requires a committed version bump.
- Use release tooling to inject the concrete alpha version at staging time.
- Remove hardcoded runtime placeholders that would survive a release cut and leak incorrect metadata.
- Use `0.1.0-alpha.1` as the default first-alpha candidate unless release management chooses a different prerelease identifier.

## Completed In This Slice

- `ontocode-rs/core/src/native_provider/copilot.rs`
  - `Copilot` user-agent and editor version headers now derive from `env!("CARGO_PKG_VERSION")`.
- `ontocode-rs/core/src/native_provider/copilot_tests.rs`
  - Added coverage proving versioned Copilot headers are emitted from the crate version.
- `ontocode-rs/cli/Cargo.toml`
  - The `ontocode` binary now builds from the real CLI entrypoint instead of a wrapper that required a sibling `codex` binary at runtime.
  - Alias-bin unit-test duplication is disabled for `ontocode` so the shared CLI entrypoint is tested once while integration coverage still verifies alias behavior.
- `ontocode-rs/target/debug/ontocode`
  - Fresh local source-built `ontocode --version` and `ontocode --help` now execute successfully and brand themselves correctly as `Ontocode CLI 0.0.0`.
- `ontocode-rs/target/release/ontocode`
  - Fresh clean release-profile `ontocode --version` and `ontocode --help` now execute successfully; the release build completed in `24m 11s`.
- `ontocode-rs/cli/src/main.rs`, `ontocode-rs/cloud-tasks/src/cli.rs`
  - User-visible CLI help text now consistently refers to `Ontocode` / `Ontocode Cloud` on the main binary surface.

## Release Surfaces And Owners

- Rust workspace release version
  - Source: `ontocode-rs/Cargo.toml`
  - Consumer path: `env!("CARGO_PKG_VERSION")` across CLI, app-server, telemetry, MCP, and runtime metadata
- npm staging
  - Source templates: `codex-cli/package.json`, `ontocode-rs/responses-api-proxy/npm/package.json`, `sdk/typescript/package.json`
  - Staging owner: `codex-cli/scripts/build_npm_package.py`
  - Multi-package staging: `scripts/stage_npm_packages.py`
- Python packaging
  - Source templates: `sdk/python/pyproject.toml`, `sdk/python-runtime/pyproject.toml`
  - Confirmed staging owners: `sdk/python/scripts/update_sdk_artifacts.py stage-sdk --sdk-version <version>` and `stage-runtime --codex-version <version>`
  - Source manifests can remain on dev placeholders when staging injects release versions
- Nix release version selection
  - Source: `flake.nix`
  - Behavior: uses workspace version when not `0.0.0`, otherwise derives a dev build version

## Alpha Cut Checklist

1. Choose the concrete release version.
   - Preferred default: `0.1.0-alpha.1`.
   - Already supported by installer tooling: `0.1.0`, `0.1.0-alpha`, `0.1.0-alpha.1`.
2. Decide release-branch version policy.
   - Option A: leave source manifests on placeholders and stage all packages with `--release-version`.
   - Option B: bump version-bearing manifests on a release branch/tag commit.
3. Stage release artifacts with explicit version injection.
   - CLI / platform npm / responses proxy / SDK: `scripts/stage_npm_packages.py --release-version <version> ...`
   - Python SDK / runtime: `sdk/python/scripts/update_sdk_artifacts.py stage-sdk --sdk-version <version>` and `stage-runtime --codex-version <version>`
4. Verify runtime metadata.
   - local source-built `ontocode --version`
   - staged release artifact version surfaces after injection
   - staged npm package metadata
   - Copilot request headers
   - app-server / MCP version surfaces
5. Resolve non-version release blocker.
   - Claude OAuth live validation remains blocked on a real redacted sample.

## Remaining Blockers

- `Claude OAuth live validation`
  - Needs one real redacted `CLAUDE_OAUTH_REDACTED_SAMPLE`.
- `Concrete alpha version choice`
  - Recommended baseline is `0.1.0-alpha.1`, but no final release-manager decision is recorded yet.
- `Workspace path rename`
  - The binary is `ontocode`, but the Rust workspace directory is still `ontocode-rs/`.
  - This is layout debt, not a runtime blocker; moving the workspace root to `ontocode-rs/` still requires a dedicated path-migration plan across Cargo, Bazel, scripts, and CI.
- `Release warning cleanup`
  - Cargo still emits multi-target duplicate-source warnings for `ontocode`/`codex` and other renamed helper binaries during builds.
  - These warnings are currently accepted as non-blocking alpha debt unless a dedicated post-alpha entrypoint/helper alias refactor is scheduled.
- `Dead-code cleanup`
  - Pre-existing warnings remain in `model-provider`, `rmcp-client`, and `core`; they do not block the alias-binary fix but still reduce alpha-release signal quality.

## Verification Commands

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core copilot
CARGO_BUILD_JOBS=8 just fmt
```
