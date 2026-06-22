# R5AA Backend OpenAPI Models Rename Risk Review

Date: 2026-06-11

## Decision

- Dispatch `codex-backend-openapi-models` -> `ontocode-backend-openapi-models`.
- Dispatch `codex_backend_openapi_models` -> `ontocode_backend_openapi_models`.
- Scope is identity-only package/lib/Bazel/import rename.

## Direct Inventory

- Cargo metadata direct reverse dependency: `codex-backend-client`.
- Active direct refs before dispatch: 23.
- Refs are confined to root workspace membership, backend-client dependency/import/re-export usage, and generated-model crate manifest/Bazel identity.

## OntoIndex Impact

- MCP impact is miswired to repo `OntoIndex`; CLI fallback was used.
- `Struct:ontocode-rs/codex-backend-openapi-models/src/models/config_bundle_response.rs:ConfigBundleResponse`: LOW, 2 impacted nodes, 2 direct, 0 affected processes.
- `Struct:ontocode-rs/codex-backend-openapi-models/src/models/rate_limit_status_payload.rs:RateLimitStatusPayload`: MEDIUM, 5 impacted nodes, 5 direct, 0 affected processes.
- `Struct:ontocode-rs/codex-backend-openapi-models/src/models/task_list_item.rs:TaskListItem`: LOW, 1 impacted node, 1 direct, 0 affected processes.
- `Module:ontocode-rs/codex-backend-openapi-models/src/lib.rs:models`: LOW, 0 impacted nodes, 0 direct, 0 affected processes.

## Guardrails

- Preserve generated OpenAPI model field names, serde attributes, constructors, and re-export semantics.
- Preserve backend-client conversion and rate-limit behavior.
- Preserve config bundle and task list behavior.
- Preserve generated source contents.
- Preserve telemetry, env/config/wire/generated names, persisted state, and the existing `codex-backend-openapi-models` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-backend-openapi-models --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-backend-client --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_backend_openapi_models|codex-backend-openapi-models`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Model

- Use `gpt-5.4-mini` high because `gpt-5.3-codex-spark` reached usage limit.
