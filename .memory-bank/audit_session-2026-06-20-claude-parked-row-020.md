name: Claude Parked Row 020 Review
desc: Row 020 stays parked because explicit task-output reads require new state/API/tool surface rather than existing planner coverage
type: audit_session
date: 2026-06-20

# Claude Parked Row 020 Review

## Decision

Row 020 stays parked. No promotion packet was written.

## Evidence

- Parked ADR row 020 says: `Keep as test coverage around existing tool planner behavior.`
- Donor source row 020 says: `Add explicit task output tool separate from task control` in `state/src/runtime/agent_jobs.rs`.
- The sources do not describe one clean existing-owner test gap.
- OntoIndex reports `ontocode-rs/state/src/runtime/agent_jobs.rs` owns `create_agent_job`, `get_agent_job`, `list_agent_job_items`, `get_agent_job_item`, `report_agent_job_item_result`, and progress/status mutations; the file is 685 lines.
- OntoIndex reports `agent_jobs_tools_enabled` in `ontocode-rs/core/src/tools/spec_plan.rs` has LOW impact and direct callers in tool planning.
- `spec_plan_tests.rs` already checks normal sessions see `spawn_agents_on_csv` and not `report_agent_job_result`, while agent-job worker sessions see both.
- `core/tests/suite/agent_jobs.rs` already covers wrong-thread result rejection and CSV export-on-completion.
- A paginated task-output read tool would require a new state/API/tool surface and likely persisted output shape decisions.

## Closure

The row remains parked. Existing planner behavior is covered, and the donor task-output read proposal is outside this parked-row plan's narrow promotion rule.
