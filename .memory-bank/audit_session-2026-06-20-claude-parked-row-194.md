# Claude Parked Row 194 Review

Date: 2026-06-20

## Decision

Row 194 stays parked.

## Source

- ADR row 194: `Partial | Conditional | NARROW | Plugin cache validation can extend existing cache path.`
- Donor row 194: `Add services-layer test script. | app-server/core services | Catches service wiring drift. | Service smoke.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-rs/core-plugins/src/store_tests.rs` already covers requested plugin cache version installation behavior.
- `ontocode-rs/core-plugins/src/manager_tests.rs` already covers curated plugin cache refresh, missing configured plugin reinstall, no-op current-cache behavior, and full-SHA to short-SHA cache migration.
- `ontocode-rs/app-server/tests/suite/v2/plugin_install.rs` already verifies remote plugin install writes the plugin to cloud and cache.
- `ontocode-rs/app-server/tests/suite/v2/plugin_list.rs` already verifies installed git-source plugin metadata is read from cache.
- Existing app-server service tests cover config/service request processor behavior; no one concrete services-layer smoke gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
