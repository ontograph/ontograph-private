name: Claude Parked Row 058 Review
desc: Row 058 stays parked because parent-child lineage and source metadata already exist, while a task-type enum would add protocol/state surface
type: audit_session
date: 2026-06-20

# Claude Parked Row 058 Review

## Decision

Row 058 remains parked. No promotion packet.

## Evidence

- Parked ADR row 058 says to add only missing parent/child relationships if needed.
- Donor row 058 asks for a task type enum for local, remote, workflow, and monitor in `protocol` / `state`, which would add public protocol/state surface rather than one owner-local missing behavior.
- Duplicate gate blocks promotion because Oh My Pi explicitly blocks Claude agent/job/session overlap for rows 057-059 and 148-150.
- `SessionSource::SubAgent(SubAgentSource::ThreadSpawn)` already carries `parent_thread_id`, `depth`, `agent_path`, `agent_nickname`, and `agent_role`.
- `SessionSource::parent_thread_id()` already derives parent lineage from sub-agent source metadata.
- `thread_spawn_edges` persists parent/child edges and has direct-child, descendant, status-filtered, and all-status tests.
- Agent-job items already carry `source_id` through create/load/export paths.
- Worker evidence found `persist_thread_spawn_edge_for_source` writes the spawn edge when a source has a parent id, and turn metadata tests assert thread-spawn subagents surface parent id and kind.
- No exactly-one owner-local failing test gap was found.

## Closure

The row stays in the NARROW parking lot. Reopen only if a senior identifies one missing relationship invariant in the existing lineage owners without adding a task-type enum or new state/protocol surface.
