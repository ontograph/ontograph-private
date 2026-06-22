# Claude Parked Row 078 Review

Date: 2026-06-20

## Scope

- Source ADR row: `ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 078.
- Donor source row: `CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 078.
- Tracking row: `CLAUDE_CODE_DONOR_DEFERRED_NARROW_REJECT_PRE_JUNIOR_PROJECT_PLAN.md` row 078.

## Decision

Row 078 stays parked. No promotion packet.

The parked ADR classifies the item as `NARROW` and says ADR hygiene belongs in docs process. The donor proposal asks to add a workflow commands section in `.memory-bank`, which is memory-bank practice rather than a core runtime feature.

## Evidence

- Duplicate gates keep Claude context, memory, prompt-cache, and command-overlap rows out of implementation dispatch unless a fresh owner-local gap is proven.
- `.memory-bank/MEMORY.md` already links the active project plan and pending backlog.
- Row 077 already records current-state memory as owned by `project_plan-current.md`, `project_pending-tasks.md`, and the read-only `status_digest` helper.
- Memory write templates already instruct future summaries to preserve workflows, common commands, verification checklists, exact command flags, and failure guidance.
- `scripts/onto_memory_tools.py` already provides repository-only read helpers for status digest, left-count reporting, and local markdown link checks.

## Outcome

No core implementation task is created. Future work should stay in memory-bank docs/process updates or repo-only helper scripts unless new product evidence proves an existing-owner failing test gap.
