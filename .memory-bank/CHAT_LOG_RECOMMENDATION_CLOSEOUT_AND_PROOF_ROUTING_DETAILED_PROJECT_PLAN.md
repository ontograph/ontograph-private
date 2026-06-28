---
name: Chat Log Recommendation Closeout And Proof Routing Detailed Project Plan
description: Detailed existing-owner implementation plan for ADR_CHAT_LOG_RECOMMENDATION_CLOSEOUT_AND_PROOF_ROUTING.md
type: project_plan
date: 2026-06-27
status: active
---

# Chat Log Recommendation Closeout And Proof Routing Detailed Project Plan

## Goal

Implement the accepted closeout-and-proof-routing ADR in the narrowest architecture-safe way:

- make no-dispatch closure deterministic
- route the user to the live authority artifact first
- name the smallest missing proof artifact for evidence-gated rows
- fail clearly on request-shape defects before blaming model choice

This plan is intentionally existing-owner only. It must improve recommendation behavior, especially for smaller models, without creating a second planner, queue runtime, donor registry, or model-routing stack.

## Status

Reviewed.

This file now records two conclusions:

- the recommendation-layer closeout plan remains valid, but current source already satisfies its active reopen gates unless a fresh failing closeout appears
- the correct first implementation path for structured output for agents is the existing agent harness, not operational-evidence first

This file defines the implementation path. It does not claim the work is complete.

Phase 0 is a hard dispatch gate. Do not start Phase 1 through Phase 4 unless current source, current artifacts, or a fresh reproduction proves that one of `R1` through `R4` is still broken.

Current baseline:

- `R1` is covered by the accepted prompt/closeout guardrail unless a fresh no-dispatch output still asks the user to continue.
- `R2` is already implemented for the three known donor left-over files unless a new or edited donor artifact again reads like a live queue.
- `R3` is satisfied for the current sub-agent donor family unless a current closure note gives vague blocked wording where a concrete proof artifact is known.
- `R4` stays closed unless a current reproduction shows alias fallback, truncated model ids, or dropped `namespace`.

Review correction:

- Do not treat `operational_evidence` as the first owner for agent structured output.
- The actual structured-output harness already exists in the per-turn request path through `Prompt.output_schema`, `final_output_json_schema`, and `create_text_param_for_request`.
- The concrete current gap is in `agent_jobs`: worker jobs persist `output_schema_json`, but the worker turn only receives that schema as prompt prose and the report path still accepts any object.

## Source Authority

Primary authority:

- [ADR_CHAT_LOG_RECOMMENDATION_CLOSEOUT_AND_PROOF_ROUTING.md](ADR_CHAT_LOG_RECOMMENDATION_CLOSEOUT_AND_PROOF_ROUTING.md)

Already accepted prerequisite:

- [ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md](ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md)

Current implementation/status note:

- [ONTOCODE_CHAT_LOG_RECOMMENDATION_IMPLEMENTATION.md](ONTOCODE_CHAT_LOG_RECOMMENDATION_IMPLEMENTATION.md)

Related live or donor artifacts that may need routing updates:

- [HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md](HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md)
- [OPENCLAW_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md](OPENCLAW_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md)
- [CLAUDE_CODE_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md](CLAUDE_CODE_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md)
- [ADR_CUSTOM_SUBAGENT_MODELS_TRACKING.md](ADR_CUSTOM_SUBAGENT_MODELS_TRACKING.md)

## Scope

In scope:

- structured closeout fields carried through the existing operational-evidence runtime
- prompt/rendering integration that prefers structured closeout facts when present
- donor and closure artifact routing that points to the current authority first
- proof-artifact wording that reopens one exact row instead of restating a backlog
- narrow request-shape validation for exact model ids and preserved `namespace`

Out of scope:

- new planner or manager-loop daemon
- new queue store or donor-state runtime
- second model registry, provider parser, or sub-agent routing layer
- broad app-server/API/schema work only to expose recommendation metadata
- reopening blocked donor rows without their exact evidence gates

## Why This Plan Exists

The accepted ADR identified four recurring failure modes:

1. after `no-dispatch`, the system can still drift into a stale `continue` handoff
2. donor review artifacts can read like live queues unless they point to the current authority first
3. blocked rows can be restated too broadly instead of naming one concrete missing proof artifact
4. model-routing failures can be misdiagnosed as model-quality issues when the real defect is request shape

The strongest fix for smaller models is to reduce inference pressure. The runtime should carry bounded closure facts so the final response can render from explicit state rather than prose-only reasoning.

## Current Owner Map

### Closeout prompt owner

- [ontocode-rs/protocol/src/prompts/base_instructions/default.md](../ontocode-rs/protocol/src/prompts/base_instructions/default.md)

Current role:

- carries the manager-loop rule to end with `nothing left in scope` when no implementation-ready task remains

### Structured closure runtime owner

- [ontocode-rs/state/src/runtime/operational_evidence.rs](../ontocode-rs/state/src/runtime/operational_evidence.rs)
- [ontocode-rs/state/src/runtime/operational_evidence_tests.rs](../ontocode-rs/state/src/runtime/operational_evidence_tests.rs)

Current role:

- already evaluates operational closure state and is the correct backbone for new bounded closeout fields

### Actual structured-output harness owners

- [ontocode-rs/core/src/client_common.rs](../ontocode-rs/core/src/client_common.rs)
- [ontocode-rs/core/src/session/turn.rs](../ontocode-rs/core/src/session/turn.rs)
- [ontocode-rs/codex-api/src/common.rs](../ontocode-rs/codex-api/src/common.rs)
- [ontocode-rs/core/src/guardian/review_session.rs](../ontocode-rs/core/src/guardian/review_session.rs)
- [ontocode-rs/core/src/tools/handlers/agent_jobs.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs.rs)
- [ontocode-rs/core/src/tools/handlers/agent_jobs_spec.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs_spec.rs)
- [ontocode-rs/state/src/model/agent_job.rs](../ontocode-rs/state/src/model/agent_job.rs)

Current role:

- `Prompt.output_schema` and `final_output_json_schema` already carry strict structured-output contracts into model turns
- guardian review already proves the child-turn pattern by submitting a turn with `final_output_json_schema`
- `agent_jobs` already persists per-job `output_schema_json`, but does not yet route that schema through the real turn harness or enforce it on `report_agent_job_result`

### Request-shape validation owners

- [ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs](../ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs)
- [ontocode-rs/core/src/client_common_tests.rs](../ontocode-rs/core/src/client_common_tests.rs)

Current role:

- already own exact visible model string checks and namespace-restoration regressions

### Tracking and donor-routing owners

- `.memory-bank` closure notes
- `.memory-bank` tracker files
- donor review files listed above

Current role:

- carry human-readable queue state and reopen-gate wording

## Challenge Review

This plan rejects broader solutions on purpose.

Rejected expansions:

- a dedicated recommendation engine
- a new closeout planner service
- donor-specific runtime queues
- a second selector parser for child models
- generic metadata plumbing across every protocol surface

Reason:

- the repo already has the right owners
- the accepted ADR already narrowed the fix
- recommendation-layer code should not reopen without fresh failing evidence
- agent structured output should extend the existing turn/output-schema harness before any operational-evidence expansion

## Implementation Principles

Every implementation step must pass both checks:

- it adds real closure, routing, proof, or validation value
- it extends the existing owner instead of introducing a parallel one

Additional rules:

- prefer bounded string/enumeration fields over free-form narrative
- keep all new closeout facts optional and fail-closed
- do not make donor files look like live backlogs
- keep small-model support explicit: render from structured state whenever available

## Required Structured Fields

The runtime-backed closeout result may add these bounded fields:

- `authority_artifact`
- `reopen_gate`
- `missing_proof_kind`
- `missing_proof_artifact`

Preferred proof-artifact kinds:

- `failing-test`
- `ux-transcript`
- `provider-id-proof`
- `role-config-ownership-proof`
- `runtime-overhead-proof`

These names may be adjusted to fit existing Rust type conventions, but the semantics must stay one-to-one with the ADR.

## Structured Field Source Of Truth

Do not add free-form closeout fields until their source is explicit.

Minimum source rules:

- `authority_artifact` must come from the current tracker, closure artifact, or ADR path that owns status for the task.
- `reopen_gate` must come from the current closure/tracking text or from a deterministic mapping from missing gate to reopen gate.
- `missing_proof_kind` should first be derived from known `missing_gates` before adding new persisted metadata.
- `missing_proof_artifact` should be a bounded string derived from the proof-kind mapping below or an already-recorded closure artifact.

Initial missing-gate mapping:

- `tests` -> `failing-test`
- `no-code-closure` -> `ux-transcript` or `role-config-ownership-proof`, depending on the closure artifact
- `fresh_target_head`, `fresh_plan_hash`, or `fresh_tracking_hash` -> refresh/reverify the named authority artifact, not a new proof kind
- `dispatch`, `impact`, `implementation`, or `detect-changes` -> keep the existing operational-evidence gate wording unless a closure artifact names a more specific proof

Add new persisted metadata only if this mapping cannot express a current failing case.

## Detailed Phases

## Review Outcome

Phase 0 review found that the recommendation layer does not currently justify Phase 1 through Phase 4 code work by itself.

What remains true:

- recommendation closeout still belongs to prompt, donor-routing, and request-shape owners
- any recommendation-layer code reopen still requires a fresh failing closeout or request-shape reproduction

What changed from the earlier implementation reading:

- if the user asks for structured output for agents, the first valid slice is not `OperationalEvidenceTaskClosureResult`
- the first valid slice is `agent_jobs` worker enforcement through the existing `final_output_json_schema` turn path

This prevents two bad outcomes:

- adding synthetic closeout metadata without a current runtime source of truth
- leaving the real agent harness gap unfixed while improving only recommendation prose

## Phase 0: Preflight And Owner Confirmation

Goal:

- confirm that the accepted ADR still matches current code and tracker reality before implementation begins
- decide whether any later phase is actually open

Tasks:

- inspect the current prompt closeout rule in `default.md`
- inspect the current closure result shape in `operational_evidence.rs`
- inspect the current tests around closure evaluation
- inspect the current request-shape regressions for exact model ids and namespace restoration
- inspect donor/closure files to see which ones still read like live queues

Acceptance:

- one short owner map is recorded in the implementation notes or tracking artifact
- no new owner is proposed for closeout/runtime/routing/validation
- any candidate file not needing change is explicitly left untouched
- every `R1` through `R4` row is classified as `covered`, `implementation-ready`, `proof-only`, `blocked`, or `closed`

Stop if:

- the current code already carries all four closeout fields and renders them correctly
- donor files already point to current authority first
- current request-shape regressions already reject the misdiagnosed cases
- current implementation status still matches [ONTOCODE_CHAT_LOG_RECOMMENDATION_IMPLEMENTATION.md](ONTOCODE_CHAT_LOG_RECOMMENDATION_IMPLEMENTATION.md)

Dispatch rule:

- If Phase 0 finds no current regression, close with `nothing left in scope`.
- If Phase 0 finds a regression, open only the smallest matching phase and record the exact failing evidence before code work begins.

## Phase 1: Structured Closure State In Operational Evidence

Goal:

- make structured closeout state the backbone for exact closure answers

Tasks:

- extend the existing closure result type in `operational_evidence.rs`
- add bounded optional fields for authority, reopen gate, proof kind, and proof artifact only after the source-of-truth rules above identify where each value comes from
- ensure closure evaluation can populate those fields without introducing donor-specific runtime state
- keep docs/design-only and proof-only closures distinct from implementation-ready closures
- preserve current behavior for tasks that do not need extra fields

Acceptance:

- closure results can carry all four fields together
- code paths without structured metadata still work
- closure logic does not introduce a second planner or queue owner
- `nothing left in scope` remains the terminal state when no implementation-ready task exists

Additional stop rule:

- do not start this phase solely to unblock agent structured output
- start this phase only when a fresh recommendation-layer closeout failure proves the existing `missing_gates` result is insufficient

Required tests:

- closure result with exact `authority_artifact`
- closure result with exact `reopen_gate`
- closure result with one concrete `missing_proof_kind` plus `missing_proof_artifact`
- closure result for docs/design-only tasks that stays closed and does not fake implementation readiness

## Phase 2: Prompt And Rendering Integration

Goal:

- render closeout answers from structured runtime facts instead of relying on prose inference alone

Tasks:

- keep the existing closeout rule in `default.md`
- add prompt wording that prefers structured closeout fields when present
- ensure the final answer path uses exact reopen gates after `no-dispatch`
- ensure no stale `continue` handoff survives when structured closeout says the queue is closed

Acceptance:

- no-dispatch answers include the exact reopen gate immediately
- if the queue is closed, the answer says `nothing left in scope`
- the response does not ask the user to continue when there is no active next task
- smaller models can produce the correct answer from bounded fields without reconstructing state from donor prose

Required tests:

- focused prompt/render regression proving no stale `continue` handoff after `no-dispatch`
- regression proving exact gate text is preferred over generic blocked wording

## Phase 3: Donor And Authority Routing Hardening

Goal:

- make donor review artifacts point to live authority first and stop acting like live queues

Tasks:

- add top-of-file current-state pointers where needed in donor review files
- point each donor artifact to the closure note or active tracker that actually owns status
- standardize wording that donor files are evidence sources, not authority queues
- ensure evidence-gated rows recommend one smallest missing proof artifact

Acceptance:

- reopened donor reviews direct the user to the live authority artifact first
- donor files do not imply they are the current implementation queue
- evidence-gated rows name one concrete missing proof artifact instead of a generic blocker paragraph

Proof-artifact routing rules:

- coverage gap: recommend a failing test
- ambiguity gap: recommend a concrete UX transcript
- selector parsing gap: recommend provider-id inventory/proof
- default-model-policy gap: recommend role/config ownership proof
- reuse-path claim gap: recommend runtime-overhead proof

Stop if:

- the donor artifact already has a current-state pointer and one exact proof artifact per gated row
- the known HarnessX, OpenClaw, and Claude donor files remain in their current state, because they already point to live authority and say they are not live queues

## Phase 4: Request-Shape Validation Hardening

Goal:

- reject malformed sub-agent dispatch input before it gets explained away as a model-choice issue

Tasks:

- verify the existing exact-model-id surface still preserves the full requested model string
- verify forwarded or replayed `spawn_agent` calls preserve `namespace`
- add narrow validation or regression coverage where truncated ids or dropped namespaces can still slip through
- keep all changes inside current sub-agent/request-shape owners

Acceptance:

- truncated model ids fail clearly
- exact configured model ids survive request formatting and forwarding
- replayed or forwarded `spawn_agent` calls preserve `namespace`
- no second model parser or dispatch layer is introduced

Required tests:

- exact visible model string regression
- namespace-preservation regression
- formatted-input restoration regression for ambiguous function-call namespace

## Phase 5: Verification And Closure

Goal:

- prove the new behavior works without widening scope

Tasks:

- run `just fmt` after code edits
- run focused project tests for the crates touched by the implementation
- run prompt/protocol tests if prompt text changed
- run narrow diff checks on all touched files
- update the live tracking/closure artifact only if the implementation actually lands

Acceptance:

- targeted tests pass for each touched owner
- no unexpected architectural spread is introduced
- the implementation status note or tracker records whether each of `R1` through `R4` is covered, completed, or still evidence-gated

## Harness Follow-On: Structured Output For Agents

This follow-on is implementation-ready even when recommendation-layer closeout stays closed.

Implementation note, 2026-06-27:

- complete: `agent_jobs` worker turns now pass `output_schema_json` through the real `final_output_json_schema` turn path
- complete: reported worker results are now validated against the stored schema before they can be accepted as completed work
- complete: focused runtime and suite coverage now proves schema propagation, schema-invalid rejection, and schema-valid completion/export

### Goal

- make agent workers use the real structured-output harness instead of treating JSON Schema as prompt prose only

### Existing Owner

- `agent_jobs` worker spawning and result reporting
- current turn/request `final_output_json_schema` path

### Current Gap

- `spawn_agents_on_csv` accepts `output_schema` and persists it as `output_schema_json`
- `build_worker_prompt` only renders that schema into text
- worker turns are spawned without `final_output_json_schema`
- `report_agent_job_result` accepts any JSON object instead of validating against the stored schema

### Unblock Decision

Do not wait on recommendation closeout metadata.

Use the existing structured-output harness already proven by guardian review:

- submit worker turns with `final_output_json_schema`
- keep the worker prompt short and task-specific
- validate reported results against the same stored schema when one is present

### Implementation Slice

1. thread `job.output_schema_json` into the spawned worker turn through the existing `final_output_json_schema` path
2. keep `build_worker_prompt` as task framing, but stop using prompt prose as the only schema contract
3. enforce `report_agent_job_result.result` against `job.output_schema_json` when present
4. preserve current permissive behavior when no schema is supplied

### Acceptance

- worker requests include the structured output schema through the normal request pipeline
- a schema-invalid worker result is rejected and not accepted as completed work
- a schema-valid worker result still completes and exports correctly
- no new planner, queue runtime, or app-server API is introduced

### Required Tests

- focused `agent_jobs` test proving spawned worker requests include `codex_output_schema`
- negative `agent_jobs` test proving invalid `report_agent_job_result` payloads are rejected when a schema is present
- positive `agent_jobs` test proving valid structured results still export successfully

### Verification

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-core spawn_agents_on_csv_runs_and_exports`
- add and run the focused `agent_jobs` tests for worker schema propagation and schema-invalid result rejection
- `CARGO_BUILD_JOBS=8 just fmt`
- `git diff --check -- ontocode-rs/core/src/tools/handlers/agent_jobs.rs ontocode-rs/core/src/tools/handlers/agent_jobs_spec.rs ontocode-rs/core/tests/suite/agent_jobs.rs`

## Task Breakdown By Recommendation

### `R1`: exact closeout after `no-dispatch`

Primary owner:

- `default.md` plus the current closeout/render path

Minimum acceptable implementation:

- exact reopen gate is emitted immediately
- no stale `continue` handoff remains

### `R2`: route donor files to live authority first

Primary owner:

- `.memory-bank` donor review artifacts

Minimum acceptable implementation:

- top-of-file current-state pointer
- explicit note that the donor artifact is not the live queue

### `R3`: recommend one smallest missing proof artifact

Primary owner:

- operational-evidence closeout fields plus closure/tracking wording

Minimum acceptable implementation:

- one exact proof artifact named for each evidence-gated closure
- no generic "blocked pending evidence" wording when a concrete proof type is known

### `R4`: validate request shape before blaming model choice

Primary owner:

- current sub-agent dispatch/request-shape tests and validation paths

Minimum acceptable implementation:

- full model id preserved
- `namespace` preserved or restored correctly

## Non-Goals

- Do not turn this into a generic project-planning framework.
- Do not add a public API for recommendation metadata.
- Do not add donor-row persistence or reopen history storage.
- Do not add heuristic model-family guessing to compensate for malformed ids.
- Do not reopen already-closed donor ideas without fresh proof.

## Verification Plan

Minimum verification set depends on the touched owners.

If `ontocode-rs/state` changes:

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-state evaluate_operational_evidence_task_closure`

If `ontocode-rs/protocol` prompt text changes:

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-protocol`

If `ontocode-rs/core` request-shape validation changes:

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-core spawn_agent_tool_preserves_exact_visible_model_string_and_namespace formatted_input_restores_unambiguous_function_call_namespace`

Always:

- run `CARGO_BUILD_JOBS=8 just fmt`
- run `git diff --check -- <touched files>`

Before commit:

- run `gn_verify_diff` or equivalent OntoIndex diff verification for the changed scope

## Deliverables

Implementation deliverables, if the plan is executed:

- bounded Rust/runtime changes in existing owners only
- focused regression tests in current test owners
- any necessary top-of-file authority-routing notes in donor artifacts
- updated implementation/tracking status that reflects real completion state

Planning deliverables from this document:

- one execution order for `R1` through `R4`
- explicit acceptance criteria
- explicit stop conditions
- explicit non-goals to keep the change narrow

## Recommended Execution Order

1. Phase 0 owner confirmation
2. Stop with `nothing left in scope` if Phase 0 finds no current regression
3. If the requested work is agent structured output, execute the `agent_jobs` harness follow-on first
4. Phase 1 structured closure state, only if structured closeout data is missing in a current failing closeout
5. Phase 2 prompt/render integration, only if the current output still produces a stale continue handoff or generic gate
6. Phase 3 donor/authority routing hardening, only if a current donor artifact reads like a live queue
7. Phase 4 request-shape validation hardening, only with a current model-id or namespace reproduction
8. Phase 5 verification and closeout

Reason for this order:

- for recommendation-layer regressions, structured closure state is still the backbone
- for agent structured output, the existing turn/output-schema harness is the real backbone
- prompt and donor routing should consume structured state, not invent parallel logic
- request-shape hardening is independent enough to stay narrow but still belongs in the same closure-quality family
- the harness follow-on fixes a real current gap without reopening already-covered recommendation work

## Reopen Gates

This plan should only reopen code work when current source proves a real gap.

Valid reopen evidence:

- a fresh no-dispatch output still asks the user to continue
- a donor artifact still behaves like a live queue
- a closure note still uses vague blocked wording when one proof artifact is already known
- a current reproduction shows truncated model ids or dropped `namespace`

Invalid reopen evidence:

- historical memory alone without a current reproduction
- donor ideas that do not extend current owners
- generic dissatisfaction with model quality

## Stop Conditions

Close with `nothing left in scope` if all of the following are true:

- no implementation-ready recommendation task remains
- current source already enforces the closeout rule
- donor/tracker artifacts already route to current authority
- evidence-gated rows already name one smallest proof artifact
- no current request-shape reproduction justifies additional code

At that point, any remaining work belongs to future evidence-driven reopen events, not this plan.
