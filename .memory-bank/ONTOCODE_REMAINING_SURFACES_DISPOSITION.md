# Ontocode Remaining `codex` Surface Disposition

Source policy: `ONTOCODE_RENAME_PROJECT_PLAN.md`

This document closes the manager review pass for remaining `codex`-named surfaces that were not safe targets for the SDK public-surface sweeps.

## Decision Summary

- Rename now:
  - public SDK examples, docs, tests, and non-generated source that can point at existing `Ontocode*` primary exports
- Preserve with compatibility:
  - `Codex*` compatibility aliases in Python and TypeScript SDKs
  - `CODEX_*` environment variables where `ONTOCODE_*` aliases now exist or are planned
  - local integration contracts such as `.codex-plugin`
- Preserve or version:
  - generated protocol model names
  - package identities
  - wire identifiers
  - telemetry schemas
- Defer:
  - Rust internal crate and helper renames
  - low-value internal type renames that do not improve the public migration

## Classified Remaining Surfaces

### 1. Generated Python Protocol Models

Examples:

- `sdk/python/src/openai_codex/generated/v2_all.py`
- `CodexAppServerProtocolV2`
- `CodexErrorInfo`
- `codexErrorInfo`

Decision:

- preserve for now

Why:

- these names are generated from protocol/schema sources
- renaming them locally would either fork generated artifacts or force a broader schema/versioning change
- the value is mostly cosmetic compared to the compatibility cost

Exit rule:

- only revisit during an explicit protocol/schema migration

### 2. Published Package and Runtime Identities

Examples:

- `openai-codex`
- `openai-codex-cli-bin`
- `@openai/codex-sdk`
- `@openai/codex-linux-x64`
- `codex_cli_bin`

Decision:

- preserve until a versioned package migration is approved

Why:

- these names are installer and release-tooling contracts
- changing them without dual-publish or alias strategy would break upgrades and bootstrap flows

Exit rule:

- change only as part of the package migration program already tracked in `ONTOCODE_PACKAGE_IDENTITY_MIGRATION.md`

### 3. Rust Internal Crates, Helpers, and Broad Internal Types

Examples:

- workspace crates prefixed `codex-*`
- helper executables such as `codex-linux-sandbox`
- internal Rust types such as `CodexAuth`, `CodexErr`, `CodexThread`, analytics event structs

Decision:

- defer

Why:

- these are high-churn, low-user-value surfaces
- broad internal renames would create large diffs without improving the external migration materially
- the project plan already treats this as optional Stage 6 work

Exit rule:

- only revisit after external migration stabilizes and only in reviewable subsystem slices

### 4. Telemetry and Analytics Event Shapes

Examples:

- `ontocode-rs/analytics/src/events.rs`
- `CodexTurnEventRequest`
- `CodexCommandExecutionEventRequest`
- `CodexRuntimeMetadata`

Decision:

- preserve

Why:

- these shapes feed analytics and observability pipelines
- renaming them has little user-facing value and risks downstream data churn
- GitNexus impact on `CodexTurnEventRequest` was low, but the rename value is still weaker than the analytics compatibility cost

Exit rule:

- only revisit if telemetry schemas are versioned independently

### 5. Wire and Integration Identifiers

Examples:

- protobuf package names
- MCP/resource identifiers
- memo/resource URIs
- repo/tooling identifiers keyed as `codex`

Decision:

- preserve or version, depending on surface

Why:

- these identifiers cross process or tool boundaries
- silent renames would break clients and stored references

Exit rule:

- only change behind explicit aliasing or versioned migration

## Closed Scope

The public-surface rename work is complete.

- completed:
  - Python SDK public-surface cleanup
  - TypeScript SDK public-surface cleanup
- intentionally not part of the completed rename project:
  - generated model renames
  - package/runtime identity renames
  - wire/protocol identifier renames
  - internal Rust crate/helper/type renames

All other currently visible `codex` surfaces are intentionally preserved or deferred by policy rather than accidentally skipped.
