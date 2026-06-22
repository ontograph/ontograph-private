# R5F Agent Graph Store Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-agent-graph-store` -> `ontocode-agent-graph-store`.
- Accepted `codex_agent_graph_store` -> `ontocode_agent_graph_store`.
- Preserved `ontocode-rs/agent-graph-store` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-agent-graph-store --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_agent_graph_store` and `codex-agent-graph-store`.
- Positive identity search for `ontocode_agent_graph_store` and `ontocode-agent-graph-store`.
- `git diff --check`
- OntoIndex CLI fallback: `detect-changes --repo codex`.

## Decision

R5F is accepted. Active old package/lib references are clean. Cargo metadata reports 67 remaining `codex-*` packages, so residual package stages continue and R6 cleanup remains blocked.
