name: Claude Parked Row 027 Review
desc: Row 027 stays parked because wildcard permission help is documentation work without fresh core evidence
type: audit_session
date: 2026-06-20

# Claude Parked Row 027 Review

## Decision

Row 027 stays parked. No promotion packet was written.

## Evidence

- Parked ADR row 027 says: `Documentation/examples do not extend core.`
- Donor source row 027 says: `Add wildcard permission rule docs in generated help` in `execpolicy` / `config`.
- No fresh bug, user-facing regression, security/safety issue, or senior-approved product requirement was found during triage.
- OntoIndex reports `ontocode-rs/core/src/exec_policy.rs` public API includes `create_exec_approval_requirement_for_command`, `render_decision_for_unmatched_command`, `load_exec_policy`, and warning/error helpers; the file is 1050 lines.
- OntoIndex impact for `create_exec_approval_requirement_for_command` is CRITICAL, with 12 impacted nodes across 6 modules.
- Existing tests already cover wildcard/glob behavior across exec-policy, config permissions, network proxy domain patterns, sandboxing policy transforms, and protocol permission profile round trips.
- The donor ask is generated help/discoverability text, not a proven core behavior gap.

## Closure

The row remains deferred. Do not turn generated-help docs into permission-policy work without concrete failure evidence or a senior-approved docs/help requirement.
