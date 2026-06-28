# ADR: Lean-ctx Translation 3D Proposal

## Status

Reopened

## Date

2026-06-27

## History

### 2026-06-28 Shutdown Pivot

The project goal changed from "expose lean-ctx through Ontocode" to "remove
dependency on external lean-ctx" because the external lean-ctx project was
expected to shut down.

That invalidated the earlier assumption of a healthy upstream service worth
integrating directly, and the repo removed the first repo-local plugin proof.

### 2026-06-28 Maintained Fork Reopen

The goal changed again: pursue a locally maintained fork as the plugin backend.

The new claim is narrower and more defensible than the old upstream-adapter
goal:

- Ontocode no longer depends on an upstream lean-ctx project staying alive
- Ontocode keeps the plugin boundary instead of porting runtime internals into
  `ontocode-core`
- the team explicitly owns protocol stability, packaging, and maintenance for
  the forked backend

## Historical Context

The original ADR treated lean-ctx as an external context engine with a stable
daemon contract and proposed a plugin-only adapter so Ontocode could reuse that
runtime without porting it into core.

That was acceptable while the external runtime remained viable. The shutdown
made the upstream-owned variant wrong, but it did not prove that the plugin
boundary itself was wrong.

## Updated Problem

Ontocode needs a path to preserve selected lean-ctx behavior without depending
on an upstream project that may disappear.

The correct target is no longer "use upstream lean-ctx as-is" and no longer
"remove the whole surface forever." The new target is "own the backend
ourselves, keep it behind the plugin boundary, and carry only the smallest
surface that remains worth maintaining."

## Decision

Pursue `Local maintained fork as plugin backend`.

Instead:

1. restore the repo-local plugin path only after the fork ownership boundary is
   explicit
2. treat the backend as Ontocode-maintained external infrastructure, not as an
   upstream dependency
3. keep the plugin boundary and avoid porting lean-ctx runtime internals into
   `ontocode-core`
4. carry only the narrow tool surface that still has concrete user value

## Architecture

- `ontocode-rs/core-plugins`: plugin manifest, marketplace, install/load, and
  MCP wiring
- maintained lean-ctx fork: backend daemon, HTTP MCP contract, auth token,
  release packaging, and runtime behavior
- `ontocode-rs/core`: only the minimum compatibility guidance and fallback
  behavior when the plugin/backend is absent
- OntoIndex and Native Context Tools: remain the fallback and comparison
  baseline, not the default replacement path for this reopened plan

## Maintained Fork Contract

- Backend repo/home:
  - canonical local development home: in-repo companion subtree
    `third_party/lean-ctx-fork`
  - Ontocode repo vendors the maintained backend outside `ontocode-rs/`
- Carried v1 tool surface:
  - `ctx_read`
  - `ctx_search`
  - `ctx_summary`
- Explicitly out of v1:
  - `ctx_shell`
  - `ctx_edit`
  - session or knowledge tools
  - any broad cache or memory substrate inside Ontocode
- Transport:
  - Streamable HTTP MCP only
  - local default endpoint: `http://127.0.0.1:7777`
  - bearer token env var: `LEANCTX_TOKEN`
- Ownership:
  - Ontocode repo owns the companion backend subtree, plugin manifest,
    marketplace wiring, and proof notes
- Local startup:
  - the backend is started separately from `third_party/lean-ctx-fork`
  - the plugin never spawns or bootstraps the backend process itself
- Fail-closed behavior:
  - the plugin may install and load, but if the required backend is absent or
    auth is missing, plugin MCP availability fails closed
  - no plugin tools should appear as healthy substitutes in that state
  - OntoIndex and native `rg` remain available regardless

## Non-Goals

- Do not depend on upstream lean-ctx staying healthy.
- Do not vendor the backend into `ontocode-core`.
- Do not create a second native core runtime stack inside Ontocode.
- Do not reopen the whole historical lean-ctx surface without a bounded list of
  tools worth carrying.
- Do not pretend shutdown-removal work was wrong; it was the correct response
  before the maintained-fork goal was chosen.

## Migration Direction

1. define the maintained-fork ownership contract and minimum carried tool
   surface
2. restore the repo-local plugin package against the maintained fork, not
   against upstream lean-ctx
3. prove install/load/fail-closed behavior through `ontocode-core-plugins`
4. document when OntoIndex/native paths remain preferred versus when the
   maintained fork is the right tool
5. keep the backend optional and bounded even after migration restarts

## Open Questions

- Which exact `ctx_*` tools are worth carrying in the maintained fork v1?
- Where will the maintained fork live, and who owns release/version policy?
- Should the plugin stay repo-local only, or become a maintained marketplace
  package later?
- What is the fail-closed behavior when the maintained backend is absent or
  misconfigured?
