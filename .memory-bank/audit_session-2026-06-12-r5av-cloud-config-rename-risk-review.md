# R5AV Cloud Config Rename Risk Review

## Decision

Dispatch R5AV as an identity-only residual package rename:

- `codex-cloud-config` -> `ontocode-cloud-config`
- `codex_cloud_config` -> `ontocode_cloud_config`

## OntoIndex Impact

- `cloud_config_bundle_loader`: CRITICAL, 10 impacted nodes, affected app-server startup/login and CLI paths.
- `cloud_config_bundle_loader_for_storage`: LOW, 0 impacted nodes, no affected processes.
- `CloudConfigBundleLoader`: ambiguous in `ontocode-config`; direct inventory is used for package/import scope.

## Direct Inventory

- Root workspace metadata and cloud-config manifest/Bazel identity.
- Dependent manifests/imports in `app-server`, `exec`, and `tui`.

## Guardrails

- Preserve cloud config bundle loading behavior.
- Preserve cache path/file/version/HMAC key compatibility, including `codex-cloud-config-bundle-cache-v1-6160ae70-bcfd-4ca8-a99b-40f73b3b072e`.
- Preserve auth retry/relogin behavior.
- Preserve app-server config-manager replacement behavior.
- Preserve exec/TUI startup loading behavior.
- Preserve metrics names, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `cloud-config` directory path.

## Verification Required

- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server cloud_config`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-exec --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_cloud_config|codex-cloud-config`
- Cargo metadata residual count, expected 25 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`
