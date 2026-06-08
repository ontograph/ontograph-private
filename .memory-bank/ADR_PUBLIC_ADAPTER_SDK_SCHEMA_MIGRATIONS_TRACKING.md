# Public Adapter SDK And Schema Migrations Tracking

Source ADR: `ADR_PUBLIC_ADAPTER_SDK_SCHEMA_MIGRATIONS.md`

## Status Key

- `pending`
- `in_progress`
- `blocked`
- `done`
- `deferred`

## Task Queue

| ID | Task | Scope | Status | Notes |
| --- | --- | --- | --- | --- |
| A1 | Concrete config schema proposal | TOML shape, trust/provenance fields, bounds, disabled state, no-secret constraints | in_progress | Drafted in ADR Stage 0; no code/schema keys added yet |
| A2 | Owner-surface map | Map proposed schema to `ConfigToml`, app-server v2, Python SDK, TypeScript SDK, and adapter-protocol fixtures | in_progress | GitNexus query identified owner surfaces and generation commands |
| A3 | Compatibility test plan | Define exact tests and generated-schema commands required before implementation | pending | Must cover unknown/disabled adapter configs and old configs without adapter keys |
| A4 | Conformance fixture expansion plan | Define fixture names and validation behavior before adding protocol fixtures | pending | Must include cancellation, errors, oversize frame, and version mismatch |
| A5 | Implementation readiness decision | Decide whether public config/API implementation may start | pending | Blocked until A1-A4 are reviewed and accepted |

## Active Task: A1/A2 Schema Proposal And Surface Map

- Started: 2026-06-08
- GitNexus evidence:
  - `codex-rs/config/src/config_toml.rs::ConfigToml`
  - `codex-rs/config/src/schema.rs::write_config_schema`
  - `codex-rs/app-server-protocol/src/schema_fixtures.rs::write_schema_fixtures`
  - `sdk/python/scripts/update_sdk_artifacts.py::generate_schema_from_pinned_runtime`
  - `sdk/python/scripts/update_sdk_artifacts.py::generate_types_from_schema_dir`
  - `codex-rs/adapter-protocol`
- Current next action: review the Stage 0 schema proposal in the ADR and either accept it for implementation planning or revise it.

## Log

- 2026-06-08: Created tracker and started A1/A2 with GitNexus evidence from config schema, app-server schema, SDK artifact generation, and adapter protocol owners.
