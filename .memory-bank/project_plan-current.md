---
name: Ontocode Current Forward Plan
description: Active project plan snapshot derived from CLAUDE_CODE_APPROACHES_FOR_CODEBASE.md and its tracking file
type: project_plan
date: 2026-06-07
status: active
---

# Ontocode Current Forward Plan

Authoritative source: `CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md`.

This file is a memory-bank summary, not the dispatch source of truth. Update the tracking file before starting or closing work.

## Current Status

| Order | Epic | IDs | Status |
| --- | --- | --- | --- |
| 1 | Provider provenance/status/capability diagnostics | 2, 3, 4, 8, 9, 12, 13, 14 | done |
| 2 | OAuth/auth-store validation and redacted diagnostics | 5, 6, 7, 11, 160 | done |
| 3 | MCP reliability and auth hardening | 141-146, 149-151, 155-157 | done |
| 4 | Hook and shell permission safety | 47-51, 53-56, 58-63, 161, 166-168, 172, 174, 175 | done |
| 5 | External adapter protocol safety | 16-30 | done |
| 6 | Session/context bounded diagnostics | 1, 87, 89, 100, 104, 109-111, 117, 119-121, 124, 130, 132, 140, 181, 185 | done |
| 7 | External-agent import internals | 213-215, 217, 218, 220 | done |
| 8 | Claude OAuth Import Wiring & Validation | Audit Gap | blocked |
| 9 | Public adapter SDK and schema migrations ADR | Next Phase | in_progress |

## Counts

- Total tracked core-natural point IDs: 86.
- Done: 86.
- In progress: 1 (Next phase ADR).
- Pending: 0.
- Blocked: 1 (Claude OAuth live sample validation).
- Not done: 0.

## Next Phase

All tracked project-plan tasks from the initial core-natural approaches slice are complete.

Upcoming work depends on:
- ADR for public adapter SDK and schema migrations. (in progress)
- Rollout of TUI context visualization.
- Multi-agent goal orchestration refinements.

## Completion Criteria For Any Epic

- Tracking file updated before dispatch and after completion.
- GitNexus context/impact recorded.
- HIGH/CRITICAL impact reported before edits and narrowed if possible.
- Existing architecture reused; no duplicate owners introduced.
- Scoped tests pass.
- `just fmt` run after Rust changes.
- GitNexus `detect_changes` run before close-out.
