# R3E App Server Rename Closure

Date: 2026-06-10

## Scope

- Accepted `ontocode-app-server` -> `ontocode-app-server`.
- Scope was package/lib/Bazel/import identity only.
- Preserved `ontocode-app-server` and `ontocode-app-server-test-notify-capture` binary names.
- Preserved JWT audience strings, app-server wire/protocol methods, socket/runtime behavior, remote-control behavior, CLI/TUI behavior, telemetry semantics, env/config semantics, and persisted state.

## Verification

- Worker verification passed `just fmt`, full `ontocode-app-server` 810/810 with 1 skipped, `codex-cli app_server`, `codex-cli remote_control_cmd`, `ontocode-tui --no-tests=pass`, Bazel lock update/check, stale-reference search, `git diff --check`, and scoped OntoIndex `gn_verify_diff`.
- Manager confirmed manifest/Bazel identity exposes `ontocode-app-server` / `ontocode_app_server`.
- Manager focused rerun `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server turn_start_emits_thread_scoped_warning_notification_for_trimmed_skills` passed.
- Manager focused rerun `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server` passed 25/25.
- Manager reran `CARGO_BUILD_JOBS=8 just bazel-lock-check`; passed.
- Manager reran `git diff --check`; passed.
- Manager scoped OntoIndex `gn_verify_diff`; passed.

## Decision

- R3E is accepted.
- Remaining R3 crates require fresh senior review before dispatch.
