# R1H OSS Internal Crate Rename Closure

Date: 2026-06-10

Scope:
- Renamed internal Rust package/lib identity from `codex-utils-oss` to `ontocode-utils-oss`.
- Updated workspace dependency, dependent exec/TUI manifests, Rust imports, Bazel crate name, and lockfiles.
- Preserved OSS provider readiness and default-model behavior.

Risk:
- OntoIndex impact before edits reported `ensure_oss_provider_ready` as CRITICAL and `get_default_model_for_oss_provider` as CRITICAL.
- Affected paths include direct exec/TUI startup and indirect CLI startup flows.
- Senior approval limited the work to identity-only package/lib/Bazel/import changes.

Verification:
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-oss`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui` passed with 2772 tests passed and 4 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli resume_merges_option_flags`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search found no `codex_utils_oss` or `codex-utils-oss` matches under `ontocode-rs`.
- `Cargo.lock` contains `ontocode-utils-oss` and no old OSS package name.
- `git diff --check` passed for the touched scope.
- Scoped OntoIndex diff verification passed.

Notes:
- `allocative` remained at the validated `0.3.4` lockfile pin.
- Remaining R1 candidates still require explicit risk review before dispatch.
