# Lean-ctx Maintained Fork Plugin Backend Tracking

Source plan: [ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL_DETAILED_PROJECT_PLAN.md](ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL_DETAILED_PROJECT_PLAN.md)

Date opened: 2026-06-27
Status: complete

## Ledger

| ID | Task | Status | Owner | Notes |
| --- | --- | --- | --- | --- |
| L3D-H0 | Historical adapter acceptance | done | manager | Historical. The first repo-local plugin direction was accepted before the shutdown pivot. |
| L3D-H1 | Historical package proof slice | done | implementation-worker | Historical. The repo-local plugin package was previously proved installable/loadable. |
| L3D-H2 | Historical shutdown-removal pass | done | manager | Historical. The repo removed the first plugin path and lean-ctx-required guidance when shutdown-removal was the active goal. |
| L3D-F0 | Maintained fork reopen and contract | done | manager | Contract defined from current plugin/MCP seams. Local backend home: `third_party/lean-ctx-fork`. Carried v1 allowlist: `ctx_read`, `ctx_search`, `ctx_summary`. Transport: Streamable HTTP MCP at `http://127.0.0.1:7777` with `LEANCTX_TOKEN`. Backend started separately. Fail-closed when backend or token is absent; OntoIndex/native remain available. |
| L3D-F1 | Repo-local plugin package restore | done | implementation-worker | Restored `plugins/ontocode-lean-ctx/`, restored `.agents/plugins/marketplace.json`, and kept the manifest plus `.mcp.json` bounded to `ctx_read`, `ctx_search`, and `ctx_summary` over required bearer-auth Streamable HTTP MCP. |
| L3D-F2 | Install/load proof against maintained fork | done | implementation-worker | Restored focused `ontocode-core-plugins` proof coverage with `install_repo_local_lean_ctx_plugin_loads_required_bearer_http_server`. Validation: `CARGO_BUILD_JOBS=8 just test -p ontocode-core-plugins` passed after `just fmt`. |
| L3D-F3 | Docs and usage guidance | done | implementation-worker | Restored selective maintained-fork guidance in `LEAN-CTX.md`, `GEMINI.md`, and `.memory-bank/reference_agent-rules.md` while keeping OntoIndex/native tooling as the baseline outside the bounded read-only path. |
| L3D-F4 | In-repo backend move and link rewrite | done | implementation-worker | Imported the maintained backend snapshot into `third_party/lean-ctx-fork/` without VCS metadata and rewrote current maintained-fork guidance to the in-repo companion subtree. |
| L3D-F5 | Repo-owned backend launcher and smoke proof | done | implementation-worker | Added repo-owned start/status/stop/smoke paths through `just` plus `scripts/run_lean_ctx_plugin_backend.sh` and `scripts/smoke_lean_ctx_plugin_backend.sh`. The smoke check starts the in-repo release binary with `LEAN_CTX_TOOL_PROFILE=ontocode`, verifies HTTP readiness, and asserts the live `tools/list` surface is exactly `ctx_read`, `ctx_search`, and `ctx_summary`. |
| L3D-F6 | Disable inherited external runtime bootstrap | done | implementation-worker | Fail-closed the adopted fork's external download installer path and marked the in-repo `just lean-ctx-plugin-backend-*` runtime flow as the only supported Ontocode operator path. |

## Manager State

- Last decision: `complete-no-open-task`
- Active next task: `none`
- Acceptance basis: upstream shutdown does not forbid the plugin boundary when Ontocode explicitly owns the backend fork.

## Current State

- The shutdown-removal pass is preserved as historical evidence, not erased.
- The repo-local plugin package and marketplace entry exist again.
- The maintained-fork contract is explicit and proved through existing plugin owners.
- The maintained backend source now lives in `third_party/lean-ctx-fork/`.
- The repo now has an owned startup path for that backend: build/start/status/stop/smoke from the repo root without any upstream checkout.
- The adopted fork no longer exposes the inherited external download bootstrap as a supported runtime path.
- Current guidance restores only the bounded read-only path; OntoIndex/native flows remain the baseline elsewhere.
- The broad `cargo test tool_profiles` host hang remains non-blocking and deferred; the exact relevant profile proof already passed earlier, so this slice did not reopen that investigation.

## Guardrails

- Do not depend on upstream lean-ctx remaining healthy.
- Do not port the backend into `ontocode-core`.
- Do not reopen the full historical `ctx_*` surface without a bounded list.
- Keep OntoIndex/native flows available as fallback and comparison paths.
