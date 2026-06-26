---
name: Hermes Donor Core Extension Tracking
description: Dispatch and verification ledger for the accepted Hermes donor core-extension rows
type: tracking
date: 2026-06-21
status: complete
---

# Hermes Donor Core Extension Tracking

Authority:
- `tmp/hermes-agent-500-tools-for-ontocode-challenged.md`
- `ADR_HERMES_DONOR_CORE_EXTENSION_SOLUTIONS.md`

## Manager Rules

- Update this file before starting each slice.
- Keep every change inside the existing owner named by the ADR.
- Do not import Hermes runtime stacks, browser/CDP tools, provider plugins, optional skills, cron, memory stores, process registries, or SQLite tracking.
- Refresh/check OntoIndex after each accepted slice.
- Use the first exact requested sub-agent model that is currently available. Today the active tool surface exposes `gpt-5.4-mini` from the requested list; unavailable requested aliases are not retried.

## Dispatch Queue

| Slice | Row | Status | Owner / Write Scope | Assigned Model | Verification |
| --- | --- | --- | --- | --- | --- |
| `HERMES-R1` | `HERMES-KEEP-03` | closed | `ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs` | `gpt-5.4-mini` | `CARGO_BUILD_JOBS=8 just test -p ontocode-core multi_agents_spec`; `CARGO_BUILD_JOBS=8 just fmt`; OntoIndex fresh/dirty check |
| `HERMES-R2` | `HERMES-KEEP-05` | closed | `ontocode-rs/tui/src/chatwidget.rs`, `ontocode-rs/tui/src/chatwidget/constructor.rs`, `ontocode-rs/tui/src/chatwidget/tool_lifecycle.rs`, `ontocode-rs/tui/src/chatwidget/tests/history_replay.rs` | `gpt-5.4-mini` | `CARGO_BUILD_JOBS=8 just test -p ontocode-tui repeated_failed_mcp_tool_calls_emit_bounded_hint_once`; `CARGO_BUILD_JOBS=8 just fmt`; OntoIndex fresh/dirty check |
| `HERMES-R3` | `HERMES-KEEP-17` | closed | `ontocode-rs/core-plugins/src/manager_tests.rs` | `gpt-5.4-mini` | `CARGO_BUILD_JOBS=8 just test -p ontocode-core-plugins`; `CARGO_BUILD_JOBS=8 just fmt`; OntoIndex fresh/dirty check |
| `HERMES-R4` | `HERMES-KEEP-20` | closed | `ontocode-rs/exec-server/src/server/handler/tests.rs` | `gpt-5.4-mini` | `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server detached_session_keeps_long_running_process_alive_then_cleans_it_up`; `CARGO_BUILD_JOBS=8 just fmt`; OntoIndex fresh/dirty check |

## Closed Without Dispatch

`HERMES-KEEP-01`, `02`, `04`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `18`, and `19` are closed unless a concrete owner-local failing fixture reopens them.

## Event Log

- 2026-06-21: Senior unblock accepted four narrow slices from the challenged Hermes donor review. Tracking opened and all four active slices marked `in-progress` before worker dispatch. OntoIndex is fresh at `d8ec11f538fb14941601332841ffd6dc1db734ac`; dirty worktree gives medium scope confidence. Available requested sub-agent model is `gpt-5.4-mini`.
- 2026-06-21: `HERMES-R3` closed. Added `remote_installed_cache_refresh_invalidates_stale_connectors` in `ontocode-rs/core-plugins/src/manager_tests.rs`, proving remote plugin connector cache clear/repopulate does not retain stale connector state. Worker reported `CARGO_BUILD_JOBS=8 just test -p ontocode-core-plugins` and `CARGO_BUILD_JOBS=8 just fmt` passed. Manager reviewed diff; OntoIndex remains fresh at HEAD with dirty-worktree medium confidence.
- 2026-06-21: `HERMES-R1` closed. Added `spawn_agent_tool_preserves_exact_model_string_and_namespace` in `ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs`, pinning exact model-string rendering, v1 namespace/function name, and hidden-model filtering. Worker reported `CARGO_BUILD_JOBS=8 just test -p ontocode-core multi_agents_spec` and `CARGO_BUILD_JOBS=8 just fmt` passed. No production code was needed. Manager reviewed diff; OntoIndex remains fresh at HEAD with dirty-worktree medium confidence.
- 2026-06-21: `HERMES-R4` closed. Added Unix-only `detached_session_keeps_long_running_process_alive_then_cleans_it_up` in `ontocode-rs/exec-server/src/server/handler/tests.rs`, proving a long-running process survives the first detach/resume handoff and exits after final shutdown. Manager tightened PID-file shell argument handling, reran `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server detached_session_keeps_long_running_process_alive_then_cleans_it_up`, and reran `CARGO_BUILD_JOBS=8 just fmt`; both passed. OntoIndex remains fresh at HEAD with dirty-worktree medium confidence.
- 2026-06-21: `HERMES-R2` closed. Added bounded repeated MCP failure state in the existing TUI tool lifecycle owner and regression coverage in `history_replay.rs`. Manager added an 80-character tool-label cap and verified the hint emits only once on the second repeated failure. `CARGO_BUILD_JOBS=8 just test -p ontocode-tui repeated_failed_mcp_tool_calls_emit_bounded_hint_once` and `CARGO_BUILD_JOBS=8 just fmt` passed. OntoIndex remains fresh at HEAD with dirty-worktree medium confidence.
- 2026-06-21: Manager closeout complete. All four accepted Hermes slices are closed; all other Hermes donor rows remain closed without dispatch unless a fresh owner-local failing fixture reopens them.
- 2026-06-21: Senior `gpt-5.4-mini` review found no blockers and one non-blocking R2 coverage gap: the 80-character label cap was not directly asserted. Manager added the long-label assertion to `repeated_failed_mcp_tool_calls_emit_bounded_hint_once`; the focused TUI test and `CARGO_BUILD_JOBS=8 just fmt` passed again.
- 2026-06-21: Follow-up review found `HERMES-R1` had proved exact model string and namespace but not restricted parent tool exposure. Added `host_context_gates_goal_and_agent_job_tools` assertions in `ontocode-rs/core/src/tools/spec_plan_tests.rs` proving a review/restricted subagent does not expose or register `spawn_agent` or the v1 multi-agent namespace. `CARGO_BUILD_JOBS=8 just test -p ontocode-core host_context_gates_goal_and_agent_job_tools` and `CARGO_BUILD_JOBS=8 just fmt` passed.
