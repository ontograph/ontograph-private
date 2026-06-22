name: Claude Parked Row 018 Review
desc: Row 018 stays parked because synthetic-output tooling is a protocol/tool-surface risk without fresh evidence
type: audit_session
date: 2026-06-20

# Claude Parked Row 018 Review

## Decision

Row 018 stays parked. No promotion packet was written.

## Evidence

- Parked ADR row 018 says: `Synthetic-output tooling risks fake evidence; require strict labeling.`
- Donor source row 018 says: `Add synthetic-output/read-only structured result tool` in `protocol` / `core/src/tools`.
- No fresh bug, user-facing regression, security/safety issue, or senior-approved product requirement was found during triage.
- Public protocol/schema/tool surfaces are ADR-gated, and this row would create a new model-visible result path rather than a local test-only gap.
- OntoIndex found existing structured-content owners around `ontocode-rs/core/src/mcp_tool_call.rs::sanitize_mcp_tool_result_for_model`, `ontocode-rs/tools/src/mcp_tool.rs::mcp_call_tool_result_output_schema`, and `protocol::mcp::CallToolResult`.
- OntoIndex also surfaced app-server schema fixture ownership and agent-job result paths as existing structured-result areas, so this is not an empty owner slot.
- The search did not identify a single existing-owner failing test that would prove strict labeling without opening a new tool/protocol surface.

## Closure

The row remains deferred. Synthetic-output tooling can only move under a senior-approved protocol/tool ADR or concrete failure evidence.
