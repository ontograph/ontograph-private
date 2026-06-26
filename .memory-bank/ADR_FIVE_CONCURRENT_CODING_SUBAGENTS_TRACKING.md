---
name: Five Concurrent Coding Sub-Agents Tracking
description: Dispatch and verification ledger for ADR_FIVE_CONCURRENT_CODING_SUBAGENTS.md
type: tracking
date: 2026-06-21
status: done
---

# Five Concurrent Coding Sub-Agents Tracking

Authority:
- `ADR_FIVE_CONCURRENT_CODING_SUBAGENTS.md`

Scope:
- Support five simultaneous direct coding sub-agents through existing multi-agent v2 owners.
- Do not add a scheduler, SQLite task table, worker pool, default worktree isolation, or recursive sub-agent fan-out.

## Tasks

| ID | Task | Owner | Status | Files |
| --- | --- | --- | --- | --- |
| FCSA-1 | Raise default v2 session cap so five coding children are allowed | gpt-5.4-mini worker | done | `ontocode-rs/core/src/config/mod.rs`, `ontocode-rs/core/src/config/config_tests.rs` |
| FCSA-2 | Remove `spawn_agent` from ordinary coding sub-agent tool catalogs | gpt-5.4-mini worker | done | `ontocode-rs/core/src/tools/spec_plan.rs`, `ontocode-rs/core/src/tools/spec_plan_tests.rs` |
| FCSA-3 | Improve cap refusal text and cover sixth-child refusal | gpt-5.4-mini worker | done | `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`, `ontocode-rs/core/src/tools/handlers/multi_agents_tests.rs` |
| FCSA-4 | Senior verification, OntoIndex diff check, and tracking closeout | manager | done | `.memory-bank/ADR_FIVE_CONCURRENT_CODING_SUBAGENTS_TRACKING.md` |
| FCSA-U1 | Hide `spawn_agent` for all code-mode sub-agent sources, not only `ThreadSpawn` | gpt-5.4-mini worker | done | `ontocode-rs/core/src/tools/spec_plan.rs`, `ontocode-rs/core/src/tools/spec_plan_tests.rs` |
| FCSA-U2 | Senior challenge, focused verification, OntoIndex refresh, and final closeout | manager | done | `.memory-bank/ADR_FIVE_CONCURRENT_CODING_SUBAGENTS_TRACKING.md`, `.memory-bank/ADR_FIVE_CONCURRENT_CODING_SUBAGENTS.md` |

## Dispatch Log

- 2026-06-21: Tracking opened before implementation dispatch. OntoIndex review found P0 cap/default and recursive `spawn_agent` exposure blockers, plus refusal-message/test gaps.
- 2026-06-21: Marked FCSA-1/FCSA-2/FCSA-3 in-progress before worker dispatch. Requested first available exact configured worker model: `gpt-5.4-mini`.
- 2026-06-21: FCSA-1 completed by worker `019eea8f-1878-7e30-a07e-527e1e784901`: default v2 session cap raised to 6; focused default-cap test passed.
- 2026-06-21: FCSA-2 completed by worker `019eea8f-189e-7ec1-ac2b-e5ab881cc82c`: coding thread-spawn sub-agents no longer receive `spawn_agent`; focused tool-plan tests passed.
- 2026-06-21: FCSA-3 completed by worker `019eea8f-18c0-7181-b2aa-f11003401727`: cap refusal text now explains slot reuse through close; five-child/sixth-refused regression passed.
- 2026-06-21: Manager integration added usage-hint wording to stop advertising recursive coding sub-agent fan-out.
- 2026-06-21: Manager verification passed all focused ADR tests and closed all FCSA rows.
- 2026-06-21: Senior review reopened the ADR for one gap: code-mode `SubAgentSource::Other(...)` sessions can still receive `spawn_agent`. Added `ADR_FIVE_CONCURRENT_CODING_SUBAGENTS_UNBLOCK.md` and marked FCSA-U1 in-progress before worker dispatch.
- 2026-06-21: FCSA-U1 completed by worker `019eeaaa-457a-7df0-b586-52ff226f1965`: all code-mode sub-agent sources now hide `spawn_agent`.
- 2026-06-21: Manager challenge tightened the test after one failed attempt: current code-mode agent-job probes do not expose CSV worker tools, so the final assertion verifies `spawn_agent` is hidden while non-recursive `send_message`/`wait_agent` remain.
- 2026-06-21: Manager verification passed all focused ADR tests and closed FCSA-U1/FCSA-U2.

## Verification Log

- FCSA-1 worker ran `CARGO_BUILD_JOBS=8 just test -p ontocode-core multi_agent_v2_default_session_thread_cap_allows_five_children` and `CARGO_BUILD_JOBS=8 just fmt`; both passed.
- FCSA-2 worker ran `CARGO_BUILD_JOBS=8 just test -p ontocode-core tools::spec_plan` and `CARGO_BUILD_JOBS=8 just fmt`; both passed.
- FCSA-3 worker ran `CARGO_BUILD_JOBS=8 just test -p ontocode-core tools::handlers::multi_agents` and `CARGO_BUILD_JOBS=8 just fmt`; both passed.
- Manager ran `CARGO_BUILD_JOBS=8 just test -p ontocode-core multi_agent_v2_default_session_thread_cap_allows_five_children coding_subagent_hides_spawn_agent_while_root_keeps_it collab_spawn_error_reports_cap_and_slot_release_guidance spawn_agent_refuses_sixth_child_after_five_open_children`; 4/4 passed.
- Manager ran `CARGO_BUILD_JOBS=8 just fmt`; passed.
- OntoIndex `gn_verify_diff` reported `FAIL` because the repo already has many unrelated dirty files outside this ADR slice; expected tests were present and no required test was missing.
- FCSA-U1 worker ran `CARGO_BUILD_JOBS=8 just test -p ontocode-core coding_subagent_hides_spawn_agent_while_root_keeps_it` and `CARGO_BUILD_JOBS=8 just fmt`; both passed before manager test tightening.
- Manager first rerun of `CARGO_BUILD_JOBS=8 just test -p ontocode-core coding_subagent_hides_spawn_agent_while_root_keeps_it` failed because the tightened test incorrectly expected code-mode agent-job CSV tools to be visible.
- Manager corrected the test to the current code-mode invariant and reran `CARGO_BUILD_JOBS=8 just test -p ontocode-core coding_subagent_hides_spawn_agent_while_root_keeps_it`; passed.
- Manager ran `CARGO_BUILD_JOBS=8 just test -p ontocode-core multi_agent_v2_default_session_thread_cap_allows_five_children coding_subagent_hides_spawn_agent_while_root_keeps_it collab_spawn_error_reports_cap_and_slot_release_guidance spawn_agent_refuses_sixth_child_after_five_open_children`; 4/4 passed.
- Manager ran `CARGO_BUILD_JOBS=8 just fmt`; passed.
