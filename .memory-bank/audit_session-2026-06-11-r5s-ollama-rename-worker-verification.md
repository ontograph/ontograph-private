# R5S Ollama Rename Worker Verification

Date: 2026-06-11

Model: `gpt-5.4-mini`

Fallback reason: `gpt-5.3-codex-spark` hit usage limit earlier in this manager run.

Reasoning effort: low; mechanical identity-only package rename with one direct downstream import rewrite.

## Result

- Implemented `codex-ollama` -> `ontocode-ollama` and `codex_ollama` -> `ontocode_ollama`.
- Preserved Ollama provider IDs, default OSS model value, `ensure_oss_ready` behavior, responses-version gate semantics, OSS provider selection, `ollama` command/process behavior, TUI/exec `--oss` startup behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `ollama` directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-ollama --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-oss --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_ollama|codex-ollama`
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Residual Count

- Cargo metadata now reports 54 remaining `codex-*` workspace packages.
