# ADR: Modular Tool Boundaries Next Phase

## Status

Proposed for senior review; no implementation dispatch is approved from this ADR yet

## Date

2026-06-23

## Context

The accepted work in [ADR_MODULAR_TOOL_BOUNDARIES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_MODULAR_TOOL_BOUNDARIES.md:1) is closed and should not be stretched into synthetic follow-up work.

OntoIndex confirms the current repo index is fresh at HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2`; scope confidence is still `medium` because the worktree is dirty.

The previous ADR already reduced the planner hotspot:

- `ontocode-rs/core/src/tools/spec_plan.rs`: 339 lines
- OntoIndex context still shows [build_tool_router](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:152) as a thin wrapper over [build_tool_specs_and_registry](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:160)

The next pressure points are different owners:

- `ontocode-rs/core/src/mcp_tool_call.rs`: 1967 lines
- `ontocode-rs/core/src/tools/registry.rs`: 759 lines
- `ontocode-rs/core/src/mcp_tool_approval_templates.rs`: 572 lines
- `ontocode-rs/core/src/tools/handlers/mcp.rs`: 539 lines
- `ontocode-rs/core/src/tools/context.rs`: 530 lines

Fresh OntoIndex evidence on these owners:

1. [handle_mcp_tool_call](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/mcp_tool_call.rs:100) remains runtime-owned by the MCP path. Upstream impact is `LOW` with one direct caller: [McpHandler.handle](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/mcp.rs:117).
2. [McpHandler.handle](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/mcp.rs:117) is already thin at runtime and delegates into `handle_mcp_tool_call`, `boxed_tool_output`, and `McpToolOutput`, but the same file still also owns tool-name normalization, tool-spec creation, hook-input normalization, and MCP search text construction.
3. [ToolRegistry.dispatch_any_with_terminal_outcome](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/registry.rs:407) is still the central owner for tool dispatch, lifecycle events, hook mediation, telemetry tags, and tool-result evidence recording. Its outgoing context touches hooks, telemetry, lifecycle, session turn state, and payload logging. That is too central for a broad cleanup slice.
4. [tools/context.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/context.rs:1) mixes the invocation model with output implementations for MCP, tool search, generic function tools, apply-patch, aborted calls, and unified exec.

## Decision

Open a new next-phase modularization ADR with only owner-local candidates. Do not reopen the closed Stage 1/2A/4A loop.

Primary rules:

1. Keep one tool router, one tool registry, one MCP runtime path, and one hook/lifecycle path.
2. Prefer extractions that reduce neighbor mixing inside an existing owner file.
3. Do not approve any slice that creates a parallel dispatcher, registry, planner, MCP adapter stack, or public tool/config/API surface.
4. Treat `tools/registry.rs` as blocked for broad modularization until one smaller internal seam is proven.

## Approved Candidate Families

These are candidates for later bounded dispatch. They are not yet implementation-approved.

### Candidate N1: `mcp_tool_call.rs` support-module extraction

Goal:

- keep `handle_mcp_tool_call` as the runtime owner
- move clearly support-local helpers out of the root file

Good extraction targets:

- span and telemetry helpers such as `emit_mcp_call_metrics`, `mcp_call_metric_tags`, `mcp_tool_call_span`, `record_server_fields`, and `record_mcp_result_span_telemetry`
- output-shaping helpers such as `sanitize_mcp_tool_result_for_model` and `truncate_mcp_tool_result_for_event`

Keep in the root owner:

- `handle_mcp_tool_call`
- approval decision flow
- guardian review flow
- policy bridging
- started/skipped/completed event emission
- metadata lookup and approved-call control flow

Rationale:

- this stays inside the MCP runtime owner
- OntoIndex impact on the root runtime function is low enough for a narrow internal split
- Stage 2A already proved that `mcp_tool_call.rs` can shrink safely when the moved code is obviously subordinate to the existing owner

Current state:

- completed on 2026-06-23 as a bounded mechanical extraction
- private sibling modules now own telemetry/span helpers and result-shaping helpers
- `handle_mcp_tool_call` remains in `mcp_tool_call.rs` as the runtime owner

### Candidate N2: `tools/context.rs` output-family split

Goal:

- keep invocation state in one place
- stop mixing unrelated tool-output types in the same file

Preferred split:

- root `context.rs` keeps `ToolCallSource`, `ToolInvocation`, `SharedTurnDiffTracker`, and lightweight shared helpers
- sibling modules own output families such as MCP output and unified-exec output

Good first targets:

- `McpToolOutput`
- `ExecCommandToolOutput`
- helper functions that are private to those output families

Rationale:

- this is pure owner-local cleanup inside the tool-output boundary
- it reduces file size without changing the router, registry, or runtime ownership graph

Current state:

- completed on 2026-06-23 as a bounded owner-local split
- `McpToolOutput` and `ExecCommandToolOutput` now live in private sibling modules
- `tools/context.rs` keeps invocation state and shared response/preview helpers

### Candidate N3: `tools/handlers/mcp.rs` spec/search helper split

Goal:

- keep `McpHandler` focused on runtime dispatch
- move static MCP tool-description shaping next to each other

Good extraction targets:

- `join_tool_name`
- `ensure_mcp_prefix`
- `create_tool_spec`
- `mcp_hook_tool_input`
- `build_mcp_search_text`

Keep in the root handler:

- `McpHandler`
- `ToolExecutor` implementation
- `CoreToolRuntime` implementation

Rationale:

- OntoIndex context shows the runtime edge is already narrow
- the file still mixes runtime invocation with naming/spec/search helpers

Current state:

- completed on 2026-06-23 as a bounded owner-local split
- static MCP naming/spec/search helpers now live in `tools/handlers/mcp_support.rs`
- `tools/handlers/mcp.rs` keeps `McpHandler` plus its runtime impls

## Explicitly Blocked Candidate

### Blocked B1: broad `tools/registry.rs` split

Blocked for now.

Reason:

- OntoIndex context for `ToolRegistry.dispatch_any_with_terminal_outcome` shows it is the live crossing point for hooks, lifecycle, telemetry, cancellation accounting, and evidence recording
- a broad split here would likely create a second dispatch owner by accident

Fresh re-challenge on 2026-06-23:

- OntoIndex still reports upstream impact as `LOW`, with two direct callers and tool-module-local blast radius only.
- That low blast radius is not enough to approve work by itself because the symbol still owns the dispatch crossing point for pre-hook execution, handler dispatch, post-hook execution, lifecycle notifications, telemetry, and turn evidence recording.
- The smallest plausible seam is the post-tool-use feedback shaping block only:
  - `PostToolUseFeedbackOutput`
  - replacement-text selection after `run_post_tool_use_hooks`
  - the guarded model-visible output replacement path
- That seam is still only a candidate. It is not approved from this ADR yet because the current proof shows extractability, but not enough behavioral or review value to justify opening a new task while the approved queue is already closed.

Reopen only if a later review can prove one small internal seam, such as a test-backed extraction of post-tool-use feedback shaping or terminal-outcome bookkeeping, without changing dispatch ownership.

## Recommended Order

1. Re-challenge `B1` only if a smaller `tools/registry.rs` seam is proven

Do not open `B1` from this ADR unless a fresh challenge narrows it first.

## Verification Rules

For any candidate promoted from this ADR:

- update tracking before dispatch
- run OntoIndex impact/context on the target symbol before code edits
- keep build/test/fmt in single mode only
- refresh OntoIndex after the bounded task closes
- keep verification scoped to the owner being changed

Suggested verification shapes:

- `N1`: `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib`; `CARGO_BUILD_JOBS=8 just test -p ontocode-core mcp_tool_call`; `CARGO_BUILD_JOBS=8 just fmt`
- `N2`: `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib --tests`; scoped `tools::context` coverage if changed; `CARGO_BUILD_JOBS=8 just fmt`
- `N3`: `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib`; scoped MCP handler coverage if changed; `CARGO_BUILD_JOBS=8 just fmt`

## Challenge Summary

What is done:

- planner modularization is done enough for the current ADR
- Stage 2A approval-template extraction is done
- `N1` MCP support-module extraction is done
- `N2` tool-output family split is done
- `N3` MCP handler support split is done
- Stage 3 is already satisfied for current optional families
- Stage 4A boundary audit is done

What is not justified:

- pretending Stage 3 still has implementation work
- broad registry surgery
- opening a registry follow-up just because `registry.rs` is still large
- a second modularization program that bypasses current owners

What this ADR does:

- opens only the next owner-local candidates
- keeps the existing accepted ADR closed
- gives a narrow queue for later senior approval
