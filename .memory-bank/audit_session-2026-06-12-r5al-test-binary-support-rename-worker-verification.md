# R5AL Test Binary Support Rename Worker Verification

Date: 2026-06-12
Status: complete
Model fallback: `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.

## Outcome

- Renamed `codex-test-binary-support` to `ontocode-test-binary-support`.
- Renamed Rust crate import `codex_test_binary_support` to `ontocode_test_binary_support`.
- Updated workspace metadata, helper crate manifest/Bazel identity, and direct dependent test harness imports/dependencies.
- Preserved `CODEX_HOME` compatibility/restoration behavior, helper dispatch semantics, user-visible test binary names, and runtime helper aliases.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-test-binary-support --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-exec-server --tests`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n 'codex_test_binary_support|codex-test-binary-support' ontocode-rs --glob '!target' || true`
- `cargo metadata --format-version 1 --no-deps`
- `git diff --check`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

## Notes

- Active old refs in `ontocode-rs` are clean.
- Cargo metadata reports 35 remaining `codex-*` packages.
