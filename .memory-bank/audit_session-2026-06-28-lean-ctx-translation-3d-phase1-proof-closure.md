# Lean-ctx Translation 3D Phase 1 Proof Closure

Date: 2026-06-28
Scope: `L3D-P1` in `ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL_DETAILED_PROJECT_PLAN.md`

## Decision

Closed `L3D-P1` after adding the smallest repo-local proof that the package
works through existing owners.

## Evidence

- `plugins/ontocode-lean-ctx/.mcp.json` now marks the lean-ctx MCP server
  `required = true` while keeping bearer-token env auth and the read-only tool
  allowlist.
- `ontocode-rs/core-plugins/src/manager_tests.rs` now includes
  `install_repo_local_lean_ctx_plugin_loads_required_bearer_http_server`,
  proving:
  - install through `PluginsManager.install_plugin`
  - config entry creation with the plugin key
  - load through `PluginsManager.plugins_for_config`
  - required Streamable HTTP MCP config with `LEANCTX_TOKEN`

## Validation

- `python3 .../validate_plugin.py plugins/ontocode-lean-ctx`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core-plugins`

## Manager Outcome

- `L3D-P1` done.
- No active follow-up task remains in this ADR.
- Reopen only if the existing HTTP MCP plugin path cannot enforce a newly
  required lean-ctx compatibility gate cleanly, or failing runtime evidence
  shows install/load or required-startup failure behavior is insufficient.
