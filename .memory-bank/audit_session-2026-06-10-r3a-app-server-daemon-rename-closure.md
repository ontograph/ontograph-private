---
name: R3A App Server Daemon Rename Closure
description: Manager acceptance record for the app-server-daemon identity-only crate rename
type: audit_session
date: 2026-06-10
status: done
---

# R3A App Server Daemon Rename Closure

## Scope Accepted

- `ontocode-app-server-daemon` renamed to `ontocode-app-server-daemon`.
- Library crate identity renamed to `ontocode_app_server_daemon`.
- Direct CLI dependency/import references updated for the daemon crate.
- App-server wire/protocol names, CLI command names, runtime socket behavior, update-loop behavior, remote-control behavior, managed install behavior, env/config semantics, telemetry, and persisted state were preserved.

## Verification

- Worker: `CARGO_BUILD_JOBS=8 just fmt` passed.
- Worker and manager: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-daemon` passed.
- Worker and manager: `CARGO_BUILD_JOBS=8 just test -p codex-cli remote_control_cmd` passed.
- Worker and manager: `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server` passed.
- Worker: `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed with no final `MODULE.bazel.lock` diff for this slice.
- Worker and manager: `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- Worker and manager: `git diff --check` passed.
- Manager metadata check confirmed package/lib identity as `ontocode-app-server-daemon` / `ontocode_app_server_daemon`.
- Manager stale-reference search found old daemon crate names only in intentional user-agent compatibility strings.
- OntoIndex `gn_verify_diff` passed for the scoped R3A file set and required tests.

## Notes

- Whole worktree remains dirty from prior accepted rename and hard-cutover slices; R3A acceptance is scoped to the daemon file set.
- Remaining R3 crates still require fresh senior review before dispatch.
