# lean-ctx — Maintained Fork Plugin Guidance

This file documents the bounded maintained-fork lean-ctx plugin path.

## Use it for

- `ctx_read`
- `ctx_search`
- `ctx_summary`

Use this plugin only when all of these are true:

- the repo-local `ontocode-lean-ctx` plugin is installed
- the maintained backend from `third_party/lean-ctx-fork` is running
- `LEANCTX_TOKEN` is set
- bounded read-only retrieval is the goal

Repo-owned startup path from the repo root:

- `just lean-ctx-plugin-backend-start`
- `just lean-ctx-plugin-backend-smoke`

Do not use the inherited upstream downloader or onboarding path inside
`third_party/lean-ctx-fork/`; the supported Ontocode path is the repo-owned
runtime flow above.

## Do not use it for

- shell execution
- editing
- session memory
- knowledge storage
- code intelligence that OntoIndex already covers better

## Baseline

- OntoIndex remains the default for code intelligence and impact analysis
- native `rg` remains the default exact-text search fallback
- native shell remains the default command path

## Failure mode

If the backend or token is missing, the plugin is fail-closed.

Do not treat that state as implicit permission to widen the plugin surface.
