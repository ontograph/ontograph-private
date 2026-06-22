# Lefties: GBrain Tool Extensions

## Status

Moved out of core ADR

## Date

2026-06-07

## Context

These GBrain-inspired ideas were removed from [ADR_GBRAIN_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_GBRAIN_TOOL_EXTENSIONS.md:1) because they either duplicate existing Ontocode owners, need a separate ADR, or do not naturally extend the core codebase.

## Moved Items

| Original points | Reason moved |
|---|---|
| `021-050` hybrid retrieval, RRF, rerank, query expansion, retrieval evals | Would create a second memory/search stack unless a dedicated memory-search ADR proves the need. |
| `051-070` graph traversal, typed edges, graph diagnostics, alias graph | Duplicates GitNexus for code intelligence and would require a separate memory-graph ADR for user memory. |
| `080`, `086-090` schema-pack scaffolding, mutation, link taxonomy, type systems | Runtime memory schema mutation is not approved. |
| `131-150` contract-first operation registry and execution surfaces | Duplicates existing tool specs, registry, and MCP handling. |
| `151-170` minions, job queues, schedulers, job logs, shell job workers | Not a natural core extension; shell execution must stay in existing shell policy/runtime owners. |
| `171-190` new guardrail runtime, policy DSL, blocking hooks | Duplicates hooks/policy/redaction owners and would need a security ADR. |
| `199-205` remote skillpack registry, marketplace, scoring, trust promotion | Product ecosystem work, not core runtime architecture. |
| `211-240` ingestion/enrichment pipelines for external sources | Importing arbitrary external knowledge is product scope and needs source/privacy ADRs. |
| `241-250`, `255-257` AI gateway, multimodal embedding, image/media handling | Provider/media runtime scope, not GBrain interop detection. |
| `261-280` benchmark suite, synthetic corpora, large eval dashboards | Useful only after accepted runtime behavior exists; belongs in testing/tooling ADRs if needed. |
| `281-300` admin UI, dashboards, browser views, support-bundle UX | Product/UI scope outside the current core extension plan. |
| `301-320` notability, domain-specific trust, source health scorecards | Domain-specific memory semantics, not generic Ontocode core. |
| `321-330` code graph retrieval, tree-sitter parser stack, call graph browser | GitNexus already owns code intelligence for this project. |
| `331-340` meeting/email/calendar/social/voice/media ingestion | External product integrations with high privacy risk. |
| `351-360` remote brain thin client, hosting, proxy, server API | Would introduce a new service boundary and credential path. |
| `361-380` external DB/storage/tiering/cache/index runtime | Storage architecture scope; not accepted without a dedicated ADR. |
| `381-390` release notes, admin docs, dashboards, roadmap automation | Operational/product process work, not codebase core functionality. |
| `391-399` broad affordability/test-generation automation | Better handled by lean-ctx project tooling if kept at all. |

## Re-entry Criteria

Any moved item can return only with:

- a concrete user problem not solved by current memories, MCP, provider, shell, diagnostics, external-agent, GitNexus, or lean-ctx owners
- a narrow ADR naming the exact owner and compatibility surface
- redaction and privacy tests for any credential, source, memory, command, or diagnostic output
- bounded context rules for anything model-visible
- GitNexus context and impact analysis before implementation
