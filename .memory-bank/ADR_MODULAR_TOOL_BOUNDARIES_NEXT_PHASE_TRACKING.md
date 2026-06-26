---
name: Modular Tool Boundaries Next Phase Tracking
description: Bounded manager-loop ledger for proposal candidates promoted from ADR_MODULAR_TOOL_BOUNDARIES_NEXT_PHASE.md
type: tracking
date: 2026-06-23
status: approved-queue-done
---

# Modular Tool Boundaries Next Phase Tracking

Authority:
- `ADR_MODULAR_TOOL_BOUNDARIES_NEXT_PHASE.md`

## Manager Rules

- Update this file before starting each bounded task and after verification.
- Use OntoIndex impact/context before production symbol edits and refresh OntoIndex after each completed bounded task.
- Keep build/test/fmt in single mode only for this loop.
- Keep work owner-local. Do not introduce a second tool registry, dispatcher, MCP runtime, or public API/config/schema surface.

## Tasks

| ID | Task | Owner | Status | Write Scope | Verification |
| --- | --- | --- | --- | --- | --- |
| `MTBNP-N1` | Extract telemetry/span and result-shaping helpers from `ontocode-rs/core/src/mcp_tool_call.rs` into private sibling modules while keeping `handle_mcp_tool_call` as the runtime owner | manager | done-approved-slice | `ontocode-rs/core/src/mcp_tool_call.rs`, new private sibling modules under `ontocode-rs/core/src/`, minimal `lib.rs` wiring, and scoped test updates only if needed | `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib`; `CARGO_BUILD_JOBS=8 just test -p ontocode-core mcp_tool_call`; `CARGO_BUILD_JOBS=8 just fmt` |
| `MTBNP-N2` | Split `ontocode-rs/core/src/tools/context.rs` by output family while keeping invocation state and shared helpers in the root owner | manager | done-approved-slice | `ontocode-rs/core/src/tools/context.rs`, new private sibling modules under `ontocode-rs/core/src/tools/`, minimal `mod.rs`/test wiring, and scoped test updates only if needed | `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib --tests`; `CARGO_BUILD_JOBS=8 just test -p ontocode-core tools::context`; `CARGO_BUILD_JOBS=8 just fmt` |
| `MTBNP-N3` | Split static naming/spec/search helpers out of `ontocode-rs/core/src/tools/handlers/mcp.rs` while keeping `McpHandler` and its runtime impls in the root owner | manager | done-approved-slice | `ontocode-rs/core/src/tools/handlers/mcp.rs`, new private sibling module(s) under `ontocode-rs/core/src/tools/handlers/`, minimal mod/test wiring, and scoped test updates only if needed | `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib`; `CARGO_BUILD_JOBS=8 just test -p ontocode-core mcp`; `CARGO_BUILD_JOBS=8 just fmt` |

## Event Log

- 2026-06-23: Manager opened next-phase tracking before code edits.
- 2026-06-23: OntoIndex impact for `handle_mcp_tool_call` is `LOW` upstream with one direct caller: `ontocode-rs/core/src/tools/handlers/mcp.rs:McpHandler.handle`.
- 2026-06-23: The worktree is dirty, including `mcp_tool_call.rs` and `mcp_tool_approval_templates.rs`, so the slice is constrained to a mechanical extraction only. No approval flow, policy, runtime, or event semantics changes are allowed.
- 2026-06-23: `MTBNP-N1` completed as a mechanical extraction only. New private sibling modules now own MCP call telemetry/span helpers and MCP result-shaping helpers; `handle_mcp_tool_call` remains the runtime owner in `mcp_tool_call.rs`.
- 2026-06-23: OntoIndex freshness remains current at HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2`; scope confidence remains `medium` because the worktree is dirty.
- 2026-06-23: Manager opened `MTBNP-N2` before code edits. The intended slice is owner-local only: keep `ToolCallSource`, `ToolInvocation`, `SharedTurnDiffTracker`, `boxed_tool_output`, `function_tool_response`, and `telemetry_preview` in `tools/context.rs`, and move output-family implementations into private sibling modules.
- 2026-06-23: `MTBNP-N2` completed as a bounded owner-local split. `McpToolOutput` now lives in `ontocode-rs/core/src/tools/context_mcp_output.rs`, `ExecCommandToolOutput` now lives in `ontocode-rs/core/src/tools/context_exec_output.rs`, and `tools/context.rs` stays the invocation/shared-helper owner.
- 2026-06-23: OntoIndex freshness remains current at HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2` after `MTBNP-N2`; scope confidence remains `medium` because the worktree is dirty.
- 2026-06-23: Manager opened `MTBNP-N3` before code edits. The intended slice is owner-local only: keep `McpHandler` plus its `ToolExecutor` and `CoreToolRuntime` impls in `tools/handlers/mcp.rs`, and move static naming/spec/search helpers into a private sibling module.
- 2026-06-23: `MTBNP-N3` completed as a bounded owner-local split. Static MCP naming/spec/search helpers now live in `ontocode-rs/core/src/tools/handlers/mcp_support.rs`, while `tools/handlers/mcp.rs` keeps `McpHandler` and its runtime impls.
- 2026-06-23: OntoIndex freshness remains current at HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2` after `MTBNP-N3`; scope confidence remains `medium` because the worktree is dirty.
- 2026-06-23: Fresh senior re-challenge of blocked `tools/registry.rs` work is complete. OntoIndex still shows `ToolRegistry.dispatch_any_with_terminal_outcome` as the central dispatch owner; no new implementation task is opened from this tracker.
- 2026-06-23: The only plausible future seam inside `tools/registry.rs` is post-tool-use feedback shaping around `PostToolUseFeedbackOutput`, but that candidate remains unapproved because current evidence does not justify reopening the queue.

## Verification Log

- 2026-06-23: `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib` passed. The only reported warning was an unrelated existing `unused_mut` in `core/src/tools/context.rs`.
- 2026-06-23: `CARGO_BUILD_JOBS=8 just test -p ontocode-core mcp_tool_call` passed. Nextest summary: 80 tests run, 80 passed.
- 2026-06-23: `CARGO_BUILD_JOBS=8 just fmt` passed after the final extraction edits.
- 2026-06-23: No new implementation task is opened from this tracking file after `MTBNP-N1`. The next valid follow-up is a fresh bounded approval of `N2` or `N3`.
- 2026-06-23: `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib --tests` passed for `MTBNP-N2`. The only reported warning was an unrelated existing dead-code helper in `core/tests/suite/code_mode.rs`.
- 2026-06-23: `CARGO_BUILD_JOBS=8 just test -p ontocode-core tools::context` passed for `MTBNP-N2`. Nextest summary: 14 tests run, 14 passed.
- 2026-06-23: `CARGO_BUILD_JOBS=8 just fmt` passed after the final `MTBNP-N2` edits.
- 2026-06-23: No new implementation task is opened from this tracking file after `MTBNP-N2`. The next valid follow-up is a fresh bounded approval of `N3`.
- 2026-06-23: `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib` passed for `MTBNP-N3`.
- 2026-06-23: `CARGO_BUILD_JOBS=8 just test -p ontocode-core mcp` passed for `MTBNP-N3`. Nextest summary: 221 tests run, 221 passed.
- 2026-06-23: `CARGO_BUILD_JOBS=8 just fmt` passed after the final `MTBNP-N3` edits.
- 2026-06-23: The approved next-phase queue is complete. No new implementation task is opened from this tracking file without a fresh senior challenge of blocked `tools/registry.rs` work or a newly identified owner-local seam.
- 2026-06-23: Fresh senior challenge completed for `tools/registry.rs`; result is still blocked for broad modularization and still below the bar for a new bounded task.
