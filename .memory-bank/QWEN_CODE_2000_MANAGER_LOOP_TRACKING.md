# Qwen Code 2000 Manager Loop Tracking

Status: complete
Date: 2026-06-22
Source: `QWEN_CODE_2000_USEFUL_APPROACHES_REVIEW.md`

## Bounds

- Process the 50 distinct Rust core-functionality seeds as review candidates, not automatic implementation tasks.
- Dispatch only if senior review or implementation review proves a concrete behavior gap in the current Rust owner.
- Keep work in existing owners: agent jobs, shell/sandbox, hooks, context/compaction, and operational evidence.
- Do not create parallel daemon owners, public APIs, config/schema surfaces, or side stacks from donor material.
- Use single OntoIndex refreshes only; do not run parallel index/build processes.

## Role Assignments

| Role | Requested Model | Actual Model | Status |
|---|---|---|---|
| senior-reviewer | `claude-sonnet-4-6`, fallback `gpt-5.4-mini` | `gpt-5.4-mini` (`019eef7c-7bc8-7b61-b4b2-bb6d2739a592`) | complete |
| implementation-worker | `gemini-pro-agent`, fallback `gpt-5.3-codex-spark`, `gpt-5.4-mini` | `gemini-pro-agent` (`019eef7c-7d0f-7ad1-8332-045ac0a35068`) | closed-timeout-no-result |
| verification-worker | `gpt-5.4-mini` | `gpt-5.4-mini` (`019eef7c-7d36-7b93-a92a-72b5382345d6`) | complete |

## Loop State

| Step | Owner | Scope | Status | Notes |
|---|---|---|---|---|
| 1 | manager | OntoIndex freshness and owner check | complete | index fresh at `1d91f6ba20637508c8087e308bb49ed520011f2f`; dirty worktree caveat |
| 2 | senior-reviewer | challenge 50 seeds for dispatchability | complete | not dispatchable as-is; all families duplicate-covered/docs-only unless fresh failing path appears |
| 3 | implementation-worker | implement only proven gap, otherwise produce no-code closeout | closed-timeout-no-result | worker timed out and was shut down; no worker edits or no-code finding were accepted |
| 4 | verification-worker | verify artifact and any worker edits | complete | 50 rows, 10 per family, no lens rows, memory links present |
| 5 | manager | final challenge, OntoIndex refresh, closeout | complete | final OntoIndex refresh completed after no-code decision |

## Current Manager Position

The artifact is already narrowed from 600 lens-expanded rows to 50 distinct behavior seeds. Senior review found no dispatchable implementation gap: all five families are duplicate-covered or docs-only without a fresh failing path. The implementation worker timed out without returning a proven gap, so no code edits were accepted.

## OntoIndex Evidence Snapshot

- Agent jobs: `run_agent_job_loop` owns recovery, progress, cancellation, CSV export, prompt construction, stale reaping, and status waiting through `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`; adjacent coverage includes `multi_agents_tests` and `agent_jobs_spec_tests`.
- Shell/sandbox: shell exposure and policy behavior route through unified exec and shell spec/snapshot tests, including visible unified exec registration and environment policy preservation.
- Hooks: `HookOutputSpiller` and `output_spill_tests` cover large-output spill, repeated-output bounding, and exactly one recovery path.
- Context/compaction: `run_turn` calls `run_pre_sampling_compact`, `run_auto_compact`, context manager history, image replacement, and bounded hook context recording; compact suite coverage includes pre-sampling compaction after resume/model switch.
- Operational evidence: `operational_evidence_import.rs` validates envelopes, bounded records, provenance, missing files, and raw artifact rejection; `operational_evidence_tests` covers missing provenance, secret/raw payload rejection, and missing-file behavior.

## Implementation Options

1. Keep closed: treat the 50 rows as covered review candidates. No code work until a fresh failing test or bug report identifies one owner-local gap.
2. Probe one narrow candidate: if product pressure exists, start with `AGENT-05-03` and add only a focused regression test for final-summary survival after resume. Do not alter agent-job runtime unless the test fails.
3. Reclassify source artifact: change `KEEP` wording in `QWEN_CODE_2000_USEFUL_APPROACHES_REVIEW.md` to `KEEP-CANDIDATE` or add a note that `KEEP` does not mean dispatchable implementation.
4. Archive as audit closure: add an `audit_session-2026-06-22-qwen-code-2000-manager-loop.md` closeout only if future managers need a durable one-page summary outside the tracker.

## Rerun 2026-06-22

Status: complete

| Step | Owner | Scope | Status | Notes |
|---|---|---|---|---|
| 1 | manager | OntoIndex freshness and artifact read | complete | index fresh at `1d91f6ba20637508c8087e308bb49ed520011f2f`; dirty worktree caveat |
| 2 | senior-reviewer | re-challenge dispatchability | complete | no rows dispatchable; `AGENT-05-03` remains closed because final-summary persistence already uses existing state/resume owners |
| 3 | implementation-worker | prove or reject one implementation gap | closed-no-result | `gemini-pro-agent` returned no usable finding; no worker edits accepted |
| 4 | verification-worker | verify artifact, tracker, and any accepted edits | complete | found per-row verdict mismatch; manager changed table verdicts from `KEEP` to `KEEP-CANDIDATE` |
| 5 | manager | final challenge and OntoIndex refresh | complete | final rerun OntoIndex refresh completed |

## Development Run 2026-06-22

Status: complete

Candidate: `AGENT-05-03`, row 283, "agent job final summary survives resume"

| Step | Owner | Scope | Status | Notes |
|---|---|---|---|---|
| 1 | manager | tracking-first start and OntoIndex freshness | complete | index fresh at `1d91f6ba20637508c8087e308bb49ed520011f2f`; dirty worktree caveat |
| 2 | manager/senior-reviewer | prove current owner-local gap | complete | production already used `COALESCE`, but no focused resume-style final-summary preservation regression was present |
| 3 | implementation-worker | add focused regression test only if coverage missing | complete | added state-runtime regression `mark_agent_job_completed_preserves_final_summary_on_resume`; no production code changed |
| 4 | verification-worker | verify changed files, tests, and scope | complete | `just fmt`; `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 169 tests; scoped `gn_verify_diff` passed |
| 5 | manager | close candidate and run one OntoIndex refresh | complete | candidate status: IMPLEMENTED-TEST-ONLY; OntoIndex refresh completed; whole-worktree test-gap remains noisy because of unrelated dirty files |

## Development Run 2026-06-22 (Row 203)

Status: complete

Candidate: `AGENT-01-03`, row 203, "agent job loop recovers running items after restart"

| Step | Owner | Scope | Status | Notes |
|---|---|---|---|---|
| 1 | manager | tracking-first start and OntoIndex freshness | complete | dirty worktree caveat remained; no stale index blocker was found |
| 2 | manager/senior-reviewer | prove current owner-local gap | complete | `recover_running_items` already resumes `AgentStatus::NotFound` workers through existing agent-control resume owners |
| 3 | implementation-worker | add focused regression test only if coverage missing | complete | no safe owner-local harness path was available without widening private test surface; no production bug was proven |
| 4 | verification-worker | verify changed files, tests, and scope | complete | earlier scratch attempts were discarded; no accepted code changes landed for row 203 |
| 5 | manager | close candidate and carry loop forward | complete | candidate status: COVERED/NO-DISPATCH |

## Full Sweep 2026-06-22

Status: complete

Final result:
- `IMPLEMENTED-TEST-ONLY`: 1 row (`AGENT-05-03`)
- `COVERED/NO-DISPATCH`: 49 rows
- `BLOCKED`: 0
- `REJECTED`: 0

Manager closeout:
- `AGENT` family: existing owners already cover recovery, cancellation, progress, CSV export, resume, prompt construction, model override, and waiter behavior through `run_agent_job_loop`, `spawn_agents_on_csv`, `multi_agents_tests`, `agent_jobs` suite coverage, and state runtime tests. Row 203 stayed no-dispatch because the code path already exists and no narrow owner-local failing test could be added without widening private surface.
- `SHELL` family: existing shell policy and safety owners already cover exposure, permission inheritance, command-safety classification, truncation, timeout/cancel behavior, and shell snapshot behavior through `unified_exec`, `shell-command`, `shell_snapshot.rs`, and related spec-plan tests.
- `HOOK` family: existing spill and hook-runtime owners already cover bounded preview, repeated-output bounding, single recovery path, structured completion events, compatibility-only legacy behavior, and transcript/path handling through `output_spill.rs`, `output_spill_tests.rs`, and `hook_runtime`.
- `CONTEXT` family: existing turn/context owners already cover pre-sampling compaction, same-model retry guard, bounded fragments, image capability handling, history preservation, and actionable overflow diagnostics through `run_turn`, compaction helpers, and `core/tests/suite/compact.rs`.
- `EVIDENCE` family: existing operational-evidence owners already cover raw artifact rejection, provenance requirements, bounded summaries, missing-file handling, secret rejection, and read-only recall boundaries through `operational_evidence_import.rs` and `operational_evidence_tests.rs`.

Skill outcome:
- The `qwen-development-loop` sweep is complete for all 50 rows from `QWEN_CODE_2000_USEFUL_APPROACHES_REVIEW.md`.
- No further code work should start from this artifact unless a fresh failing behavior, bug report, or missing compatibility test is identified in one of the existing owners.
