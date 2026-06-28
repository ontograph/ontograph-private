---
name: OpenHuman Donor Core Extension Implementation Project Plan
description: Current queue state and implementation order for the six retained OpenHuman donor rows after OntoIndex challenge
type: project_plan
date: 2026-06-28
status: partial-no-dispatch
---

# OpenHuman Donor Core Extension Implementation Project Plan

## Goal

Implement only the retained OpenHuman donor rows that still satisfy the repo bar:

1. new relative to current Ontocode source
2. extending an existing core owner

This is an implementation order, not approval to build all six rows at once.

Keep one active slice at a time.

Current queue state on `2026-06-28`:

- `OH-P3` is completed.
- no other row is implementation-ready from current source plus donor evidence.
- there is no `active_next_task`; the queue is currently no-dispatch until one exact reopen gate is satisfied.

## Source Review

Authoritative donor review:

- [OPENHUMAN_DONOR_2000_USEFUL_APPROACHES_REVIEW.md](OPENHUMAN_DONOR_2000_USEFUL_APPROACHES_REVIEW.md)

Retained rows from that review:

- `OH-01` pre-sampling context-enrichment pass
- `OH-02` wrapped-bundle salvage
- `OH-03` read-only scout helper
- `OH-04` recoverable lossy compaction
- `OH-05` content-kind-aware reduction
- `OH-06` unattended approval clamp

## OntoIndex Evidence

OntoIndex was used before writing this plan.

Current index state used for this challenge pass:

- indexed commit matches HEAD
- dirty worktree is large, so owner/risk checks are reliable but broad current-worktree conclusions are not

Relevant current owners already in place:

- `ontocode-rs/core/src/session/turn.rs`
  - owns `run_turn`
  - owns `run_pre_sampling_compact`
- `ontocode-rs/core/src/session/mod.rs`
  - owns initial context assembly and skills injection
- `ontocode-rs/core/src/context/*`
  - owns bounded contextual fragments
- `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`
  - owns `run_agent_job_loop`
  - owns `build_worker_prompt`
- `ontocode-rs/core/src/tools/planning/native.rs`
  - owns agent-job tool gating
- `ontocode-rs/ext/goal/src/runtime.rs`
  - owns `continue_if_idle`
- `ontocode-rs/core/src/tools/context_exec_output.rs`
  - owns formatted exec tool output payloads
- `ontocode-rs/core/src/state/additional_context.rs`
  - owns merge-and-emit behavior for `additional_context`
- `ontocode-rs/core/src/session/handlers.rs`
  - owns turn-start injection of `additional_context`
- `ontocode-rs/core/src/context/internal_model_context.rs`
  - owns hidden runtime context fragments
- `ontocode-rs/tools/src/tool_output.rs`
  - owns the `post_tool_use_response` tool-output contract
- `ontocode-rs/tools/src/json_schema.rs`
  - already owns schema compaction
- `ontocode-rs/core/src/tools/output_reducer_tests.rs`
  - existing reducer coverage seam

Current-source coverage that constrains implementation:

- pre-sampling compaction already exists, so `OH-01` must be an enrichment pass, not a second compaction path
- `additional_context` already has a concrete path through `state.additional_context.merge(...)`, `session/handlers.rs`, `session/mod.rs`, and `core/tests/suite/additional_context.rs`, so `OH-P1` must stay on that path instead of inventing a new parser surface
- helper skill suppression already exists in guardian/subagent flows, so no new work is needed there
- goal continuation already exists, so `OH-06` is only about stricter unattended approval behavior
- `run_agent_job_loop` and `build_worker_prompt` have LOW blast radius, but the only proven upstream caller is `spawn_agents_on_csv`, so `OH-P4` is not implementation-ready unless a non-CSV existing caller is first proven
- `ExecCommandToolOutput.post_tool_use_response` has isolated impact, so `OH-P5` should stay inside existing tool-output and reducer owners and must not expand into a new storage/API surface

## Stage Readiness

Completed:

1. `OH-P3` unattended approval clamp

Not implementation-ready:

1. `OH-P1` only if current source proves wrapped bounded fragments can be salvaged on the existing `additional_context` / context-fragment path without a new parser product, new public syntax, or generalized prose-envelope support
2. `OH-P2` only if it can be expressed as existing hidden-context injection before sampling and does not require a second pre-turn pipeline or enrichment registry
3. `OH-P4` only if an existing non-CSV caller and existing result sink are identified
4. `OH-P5A` only if recoverability stays turn-local and does not require new persistence, public handles, or cross-turn lookup
5. `OH-P5B` only after `OH-P5A` or after proof that current reducer seams can preserve kind-specific markers without a retrieval layer

## 2026-06-28 OH-P3 Closure

OntoIndex-grounded challenge and current-source verification promoted `OH-P3` ahead of the earlier `OH-P1` default-first assumption.

Why the queue changed:

- donor/source review did not prove that `OH-P1` is a narrow parser-hardening fix inside the already-owned `additional_context` path
- current source already owns idle continuation, so `OH-P3` had a concrete existing-owner extension path
- impact checks kept the change out of high-risk session-construction owners

Latest impact evidence used for this closure:

- `continue_if_idle`: `LOW`
- `Session.try_start_turn_if_idle`: `LOW`
- `CodexThread.try_start_turn_if_idle`: `LOW`
- `Session.maybe_start_goal_continuation_turn`: `LOW`
- `ToolOrchestrator.run`: `HIGH`
- `Session.strict_auto_review_enabled_for_turn`: `HIGH`
- `Session.new_default_turn_with_sub_id`: `CRITICAL`

Implementation shape that landed:

- mark idle goal-continuation turns with a turn-state bit: `unattended_read_only_filesystem`
- set that bit in `Session::maybe_start_goal_continuation_turn`
- derive an ephemeral read-only filesystem permission profile inside `ToolOrchestrator::run`
- downgrade write filesystem entries to read entries for restricted policies
- clamp unrestricted / external filesystem policies to `read_only()`
- preserve current network policy by leaving `TurnContext.network` unchanged
- do not mutate stored session settings
- do not widen `strict_auto_review`
- do not change `Session.new_default_turn_with_sub_id`

Files changed for the landed slice:

- `ontocode-rs/core/src/goals.rs`
- `ontocode-rs/core/src/session/mod.rs`
- `ontocode-rs/core/src/session/tests.rs`
- `ontocode-rs/core/src/state/turn.rs`
- `ontocode-rs/core/src/tools/orchestrator.rs`
- `ontocode-rs/core/src/tools/orchestrator_tests.rs`

Scoped verification:

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core active_goal_idle_continuation_marks_turn_read_only_for_tools unattended_read_only_permission_profile_preserves_network_policy clamp_file_system_policy_to_read_only_downgrades_writes_and_preserves_denies`
- `git diff --check -- .memory-bank/OPENHUMAN_DONOR_CORE_EXTENSION_IMPLEMENTATION_PROJECT_PLAN.md ontocode-rs/core/src/goals.rs ontocode-rs/core/src/session/mod.rs ontocode-rs/core/src/session/tests.rs ontocode-rs/core/src/state/turn.rs ontocode-rs/core/src/tools/orchestrator.rs ontocode-rs/core/src/tools/orchestrator_tests.rs`

Verification result:

- focused tests passed
- `gn_verify_diff` stayed noisy because the worktree is broadly dirty; it still confirmed the expected touched core files are part of the current diff

## Non-Goals

- Do not port OpenHuman.
- Do not add voice, meeting-agent, mascot, dashboard, theme, or personal-data ingestion features.
- Do not add a new tool registry, worker framework, context service, memory platform, or background scheduler.
- Do not add app-server APIs, schemas, or config keys unless a later ADR explicitly requires them.
- Do not add donor runtime dependencies.
- Do not create a second compaction pipeline beside existing output/context owners.
- Do not persist new payload caches unless a retained slice proves it is necessary.

## Delivery Rule

Use the smallest slice that adds real behavior.

Order the work once a row reopens:

1. unattended safety clamp is already complete
2. parser hardening only after the existing bounded-fragment path is proven
3. context enrichment only after a concrete existing injection path is proven
4. helper-worker specialization only after a non-CSV caller is proven
5. output compaction only after current output owners are proven stable and recoverability remains owner-local

## Stage 0: Baseline And Impact

Purpose: force owner inspection before any code edit.

Files to read:

- `ontocode-rs/core/src/session/turn.rs`
- `ontocode-rs/core/src/session/mod.rs`
- `ontocode-rs/core/src/context/*`
- `ontocode-rs/core/src/context/internal_model_context.rs`
- `ontocode-rs/core/src/state/additional_context.rs`
- `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`
- `ontocode-rs/core/src/tools/planning/native.rs`
- `ontocode-rs/ext/goal/src/runtime.rs`
- `ontocode-rs/core/src/tools/context_exec_output.rs`
- `ontocode-rs/core/src/tools/output_reducer_tests.rs`

Required OntoIndex checks before editing any symbol:

```text
impact: run_turn
impact: run_pre_sampling_compact
impact: continue_if_idle
impact: run_agent_job_loop
impact: build_worker_prompt
impact: ExecCommandToolOutput.post_tool_use_response
```

Done when:

- every edited symbol has an impact summary
- no HIGH/CRITICAL warning is ignored
- one active slice is chosen

## Stage 1: OH-P1 Wrapped-Bundle Salvage

Status: gated, no-dispatch

Purpose: take the cheapest safe win first.

Target:

- accept bounded `additional_context` or hook-style context fragments even when wrapped in extra prose or envelope text

Likely owners:

- `ontocode-rs/core/src/state/additional_context.rs`
- `ontocode-rs/core/src/session/handlers.rs`
- `ontocode-rs/core/src/session/mod.rs`
- `ontocode-rs/core/tests/suite/additional_context.rs`
- `ontocode-rs/core/src/context/contextual_user_message.rs` only if fragment recognition must change

Rules:

- salvage only bounded fragments already routed through existing context-fragment owners
- fail closed when the payload is ambiguous
- do not generalize to arbitrary user text that merely looks like XML-ish markup
- do not add a new parser product or public syntax
- keep byte/token caps unchanged
- do not change app-server wire shapes unless focused tests prove the current path cannot express the fix

Suggested tests:

- valid fragment with prose prefix/suffix still parses
- malformed wrapper still fails closed
- oversized wrapped fragment still truncates or rejects under current caps

Done when:

- bounded wrapped fragments survive
- ambiguous or oversized payloads still fail closed

## Stage 2: OH-P2 Pre-Sampling Context-Enrichment Pass

Status: gated, no-dispatch

Purpose: add enrichment, not another compaction path.

Target:

- run a deterministic bounded context-enrichment pass before primary sampling when a fresh thread lacks enough local context

Likely owners:

- `ontocode-rs/core/src/session/turn.rs`
- `ontocode-rs/core/src/session/mod.rs`
- `ontocode-rs/core/src/context/internal_model_context.rs`
- `ontocode-rs/core/src/context_manager/*` only for bounded retention/truncation, not as a new enrichment source

Rules:

- reuse existing pre-turn flow
- express enrichment as an existing contextual fragment or internal context injection
- no model-visible new tool
- no web/data sweep
- no second compaction engine
- keep enrichment deterministic and bounded
- if the slice needs a new enrichment source registry or a second pre-turn pipeline, drop it

Suggested tests:

- fresh-thread turn injects enrichment exactly once
- existing context prevents duplicate enrichment
- resumed or already-enriched turns do not double-run

Done when:

- enrichment is a distinct bounded pre-sampling step
- duplicate enrichment is prevented

## Stage 3: OH-P3 Unattended Approval Clamp

Status: completed on `2026-06-28`

Purpose: harden idle continuation before broader helper automation.

Target:

- unattended continuation must inherit stricter approval behavior for irreversible actions than an interactive turn

Likely owners:

- `ontocode-rs/ext/goal/src/runtime.rs`
- existing approval/permission owners in `ontocode-rs/core/src/tools/*`
- prompt/guardian owners only if the clamp cannot be expressed in runtime policy

Rules:

- do not add a new approval system
- fail closed for unattended irreversible actions
- interactive turns must keep current behavior
- keep the clamp local to continuation/background origin, not global

Suggested tests:

- idle continuation can still perform read-only work
- idle continuation cannot cross the new irreversible-action boundary
- interactive turn with same request still follows current approval flow

Done when:

- unattended origin materially changes approval behavior
- interactive approval semantics remain intact

Closure:

- implemented as a turn-state flag plus orchestrator-local filesystem clamp inside existing owners
- idle continuation now retains read-only file access while fail-closing write-capable filesystem policies
- interactive turns keep the current permission path
- network policy is preserved
- no new approval system, prompt lane, or global continuation mode was introduced

## Stage 4: OH-P4 Read-Only Scout Helper

Status: gated, no-dispatch

Purpose: specialize existing worker infrastructure for bounded context scouting.

Challenge gate:

- current source proves `run_agent_job_loop` and `build_worker_prompt` only on the CSV job path
- do not dispatch this slice until an existing non-CSV caller and result sink are identified

Target:

- a scout-style helper mode that is read-only, leaf-only, and hard-capped

Likely owners:

- `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`
- `ontocode-rs/core/src/tools/planning/native.rs`
- `ontocode-rs/core/src/agent/control.rs`
- worker prompt assembly in existing multi-agent/agent-jobs owners only if the caller proof exists

Rules:

- reuse existing worker plumbing
- no new scheduler
- no write tools
- no recursive helper orchestration
- bounded output only
- if this requires turning CSV job owners into a generic helper runtime, stop and drop the slice

Suggested tests:

- scout helper receives only allowed tools
- scout helper cannot mutate files or request escalations
- scout output stays bounded and returns to parent in existing result flow

Done when:

- helper mode exists without new runtime owner
- write/escalation paths are blocked in scout mode

## Stage 5: OH-P5 Recoverable Lossy Compaction

Purpose: extend output shaping without making data loss invisible.

### OH-P5A: Recoverable Lossy Infra

Status: gated, no-dispatch

Target:

- allow lossy reduction only when the original payload remains retrievable through an explicit bounded handle

Likely owners:

- `ontocode-rs/core/src/tools/context_exec_output.rs`
- `ontocode-rs/core/src/tools/registry.rs`
- `ontocode-rs/tools/src/tool_output.rs`
- existing bounded-output helpers

Rules:

- no persistence layer unless truly required
- retrieval handle must be explicit
- no silent dropping of original payloads
- if recoverability needs a new persistent store, public API, or cross-turn handle lookup, stop and drop the slice

Suggested tests:

- reduced payload includes retrieval metadata
- original payload can be recovered through the bounded path
- no handle means no lossy reduction

### OH-P5B: Content-Kind-Aware Reducers

Status: gated, no-dispatch

Target:

- add per-kind reducers for code, diff, and search-like payloads that preserve high-signal markers

Likely owners:

- same output/context owners as OH-P5A
- existing reducer tests in `ontocode-rs/core/src/tools/output_reducer_tests.rs`

Rules:

- preserve signatures, paths, line numbers, and obvious risk markers
- no LLM summarization
- no separate reduction engine

Suggested tests:

- code output keeps signatures and `panic`/`unsafe`/`TODO`-style markers
- diff output keeps hunk anchors and omitted-count summaries
- search output keeps file paths, line numbers, and omitted count

Done when:

- recoverability exists before kind-aware lossy reduction is enabled
- high-signal markers survive every reducer

## Active Queue State

None.

`OH-P3` is the only row that was implementation-ready under the current donor review plus current-source evidence, and it is now complete.

There is no remaining implementation-ready slice in this queue.

Exact reopen gates:

- `OH-P1` reopens only when fresh current-source proof shows wrapped bounded fragments already travel through an existing `additional_context` or context-fragment path and the needed salvage is fail-closed parser hardening rather than a new envelope parser or public syntax.
- `OH-P2` reopens only when a distinct bounded enrichment fragment can be injected in the existing pre-sampling flow without adding a second pre-turn pipeline, a new enrichment registry, or model-visible tooling.
- `OH-P4` reopens only when an existing non-CSV caller and existing result sink are proven for scout-mode work inside current multi-agent owners.
- `OH-P5A` reopens only when recoverable lossy reduction can stay turn-local and explicit without persistence, public handles, or cross-turn lookup.
- `OH-P5B` reopens only after `OH-P5A` lands or current reducer seams prove they can preserve code/search/diff signal without a new retrieval layer.

Without one of those new evidence sources, do not rewrite this queue again.

## Validation

From `ontocode-rs/`:

```bash
CARGO_BUILD_JOBS=8 just fmt
```

Per-slice tests:

- session/context changes: `CARGO_BUILD_JOBS=8 just test -p ontocode-core`
- goal-runtime changes: `CARGO_BUILD_JOBS=8 just test -p ontocode-ext-goal`
- output-shaping changes: `CARGO_BUILD_JOBS=8 just test -p ontocode-core`

Before any commit:

```bash
cd /opt/demodb/_workfolder/ontocode
git diff --check -- <touched-files>
```

If any Rust code changes land in shared core owners, run:

```bash
cd /opt/demodb/_workfolder/ontocode
ontoindex analyze --skills --skip-agents-md
```

## Reopen Gates

Do not reopen dropped OpenHuman families unless a separate accepted source of truth appears for:

- public API/schema changes
- hosted service dependencies
- persistent donor memory platform
- product UX layers
- broad personal-data ingestion

Keep this plan focused on the six retained core-extension rows only.
