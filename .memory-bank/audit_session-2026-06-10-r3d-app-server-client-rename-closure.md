# R3D App Server Client Rename Closure

Date: 2026-06-10

## Scope

- Accepted `ontocode-app-server-client` -> `ontocode-app-server-client`.
- Scope was package/lib/Bazel/import identity only.
- In-process client behavior, remote websocket/unix-socket behavior, request/notification routing, server-request resolution, auth header policy, runtime start args, TUI/exec behavior, telemetry semantics, env/config semantics, and persisted state were preserved.

## Verification

- Worker verification passed `just fmt`, `ontocode-app-server-client`, current `ontocode-exec --no-tests=pass`, `ontocode-tui --no-tests=pass`, Bazel lock update/check, stale-reference search, `git diff --check`, and scoped OntoIndex `gn_verify_diff`.
- Manager confirmed manifest/Bazel identity exposes `ontocode-app-server-client` / `ontocode_app_server_client`.
- Manager stale-reference search found only intentional old-name README/comment/test-client strings.
- Manager reran `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-client`; passed 26/26.
- Manager reran `CARGO_BUILD_JOBS=8 just test -p ontocode-exec --no-tests=pass`; passed 122/122.
- Manager reran `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`; passed 2772/2772, 4 skipped.
- Manager reran `CARGO_BUILD_JOBS=8 just bazel-lock-check`; passed.
- Manager scoped OntoIndex `gn_verify_diff`; passed.

## Decision

- R3D is accepted.
- The next R3 task requires fresh senior review. The known broad `ontocode-app-server` skills warning-count fixture drift should be unblocked before an app-server package rename is dispatched.
