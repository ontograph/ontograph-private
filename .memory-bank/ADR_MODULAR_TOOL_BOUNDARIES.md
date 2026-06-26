# ADR: Modular Tool Boundaries Without A Second Runtime

## Status

Accepted as architecture direction; implementation must land in small staged slices

## Date

2026-06-22

## Context

The project has a large and growing tool surface. The risk is not lack of extension points. The risk is neighbor-domain mixing: MCP behavior, tool routing, extension wiring, provider logic, shell logic, and context packaging bleeding into the same modules.

Current code already has the main seams:

- model-visible tool planning and router composition in [spec_plan.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:152)
- runtime tool registry in [registry.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/registry.rs:326)
- typed extension contribution registry in [registry.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/extension-api/src/registry.rs:13)
- extension-owned tool contribution points in [extension.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/web-search/src/extension.rs:105) and [extension.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/memories/src/extension.rs:93)
- MCP runtime ownership in [connection_manager.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/codex-mcp/src/connection_manager.rs:105)

The concentration problem is visible in current file sizes:

- `ontocode-rs/core/src/tools/spec_plan.rs`: 1075 lines
- `ontocode-rs/core/src/mcp_tool_call.rs`: 2154 lines
- `ontocode-rs/ext/extension-api/src/registry.rs`: 232 lines

The project rules already reject duplicate architecture:

- no second provider registry
- no second MCP manager
- no second tool registry
- no parallel shell, hook, or context pipeline

The modularization plan must therefore deepen the existing seams instead of inventing a new platform.

## Decision

Adopt a layered modularization path built on the existing router, registry, extension, and MCP owners.

Primary decision:

1. Keep exactly one core tool router and one runtime tool registry.
2. Split tool planning by source family, not by ad hoc helper accumulation.
3. Keep extension-owned tool families on the existing `ToolContributor` seam.
4. Keep MCP runtime and MCP auth/status/tool visibility under `McpConnectionManager` and existing RMCP owners.
5. Push domain logic back to its owning crate; keep `core` as orchestration, exposure policy, and dispatch composition.

## Accepted Architecture Shape

### A. Thin tool planner composition

`ontocode-rs/core/src/tools/spec_plan.rs` should become a thin composer over smaller planning modules.

Preferred module split:

- `tools/planning/native.rs`
- `tools/planning/mcp.rs`
- `tools/planning/extensions.rs`
- `tools/planning/dynamic.rs`
- `tools/planning/hosted.rs`

Each planner module should contribute planned runtimes and model-visible specs for one source family only. The root planner should only compose, dedupe, and apply exposure policy.

### B. Domain-family ownership, not tool-name ownership

Group tool code by owner domain instead of by individual tool names scattered across unrelated modules.

Preferred domain families:

- workspace-read
- workspace-write
- process-exec
- mcp-bridge
- provider-facing tool adapters
- memory-tools

Each family owns its spec builders, executors, local redaction/helpers, and tests.

Clarification:

- `provider-facing tool adapters` may package tool-facing glue only.
- provider runtime behavior, descriptors, auth handling, capability logic, and model selection remain in `model-provider` and related existing provider owners.
- no new provider tool-family layer may become a shadow owner for provider runtime behavior.

### C. Extension-owned optional tool families

Optional or feature-scoped tool families should use the existing extension registry and `ToolContributor` path rather than adding a second registration mechanism.

Extension placement rule:

- use an extension crate when the feature is optional and already fits the contributor model: tool contribution, config contribution, lifecycle hooks, bounded context contribution, or related extension-owned state
- do not move mandatory runtime orchestration, mandatory session wiring, MCP runtime internals, provider runtime internals, or shell runtime internals into extension crates just for modularity
- extension crates are not a generic dumping ground for code that `core` has outgrown

Good fits:

- web search
- memories
- image generation
- future optional tool families with their own config and lifecycle

Not a fit:

- MCP transport/runtime internals
- provider runtime internals
- shell runtime internals

These remain with their existing owners.

### D. Hard boundaries for high-risk domains

The following boundaries are mandatory:

- MCP transport, auth, retries, status, and server lifecycle stay in `codex-mcp` and `rmcp-client`
- provider descriptors and runtime behavior stay in `model-provider`
- shell execution and policy stay in `exec*`, `shell-*`, and `sandboxing`
- model context injection stays in `context-fragments` and approved context contributors
- extension contract definitions stay in `ext/extension-api`

`core` may orchestrate these domains but must not absorb their business logic.

## Rejected Solutions

### Rejected: second global tool registry

Rejected because [spec_plan.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:160), [router.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/router.rs:35), and [registry.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/registry.rs:326) already own this responsibility. A second registry would duplicate exposure, namespace, and dispatch policy.

### Rejected: generic modular-tools platform

Rejected because it would create a second runtime surface and move complexity instead of removing it.

### Rejected: plugin-first rewrite

Rejected because the repo already has an extension API. A rewrite would churn stable paths before the planner and ownership boundaries are cleaned up.

### Rejected: unified cross-domain capability manager

Rejected because it mixes unrelated concerns. Provider, MCP, shell, hooks, and context have different risk profiles and different existing owners.

### Rejected: MCP replacement as a parallel stack

Rejected because MCP mutations, runtime state, and visibility already belong to `McpConnectionManager` and RMCP paths. Any replacement must be an owner-local refactor or adapter behind the existing owner, not a sibling runtime.

## Implementation Stages

### Stage 1: planner extraction only

Goal:

- reduce `spec_plan.rs` to composition/orchestration

Allowed changes:

- extract source-family planning modules
- move local helper functions out of `spec_plan.rs`
- keep behavior identical

Not allowed:

- new tool schemas
- new public config
- new app-server API
- new runtime owners

### Stage 2: split oversized MCP tool-call shaping

Goal:

- break `mcp_tool_call.rs` into concern-local modules

Initial proposal split:

- approval and review flow
- MCP metadata and visibility lookup
- tool-call execution and result shaping
- truncation/redaction
- fixture/test helpers

Rationale:

- OntoIndex evidence shows `handle_mcp_tool_call` is coupled to approval flow, metadata lookup, policy bridging, start notifications, and approved-call handling; splitting only by request/response formatting would cut across the actual seams.

Not allowed:

- alternate MCP runtime
- new MCP registry
- new auth persistence path

Fresh senior-review narrowing on 2026-06-23:

- only one Stage 2 slice is currently implementation-approved
- approved slice: move serialized MCP approval elicitation payload shaping into `ontocode-rs/core/src/mcp_tool_approval_templates.rs`
- approved moved items only:
  - `McpToolApprovalPromptOptions`
  - `McpToolApprovalElicitationRequest`
  - `mcp_tool_approval_prompt_options`
  - `build_mcp_tool_approval_elicitation_request`
  - `build_mcp_tool_approval_elicitation_meta`
  - `build_mcp_tool_approval_display_params`
- keep in `mcp_tool_call.rs`:
  - `handle_mcp_tool_call`
  - `maybe_request_mcp_tool_approval`
  - `build_mcp_tool_approval_question`
  - `build_mcp_tool_approval_fallback_message`
  - `parse_mcp_tool_approval_elicitation_response`
  - `request_user_input_response_from_elicitation_content`
  - `parse_mcp_tool_approval_response`
  - `normalize_approval_decision_for_mode`
  - approval decision application, guardian review flow, telemetry, sanitization, and event-emission helpers
- prompt wording, approval decision semantics, visibility lookup, and runtime flow are explicitly out of scope for this slice

Current state after bounded loop closure on 2026-06-23:

- the approved Stage 2A slice above is implemented and accepted
- no broader Stage 2 split is currently approved for dispatch
- any later Stage 2 follow-up must start with a fresh senior review that names one owner-local slice and keeps `handle_mcp_tool_call` runtime ownership intact

### Stage 3: move optional families to owner-local extension crates where missing

Goal:

- make optional families extension-owned where that already matches the architecture

Current state after review on 2026-06-23:

- this is already satisfied for the current optional families that were in scope for review
- existing owner-local `ToolContributor` seams already cover:
  - web search
  - memories
  - goal
  - image generation
- Stage 3 is therefore not an active remaining implementation task in this ADR unless a new optional family is introduced later

Allowed:

- owner-local tool contributors
- owner-local tests
- owner-local config wiring

Not allowed:

- moving mandatory runtime domains like MCP or shell behind optional extensions

### Stage 4: add owner-boundary checks

Goal:

- prevent regression into neighbor mixing

Examples:

- no provider-runtime imports from MCP planning modules
- no MCP auth logic inside extension crates
- no shell runtime helpers inside context modules

These checks must start as focused ownership-flow tests or static repository audits before any stronger linting is considered.

Current state after review on 2026-06-23:

- one narrow Stage 4 slice is now approved and accepted:
  - source-level static audit in `tools::spec_plan::tests::mcp_planning_module_keeps_provider_and_auth_owners_out`
  - current guardrail scope: keep provider/auth owners out of `ontocode-rs/core/src/tools/planning/mcp.rs`
- no broader Stage 4 linting or repository-wide audit framework is approved from this ADR
- any future Stage 4 work must still be reopened as one bounded guardrail task with an explicit owner and verification shape

Minimum boundary-check questions:

- who owns tool visibility
- who owns approval/policy decisions
- who owns runtime execution
- who owns redaction and output shaping

Import-only checks are insufficient when call-path ownership is the real mixing risk.

## Review Rules For Future Tool Additions

Any new tool proposal must pass all of these checks:

1. It belongs to one existing owner.
2. It does not require a second registry or manager.
3. It does not widen public API/config/schema without an ADR.
4. Its model-visible surface reuses the existing router/registry/exposure path.
5. Its runtime behavior stays inside the domain owner crate.

If a proposal needs provider, MCP, shell, and context logic in the same new module, the proposal should be rejected or narrowed.

## First Recommended PR Slice

The first safe slice from this ADR was Stage 1:

- extract planner modules from `ontocode-rs/core/src/tools/spec_plan.rs`
- keep `ToolRouter`, `ToolRegistry`, extension registry, and `McpConnectionManager` unchanged
- add focused regression tests around spec composition and namespace/exposure behavior

That was the right first move because it delivered real modularity with the smallest architectural risk.

Current dispatch gate after the 2026-06-23 bounded review loop:

- Stage 1 is implemented and accepted.
- Stage 2A is implemented and accepted as the narrow approval-template extraction only.
- Stage 3 is not an active implementation queue for the currently reviewed optional families.
- Stage 4A is implemented and accepted as one narrow planning-boundary guardrail.
- No broader Stage 2, Stage 3, or Stage 4 program is approved from this ADR.
- Any further modularization must start from a fresh senior review that names one bounded owner-local slice and proves it does not create a parallel owner.
