# R3C App Server Transport Rename Closure

Date: 2026-06-10

## Scope

- Accepted `ontocode-app-server-transport` -> `ontocode-app-server-transport`.
- Scope was package/lib/Bazel/import identity only.
- Runtime transport behavior, remote-control behavior, auth policy behavior, socket paths, CLI command behavior, telemetry semantics, env/config semantics, and persisted state were preserved.

## Verification

- Worker verification covered `ontocode-app-server-transport`, `ontocode-app-server-client`, `ontocode-app-server-daemon`, focused `codex-cli` app-server and remote-control tests, `ontocode-tui`, `ontocode-exec --no-tests=pass`, Bazel lock update/check, stale-reference search, `git diff --check`, and scoped OntoIndex `gn_verify_diff`.
- Manager reran `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport remote_control_waits_for_account_id_before_enrolling`; passed.
- Manager reran `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport`; passed 105/105.
- Manager reran broad `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`; app-server transport and remote-control coverage passed, but the suite failed one unrelated skills warning-count fixture that expects 7 omitted skills while current runtime reports 14.

## Decision

- R3C is accepted.
- Do not dispatch another R3 slice without a fresh senior risk review and tracking update.
