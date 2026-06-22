---
name: Claude Parked Row 002 Review
description: Triage closure for Claude Code donor parked row 002
type: audit_session
date: 2026-06-20
status: closed
---

# Claude Parked Row 002 Review

Row:
- `002`

Current verdict:
- `NARROW`

Parked ADR text:
- `Model presets must be config-driven and avoid a second model catalog.`

Donor review text:
- `Add a tool preset abstraction even if only default exists.`

Duplicate gate:
- No dispatchable overlap found in Gemini or Oh My Pi pre-junior plans.

OntoIndex owner:
- `ontocode-rs/core/src/tools/spec_plan.rs`
- OntoIndex reports public API `build_tool_router`, `search_tool_enabled`, and `tool_suggest_enabled`.
- OntoIndex reports 996 lines and 75 symbols in the owner file.
- `build_tool_router` upstream impact is LOW, with direct caller `ToolRouter::from_turn_context`.

Decision:
- Row 002 stays parked.
- A preset abstraction with only `default` is not a concrete gap and would add structure without behavior.
- No promotion packet was written because there is no single existing-owner failing test gap.

Verification:
- Parked ADR table count remains 146.
- `git diff --check -- .memory-bank/CLAUDE_CODE_DONOR_DEFERRED_NARROW_REJECT_PRE_JUNIOR_PROJECT_PLAN.md` passed.
