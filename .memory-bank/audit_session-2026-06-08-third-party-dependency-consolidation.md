# Audit: Third-Party Dependency Consolidation

Date: 2026-06-08

## Event

Reviewed `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md` and inlined the dependency boundary from `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md`.

## Decision

- `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md` is now the canonical consolidation record for GitNexus and lean-ctx derived third-party dependencies.
- Ontocode will use one `operational_evidence_records` backbone model with domains such as `code_graph`, `workflow`, `test`, `doc`, `redaction`, and `architecture`.
- GitNexus analyzer dependencies may exist only inside the hermetic local evidence binary.
- Lean-ctx remains external development workflow tooling and must not become an Ontocode runtime, app-server, SDK, state, memory, context, or model-visible dependency.
- Stage 0 Lean-CTX repository scripts remain bootstrap-only and standard-library-first.

## Follow-Up

- Next core implementation card should target GitNexus `G1 - Operational Evidence State Model`.
- Do not create a separate Lean-CTX B0 schema.
- Any third-party dependency exception must satisfy the consolidated boundary before implementation.
