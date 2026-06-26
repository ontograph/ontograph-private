---
name: Donor Tool Proposals Consolidation Tracking
description: Dispatch and verification ledger for the four accepted/narrowed candidates in ADR_DONOR_TOOL_PROPOSALS_CONSOLIDATION.md
type: tracking
date: 2026-06-21
status: closed-narrowed
---

# Donor Tool Proposals Consolidation Tracking

Authority:
- `ADR_DONOR_TOOL_PROPOSALS_CONSOLIDATION.md`

## Manager Rules

- Update this file before starting each bundle and after closure.
- Keep every change inside the existing owner named by the ADR.
- Do not dispatch verification-only candidates without a reproduced current-owner failing test.
- Do not add a normal `structured_output` tool, citation runtime, second formatter model, parallel model loop, or unbounded web fetch.
- Run OntoIndex context/impact before production symbol edits and check freshness after each accepted bundle.
- Use requested sub-agent models in this order when available today: `gemini-pro-agent`, `gpt-5.3-codex-spark`, `gpt-5.4-mini`.
- Single-mode recovery rule: run exactly one `DTP` bundle at a time. Do not
  dispatch parallel workers for this ADR, and do not run overlapping
  `cargo`/`just`/`bazel`/format/test/build commands. For this ADR, use
  `CARGO_BUILD_JOBS=1` even though the repo default allows more jobs.

## Dispatch Queue

| Bundle | Candidate | Status | Owner / Write Scope | Assigned Model | Verification |
| --- | --- | --- | --- | --- | --- |
| `DTP-R1` | Structured final output enforcement | done-first-slice | `ontocode-rs/protocol`, `ontocode-rs/core/src/session*`, final-output request/response handling | local single-mode recovery | `CARGO_BUILD_JOBS=1 just fmt`; `CARGO_BUILD_JOBS=1 just test -p ontocode-core session::turn_context::tests::validates_final_output_json_schema_shape session::turn_context::tests::turn_context_records_bounded_tool_result_evidence` |
| `DTP-R2` | Evidence ledger for tool results | done | existing context fragments, tool output handling, session facts; no separate citation runtime | local single-mode recovery | `CARGO_BUILD_JOBS=1 just fmt`; `CARGO_BUILD_JOBS=1 just test -p ontocode-protocol read_evidence`; `CARGO_BUILD_JOBS=1 just test -p ontocode-core session::turn_context::tests::turn_context_records_bounded_tool_result_evidence context::operational_evidence_context::tests::renders_operational_evidence_context_fragment_as_bounded_user_context context::operational_evidence_context::tests::truncates_operational_evidence_context_fragment_to_the_byte_cap session::tests::build_initial_context_includes_thread_scoped_operational_evidence_context` |
| `DTP-R3` | Deterministic final answer verifier | done-narrowed | session finalization, turn diff tracker, test evidence; no second formatter model or parallel model loop | local single-mode recovery | `CARGO_BUILD_JOBS=1 just fmt`; `CARGO_BUILD_JOBS=1 just test -p ontocode-core final_answer_verifier stream_events_utils::tests::final_answer_verifier_warns_when_test_claim_lacks_evidence` |
| `DTP-R4` | Guarded web fetch | closed-no-new-work | hosted web search / network policy owners, `spec_plan`, hosted tool specs, web policy tests | local single-mode recovery | `CARGO_BUILD_JOBS=1 just test -p ontocode-core parses_web_search_open_page_call parses_web_search_find_in_page_call`; `CARGO_BUILD_JOBS=1 just test -p ontocode-core hosted_tools_follow_provider_auth_model_and_config_gates` |

## Event Log

- 2026-06-21: Manager opened tracking for the four active KEEP candidates from `ADR_DONOR_TOOL_PROPOSALS_CONSOLIDATION.md`. OntoIndex freshness check reports indexed HEAD matches current HEAD `8b61fd0dbfa32aa9e4a00ef15930e5b4fcb9119f`; dirty worktree gives medium scope confidence. Verification-only candidates remain parked.
- 2026-06-21: Marked `DTP-R1` through `DTP-R4` in progress before worker dispatch.
- 2026-06-21: Rejected the first `DTP-R3` closure because it was memory-only and did not implement or prove the deterministic final-answer verifier behavior required by the ADR. Reopened for code-backed implementation or focused existing-behavior proof.
- 2026-06-21: `DTP-R2` dispatch to `gpt-5.3-codex-spark` failed before work started because that model rejected the inherited `image_generation` tool. Per same-day retry rules, redispatched `DTP-R2` to `gpt-5.4-mini`.
- 2026-06-21: Performance recovery review found the parallel `DTP-R1` through
  `DTP-R4` dispatch/build plan was too expensive for this worktree. Switched
  the ADR loop to single mode: keep only `DTP-R1` active, pause `DTP-R2`
  through `DTP-R4`, avoid parallel sub-agent dispatch, and run all Rust
  build/test/fmt commands sequentially with `CARGO_BUILD_JOBS=1`.
- 2026-06-21: Closed first `DTP-R1` slice in single-mode recovery. Minimal
  implementation validates non-null `final_output_json_schema` updates as
  non-empty JSON objects in the existing session turn path and maps invalid
  shapes to the existing invalid-request path. OntoIndex impact for
  `dispatch_any_with_terminal_outcome` was LOW while fixing a compile blocker
  left by the paused evidence-ledger edit; the earlier DTP-R1 impact fallback
  was narrow file inspection after an FTS timeout. Verification passed with
  `CARGO_BUILD_JOBS=1 just fmt` and scoped `ontocode-core` tests for final
  output schema validation plus bounded tool-result evidence. Dirty worktree
  scope confidence remains medium because paused `DTP-R2`/`DTP-R4` edits are
  still present.
- 2026-06-21: Activated `DTP-R2` as the only in-progress single-mode bundle
  after OntoIndex freshness check reported indexed HEAD equals current HEAD
  `8b61fd0dbfa32aa9e4a00ef15930e5b4fcb9119f` with dirty-worktree medium
  scope confidence.
- 2026-06-21: Closed `DTP-R2` in single-mode recovery. Implementation stayed
  inside existing evidence/context/session owners: bounded `FileReadEvidence`
  buckets now carry paths, tool names, tests, policy checks, and source
  references; successful tool results feed the per-turn evidence path; accepted
  operational evidence can render through a capped `ContextualUserFragment`.
  No separate citation runtime, persistence stack, or public API was added.
  Verification passed with protocol `read_evidence` tests and scoped core
  turn/context/session evidence tests under `CARGO_BUILD_JOBS=1`.
- 2026-06-21: Activated `DTP-R3` as the only in-progress single-mode bundle
  after post-`DTP-R2` OntoIndex freshness check reported indexed HEAD equals
  current HEAD `8b61fd0dbfa32aa9e4a00ef15930e5b4fcb9119f` with dirty-worktree
  medium scope confidence.
- 2026-06-21: Closed narrowed `DTP-R3` slice in single-mode recovery. Implementation adds a
  deterministic final-answer verifier in the existing completed-response path:
  final assistant answers that claim tests, policy checks, or source changes
  now emit a bounded warning when the current turn has no matching recorded
  evidence. No second formatter model, model loop, or parallel verifier runtime
  was added. Verification passed with focused verifier and stream-event tests
  under `CARGO_BUILD_JOBS=1`.
- 2026-06-21: Activated `DTP-R4` as the only in-progress single-mode bundle
  after post-`DTP-R3` OntoIndex freshness check reported indexed HEAD equals
  current HEAD `8b61fd0dbfa32aa9e4a00ef15930e5b4fcb9119f` with dirty-worktree
  medium scope confidence.
- 2026-06-21: Closed `DTP-R4` with no new code by senior-narrowed existing-owner proof. A new
  Rust fetcher or unsupported hosted `web_fetch` tool would create a parallel
  network/tool stack. Existing hosted `web_search` already carries guarded
  `open_page` and `find_in_page` fetch-style actions, and `spec_plan` gates the
  hosted tool by provider support, config, standalone web-run availability, and
  web-search mode. Verification passed with web-search action parsing tests and
  hosted tool gating tests under `CARGO_BUILD_JOBS=1`.
- 2026-06-21: Final single-mode validation: `CARGO_BUILD_JOBS=1 just fix -p
  ontocode-core` completed, followed by `CARGO_BUILD_JOBS=1 just fmt`.
  OntoIndex freshness still reports indexed HEAD equals current HEAD
  `8b61fd0dbfa32aa9e4a00ef15930e5b4fcb9119f`. `gn_verify_diff` returned
  FAIL only because the worktree contains many unrelated dirty files outside
  this ADR scope; it reported no missing required tests.
- 2026-06-21: Senior challenge pass corrected closure wording. `DTP-R1` is a
  first-slice schema-shape validation closure, not full redaction/conformance
  diagnostics. `DTP-R3` is a narrowed warning verifier for tests, policy checks,
  and source-change claims, not exact file/command/failure/approval
  verification. `DTP-R4` is closed with no new code because hosted
  `web_search` already owns open-page/find-in-page fetch-style actions. Dirty
  `spec_plan.rs` changes for spawn-agent/tool-search behavior are unrelated to
  DTP and must stay out of a DTP-only commit.
