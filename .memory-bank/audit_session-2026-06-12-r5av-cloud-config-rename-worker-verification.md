# R5AV Cloud Config Rename Worker Verification

## Scope

- Renamed `codex-cloud-config` -> `ontocode-cloud-config`.
- Renamed `codex_cloud_config` -> `ontocode_cloud_config`.
- Preserved the existing `cloud-config` directory path and all cache/auth/app-server/exec/TUI behavior.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server cloud_config`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-exec --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search
- Cargo metadata residual count
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

## Result

- The only remaining old-name ref in active source is the intentional cache marker `codex-cloud-config-bundle-cache-v1-6160ae70-bcfd-4ca8-a99b-40f73b3b072e`.
- Cargo metadata reports 25 remaining `codex-*` packages after this slice.
- OntoIndex `detect-changes --repo codex` remains high-risk because of pre-existing unrelated dirty-tree churn.
- Worker runtime model: `gpt-5.4-mini` after Spark usage-limit fallback.
