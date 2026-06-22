# R5F Agent Graph Store Rename Worker Verification

Date: 2026-06-11

## Scope

- Renamed only Cargo package `codex-agent-graph-store` to `ontocode-agent-graph-store`.
- Renamed only Rust lib crate `codex_agent_graph_store` to `ontocode_agent_graph_store`.
- Preserved `ontocode-rs/agent-graph-store` directory path.

## Preserved

- Graph-store behavior.
- State/protocol semantics.
- Code-graph backbone decisions.
- Env/config/wire/generated names.
- Telemetry/product strings.
- Persisted state.

## Verification

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-agent-graph-store --no-tests=pass` passed: 4 tests.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-update` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- Active-source stale-reference search for `codex_agent_graph_store` and `codex-agent-graph-store` returned no hits.
- `git diff --check` passed.
- OntoIndex CLI `detect-changes --repo codex` ran; it reported the known broad dirty-tree high-risk context, not a scoped R5F-only verdict.
- `cargo metadata --no-deps --format-version 1` count check reported 67 remaining `codex-*` packages after this slice.

## Notes

- Remaining old-name references are historical/planning/inventory memory-bank entries, not active source references.
- OntoIndex MCP impact was unavailable for this repo because the MCP facade was bound to `OntoIndex`; CLI `impact --repo codex AgentGraphStore` was used before edits and matched the recorded LOW-risk shape.
