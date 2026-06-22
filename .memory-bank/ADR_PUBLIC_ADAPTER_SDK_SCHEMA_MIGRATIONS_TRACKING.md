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
| A1 | Concrete config schema proposal | TOML shape, trust/provenance fields, bounds, disabled state, no-secret constraints | done | Accepted in ADR Stage 0 |
| A2 | Owner-surface map | Map proposed schema to `ConfigToml`, app-server v2, Python SDK, TypeScript SDK, and adapter-protocol fixtures | done | Accepted owner map and generation commands recorded in ADR |
| A3 | Compatibility test plan | Define exact tests and generated-schema commands required before implementation | done | Accepted in ADR with config/app-server/SDK gating coverage |
| A4 | Conformance fixture expansion plan | Define fixture names and validation behavior before adding protocol fixtures | done | Accepted as inert transcript expansion track with required cases |
| A5 | Implementation readiness decision | Decide whether public config/API implementation may start | done | Stage 0 planning accepted; runtime/public rollout remains gated by follow-on tracks |

## Active Task: Follow-On Track Dispatch Preparation

- Started: 2026-06-13
- GitNexus evidence:
  - `ontocode-rs/config/src/config_toml.rs::ConfigToml`
  - `ontocode-rs/config/src/schema.rs::write_config_schema`
  - `ontocode-rs/app-server-protocol/src/schema_fixtures.rs::write_schema_fixtures`
  - `sdk/python/scripts/update_sdk_artifacts.py::generate_schema_from_pinned_runtime`
  - `sdk/python/scripts/update_sdk_artifacts.py::generate_types_from_schema_dir`
  - `ontocode-rs/adapter-protocol`
- Current next action: derive implementation tasks P1-P4 from the accepted ADR once the protocol-stage rename tree is stable enough for safe code dispatch.

## Log

- 2026-06-08: Created tracker and started A1/A2 with GitNexus evidence from config schema, app-server schema, SDK artifact generation, and adapter protocol owners.
- 2026-06-13: Accepted Stage 0 schema proposal, owner map, compatibility test plan, conformance expansion plan, and readiness decision; follow-on work is now P1-P4 implementation planning.
