# RTK Donor Native Lean-ctx Backend Project Plan

Status: bounded implementation complete, no further dispatch
Date: 2026-06-28
Source review: [RTK_DONOR_2000_USEFUL_TOOLS_FOR_LEAN_CTX_PLUGIN.md](RTK_DONOR_2000_USEFUL_TOOLS_FOR_LEAN_CTX_PLUGIN.md)

## Goal

Implement only the native backend work that survives the RTK donor challenge.

The target is the Ontocode-maintained lean-ctx backend in
`third_party/lean-ctx-fork`, exposed through the existing
`plugins/ontocode-lean-ctx` package boundary. This plan must not create plugin
preflight scripts, docs validators, shell hooks, telemetry, TOML filter
runtimes, or wrapper tools.

## Current Decision

`RTK-LCTX-E0` is complete.

Direct source review proved one native gap and only one native gap worth
implementing in this pass:

- `RTK-LCTX-N4` was real: the maintained backend had no backend-native
  `ontocode` profile that exposed only `ctx_read`, `ctx_search`, and
  `ctx_summary`, and the default invoker path could still advertise or reach
  broader tools.

This pass closes `RTK-LCTX-N4` and leaves the rest no-dispatch until new
source or test evidence appears.

The plugin package remains a client-side allowlist only. Backend-native
enforcement applies when the maintained fork is started with
`LEAN_CTX_TOOL_PROFILE=ontocode`; current streamable HTTP MCP config does not
inject backend env for this plugin.

## Binding Boundary

Allowed owners:

- `third_party/lean-ctx-fork/rust/src/core/tool_profiles.rs`
- `third_party/lean-ctx-fork/rust/src/server/call_tool.rs`
- `third_party/lean-ctx-fork/rust/src/server/server_handler.rs`
- `third_party/lean-ctx-fork/rust/src/server/tool_visibility.rs`
- `third_party/lean-ctx-fork/rust/src/dashboard/routes/settings.rs`
- `third_party/lean-ctx-fork/rust/src/cli/profile_cmd.rs`
- `third_party/lean-ctx-fork/rust/src/setup/mod.rs`
- backend-local tests beside those owners

Evidence-only reviewed owners:

- `third_party/lean-ctx-fork/rust/src/tools/ctx_read/`
- `third_party/lean-ctx-fork/rust/src/tools/ctx_search.rs`
- `third_party/lean-ctx-fork/rust/src/tools/ctx_summary.rs`

Allowed carried surface:

- `ctx_read`
- `ctx_search`
- `ctx_summary`
- backend-native allowlist/capability/provenance behavior needed for those
  tools

Rejected for this plan:

- plugin `scripts/check.sh`
- plugin README automation
- package validator wrappers
- RTK shell command rewrites
- `ctx_shell`, `ctx_edit`, session tools, knowledge tools
- RTK telemetry, savings dashboards, analytics DBs
- TOML filter DSL or command proxy runtime
- hidden downloads or dependence on upstream lean-ctx availability

## OntoIndex Baseline

`gn_ensure_fresh(repo=codex)` was run on 2026-06-28:

- indexed HEAD: `5edde24a78efe0f10bc710936dfa228427ab7fd1`
- current HEAD: `5edde24a78efe0f10bc710936dfa228427ab7fd1`
- index state: fresh
- limitations: dirty worktree and missing embeddings

Use OntoIndex for routing and impact checks, but use direct backend source and
tests as authority.

## Dispatch Result

1. `RTK-LCTX-E0`: complete.
2. `RTK-LCTX-N4`: complete.
3. `RTK-LCTX-N1`, `RTK-LCTX-N2`, `RTK-LCTX-N3`, `RTK-LCTX-N5`: no-dispatch.

## Task Results

### `RTK-LCTX-E0` native backend gap audit

Status: complete.

Goal: inspect current backend source and tests for the nine native candidates
from the RTK review, then decide whether any implementation task is justified.

Required source inspection:

- `third_party/lean-ctx-fork/rust/src/tools/ctx_read/`
- `third_party/lean-ctx-fork/rust/src/tools/ctx_search.rs`
- `third_party/lean-ctx-fork/rust/src/tools/ctx_summary.rs`
- `third_party/lean-ctx-fork/rust/src/server/tool_visibility.rs`
- related backend-local tests

Required OntoIndex checks:

- `search({action: "semantic", repo: "codex", query: "lean-ctx ctx_read ctx_search ctx_summary tool visibility"})`
- `inspect({action: "context", repo: "codex", target: "<chosen backend symbol>"})` for any symbol proposed for edit
- `impact({action: "symbol", repo: "codex", target: "<chosen backend symbol>", direction: "upstream"})` before any code edit

Evidence result:

- `RTK-LCTX-N1`: covered. `ctx_read` already has bounded read-mode and
  anti-inflation tests in `third_party/lean-ctx-fork/rust/src/tools/ctx_read/`.
- `RTK-LCTX-N2`: covered. `ctx_search` already has size, deadline, and
  partial-result bounds in `third_party/lean-ctx-fork/rust/src/tools/ctx_search.rs`.
- `RTK-LCTX-N3`: no gap proven. `ctx_summary` clamp behavior exists; no direct
  failing bound was proven in this pass.
- `RTK-LCTX-N4`: gap proven. Backend-native profile/visibility enforcement was
  missing in `third_party/lean-ctx-fork/rust/src/core/tool_profiles.rs`,
  `third_party/lean-ctx-fork/rust/src/server/tool_visibility.rs`, and dispatch.
- `RTK-LCTX-N5`: no gap proven. No misleading backend provenance or numeric
  claim surface was proven in current source.

OntoIndex note:

- `gn_ensure_fresh(repo=codex)` was fresh at the current HEAD, but
  `inspect/impact` could not resolve imported lean-ctx fork symbols such as
  `ToolProfile`, `needs_invoker`, or `call_tool_guarded`. Direct source was the
  authority for this pass.

### `RTK-LCTX-N1` bounded read proof or fix

Status: closed no-dispatch.

Goal: if `E0` proves a real `ctx_read` gap, add the smallest backend-local test
or fix for bounded output, anti-inflation, line/window limits, or read-mode
behavior.

Expected files:

- `third_party/lean-ctx-fork/rust/src/tools/ctx_read/*`

Acceptance:

- preserves existing `ctx_read` modes
- does not add a plugin wrapper or new tool
- proves the gap with a focused backend-local test

Validation:

- `cd /opt/demodb/_workfolder/ontocode/third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test ctx_read`

### `RTK-LCTX-N2` bounded search proof or fix

Status: closed no-dispatch.

Goal: if `E0` proves a real `ctx_search` gap, add the smallest backend-local
test or fix for grouped output, truncation, partial-result behavior, or search
deadline handling.

Expected files:

- `third_party/lean-ctx-fork/rust/src/tools/ctx_search.rs`
- related backend-local tests

Acceptance:

- keeps search read-only and bounded
- does not add command proxy or RTK filter runtime
- failure and partial-result output remains actionable

Validation:

- `cd /opt/demodb/_workfolder/ontocode/third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test ctx_search`

### `RTK-LCTX-N3` bounded summary proof or fix

Status: closed no-dispatch.

Goal: if `E0` proves a real `ctx_summary` gap, add the smallest backend-local
test or fix for recall/list bounds or honest empty/error output.

Expected files:

- `third_party/lean-ctx-fork/rust/src/tools/ctx_summary.rs`
- `third_party/lean-ctx-fork/rust/src/core/session_summary/*` only if the gap
  is owned below the tool wrapper

Acceptance:

- output remains bounded
- no session or knowledge tools are exposed through the plugin allowlist
- no telemetry or analytics DB is introduced

Validation:

- `cd /opt/demodb/_workfolder/ontocode/third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test summary`

### `RTK-LCTX-N4` backend allowlist or capability proof

Status: complete.

Goal: if `E0` proves the Ontocode backend mode can advertise or enable more
than the carried v1 surface, add the smallest backend-native allowlist,
visibility, or capability fix.

Expected files:

- `third_party/lean-ctx-fork/rust/src/server/tool_visibility.rs`
- backend registry/server files only if source evidence points there

Implemented files:

- `third_party/lean-ctx-fork/rust/src/core/tool_profiles.rs`
- `third_party/lean-ctx-fork/rust/src/server/tool_visibility.rs`
- `third_party/lean-ctx-fork/rust/src/server/server_handler.rs`
- `third_party/lean-ctx-fork/rust/src/server/call_tool.rs`
- `third_party/lean-ctx-fork/rust/src/dashboard/routes/settings.rs`
- `third_party/lean-ctx-fork/rust/src/cli/profile_cmd.rs`

Acceptance:

- Ontocode backend mode reports or exposes only `ctx_read`, `ctx_search`, and
  `ctx_summary`
- `ctx_shell`, `ctx_edit`, session tools, and knowledge tools stay excluded
- plugin config does not become the only enforcement point
- the package allowlist and backend profile stay aligned, but the package does
  not itself activate `LEAN_CTX_TOOL_PROFILE=ontocode`

Validation:

- `cd /opt/demodb/_workfolder/ontocode/third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test tool_visibility`
- `cd /opt/demodb/_workfolder/ontocode/third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test tool_profiles`
- `cd /opt/demodb/_workfolder/ontocode/third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test ontocode_profile_blocks_noncarried_tool_dispatch`

Implementation result:

- added `ToolProfile::Ontocode`
- made `needs_invoker(...)` refuse `ctx_call` auto-advertising for that profile
- added `profile_allows_dispatch(...)` and enforced it in `call_tool_guarded`
- updated dashboard and CLI profile surfaces so the new profile is selectable
  and correctly named
- clarified setup output so it advertises `lean-ctx tools ontocode`

### `RTK-LCTX-N5` provenance and claim guard proof

Status: closed no-dispatch.

Goal: if `E0` proves backend metadata or savings claims are misleading for the
Ontocode-maintained fork, add the smallest backend-local correction.

Expected files:

- backend version/provenance owner found by `E0`
- backend read/search docs or tests only if they contain the unproven claim

Acceptance:

- no update check or external release dependency is added
- no numeric token-savings claim is exposed without fixture/test evidence
- local fork provenance is visible enough for operators to know this is the
  Ontocode-maintained backend

Validation:

- command selected by `E0` for the actual owner
- file-scoped `git diff --check`

## Stop Condition Reached

No further dependency-ready task remains from this plan.

The remaining RTK-native candidates are either already covered or were not
proven by direct source and test evidence.

## Manager Loop Rule

When continuing this plan:

1. Reopen only with new source or failing-test evidence against
   `ctx_read`, `ctx_search`, `ctx_summary`, or the backend-native allowlist.
2. Do not reopen for plugin-wrapper, telemetry, shell, filter-runtime, or docs
   automation ideas.
