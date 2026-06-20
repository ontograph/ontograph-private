---
name: Claude Parked Row 004 Review
description: Triage closure for Claude Code donor parked row 004
type: audit_session
date: 2026-06-20
status: closed
---

# Claude Parked Row 004 Review

Row:
- `004`

Current verdict:
- `NARROW`

Parked ADR text:
- `Use only for measurable planning misses; avoid prompt churn.`

Donor review text:
- `Add explicit disabled-reason metadata per tool.`

Duplicate gate:
- No exact Gemini or Oh My Pi pre-junior duplicate.
- Active Claude KEEP row `010` already owns explicit disabled-tool reasons in the current tool exposure path.

OntoIndex owner:
- `ontocode-rs/core/src/tools/spec_plan.rs`
- Public API includes `build_tool_router`, `search_tool_enabled`, and `tool_suggest_enabled`.
- `build_tool_router` upstream impact is LOW via `ToolRouter::from_turn_context`.

Decision:
- Row 004 stays parked.
- No standalone promotion packet; the useful slice is already owned by active KEEP row 010.
