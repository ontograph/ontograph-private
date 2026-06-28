# Lean-ctx Maintained Fork Contract

Date: 2026-06-28
Task: `L3D-F0`
Status: done

## Contract

- maintained fork local development home: `third_party/lean-ctx-fork`
- carried v1 tool allowlist: `ctx_read`, `ctx_search`, `ctx_summary`
- transport: Streamable HTTP MCP only
- local default endpoint: `http://127.0.0.1:7777`
- bearer token env var: `LEANCTX_TOKEN`
- backend started separately; plugin does not spawn it
- fail closed when backend or token is absent
- OntoIndex and native `rg` remain available independently

## Evidence

- `ontocode-rs/core-plugins/src/manager.rs`
- `ontocode-rs/core/src/mcp.rs`
- `ontocode-rs/core-plugins/src/test_support.rs`
- historical proof note `audit_session-2026-06-28-lean-ctx-translation-3d-phase1-proof-closure.md`

## Manager outcome

- `L3D-F0` closed
- `L3D-F1` is now the active next task
