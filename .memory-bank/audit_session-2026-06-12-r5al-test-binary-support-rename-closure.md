# R5AL Test Binary Support Rename Closure

Date: 2026-06-12
Status: accepted
Model fallback: `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.

## Outcome

- Accepted `codex-test-binary-support` as `ontocode-test-binary-support`.
- Accepted `codex_test_binary_support` as `ontocode_test_binary_support`.
- Preserved helper dispatch semantics, ctor timing assumptions, tempdir lifetime, arg0 alias installation behavior, `CODEX_HOME` compatibility/restoration behavior, user-visible test binary names, runtime helper aliases, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `test-binary-support` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-test-binary-support --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-exec-server --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n 'codex_test_binary_support|codex-test-binary-support' ontocode-rs --glob '!target' || true`
- `cargo metadata --format-version 1 --no-deps`
- `git diff --check`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

## Notes

- Active old refs in `ontocode-rs` are clean.
- Cargo metadata reports 35 remaining `codex-*` packages.
- OntoIndex fallback still reports the known broad dirty-tree high-risk context: 200 files, 312 symbols, 8 affected processes.
