# R5AU Code Mode Rename Risk Review

## Decision

Dispatch R5AU as an identity-only residual package rename:

- `codex-code-mode` -> `ontocode-code-mode`
- `codex_code_mode` -> `ontocode_code_mode`

## OntoIndex Impact

- `CodeModeSession`: LOW, 1 impacted implementor, no affected processes.
- `RuntimeResponse`: LOW, 0 impacted nodes, no affected processes.
- `parse_exec_source`: LOW, 2 impacted nodes in core code-mode execution, no affected processes.
- `CodeModeService`, `PUBLIC_TOOL_NAME`, and `ToolDefinition`: ambiguous because core/tools expose similarly named wrappers; direct inventory is used for package/import scope.

## Direct Inventory

- Root workspace metadata and code-mode manifest/Bazel identity.
- Dependent manifests/imports in `core`, `rollout-trace`, and `tools`.

## Guardrails

- Preserve code-mode tool names and wait/execute behavior.
- Preserve runtime response semantics and nested tool-call classification.
- Preserve V8/sandbox feature behavior.
- Preserve source parsing and model-visible tool descriptions.
- Preserve rollout trace decoding and tools crate augmentation behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `code-mode` directory path.

## Verification Required

- `CARGO_BUILD_JOBS=8 just test -p ontocode-code-mode --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core code_mode`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core spec_plan`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tools code_mode`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rollout-trace code`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_code_mode|codex-code-mode`
- Cargo metadata residual count, expected 26 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`
