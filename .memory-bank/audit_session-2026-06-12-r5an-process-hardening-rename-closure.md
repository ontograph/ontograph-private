# R5AN Process Hardening Rename Closure

Date: 2026-06-12
Status: accepted
Model fallback: `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.

## Outcome

- Accepted `codex-process-hardening` as `ontocode-process-hardening`.
- Accepted `codex_process_hardening` as `ontocode_process_hardening`.
- Preserved prctl/ptrace/setrlimit behavior, process dump disabling, core dump disabling, LD_/DYLD_ stripping, non-UTF-8 env-key handling, public proxy binary/npm/docs/config compatibility surfaces, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `process-hardening` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-process-hardening --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-responses-api-proxy --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n 'codex_process_hardening|codex-process-hardening' ontocode-rs --glob '!target' || true`
- `cargo metadata --format-version 1 --no-deps`
- `git diff --check`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

## Notes

- Active old refs in `ontocode-rs` are clean.
- Cargo metadata reports 33 remaining `codex-*` packages.
- OntoIndex fallback still reports the known broad dirty-tree high-risk context: 200 files, 312 symbols, 8 affected processes.
