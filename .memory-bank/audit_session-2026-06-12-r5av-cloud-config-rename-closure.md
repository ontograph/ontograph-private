# R5AV Cloud Config Rename Closure

## Scope

Accepted the identity-only residual crate rename:

- `codex-cloud-config` -> `ontocode-cloud-config`
- `codex_cloud_config` -> `ontocode_cloud_config`

## Preserved Behavior

- Cloud config bundle loading, cache path/file/version/HMAC key compatibility, auth retry/relogin behavior, app-server config-manager replacement behavior, exec/TUI startup loading behavior, metrics names, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `cloud-config` directory path stayed unchanged.
- The persisted compatibility marker `codex-cloud-config-bundle-cache-v1-6160ae70-bcfd-4ca8-a99b-40f73b3b072e` stayed unchanged.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server cloud_config`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-exec --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_cloud_config|codex-cloud-config`
- Cargo metadata residual count: 25 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Notes

- The only remaining old-name ref in `ontocode-rs` is the intentional persisted cache marker.
- OntoIndex reports the known broad high-risk dirty tree from the accumulated rename program.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
