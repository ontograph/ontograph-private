# R5AC Experimental API Macros Rename Risk Review

Date: 2026-06-11

## Candidate

- `codex-experimental-api-macros` -> `ontocode-experimental-api-macros`
- `codex_experimental_api_macros` -> `ontocode_experimental_api_macros`

## Current Inventory

- Cargo metadata direct reverse dependency: `ontocode-app-server-protocol`.
- Active refs: 15.
- Ref scope: root workspace metadata, app-server-protocol dependency/import usage, and macro crate manifest/Bazel identity.

## OntoIndex CLI Fallback Impact

- `derive_experimental_api`: LOW, 0 impacted nodes, 0 affected processes.
- `ExperimentalApi`: LOW, 0 impacted nodes, 0 affected processes.
- `field_serialized_name`: LOW, 2 impacted nodes, 0 affected processes.
- `experimental_reason`: HIGH, 18 impacted nodes, 17 direct, 0 affected processes.
- HIGH reason: `experimental_reason` is referenced through app-server-protocol experimental API tests and macro helper flow.

## Guardrails

- Only package/proc-macro lib/Bazel/import identity may change.
- Preserve `#[derive(ExperimentalApi)]` expansion behavior.
- Preserve `#[experimental(...)]` parsing and nested experimental propagation.
- Preserve inventory registration and serialized field-name conversion.
- Preserve app-server v2 experimental gating and schema behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and folder path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-experimental-api-macros --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_experimental_api_macros|codex-experimental-api-macros`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Decision

- Approved as R5AC only because it is an identity-only rename with one direct dependent.
- Work must run on fallback `gpt-5.4-mini` after Spark usage-limit fallback and record that fallback in output/tracking.
