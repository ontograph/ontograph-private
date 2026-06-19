# R5AI Prompts Rename Risk Review

Date: 2026-06-12

## Candidate

- `codex-prompts` -> `ontocode-prompts`
- `codex_prompts` -> `ontocode_prompts`

## Inventory

- Cargo metadata direct reverse dependencies: `ontocode-core`
- Active direct refs: 21
- Ref locations: root workspace metadata, core dependency/import/re-export usage, core tests, and prompts manifest/Bazel identity.

## OntoIndex Impact

- `Const:ontocode-rs/prompts/src/agents.rs:HIERARCHICAL_AGENTS_MESSAGE`: LOW, 0 impacted, 0 processes.
- `Const:ontocode-rs/prompts/src/compact.rs:SUMMARIZATION_PROMPT`: LOW, 0 impacted, 0 processes.
- `Const:ontocode-rs/prompts/src/realtime.rs:BACKEND_PROMPT`: LOW, 0 impacted, 0 processes.
- `Function:ontocode-rs/prompts/src/review_request.rs:review_prompt`: LOW, 3 impacted, 1 direct, 2 modules, 0 processes.
- `Struct:ontocode-rs/prompts/src/permissions_instructions.rs:PermissionsInstructions`: LOW, 0 impacted, 0 processes.
- `Impl:ontocode-rs/prompts/src/permissions_instructions.rs:PermissionsInstructions`: LOW, 0 impacted, 0 processes.
- `Function:ontocode-rs/prompts/src/goals.rs:continuation_prompt`: LOW, 0 impacted, 0 processes.
- `Function:ontocode-rs/prompts/src/goals.rs:budget_limit_prompt`: LOW, 0 impacted, 0 processes.
- `Const:ontocode-rs/prompts/src/apply_patch.rs:APPLY_PATCH_TOOL_INSTRUCTIONS`: LOW, 0 impacted, 0 processes.
- `Function:ontocode-rs/prompts/src/review_exit.rs:render_review_exit_success`: LOW, 2 impacted, 1 direct, 1 module, 0 processes.

## Decision

- Proceed as an identity-only package/lib/Bazel/import rename.
- This slice must not change any prompt text, template rendering, context-fragment shape, or user-visible review/realtime/goal/permission behavior.

## Guardrails

- Preserve prompt text, template files, permissions instructions/context-fragment behavior, realtime prompts, compaction prompts, review prompts, goal prompts, and apply-patch tool instructions.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `prompts` directory path.
- Verify with prompts package tests, focused core prompt/realtime/permission/compaction checks, fmt, Bazel lock checks, active-source stale-reference search, metadata count, diff check, and OntoIndex CLI fallback verification.
