---
name: Claude Parked Row 007 Review
description: Triage closure for Claude Code donor parked row 007
type: audit_session
date: 2026-06-20
status: closed
---

# Claude Parked Row 007 Review

Row:
- `007`

Current verdict:
- `NARROW`

Parked ADR text:
- `Keep as bounded ranking metadata, not a new planner.`

Donor review text:
- `Add tool alias resolution at registry boundary.`

Duplicate gate:
- No dispatchable Gemini or Oh My Pi duplicate.

OntoIndex owner:
- `ontocode-rs/protocol/src/tool_name.rs`
- OntoIndex reports public API `new`, `plain`, and `namespaced`.
- OntoIndex reports 77 lines and 11 symbols.
- `ToolName::namespaced` upstream impact is CRITICAL: 52 impacted nodes, 36 direct callers, 6 affected modules.

Decision:
- Row 007 stays parked.
- Alias resolution at the registry boundary is not reducible to one owner-local test in `tool_name.rs`.
- Existing registry tests already cover explicit plain/namespaced lookup and spawn-agent matcher aliasing.
- No promotion packet was written.
