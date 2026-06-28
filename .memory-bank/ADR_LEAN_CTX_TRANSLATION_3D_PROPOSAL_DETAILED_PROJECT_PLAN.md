---
name: Lean-ctx Maintained Fork Plugin Backend Detailed Project Plan
description: Execution-grade plan for restoring the lean-ctx plugin path against an Ontocode-maintained forked backend
type: project_plan
date: 2026-06-28
status: complete
---

# Lean-ctx Maintained Fork Plugin Backend Detailed Project Plan

## Goal

Restart lean-ctx migration as `Local maintained fork as plugin backend`.

The target is not upstream lean-ctx and not a native core port. The target is
an Ontocode-owned forked backend that stays behind the existing plugin and MCP
boundary, with the maintained source now living in this repo as a companion
subtree.

## Source Authority

Primary authority:

- [ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL.md](ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL.md)

Current architecture seams:

- [Ontocode plugin package and marketplace owner](../ontocode-rs/core-plugins)
- [Ontocode MCP tool-call lifecycle](../ontocode-rs/core/src/mcp_tool_call.rs)
- [ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md](ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md)
- [ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md](ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md)

## Boundary Statement

This plan is about controlled reintroduction through an owned backend.

Allowed:

- restoring the repo-local plugin package
- targeting an Ontocode-maintained fork instead of upstream lean-ctx
- carrying a small explicit `ctx_*` tool allowlist through the existing plugin
  and HTTP MCP path
- keeping OntoIndex and native tooling as fallback and comparison paths

Not allowed:

- depending on upstream lean-ctx release health
- porting the backend into `ontocode-core`
- recreating a second native tool registry or session/search stack in core
- reopening the whole historical lean-ctx feature set without a bounded carried
  surface

Current OntoIndex/source challenge:

- the plugin/package and MCP seams already exist in `ontocode-core-plugins` and
  `core/src/mcp.rs`
- the maintained fork backend now lives in `third_party/lean-ctx-fork`
- the plugin/MCP contract stays the narrow owner boundary; importing the source
  does not reopen native-core porting

## Decision Summary

Use the plugin boundary, but own the backend:

- plugin/package and MCP wiring stay in `ontocode-core-plugins`
- backend protocol and runtime move to an Ontocode-maintained fork
- OntoIndex/native flows stay available and documented as fallback or preferred
  alternatives where they are still better

## F0 Contract Decisions

- Maintained fork local development home: `third_party/lean-ctx-fork`
- Carried v1 tool allowlist: `ctx_read`, `ctx_search`, `ctx_summary`
- Excluded from v1: `ctx_shell`, `ctx_edit`, session tools, knowledge tools,
  and any native cache/store clone inside Ontocode
- Plugin transport: Streamable HTTP MCP only
- Local default endpoint: `http://127.0.0.1:7777`
- Auth token env var: `LEANCTX_TOKEN`
- Release ownership:
  - Ontocode repo owns the companion backend subtree, plugin package,
    marketplace entry, and proof coverage
- Startup rule: backend started separately from `third_party/lean-ctx-fork`;
  the plugin
  does not spawn it
- Fail-closed rule:
  - plugin MCP server remains required
  - absent backend or missing token leaves the plugin unavailable rather than
    degrading into fake success
  - OntoIndex/native tooling stays available independently

## Owner Map

| Surface | Owner | Rule |
| --- | --- | --- |
| Repo-local plugin package | `ontocode-core-plugins` | Recreate the package and marketplace entry cleanly through existing plugin owners. |
| Maintained fork runtime | fork owner defined by this project | Own daemon behavior, auth token, transport contract, packaging, and updates outside `ontocode-core`. |
| Fallback code intelligence and exact search | OntoIndex and native `rg` | Keep fallback paths explicit so plugin absence does not break normal work. |
| Docs and rule guidance | `.memory-bank/` plus repo guidance files | Explain when the maintained fork is preferred and when OntoIndex/native tooling remains the right path. |

## Plan Summary

1. Reopen the project around an Ontocode-maintained fork backend.
2. Define the bounded carried tool surface and fork ownership contract.
3. Recreate the repo-local plugin package against that backend.
4. Prove install/load/fail-closed behavior.
5. Update rules so the plugin is recommended only where it clearly beats
   OntoIndex/native paths.

## Phase 0: Reopen And Fork Contract

### Objective

Replace the shutdown-removal goal with a controlled maintained-fork goal.

### Tasks

- define where the maintained fork lives
- define the bounded v1 tool allowlist
- define ownership for release/versioning/auth token policy
- define fail-closed behavior when the backend is absent

### Exit criteria

- the maintained-fork boundary is explicit
- the v1 carried surface is bounded
- the backend is clearly Ontocode-owned, not upstream-owned
- the repo path or package home for the maintained fork is named
- token, endpoint, and local startup expectations are written down
- concrete contract selected:
  - local home `third_party/lean-ctx-fork`
  - allowlist `ctx_read`, `ctx_search`, `ctx_summary`
  - `LEANCTX_TOKEN`
  - `http://127.0.0.1:7777`

### Reopen gate for later phases

Do not start Phase 1, 2, or 3 until Phase 0 names:

- the maintained fork location
- the bounded v1 `ctx_*` allowlist
- the version/release owner
- the fail-closed behavior when the backend is absent

## Phase 1: Plugin Package Restore

### Objective

Restore the repo-local plugin path against the maintained fork.

### Tasks

- recreate `plugins/ontocode-lean-ctx/`
- restore `.agents/plugins/marketplace.json`
- keep the plugin manifest and `.mcp.json` minimal and bounded
- restore focused `ontocode-core-plugins` install/load proof coverage
- reuse existing `core-plugins` test helpers and manifest/marketplace patterns;
  do not invent a second scaffold path

### Exit criteria

- repo-local plugin package exists again
- plugin install/load proof passes against the maintained-fork contract
- plugin failure is explicit and fail-closed

## Phase 2: Backend Ownership Wiring

### Objective

Connect the plugin to an owned backend rather than to upstream lean-ctx.

### Tasks

- import the maintained backend into `third_party/lean-ctx-fork`
- document endpoint, token, and startup contract for the maintained fork
- define update policy for the in-repo backend subtree
- keep backend ownership out of `ontocode-core`

### Exit criteria

- no remaining assumption depends on upstream lean-ctx
- maintained fork ownership is explicit
- plugin/backend contract is stable enough for bounded use

## Phase 3: Docs, Rules, And Usage Guidance

### Objective

Document the maintained-fork path without overclaiming it.

### Tasks

- update plan and tracking files
- update memory-bank summaries and audit notes
- restore lean-ctx guidance only where the maintained fork is the intended
  tool, not as a blanket replacement for OntoIndex/native tooling
- document fallback behavior clearly

### Exit criteria

- current plans direct work toward the maintained-fork plugin backend
- current rules distinguish maintained-fork usage from OntoIndex/native usage
- historical shutdown-removal notes remain preserved as history

## Non-Goals

- No upstream dependency restoration.
- No backend port into `ontocode-core`.
- No broad unbounded `ctx_*` surface.
- No claim that OntoIndex/native tooling should disappear.

## Stop Rule

If the maintained fork cannot be assigned an owner, packaged, and versioned as
an Ontocode-owned backend, stop. In that case, reopen the shutdown-removal
position rather than keeping a fake migration goal alive.
