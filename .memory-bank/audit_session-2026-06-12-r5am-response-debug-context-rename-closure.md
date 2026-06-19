# R5AM Response Debug Context Rename Closure

Date: 2026-06-12
Status: accepted
Model fallback: `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.

## Outcome

- Accepted `codex-response-debug-context` as `ontocode-response-debug-context`.
- Accepted `codex_response_debug_context` as `ontocode_response_debug_context`.
- Preserved diagnostic extraction semantics, request-id/header precedence, CF ray extraction, auth-error extraction, encoded error-code decoding, telemetry HTTP-body omission, redaction/security behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `response-debug-context` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-response-debug-context --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n 'codex_response_debug_context|codex-response-debug-context' ontocode-rs --glob '!target' || true`
- `cargo metadata --format-version 1 --no-deps`
- `git diff --check`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

## Notes

- Active old refs in `ontocode-rs` are clean.
- Cargo metadata reports 34 remaining `codex-*` packages.
- OntoIndex fallback still reports the known broad dirty-tree high-risk context: 200 files, 312 symbols, 8 affected processes.
