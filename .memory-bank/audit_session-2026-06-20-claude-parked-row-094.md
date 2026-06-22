# Claude Parked Row 094 Review

Date: 2026-06-20

## Decision

Row 094 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 094 says to use this only for diagnostics, not context mutation.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 094 proposes keeping token budget disabled for subagents by default under `core/src/agent`.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `ontocode-rs/core/src/agent/control.rs` owns multi-agent spawn/resume/send/list/status behavior and subagent usage-hint injection.
- Existing agent/control and config tests cover usage-hint defaults and disabled-hint behavior.
- No current token-budget field, diagnostic-only subagent budget default, or owner-local failing test gap was found in the agent owner.
- Goal token budgets and continuation behavior are separate persisted goal-runtime surfaces, not subagent diagnostics.

## Outcome

No implementation dispatch. Adding subagent token-budget defaults would introduce new continuation/runtime or context behavior instead of hardening an existing diagnostic owner.
