# Audit: External-Agent Interop ADR Consolidation

Date: 2026-06-08

## Event

Consolidated Gemini CLI, Hermes Agent, and GBrain interop ADRs into `ADR_EXTERNAL_AGENT_INTEROP_DETECTORS_CONSOLIDATION.md`.

## Decision

- The new consolidation ADR is the dispatch authority for all retained Stage 0 interop detector work from the three source ADRs.
- Gemini CLI, Hermes Agent, and GBrain now share one redacted read-only report contract.
- Source ADRs remain historical evidence and no longer approve independent detector stacks.
- Future Rust integration must extend existing external-agent config/migration owners and must run GitNexus impact first.

## Guardrails

- No provider runtime changes.
- No credential import or persistence.
- No MCP mutation.
- No command, plugin, skill, job, or gateway execution.
- No model-visible context or app-server API in Stage 0.
- No raw content, secrets, shell output, trajectory body, memory page content, or source content in reports.

## Follow-Up

- If implementation starts, use one shared script/report contract rather than three independent parsers.
- Keep source-specific wrappers thin and compatible with the consolidated schema.
