# R3D App Server Client Rename Risk Review

Date: 2026-06-10

## Candidate

- `ontocode-app-server-client` -> `ontocode-app-server-client`.
- Scope approved only for package/lib/Bazel/import identity changes.

## OntoIndex Evidence

- `InProcessAppServerClient` struct impact: LOW, 1 upstream impacted test node.
- `AppServerClient` enum impact: LOW, 0 upstream impacted nodes.
- `InProcessAppServerClient` impl impact: LOW, 0 upstream impacted nodes.
- `AppServerClient` impl impact: LOW, 0 upstream impacted nodes.
- OntoIndex repo path: `/opt/demodb/_workfolder/ontocode`.

## Direct Inventory

- Active package references are in workspace metadata, the app-server-client crate, app-server comments/docs, `ontocode-exec`, and `ontocode-tui`.
- Direct dependent test coverage should include `ontocode-app-server-client`, `ontocode-exec`, and `ontocode-tui`.

## Guardrails

- Do not change app-server wire protocol, request/notification routing, server-request resolution, websocket or unix-socket behavior, in-process lifecycle, runtime start args, auth header transport policy, CLI/TUI/exec behavior, telemetry semantics, env/config semantics, or persisted state.
- Do not rename test client names, app-server comments, or compatibility strings unless a failing package-identity check proves the rename is required.

## Decision

- Approved as R3D, one slice only.
