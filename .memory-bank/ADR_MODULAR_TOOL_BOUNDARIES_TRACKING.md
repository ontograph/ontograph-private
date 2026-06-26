---
name: Modular Tool Boundaries Tracking
description: Bounded manager-loop ledger for the approved modular-boundaries slices from ADR_MODULAR_TOOL_BOUNDARIES.md
type: tracking
date: 2026-06-22
status: no-dispatch-after-stage-4a
---

# Modular Tool Boundaries Tracking

Authority:
- `ADR_MODULAR_TOOL_BOUNDARIES.md`

## Manager Rules

- Update this file before starting each bounded task and after verification.
- Use OntoIndex impact/context before production symbol edits and refresh OntoIndex after each completed bounded task.
- Do not dispatch any new slice from this ADR unless a fresh senior review re-approves one bounded follow-up task.
- Keep work inside existing tool-planning owners. Do not add a second tool registry, second MCP runtime, or new public tool/config/API surface.
- Do not run parallel Rust build/test/fmt commands for this ADR loop.
- Requested implementation-worker model order for this loop is `gemini-pro-agent`, `gpt-5.3-codex-spark`, `gpt-5.4-mini`; use the first available model today and record any fallback.
- Requested senior-reviewer model is `claude-sonnet-4-6`, fallback `gpt-5.4-mini`; use fallback when the requested model is unavailable in the active tool surface.

## Tasks

| ID | Task | Owner | Status | Write Scope | Assigned Model | Verification |
| --- | --- | --- | --- | --- | --- | --- |
| `MTB-S1` | Extract Stage 1 tool-planning modules from `spec_plan.rs` without behavior change | implementation-worker | done-first-slice | `ontocode-rs/core/src/tools/spec_plan.rs`, new `ontocode-rs/core/src/tools/planning/*`, `ontocode-rs/core/src/tools/spec_plan_tests.rs`, and any minimal `mod.rs` wiring under `ontocode-rs/core/src/tools/` | `gpt-5.4-mini` after fallback chain | `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib`; `CARGO_BUILD_JOBS=8 just test -p ontocode-core tools::spec_plan`; `CARGO_BUILD_JOBS=8 just fmt` |
| `MTB-V1` | Senior review, focused verification, OntoIndex refresh, and ledger update for `MTB-S1` | manager + verification-worker | done-first-slice | manager-owned tracking and scoped verification only | `gpt-5.4-mini` fallback | local manager verification plus `gpt-5.4-mini` senior review; OntoIndex fresh at HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2` |
| `MTB-S2A` | Extract MCP approval elicitation payload shaping from `mcp_tool_call.rs` into `mcp_tool_approval_templates.rs` without behavior change | implementation-worker | done-approved-slice | `ontocode-rs/core/src/mcp_tool_call.rs`, `ontocode-rs/core/src/mcp_tool_approval_templates.rs`, and any minimal core module wiring or scoped tests needed for this extraction only | `gpt-5.4-mini` after same-day fallback chain | `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib`; `CARGO_BUILD_JOBS=8 just test -p ontocode-core mcp_tool_call`; `CARGO_BUILD_JOBS=8 just fmt` |
| `MTB-V2A` | Senior review, focused verification, OntoIndex refresh, and ledger update for `MTB-S2A` | manager + verification-worker | done-approved-slice | manager-owned tracking and scoped verification only | `gpt-5.4-mini` fallback | local manager verification plus senior challenge of returned diff and OntoIndex freshness check |
| `MTB-S4A` | Add one focused Stage 4 static boundary audit that keeps provider/auth owners out of `ontocode-rs/core/src/tools/planning/mcp.rs` | manager | done-approved-slice | `.memory-bank/ADR_MODULAR_TOOL_BOUNDARIES_TRACKING.md`, `ontocode-rs/core/src/tools/spec_plan_tests.rs`, and any minimal test-only imports needed for this audit | manager-local | `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib --tests`; `CARGO_BUILD_JOBS=8 just test -p ontocode-core tools::spec_plan`; `CARGO_BUILD_JOBS=8 just fmt` |

## Event Log

- 2026-06-22: Manager opened tracking before dispatch. OntoIndex freshness was stale at start (`1d91f6ba20637508c8087e308bb49ed520011f2f` indexed vs `2e72a6d25e147f0619863e7721107b6f11a87fc2` current), so the manager ran a single local `ontoindex analyze`. Freshness is now restored at HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2`; worktree scope confidence remains medium because the repo is dirty.
- 2026-06-22: Senior challenge narrowed the ADR to one implementation-approved slice: Stage 1 planner extraction only. Stages 2 through 4 remain proposal-only.
- 2026-06-22: OntoIndex impact for `build_tool_router` is LOW with one upstream caller (`ToolRouter.from_turn_context`). OntoIndex context for `build_tool_specs_and_registry` shows the current seam is local to `spec_plan.rs` and composes `add_tool_sources`, `append_tool_search_executor`, `prepend_code_mode_executors`, and `build_model_visible_specs_and_registry`.
- 2026-06-22: Dispatch target is intentionally narrow: split source-family planning modules out of `spec_plan.rs` while keeping `ToolRouter`, `ToolRegistry`, extension registry, MCP runtime ownership, tool exposure behavior, and namespace semantics unchanged.
- 2026-06-22: Marked `MTB-S1` in progress before worker dispatch. Requested implementation-worker order was `gemini-pro-agent`, `gpt-5.3-codex-spark`, `gpt-5.4-mini`; `gemini-pro-agent` is unavailable in the current tool surface, so dispatch used fallback `gpt-5.3-codex-spark` worker `019ef13d-085f-7130-b04d-3fde08b94512`.
- 2026-06-22: `gpt-5.3-codex-spark` failed before work started because the inherited tool surface included unsupported `image_generation`. Per same-day retry rules, do not retry Spark again today for this loop; redispatch `MTB-S1` to `gpt-5.4-mini`.
- 2026-06-22: Worker `019ef13d-caa7-7021-add3-f36fc8b669d8` completed an initial extraction pass on `gpt-5.4-mini`, but senior review rejected acceptance on scope. Two drift items must be removed before closure: (1) code-mode subagent `spawn_agent` hiding plus the matching new test are unrelated tool-exposure behavior changes, and (2) `append_tool_search_executor` search metadata rewrites change semantics instead of preserving Stage 1 behavior. The extraction itself appears structurally sound and `cargo check -p ontocode-core --lib` passed for the senior reviewer; redo is required to narrow the diff back to extraction-only.
- 2026-06-22: Worker redo removed both rejected drifts and kept the Stage 1 extraction. Planning code is now split into `tools/planning/{native,mcp,dynamic,extensions,hosted}.rs` plus shared `planning/mod.rs`, while `spec_plan.rs` stays the thin composer/orchestration boundary.
- 2026-06-22: Manager spot-check confirmed the rejected drift markers are absent: no `coding_subagent_hides_spawn_agent_while_root_keeps_it` test remains, and `append_tool_search_executor` no longer rewrites `ToolSearchInfo` metadata.
- 2026-06-22: Local verification passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib` and `CARGO_BUILD_JOBS=8 just fmt`. The then-current rerun of `CARGO_BUILD_JOBS=8 just test -p ontocode-core tools::spec_plan` still failed outside this slice on an unrelated `agent_jobs_tests.rs` drift, which did not reopen `MTB-S1`.
- 2026-06-22: OntoIndex was refreshed again after the accepted redo; freshness is restored at HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2` with dirty-worktree medium scope confidence. `gn_verify_diff` still reports FAIL because the repository contains many unrelated dirty files and it cannot infer the executed tests from this mixed worktree. This does not reopen `MTB-S1`.

## Verification Log

- Senior reviewer fallback `gpt-5.4-mini` rejected the first worker return on scope and identified two exact drift sites: `ontocode-rs/core/src/tools/planning/native.rs:266-345` plus `ontocode-rs/core/src/tools/spec_plan_tests.rs:1022-1076` for the code-mode `spawn_agent` behavior change, and `ontocode-rs/core/src/tools/spec_plan.rs:282-333` for the tool-search metadata rewrite.
- Worker redo confirmed both drift items were removed before closure.
- Manager reran `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib`; passed with one unrelated warning in `core/src/tools/context.rs`.
- Manager reran `CARGO_BUILD_JOBS=8 just fmt`; passed.
- Manager reran `CARGO_BUILD_JOBS=8 just test -p ontocode-core tools::spec_plan`; it failed at the time on the same unrelated `agent_jobs_tests.rs` drift.
- OntoIndex `gn_ensure_fresh` reports indexed HEAD equals current HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2`; worktree remains dirty.
- OntoIndex `gn_verify_diff` reported `FAIL` because the repo has many unrelated changed files and no executed tests were supplied to the tool, not because the accepted Stage 1 write scope drifted.
- 2026-06-23 later follow-up: the unrelated `agent_jobs_tests.rs` drift was fixed separately and is not an open blocker for this ADR.
- 2026-06-23: Manager resumed the bounded loop with fresh OntoIndex status. Indexed HEAD still matches current HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2`; dirty-worktree scope confidence remains medium.
- 2026-06-23: No new implementation dispatch was opened. At that point `MTB-S1` was already accepted as `done-first-slice`, and no later slice was approved until the fresh senior review that reopened only `MTB-S2A`.
- 2026-06-23: Bounded loop closed no-dispatch. The only valid next manager action from this ADR is a fresh senior challenge card that explicitly re-approves one later modularization slice.
- 2026-06-23: Fresh senior review reopened Stage 2 only as one narrow implementation-approved slice. OntoIndex impact for `handle_mcp_tool_call` is LOW upstream with one direct caller (`ontocode-rs/core/src/tools/handlers/mcp.rs:McpHandler.handle`), but helper-level movement is higher risk, so the accepted slice moves only serialized elicitation payload shaping.
- 2026-06-23: Manager marked `MTB-S2A` in progress before worker dispatch. Same-day model history still blocks retrying `gemini-pro-agent` and `gpt-5.3-codex-spark`, so the active implementation dispatch target is `gpt-5.4-mini`.
- 2026-06-23: Worker `019ef2c1-93d5-7351-a544-65e45174c636` completed `MTB-S2A` on `gpt-5.4-mini` inside the approved write scope: `ontocode-rs/core/src/mcp_tool_call.rs`, `ontocode-rs/core/src/mcp_tool_approval_templates.rs`, and minimal `ontocode-rs/core/src/mcp_tool_call_tests.rs` import wiring. Manager review accepted the slice after confirming prompt wording, approval semantics, telemetry, sanitization, and event emission remained untouched.
- 2026-06-23: OntoIndex freshness remains current at HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2` after `MTB-S2A`; worktree scope confidence remains medium because the repository is dirty.
- 2026-06-23: Local manager verification for `MTB-S2A` passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib` after final import cleanup. `CARGO_BUILD_JOBS=8 just fmt` also passed.
- 2026-06-23: Local rerun of `CARGO_BUILD_JOBS=8 just test -p ontocode-core mcp_tool_call` no longer failed in `mcp_tool_call_tests.rs`; the remaining failure at that time was the same unrelated package-wide `agent_jobs_tests.rs` drift.
- 2026-06-23: Senior-review follow-up found no next implementation-approved slice after `MTB-S2A`. Stage 3 is already on the existing `ToolContributor` seam for optional families, and Stage 4 is guardrail work rather than one bounded modularization slice. The bounded manager loop is therefore closed as no-dispatch pending a newly narrowed ADR task.
- 2026-06-23: `MTB-S2A` and `MTB-V2A` are accepted as `done-approved-slice`. The next valid bounded-loop action is a fresh senior challenge to decide whether any later Stage 2, 3, or 4 slice can be approved without crossing the ADR boundary.
- 2026-06-23: Fresh senior challenge reopened Stage 4 as one narrow guardrail slice only: add a source-level static audit that prevents provider/auth owner imports from creeping into `ontocode-rs/core/src/tools/planning/mcp.rs`. The approved slice is test-only and does not introduce a new lint framework, new runtime owner, or broader repository audit surface.
- 2026-06-23: `MTB-S4A` completed manager-local. The new `tools::spec_plan::tests::mcp_planning_module_keeps_provider_and_auth_owners_out` source audit reads `src/tools/planning/mcp.rs` and fails if provider/auth owner imports or helper calls are added there.

## Stage 4 Verification

- 2026-06-23: Manager reran `CARGO_BUILD_JOBS=8 just fmt`; passed after `MTB-S4A`.
- 2026-06-23: Manager reran `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib --tests`; passed for `MTB-S4A` with the same unrelated warnings in `core/src/tools/context.rs` and `core/tests/suite/code_mode.rs`.
- 2026-06-23: Manager reran `CARGO_BUILD_JOBS=8 just test -p ontocode-core tools::spec_plan`; passed. The new Stage 4 audit test passed alongside the existing `tools::spec_plan` suite.
- 2026-06-23: `MTB-S4A` is accepted as `done-approved-slice`. No further Stage 4 dispatch is approved from this ADR.
