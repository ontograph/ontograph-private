name: Claude Parked Row 059 Review
desc: Row 059 stays parked because scheduler and background-monitor task types lack fresh requirements and existing agent-job/MCP status owners already cover adjacent behavior
type: audit_session
date: 2026-06-20

# Claude Parked Row 059 Review

## Decision

Row 059 remains parked. No promotion packet.

## Evidence

- Parked ADR row 059 says scheduler changes need concrete concurrency requirements.
- Donor row 059 asks for a background monitor task type in `state` / MCP monitor, which would add a new long-running monitoring task surface.
- Duplicate gate blocks promotion because Oh My Pi explicitly blocks Claude agent/job/session overlap for rows 057-059 and 148-150.
- Existing agent-job execution already clamps requested concurrency with `normalize_concurrency`, handles `AgentLimitReached`, and stops future item spawning after cancellation.
- Existing tests cover agent-job cancellation halting future items and preserving progress counts.
- MCP status and monitoring are already owned by `codex-mcp` connection manager and app-server MCP status processors.
- OntoIndex reports `ontocode-rs/codex-mcp/src/connection_manager.rs` is an 823-line owner with readiness, startup failure, server-info, resource, and tool APIs.
- No fresh bug, regression, security, safety, product evidence, or concrete concurrency requirement was found.

## Closure

The row stays in the DEFER parking lot. Reopen only with specific requirements or failure evidence that can be handled in the existing agent-job or MCP status owners without adding a background monitor task type.
