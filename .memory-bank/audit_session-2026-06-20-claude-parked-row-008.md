---
name: Claude Parked Row 008 Review
description: Triage closure for Claude Code donor parked row 008
type: audit_session
date: 2026-06-20
status: closed
---

# Claude Parked Row 008 Review

Row:
- `008`

Current verdict:
- `NARROW`

Parked ADR text:
- `Only useful if current generated text lacks source attribution.`

Donor review text:
- `Add deny-rule filtering as a registry step.`

Duplicate gate:
- Source attribution is already active Claude KEEP row `072`.
- No Gemini or Oh My Pi dispatchable duplicate for deny-rule filtering.

OntoIndex owner:
- `ontocode-rs/core/src/tools/spec_plan.rs`
- OntoIndex reports public API `build_tool_router`, `search_tool_enabled`, and `tool_suggest_enabled`.
- OntoIndex reports 996 lines and 75 symbols.
- `build_tool_router` upstream impact is LOW via `ToolRouter::from_turn_context`.

Decision:
- Row 008 stays parked.
- No promotion packet: the parked attribution slice is already owned, and the donor deny-rule slice has no current config deny surface to test without adding new architecture.
