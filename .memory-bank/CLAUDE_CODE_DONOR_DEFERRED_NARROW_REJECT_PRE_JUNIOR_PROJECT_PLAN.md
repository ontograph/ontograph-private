name: Claude Code Donor Parked Rows Pre-Junior Project Plan
desc: Junior-safe triage plan for parked Claude Code donor rows from ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md
type: project_plan
date: 2026-06-16
status: challenged

# Claude Code Donor Parked Rows Pre-Junior Project Plan

## Goal

Use [ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md) as a parking-lot review queue.

This is not an implementation plan for the 146 parked rows. A pre-junior may only verify one row, prove whether it should stay parked, or prepare a senior-ready promotion packet. No Rust code is changed from this plan.

## Challenge Review

- This plan is a brake, not a backlog. It must reduce accidental scope, not create tasks.
- Pre-juniors cannot change row verdicts, move rows between ADRs, or edit the active `KEEP` ADR.
- A promotion packet is only a handoff note for senior review. It does not authorize implementation.
- If a row overlaps Gemini or Oh My Pi plans, leave it parked and link the duplicate. Do not consolidate by copying tasks.
- If OntoIndex is unavailable, the row is blocked. Do not replace the owner check with grep or guesswork.
- OntoIndex freshness is required. If `indexedCommit` differs from `currentCommit`, stop and coordinate before using the plan.

## OntoIndex Owner Challenge

Fresh OntoIndex evidence reinforces that this plan must stay triage-only:

- `ontocode-rs/codex-mcp/src/connection_manager.rs` is already the MCP owner for tool visibility, approval policy, permissions, server/resource lists, resource reads, and tool calls. It is 823 lines, so MCP rows cannot become new browser/debugger/resource surfaces.
- `ontocode-rs/core/src/session/turn.rs` owns turn execution, prompt building, tool construction, compaction, and context updates. It is 2252 lines, so context/cache rows cannot become new prompt-cache or speculative-context behavior.
- `ontocode-rs/hooks/src/engine/discovery.rs` owns hook discovery. It is 1087 lines, so hook rows cannot become a second hook registry or policy layer.
- `ontocode-rs/state/src/runtime/agent_jobs.rs` owns job creation, item state, cancellation, results, and progress. It is 685 lines, so agent rows cannot become scheduler, session-command, or persisted-state work.

Challenge outcome: a pre-junior may produce evidence only. Promotion requires a senior to move the row to an accepted ADR or a separate tracked plan.

## Allowed Work

- Pick exactly one parked row.
- Confirm its current verdict: `NARROW`, `DEFER`, or `REJECT`.
- Check whether the same idea is already covered by Gemini or Oh My Pi donor plans.
- Use OntoIndex to identify the existing owner module.
- Write a short promotion packet only if the row can be reduced to one existing-owner test gap.

## Non-Goals

- Do not implement parked rows directly.
- Do not add runtime state, context fragments, tool registries, MCP browsers, hook registries, scheduler behavior, UI surfaces, plugin systems, or eval frameworks.
- Do not reopen `REJECT` rows unless a senior first changes the ADR verdict.
- Do not create a new ADR for every row. One row needs one concise note.
- Do not edit verdicts, row counts, or parked-row tables from this plan.

## Stage 0: Preflight

Read:

- [ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md)
- [ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW.md](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW.md)
- [GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_PRE_JUNIOR_PROJECT_PLAN.md](GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_PRE_JUNIOR_PROJECT_PLAN.md)
- [OH_MY_PI_DONOR_KEEP_REFACTOR_MAP_PRE_JUNIOR_PROJECT_PLAN.md](OH_MY_PI_DONOR_KEEP_REFACTOR_MAP_PRE_JUNIOR_PROJECT_PLAN.md)

Checks:

- Verify the parked table still has 146 rows.
- Pick one row only.
- Record the row ID, verdict, category, and claimed owner.

Acceptance:

- No code changed.
- The task note names one row and one existing owner area.
- The task note quotes the parked ADR row text or links the row ID.

## Stage 1: Duplicate Gate

Task:

- Search the Gemini and Oh My Pi pre-junior plans for the same behavior.
- If the idea is already covered there, leave the Claude row parked.
- If the idea conflicts with a blocked scope there, leave the Claude row parked.

Acceptance:

- The note says either `duplicate`, `blocked`, or `not duplicated`.
- Duplicates are not promoted.
- A duplicate row gets no promotion packet.

## Stage 2: OntoIndex Owner Check

Use OntoIndex on the relevant existing owner before proposing any promotion:

- MCP/resource/debug rows: `ontocode-rs/codex-mcp/src/connection_manager.rs`
- Context/cache/speculation rows: `ontocode-rs/core/src/session/turn.rs`, `ontocode-rs/core/src/compact.rs`, or `ontocode-rs/core/src/session/turn_context.rs`
- Hook rows: `ontocode-rs/hooks/src/engine/discovery.rs` or `ontocode-rs/core/src/hook_runtime.rs`
- Agent/job/session rows: `ontocode-rs/state/src/runtime/agent_jobs.rs`
- Code-search rows: keep rejected; OntoIndex is the owner.

Acceptance:

- The note includes the OntoIndex owner file.
- The note includes one OntoIndex fact, such as the owner file public API or module role.
- For hot owners, the note includes the OntoIndex line count and explains why no direct edit is proposed.
- If OntoIndex shows the owner already covers the behavior, the row stays parked.
- If OntoIndex is locked or unavailable, stop; do not substitute manual guessing.

## Stage 3: Narrowing Packet

Only for `NARROW` rows.

Write a concise packet with:

- row ID and current verdict
- exact existing owner file
- one missing behavior
- one smallest failing test that would prove the gap
- why this is not a new runtime concept
- duplicate check result

Packet format:

```md
Row:
Current verdict:
Duplicate gate:
OntoIndex owner:
Missing behavior:
Smallest failing test:
Why this is not new architecture:
Hot-owner risk:
Senior decision needed:
```

Acceptance:

- The packet fits in one short section.
- It does not propose production code.
- It does not introduce new architecture.
- It does not name an implementation worker.
- It is written as a handoff note, not committed into an ADR unless a senior asks.

## Stage 4: Deferred Row Review

Only for `DEFER` rows.

A deferred row can move forward only with fresh evidence:

- a real bug,
- a user-facing regression,
- a security/safety issue,
- or a senior-approved product requirement.

Acceptance:

- Without fresh evidence, leave the row parked.
- With evidence, write a promotion packet and stop for senior review.
- Evidence must be specific; "nice to have" is not enough.

## Stage 5: Rejected Row Review

Only for `REJECT` rows.

Rejected rows stay rejected by default.

Current hard rejects:

- 121: duplicate code exploration
- 124: raw code exposure
- 125: duplicate symbol search
- 126: duplicate file search
- 127: grep wrapper

Acceptance:

- Do not promote these from this plan.
- Pre-juniors may only confirm why the row is rejected.
- If a senior changes one verdict, start again at Stage 0.

## Closure Checklist

- Row count remains 146 unless a senior explicitly moves a row out.
- Any doc edit links back to the parked ADR.
- No Rust files changed.
- `git diff --check` passes for edited markdown.

## Stop Conditions

Stop and ask for review if the task needs:

- code changes,
- a new public API/config/schema,
- a new context fragment,
- a new MCP, hook, job, plugin, eval, or UI owner,
- changes to `KEEP` rows in the parent ADR,
- or more than one parked row.
