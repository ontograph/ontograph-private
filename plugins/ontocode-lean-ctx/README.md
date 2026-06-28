# ontocode-lean-ctx

Repo-local plugin package for the Ontocode-maintained lean-ctx fork backend.

## Contract

- backend local development home: `third_party/lean-ctx-fork`
- transport: Streamable HTTP MCP
- default endpoint: `http://127.0.0.1:7777`
- bearer token env var: `LEANCTX_TOKEN`
- backend profile: start the maintained fork with `LEAN_CTX_TOOL_PROFILE=ontocode`
- v1 tool allowlist:
  - `ctx_read`
  - `ctx_search`
  - `ctx_summary`

## Non-goals

- no `ctx_shell`
- no `ctx_edit`
- no session or knowledge tools
- no in-process backend runtime
- no plugin-owned backend process spawning

## Failure mode

The plugin is allowed to install and load, but it is fail-closed when the
required backend or auth token is missing.

OntoIndex and native `rg` remain the fallback path.

## Repo-Owned Runtime Path

From the repo root:

- `just lean-ctx-plugin-backend-build`
- `just lean-ctx-plugin-backend-start`
- `just lean-ctx-plugin-backend-status`
- `just lean-ctx-plugin-backend-stop`
- `just lean-ctx-plugin-backend-smoke`

Defaults:

- host: `127.0.0.1`
- port: `7777`
- token env var: `LEANCTX_TOKEN`
- fallback local token when unset: `ontocode-lean-ctx-dev`
- backend profile: `LEAN_CTX_TOOL_PROFILE=ontocode`

Override host or port with `LEAN_CTX_HOST` and `LEAN_CTX_PORT`.

Do not use the inherited upstream downloader or onboarding path inside
`third_party/lean-ctx-fork/`; Ontocode supports the maintained backend through
the repo-owned commands above.
