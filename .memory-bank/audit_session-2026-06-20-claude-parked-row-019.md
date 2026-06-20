name: Claude Parked Row 019 Review
desc: Row 019 stays parked because typed ask-user choices already exist and the tool-deny path is a separate covered approval surface
type: audit_session
date: 2026-06-20

# Claude Parked Row 019 Review

## Decision

Row 019 stays parked. No promotion packet was written.

## Evidence

- Parked ADR row 019 says: `Extend current tool deny path, not a new policy layer.`
- Donor source row 019 says: `Add ask-user tool with typed choices` in `protocol/src/request_user_input.rs`.
- The two sources do not identify one clean missing behavior.
- `ontocode-rs/protocol/src/request_user_input.rs` already defines `RequestUserInputQuestionOption` and `RequestUserInputQuestion.options`.
- `ontocode-rs/core/src/tools/handlers/request_user_input_spec_tests.rs` already covers the model-visible questions/options schema and mode availability wording.
- `ontocode-rs/core/tests/suite/request_user_input.rs` covers round trip behavior and mode rejection.
- MCP approval prompts already build typed `RequestUserInputQuestionOption` choices for accept, cancel, session remember, and persistent remember.
- OntoIndex reports `request_user_input.rs` as a 56-line protocol model file and `RequestUserInputHandler` as LOW impact with direct Tools/Handlers coverage.
- Tool-deny behavior is spread across existing approval owners: MCP approvals, request-permissions, network denial, and guardian review. No single existing-owner failing test gap was found.

## Closure

The donor typed-choice behavior is already implemented, and the parked deny-path idea cannot be reduced to one local request-user-input test. The row remains parked.
