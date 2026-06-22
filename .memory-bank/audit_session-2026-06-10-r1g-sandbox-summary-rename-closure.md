# R1G Sandbox Summary Internal Crate Rename Closure

Date: 2026-06-10

Scope:
- Renamed internal Rust package/lib identity from `codex-utils-sandbox-summary` to `ontocode-utils-sandbox-summary`.
- Updated workspace dependency, dependent crate manifests, Rust imports, Bazel crate name, and lockfiles.
- Preserved sandbox-summary behavior and status text semantics.

Risk:
- OntoIndex impact before edits reported `summarize_permission_profile` as CRITICAL and `summarize_sandbox_policy` as HIGH through TUI status/MCP display paths.
- Senior approval limited the work to identity-only package/lib/Bazel/import changes.

Verification:
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-sandbox-summary`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui` passed with 2772 tests passed and 4 skipped.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search found no `codex_utils_sandbox_summary` or `codex-utils-sandbox-summary` matches under `ontocode-rs`.
- `git diff --check` passed for the touched scope.
- Scoped OntoIndex diff verification passed.

Notes:
- Regenerated lockfile initially drifted `allocative` to `0.3.6`; it was restored to the previously validated non-yanked `0.3.4` pin before verification.
- Remaining R1 candidates still require explicit risk review before dispatch.
