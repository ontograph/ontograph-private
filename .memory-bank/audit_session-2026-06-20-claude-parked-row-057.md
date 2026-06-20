name: Claude Parked Row 057 Review
desc: Row 057 stays parked because agent job list/status/cancel behavior already has an owner and tests, while a kill interface would add surface area
type: audit_session
date: 2026-06-20

# Claude Parked Row 057 Review

## Decision

Row 057 remains parked. No promotion packet.

## Evidence

- Parked ADR row 057 says to avoid duplicating current job list/status queries.
- Donor row 057 asks to separate task kill interface from spawn/render in `state`, which would create a new surface rather than one owner-local missing behavior.
- Duplicate gate found no Gemini dispatchable slice and no Oh My Pi reopen lane for agent-job kill/list/status behavior.
- OntoIndex reports `ontocode-rs/state/src/runtime/agent_jobs.rs` is a 685-line owner with create, get, list, status, cancel, item-result, and progress APIs.
- `mark_agent_job_cancelled` has LOW upstream impact through `report_agent_job_result`; `list_agent_job_items` has LOW upstream impact through `run_agent_job_loop` and `spawn_agents_on_csv`.
- Current implementation exposes `list_agent_job_items`, `mark_agent_job_cancelled`, `is_agent_job_cancelled`, and `get_agent_job_progress`, and the job loop consumes those APIs.
- Existing core integration tests cover the stop/cancel path and status/progress shape, so no exactly-one missing failing test gap was found.

## Closure

The row stays in the NARROW parking lot. Reopen only if a senior identifies one existing-owner regression test that fails without adding a separate job-control interface.
