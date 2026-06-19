# R5L Collaboration Mode Templates Rename Risk Review

Date: 2026-06-11

## Candidate

- `codex-collaboration-mode-templates` -> `ontocode-collaboration-mode-templates`.
- `codex_collaboration_mode_templates` -> `ontocode_collaboration_mode_templates`.
- Identity-only package/lib/Bazel/import rename.

## Evidence

- Cargo metadata reports one direct reverse dependency: `codex-models-manager`.
- Direct text inventory has seven active refs confined to root workspace metadata, `collaboration-mode-templates/Cargo.toml`, `collaboration-mode-templates/BUILD.bazel`, `models-manager/Cargo.toml`, and `models-manager/src/collaboration_mode_presets.rs`.
- OntoIndex CLI impact for exact constants `DEFAULT` and `PLAN` reports LOW risk with zero direct impacted nodes.

## Guardrails

- Preserve all template file contents.
- Preserve collaboration-mode preset behavior and `models-manager` semantics.
- Preserve compile data/template packaging and the existing `collaboration-mode-templates` directory path.
- Preserve env/config/wire/generated names, telemetry/product strings, and persisted state.

## Decision

R5L is approved for dispatch as a bounded residual package identity slice. No behavior edits or template-content rewrites are approved.
