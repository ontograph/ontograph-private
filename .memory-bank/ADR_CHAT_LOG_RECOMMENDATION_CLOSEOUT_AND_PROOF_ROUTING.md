# ADR: Chat Log Recommendation Closeout And Proof Routing

## Status

Accepted

## Date

2026-06-27

## Context

Recent chat-log review found three recurring failure modes in bounded manager loops and donor-backed reopen queues:

1. after `no-dispatch`, the system can still drift into a stale `continue` handoff instead of answering with the exact reopen gate
2. donor review files can be read like live queues unless they point to the current closure artifact or active tracker first
3. blocked rows can be restated too generically instead of naming the smallest missing proof artifact
4. model-routing failures can be misdiagnosed as "bad model choice" when the real defect is request shape, such as a truncated model id or dropped `namespace`

Current owner evidence, using OntoIndex with stale-index caution plus direct source reads:

- Prompt closeout already says `nothing left in scope` when no implementation-ready task remains in [ontocode-rs/protocol/src/prompts/base_instructions/default.md](../ontocode-rs/protocol/src/prompts/base_instructions/default.md:121).
- Structured closure evaluation already exists in [ontocode-rs/state/src/runtime/operational_evidence.rs](../ontocode-rs/state/src/runtime/operational_evidence.rs:881) with focused tests in [ontocode-rs/state/src/runtime/operational_evidence_tests.rs](../ontocode-rs/state/src/runtime/operational_evidence_tests.rs:1686).
- Exact model-string and tool-namespace rendering already have a focused regression owner in [ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs](../ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs:120).
- Function-call namespace restoration already has an owner in [ontocode-rs/core/src/client_common_tests.rs](../ontocode-rs/core/src/client_common_tests.rs:173).
- Donor left-over files are already narrowed to evidence-gated rows only:
  - [HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md](HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md:8)
  - [OPENCLAW_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md](OPENCLAW_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md:8)
  - [CLAUDE_CODE_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md](CLAUDE_CODE_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md:8)

The accepted shared guardrail in [ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md](ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md:24) already fixes the completion-boundary ping-pong and already sets the rule that loops with no implementation-ready task must end with `nothing left in scope`.

## Options Considered

### Option 1: Prompt-only closeout tightening

Use the existing prompt owner to require:

- exact reopen gate after `no-dispatch`
- no stale `continue` handoff
- explicit `nothing left in scope`

Pros:

- smallest write set
- reuses the existing prompt owner

Cons:

- relies on model compliance
- weaker for smaller models and noisy turns

### Option 2: Structured closeout payload in the existing operational-evidence runtime

Extend the existing closure/result owner with bounded fields such as:

- `authority_artifact`
- `reopen_gate`
- `missing_proof_kind`
- `missing_proof_artifact`

Pros:

- best support for smaller models because the final response can be rendered from structured state instead of inferred from prompt prose
- one existing runtime owner already evaluates closure state
- allows exact authority, gate, and proof artifact to be carried together

Cons:

- more implementation than prompt-only
- requires careful test coverage so docs-only and code tasks still close correctly

### Option 3: Donor/tracker artifact hardening

Standardize donor and closure files so they always point to live authority first and always name the smallest missing proof artifact for evidence-gated rows.

Pros:

- cheap
- reduces queue-state drift immediately
- good fit for human-driven manager loops

Cons:

- documentation-driven, not runtime-enforced

### Option 4: Request-shape validation for exact model id and preserved namespace

Tighten validation around sub-agent dispatch so request-shape defects fail clearly.

Pros:

- addresses the real cause of a recurring recommendation mistake
- existing focused test owners already exist

Cons:

- does not solve no-dispatch closeout or donor authority drift by itself

## Decision

Adopt a combined existing-owner plan:

- Option 2 is the backbone
- Option 1 stays as the model-visible closeout rule
- Option 3 stays as the tracker/donor hygiene rule
- Option 4 stays as the request-shape guard

Why this combination:

- Option 2 gives the strongest support for smaller models because it reduces reliance on free-form reasoning and lets the runtime hand the model bounded, explicit closeout facts.
- Option 1 keeps the shared prompt aligned with the accepted loop policy.
- Option 3 prevents human/operator drift when older donor files are reopened.
- Option 4 prevents false "bad model choice" diagnoses when the actual bug is malformed dispatch input.

## Implementation Direction

### R1: closeout rendering

Keep the existing prompt rule in [default.md](../ontocode-rs/protocol/src/prompts/base_instructions/default.md:121), but render from structured closure state whenever that state is available.

Required closeout fields:

- queue outcome
- authority artifact
- exact reopen gate
- smallest missing proof artifact

### R2: structured closure state

Extend the existing operational-evidence closure result rather than creating a second planner or queue runtime.

Allowed new bounded fields:

- `authority_artifact`
- `reopen_gate`
- `missing_proof_kind`
- `missing_proof_artifact`

Not allowed:

- new planner service
- second queue owner
- donor-specific runtime state

### R3: donor/tracker authority routing

Keep donor review files explicitly marked as donor evidence and require them to point at the live authority artifact first.

For evidence-gated rows, closure artifacts should prefer one concrete missing proof artifact:

- failing test for coverage gaps
- concrete UX transcript for ambiguity gaps
- provider-id inventory/proof for selector parsing gaps
- role/config ownership proof for default-model-policy gaps
- runtime-overhead proof for reuse-path claims

### R4: request-shape validation

Fail clearly when:

- a sub-agent request uses a truncated model id instead of an exact id
- a forwarded or replayed `spawn_agent` function call loses its `namespace`

This must extend existing validation and test owners, not create a second sub-agent routing layer.

## Non-Goals

- Do not create a new planner, manager-loop daemon, or queue store.
- Do not add a second model registry or provider parser.
- Do not widen app-server or collaboration APIs just to surface recommendation metadata.
- Do not promote donor left-over rows to implementation without their exact evidence gates.

## Verification Plan

- Prompt/closeout tests proving no stale `continue` handoff after `no-dispatch`.
- Operational-evidence tests proving closure results can carry authority, gate, and proof artifact fields.
- Focused regressions for exact visible model string and namespace preservation.
- Focused regressions for unambiguous namespace restoration on formatted input.
- Narrow `git diff --check` on the touched files.

## Consequences

Manager-loop closeout becomes more deterministic and less sensitive to model capability.

Smaller models benefit because they no longer need to infer:

- whether a donor file is live authority
- which reopen gate applies
- which proof artifact is the next valid ask

Instead, those facts can be carried through existing runtime and tracker owners in bounded form.
