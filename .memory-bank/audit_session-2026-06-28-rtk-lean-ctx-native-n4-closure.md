# RTK Lean-ctx Native N4 Closure

Date: 2026-06-28
Plan: `RTK_DONOR_2000_USEFUL_TOOLS_FOR_LEAN_CTX_PLUGIN_PROJECT_PLAN.md`

## Decision

Close `RTK-LCTX-E0` and `RTK-LCTX-N4`.

Do not open `RTK-LCTX-N1`, `RTK-LCTX-N2`, `RTK-LCTX-N3`, or `RTK-LCTX-N5`
without new source or failing-test evidence.

## Evidence

- Direct source review showed `ctx_read` and `ctx_search` are already bounded.
- No direct failing summary bound or provenance-claim defect was proven.
- The maintained fork lacked a backend-native profile that both advertised and
  dispatched only the carried Ontocode read-only surface.
- OntoIndex was fresh at HEAD, but imported lean-ctx fork symbols were not
  resolvable through `inspect` or `impact`, so direct source was the authority.

## Landed Change

- Added `ToolProfile::Ontocode`.
- Prevented `ctx_call` auto-advertising in that profile.
- Blocked direct dispatch outside `ctx_read`, `ctx_search`, and `ctx_summary`
  when that profile is active.
- Updated CLI and dashboard profile surfaces to recognize `ontocode`.
- Kept the plugin package as a client-side allowlist; backend-native
  enforcement requires starting the maintained fork with
  `LEAN_CTX_TOOL_PROFILE=ontocode`.

## Validation

- `CARGO_BUILD_JOBS=8 cargo fmt --manifest-path third_party/lean-ctx-fork/rust/Cargo.toml`
- `cd third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test tool_visibility`
- `cd third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test tool_profiles`
- `cd third_party/lean-ctx-fork/rust && CARGO_BUILD_JOBS=8 cargo test ontocode_profile_blocks_noncarried_tool_dispatch`
- `git diff --check -- third_party/lean-ctx-fork/rust/src/core/tool_profiles.rs third_party/lean-ctx-fork/rust/src/server/tool_visibility.rs third_party/lean-ctx-fork/rust/src/server/server_handler.rs third_party/lean-ctx-fork/rust/src/server/call_tool.rs third_party/lean-ctx-fork/rust/src/dashboard/routes/settings.rs third_party/lean-ctx-fork/rust/src/cli/profile_cmd.rs`

## Reopen Gate

Reopen only with new direct source or failing-test evidence against the carried
read-only tools or the backend-native `ontocode` profile enforcement.
