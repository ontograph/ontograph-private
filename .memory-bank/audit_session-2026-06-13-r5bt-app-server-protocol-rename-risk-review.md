# R5BT App-Server Protocol Rename Risk Review

Date: 2026-06-13

## Scope

- Rename `ontocode-app-server-protocol` to `ontocode-app-server-protocol`.
- Rename `codex_app_server_protocol` to `ontocode_app_server_protocol`.
- Keep the existing `app-server-protocol` directory path.

## Decision

- The prior R5B protocol gate is explicitly lifted by user instruction for the remaining protocol crates.
- This slice still remains protocol-sensitive and must stay package/lib/Bazel/import identity only.

## OntoIndex

- `ClientRequest`: UNKNOWN/not found.
- `ServerNotification`: UNKNOWN/not found.
- Direct inventory shows this crate re-exports v1/v2 protocol types, schema generation helpers, JSON-RPC helpers, event mapping, and thread-history builders.
- `codex-protocol` remains a live dependency during this slice and must not be renamed yet.

## Guardrails

- Do not change wire field names, serde tags, RPC method names, payload shapes, or TS export paths.
- Do not change schema generation behavior, experimental markers, fixture content, or app-server compatibility behavior.
- Do not change `codex-protocol` or `codex_protocol` in this slice.
- Do not change public API docs semantics, thread-history mapping behavior, event mapping behavior, or app-server request/notification semantics.
- Preserve generated-schema, telemetry/product strings, persisted state, and directory paths.

## Verification Required

- App-server-protocol package tests.
- App-server package compile/tests if imports changed.
- App-server test-client compile if imports changed.
- Core/app-server-client/TUI compile checks if imports changed.
- `just fmt`.
- `just bazel-lock-update` and `just bazel-lock-check`.
- Stale-reference search.
- Cargo metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
