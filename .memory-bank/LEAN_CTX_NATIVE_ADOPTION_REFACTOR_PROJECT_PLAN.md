---
name: Lean-ctx Native Adoption Refactor Project Plan
description: Refactoring plan for making the maintained lean-ctx fork a native Ontocode plugin backend without upstream/runtime dependency
type: project_plan
date: 2026-06-28
status: proposed
---

# Lean-ctx Native Adoption Refactor Project Plan

## Goal

Make `third_party/lean-ctx-fork` a maintained Ontocode plugin backend, not an
imported upstream product that happens to run from this repository.

The current plugin path works as a bounded Streamable HTTP MCP integration, but
the fork still carries broad upstream CLI/product behavior, a full registry that
is filtered late, default features unrelated to the plugin contract, and startup
side effects that are not native to Ontocode's plugin model.

The refactor target is narrow:

- keep `plugins/ontocode-lean-ctx` as the plugin boundary
- keep Streamable HTTP MCP as the transport
- keep the v1 carried tool surface to `ctx_read`, `ctx_search`, and
  `ctx_summary`
- remove operational dependence on upstream lean-ctx release/update/install
  paths
- make the maintained backend source and runtime behavior explicitly
  Ontocode-owned

## Source Evidence

OntoIndex baseline:

- `gn_ensure_fresh(repo=codex)` reported indexed HEAD and current HEAD both at
  `5edde24a78efe0f10bc710936dfa228427ab7fd1`.
- Limitations: dirty worktree and missing embeddings; OntoIndex is useful for
  routing the existing MCP/plugin owner but direct fork source is authoritative
  for `third_party/lean-ctx-fork`.

Current authoritative source:

- `plugins/ontocode-lean-ctx/README.md` defines the plugin contract:
  Streamable HTTP MCP, `LEANCTX_TOKEN`, `LEAN_CTX_TOOL_PROFILE=ontocode`, and
  exactly `ctx_read`, `ctx_search`, `ctx_summary`.
- `plugins/ontocode-lean-ctx/.mcp.json` requires the HTTP MCP backend and
  enables only those three tools.
- `third_party/lean-ctx-fork/rust/src/core/tool_profiles.rs` already contains
  `ToolProfile::Ontocode` and the three-tool allowlist.
- `third_party/lean-ctx-fork/rust/src/server/tool_visibility.rs` already blocks
  `ctx_call` invoker exposure for `ToolProfile::Ontocode` and makes dispatch
  allowlist enforcement authoritative.
- `third_party/lean-ctx-fork/rust/src/server/call_tool.rs` already rejects
  direct calls to tools outside the Ontocode profile.
- `third_party/lean-ctx-fork/rust/src/server/registry.rs` still registers the
  broad upstream-style tool set and filters later.
- `third_party/lean-ctx-fork/rust/src/cli/dispatch/mod.rs` still exposes broad
  product commands such as setup/install/update/dashboard/proxy/cloud/dev-install.
- `third_party/lean-ctx-fork/rust/src/cli/dispatch/server.rs`,
  `third_party/lean-ctx-fork/rust/src/server/server_handler.rs`,
  `third_party/lean-ctx-fork/rust/src/server/call_tool.rs`, and
  `third_party/lean-ctx-fork/rust/src/http_server/mod.rs` still contain startup
  or periodic side effects: hook/rule refresh, version checks, proxy/autopublish
  hooks, plugin manager hooks, cloud sync, archive cleanup, cognition scheduling,
  and index prewarm.
- `third_party/lean-ctx-fork/rust/Cargo.toml` still defaults broad features:
  `tree-sitter`, `embeddings`, `http-server`, `team-server`, `secure-update`,
  and `jemalloc`.
- Fork size baseline excluding `target`: about 2,093 files, about 403,540 Rust
  source lines under `rust/src`, about 32 MiB on disk.

## Architecture Options

### Option 1: Slim current maintained fork backend

Keep the current plugin package and Streamable HTTP MCP backend, then add an
explicit Ontocode backend mode inside the fork.

This is the recommended path because it preserves working behavior while
removing the worst non-native surfaces in small bounded slices.

Expected result:

- `just lean-ctx-plugin-backend-*` remains the operator path
- the backend registers only carried tools in Ontocode mode
- product CLI commands and startup side effects are blocked or skipped in
  Ontocode mode
- no new adapter interface and no `ontocode-core` port

### Option 2: Extract a plugin-owned backend crate/package

After Option 1 proves the smaller mode, move only the carried backend code into
a plugin-owned crate or package below the plugin/backend owner.

Use this only if the slim mode remains hard to package or test because too much
unrelated product code still compiles into the backend.

Expected result:

- source remains in-repo and self-contained
- public runtime remains Streamable HTTP MCP
- extracted crate exposes only backend server startup plus carried tools
- compatibility shims remain in the fork while the plugin package moves to the
  extracted owner

### Option 3: Port only the three tools into a native extension owner

Port `ctx_read`, `ctx_search`, and `ctx_summary` behavior into an Ontocode-owned
extension crate only if evidence shows the fork backend is still too costly to
carry.

This is higher risk because it risks recreating a second code-intelligence stack
beside OntoIndex/native owners. It requires an ADR before implementation.

### Option 4: Delete the fork backend and replace with OntoIndex/native tools

Use this only if compatibility with the lean-ctx tool names is no longer needed
or if the maintained fork cannot be made self-contained.

This is a shutdown/replacement decision, not a refactor of the current plugin.

## Recommendation

Proceed with Option 1 first.

Option 1 is the shortest path that adds real operational value: it removes
non-native behavior while preserving the working plugin boundary and current
smoke proof. Options 2-4 remain decision gates, not current implementation work.

## Challenge Decision

Accepted after review, with the dispatch queue narrowed.

The plan direction is valid, but not every phase is implementation-ready. The
existing fork already has `ToolProfile::Ontocode`, env/config profile
resolution, invoker blocking, and dispatch allowlist enforcement. Do not add a
new mode abstraction just to name existing behavior.

Dispatch policy:

- `LCTX-REF-0` is the only immediately dispatch-ready task.
- `LCTX-REF-1` is conditional evidence work; add code only if Phase 0 proves
  existing profile APIs cannot support registry/startup gates directly.
- `LCTX-REF-2` is the first likely code slice after Phase 0.
- `LCTX-REF-3` must target network/global/user-config mutation side effects
  first; local maintenance/index work stays until tests prove it is unnecessary
  for `ctx_read`, `ctx_search`, and `ctx_summary`.
- `LCTX-REF-4`, `LCTX-REF-5`, and extraction/native replacement remain gated,
  no-dispatch tasks until fresh evidence justifies them.
- `LCTX-REF-6` must inspect existing backend metadata first; add fields only if
  current metadata cannot prove mode/provenance.

## Non-Goals

- No port into `ontocode-core`.
- No new provider, tool registry, MCP transport, or plugin-spawning owner.
- No `ctx_shell`, `ctx_edit`, session tools, knowledge tools, dashboard, proxy,
  cloud sync, updater, installer, hook installer, broad shell execution, or
  external package-manager behavior in the Ontocode backend mode.
- No plugin wrapper scripts as the authority for safety. Enforcement belongs in
  the backend profile/registry/startup owner and existing plugin/MCP config.
- No hidden download step, upstream checkout, or dependency on upstream
  availability for normal use.
- No numeric savings/telemetry claims unless they are backed by current
  backend-local tests or fixtures.

## Target Architecture

The native shape should be:

- `plugins/ontocode-lean-ctx`
  - plugin manifest and `.mcp.json`
  - no process spawning
  - required bearer-auth Streamable HTTP MCP endpoint
  - enabled tools limited to `ctx_read`, `ctx_search`, `ctx_summary`
- `third_party/lean-ctx-fork/rust`
  - maintained source owner for the backend runtime
  - explicit Ontocode backend mode
  - mode-aware registry construction, not broad registration plus late filtering
  - side-effect-free startup in Ontocode mode
  - minimal feature/dependency profile for plugin backend builds
  - provenance that points at the in-repo maintained fork, not upstream release
    health
- OntoIndex/native fallback
  - remains the baseline when the plugin backend is absent or a task is outside
    the carried read-only surface

## Valuable Proposals Kept

| Proposal | Keep as | Reason |
| --- | --- | --- |
| Backend-only Ontocode mode | Phase 1 | Makes the fork API native to Ontocode without a new adapter interface. |
| Registry slimming | Phase 2 | Removes broad tool construction from the carried backend surface. |
| Startup side-effect removal | Phase 3 | Prevents hooks, proxy, cloud, update, telemetry, and maintenance behavior from running in plugin backend mode. |
| CLI/product surface blocking | Phase 4 | Stops inherited upstream commands from being presented as supported Ontocode operations. |
| Feature/dependency slimming | Phase 5 | Reduces compile/runtime cost and removes unrelated upstream features. |
| Backend capability/provenance response | Phase 6 | Gives operators a backend-native way to verify mode, version, and allowed tools. |
| Native tests for bounds and allowlist | All phases | Keeps proof close to the backend owner, not in external scripts. |
| Optional extraction | Decision gate | Useful only after slimming proves the actual residual cost. |
| Optional native extension/replacement | Decision gate | Useful only if the fork is no longer worth carrying. |

## Rejected Or Deferred Proposals

| Proposal | Disposition | Reason |
| --- | --- | --- |
| New plugin adapter interface | Rejected | Existing plugin + HTTP MCP path already owns this boundary. |
| Plugin-owned backend spawning | Rejected | Current contract says backend is started separately; spawning needs a separate ADR. |
| Wrapper preflight/check scripts as safety authority | Rejected | Safety must live in backend/profile/plugin config, not scripts. |
| Shell rewrite/filter runtime | Rejected | Violates read-only plugin surface and creates a second shell owner. |
| Telemetry/savings dashboard/analytics DB | Rejected | Not required for `ctx_read`, `ctx_search`, `ctx_summary`; creates privacy and maintenance surface. |
| Cloud/proxy/dashboard parity | Rejected | Upstream product surfaces are outside Ontocode plugin backend scope. |
| Immediate extraction | Deferred | Do after mode slimming proves what still needs extraction. |
| Immediate native port | Deferred | Requires ADR and compatibility decision. |

## Phase 0: Baseline Inventory And Guards

Status: not started.

Goal: lock the current behavior before refactoring.

Tasks:

- record current plugin contract from `plugins/ontocode-lean-ctx`
- record current backend mode/profile behavior from `tool_profiles.rs`,
  `tool_visibility.rs`, and `call_tool.rs`
- record current broad registry and side-effect owners
- add or confirm tests that prove:
  - `tools/list` in Ontocode mode returns exactly `ctx_read`, `ctx_search`,
    `ctx_summary`
  - direct `ctx_shell` and `ctx_edit` calls are rejected in Ontocode mode
  - repo smoke still passes

Validation:

- `bash -n scripts/run_lean_ctx_plugin_backend.sh scripts/smoke_lean_ctx_plugin_backend.sh`
- `just lean-ctx-plugin-backend-smoke`
- focused fork tests for tool profile/visibility/dispatch

Exit criteria:

- current behavior is proven before architecture edits
- exact source owners for each side effect are named in tracking

## Phase 1: Native Ontocode Backend Mode API Evidence

Status: conditional after Phase 0.

Goal: prove whether the existing `ToolProfile::Ontocode` API is enough for
registry and startup gates.

Preferred design:

- keep `ToolProfile::Ontocode`
- prefer direct reuse of existing profile/config resolution
- add one small explicit mode resolver only if Phase 0 proves callers cannot
  ask the required question without duplicating profile logic
- treat `LEAN_CTX_TOOL_PROFILE=ontocode` as the compatibility input
- optionally add a backend-local CLI flag only if current source proves env-only
  mode is too ambiguous for tests or packaging
- avoid a new public plugin API unless an ADR approves it

Expected files:

- `third_party/lean-ctx-fork/rust/src/core/tool_profiles.rs`
- `third_party/lean-ctx-fork/rust/src/core/config.rs` or the nearest existing
  config/profile owner
- backend-local tests beside those owners

Acceptance:

- either no code change is needed, or callsites can ask a named question such
  as "is this the Ontocode backend mode?" without duplicating profile logic
- no new bool-parameter API
- no second registry/adapter abstraction
- compatibility with current launcher remains intact

## Phase 2: Register Only Carried Tools In Ontocode Mode

Status: open after Phase 1.

Goal: stop constructing the full upstream registry for Ontocode backend mode.

Tasks:

- add a registry constructor that uses the existing `ToolProfile::Ontocode`
  decision and registers only:
  - `ctx_read`
  - `ctx_search`
  - `ctx_summary`
- preserve the existing full registry for non-Ontocode fork modes until a later
  deletion decision
- keep `profile_allows_dispatch` as a defense-in-depth gate
- add tests proving the Ontocode registry names are exactly the three carried
  tools

Expected files:

- `third_party/lean-ctx-fork/rust/src/server/registry.rs`
- `third_party/lean-ctx-fork/rust/src/server/mod.rs`
- `third_party/lean-ctx-fork/rust/src/tools/mod.rs` only if current ownership
  requires it

Acceptance:

- `build_registry` or its selected replacement does not register shell/edit,
  session, knowledge, plugin, proxy, cloud, or addon tools in Ontocode mode
- existing smoke still reports exactly the three tools
- direct non-carried calls still fail closed

## Phase 3: Skip External Startup Side Effects In Ontocode Mode

Status: open after Phase 1.

Goal: make plugin backend startup quiet with respect to network, global config,
user config, and updater behavior.

Tasks:

- guard or remove in Ontocode mode:
  - proxy autostart
  - wrapped publish/autopush
  - plugin manager startup hooks from the fork
  - rules injection and hook refresh
  - version/update background checks
  - cloud background sync
- keep local maintenance/index work until Phase 0 or focused tests prove it is
  not required for carried read/search/summary behavior
- keep only runtime work required for the three carried tools and HTTP MCP
  serving
- add backend-local tests for the mode gates where practical

Expected files:

- `third_party/lean-ctx-fork/rust/src/cli/dispatch/server.rs`
- `third_party/lean-ctx-fork/rust/src/server/server_handler.rs`
- `third_party/lean-ctx-fork/rust/src/server/call_tool.rs`
- `third_party/lean-ctx-fork/rust/src/http_server/mod.rs`
- nearby mode helper tests

Acceptance:

- starting the plugin backend does not install hooks, refresh global rules,
  auto-start proxies, contact cloud/update endpoints, or publish telemetry
- search/read behavior still works
- local maintenance is removed only with direct proof that the carried tools do
  not depend on it
- side effects remain available only for non-Ontocode fork modes until a later
  deletion decision

## Phase 4: Block Unsupported Product CLI In Ontocode Runtime

Status: gated, no dispatch.

Goal: stop inherited upstream CLI commands from looking supported in the
Ontocode-maintained backend flow.

Open only if Phase 0 proves unsupported product commands are reachable from the
supported Ontocode backend operator path or are documented as supported for that
path.

Tasks:

- make help/status/provenance in Ontocode mode advertise only the supported
  backend commands
- fail closed for `setup`, `install`, `update`, `dashboard`, `proxy`, `cloud`,
  `dev-install`, `addon`, `plugin`, shell wrappers, and other unrelated product
  commands when the binary is invoked in Ontocode backend mode
- preserve non-Ontocode fork behavior until the deletion gate is reached

Expected files:

- `third_party/lean-ctx-fork/rust/src/cli/dispatch/mod.rs`
- `third_party/lean-ctx-fork/rust/src/cli/dispatch/help.rs`
- `third_party/lean-ctx-fork/rust/src/cli/dispatch/lifecycle.rs`
- `third_party/lean-ctx-fork/rust/src/cli/dispatch/network.rs`

Acceptance:

- Ontocode operator docs and CLI behavior agree
- unsupported commands say they are unsupported in Ontocode backend mode
- no hidden downloader or updater path remains reachable in normal plugin
  backend usage

## Phase 5: Feature And Dependency Slimming

Status: gated, no dispatch.

Goal: reduce the backend build/runtime surface to what the plugin actually
needs.

Open only after Phases 2-3 land and current measurements prove a build,
runtime, or review-cost problem that cannot be handled by registry/startup
gates alone.

Tasks:

- measure the current backend build/runtime cost after registry/startup gates
- define a feature profile for the Ontocode plugin backend only if measurement
  proves it is needed
- remove or make optional for that profile:
  - `team-server`
  - `secure-update`
  - proxy/cloud/dashboard-only dependencies
  - embeddings/neural dependencies unless proven required for carried
    `ctx_search`
  - `jemalloc` if not needed for the maintained backend runtime
- update repo-owned build recipe only after the feature profile is proven
- run lockfile/update checks required by the fork and by the parent repo if
  dependency metadata changes

Expected files:

- `third_party/lean-ctx-fork/rust/Cargo.toml`
- `justfile`
- build/smoke scripts only if command flags change

Acceptance:

- `just lean-ctx-plugin-backend-build` builds the slim backend path
- smoke still passes
- dependency changes are explicit and lockfiles are refreshed if needed
- no dependency churn is performed without measured value

## Phase 6: Backend Capability And Provenance Output

Status: open after Phase 2.

Goal: make mode and provenance verifiable from the backend itself.

Tasks:

- inspect the existing `.well-known/mcp-server.json`, status, or metadata
  output first
- add a backend-native health/capability response or existing-status extension
  only if current metadata cannot report:
  - mode: Ontocode plugin backend
  - allowed tools
  - fork provenance/version
  - no upstream updater/downloader support in this mode
- keep sensitive values redacted
- do not expose a new public app-server API

Expected files:

- existing HTTP server/status/capability owner found during Phase 0
- `plugins/ontocode-lean-ctx/README.md` only after backend behavior exists

Acceptance:

- operators can prove the backend is the maintained Ontocode mode
- provenance does not point users to upstream release health as the authority
- output contains no token, path, credential, or private user data leak

## Phase 7: Optional Extraction Decision

Status: gated, no dispatch.

Open only after Phases 1-6 land and current evidence shows the slim fork still
has unacceptable build/test/review cost.

Decision inputs:

- changed-file and changed-symbol blast radius from OntoIndex or direct source
  where the fork is not indexed
- build time and dependency delta after Phase 5
- remaining product code still compiled into the plugin backend
- compatibility risk of changing the runtime binary/package layout

Possible outcomes:

- extract carried backend code into a plugin-owned crate/package
- keep slim fork as-is
- write an ADR for native extension port
- write an ADR for deletion/replacement with OntoIndex/native tools

## Implementation Queue

| ID | Task | Status | Dependencies | Expected scope |
| --- | --- | --- | --- | --- |
| LCTX-REF-0 | Baseline inventory and guard tests | open, dispatch-ready | none | source review, tests only |
| LCTX-REF-1 | Native Ontocode backend mode API evidence | conditional | LCTX-REF-0 | reuse existing profile API, add helper only if proven necessary |
| LCTX-REF-2 | Ontocode-mode registry slimming | open | LCTX-REF-1 | registry and tests |
| LCTX-REF-3 | External startup side-effect gates | open | LCTX-REF-1 | network/global/user-config side effects first |
| LCTX-REF-4 | Unsupported CLI command gates | gated | LCTX-REF-0 | only if reachable from supported backend path |
| LCTX-REF-5 | Feature/dependency slimming | gated | LCTX-REF-2, LCTX-REF-3 | measurement before Cargo/build changes |
| LCTX-REF-6 | Backend capability/provenance output | open | LCTX-REF-2 | inspect existing metadata first |
| LCTX-REF-7 | Extraction/native replacement decision | gated | LCTX-REF-1 through LCTX-REF-6 | ADR or no-dispatch |

## Validation Policy

For every code slice:

- run OntoIndex routing/impact checks where the changed symbol is indexed
- record OntoIndex limitations when the fork symbol is not indexed
- inspect direct source as authority for fork internals
- run `just fmt` from the repo default workflow after Rust edits
- run focused fork tests for changed backend owners
- run `just lean-ctx-plugin-backend-smoke`
- run `git diff --check -- <changed files>`

Required focused validation candidates:

- `bash -n scripts/run_lean_ctx_plugin_backend.sh scripts/smoke_lean_ctx_plugin_backend.sh`
- `cd third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test tool_visibility`
- `cd third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test tool_profiles`
- `cd third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test ontocode_profile_blocks_noncarried_tool_dispatch`
- additional focused tests matching the edited owner
- `just lean-ctx-plugin-backend-smoke`

Do not claim a phase complete if validation is skipped unless the skip reason
is recorded with the exact blocker.

## Stop And Reopen Gates

Stop immediately if:

- a proposed change requires `ontocode-core` ownership without an ADR
- a slice needs a new public plugin API, app-server API, config key, or spawned
  runtime owner without an ADR
- the implementation would expose shell/edit/session/knowledge/cloud/proxy
  surfaces through the Ontocode plugin backend
- validation proves the current three-tool plugin contract regressed
- direct source evidence shows the fork cannot be made self-contained without a
  larger architecture decision

Reopen extraction only when:

- Phases 1-6 have landed or been explicitly rejected with evidence, and
- the remaining broad fork surface still imposes measurable build/runtime/review
  cost that cannot be removed in place.

Reopen native replacement only when:

- compatibility with lean-ctx tool names is no longer required, or
- the maintained fork cannot be made self-contained and side-effect-free.

## Manager Loop Rule

This document is a project plan, not live tracking. Before implementation,
create a tracking file or update the active tracking document with one
`active_next_task`.

For manager-loop execution:

1. If `active_next_task` exists, execute it.
2. Else promote the next dependency-ready `open` task.
3. Execute one bounded slice at a time.
4. Re-read tracking after each slice.
5. Refuse tracker rewrites that do not include new source, test, or validation
   evidence.
