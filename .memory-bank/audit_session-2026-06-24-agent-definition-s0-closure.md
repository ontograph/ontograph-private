---
name: Agent Definition S0 Closure
description: Closure note for the read-only `/agent` picker role-discovery slice
type: audit-session
date: 2026-06-24
status: accepted
---

# Agent Definition S0 Closure

Scope:
- `ontocode-rs/tui/src/app/session_lifecycle.rs`
- `ontocode-rs/tui/src/app/tests.rs`

Decision:
- close `AGDEF-S0` as implemented

What changed:
- the existing `/agent` picker now opens when there are configured `agent_roles` even if no live sub-agent threads exist yet
- the picker appends a disabled read-only "Available role definitions" section sourced from `self.config.agent_roles`
- no creation flow, hot reload, slash-dispatch rewrite, or secondary registry was added

Verification:
- passed `CARGO_BUILD_JOBS=8 just test -p ontocode-tui open_agent_picker_shows_configured_agent_roles_when_no_threads_exist`
- passed `CARGO_BUILD_JOBS=8 just test -p ontocode-tui open_agent_picker_keeps_missing_threads_for_replay`

Caveat:
- a broader `CARGO_BUILD_JOBS=8 just test -p ontocode-tui app::tests open_agent_picker` run still hit unrelated dirty-worktree snapshot/name drift outside this slice, including existing `.snap.new` churn and older `codex` vs `ontocode` expectation mismatches

Next:
- keep `AGDEF-S1` pending until a concrete repo-local role scaffold write path is justified over the now-implemented read-only discovery path
