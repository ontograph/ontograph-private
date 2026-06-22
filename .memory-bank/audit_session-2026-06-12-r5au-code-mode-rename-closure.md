# R5AU Code Mode Rename Closure

## Scope

Accepted the identity-only residual crate rename:

- `codex-code-mode` -> `ontocode-code-mode`
- `codex_code_mode` -> `ontocode_code_mode`

## Preserved Behavior

- Code-mode tool names, wait/execute behavior, runtime response semantics, nested tool-call classification, V8/sandbox feature behavior, source parsing, model-visible tool descriptions, rollout trace decoding, tools crate augmentation behavior, env/config/wire/generated names, telemetry/product strings, persisted state, the existing `code-mode` directory path, and the runtime sentinel `__codex_code_mode_exit__` stayed unchanged.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-code-mode --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core code_mode`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core spec_plan`
- `CARGO_BUILD_JOBS=8 just test -p codex-tools code_mode`
- `CARGO_BUILD_JOBS=8 just test -p codex-rollout-trace code`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_code_mode|codex-code-mode`
- Cargo metadata residual count: 26 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Notes

- The only remaining old-name ref in `ontocode-rs` is the intentional runtime sentinel `__codex_code_mode_exit__`.
- `codex-tools` and `codex-rollout-trace` are not renamed yet, so manager verification used those current package IDs for dependent checks.
- OntoIndex reports the known broad high-risk dirty tree from the accumulated rename program.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
