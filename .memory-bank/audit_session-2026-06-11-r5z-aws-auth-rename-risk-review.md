# R5Z AWS Auth Rename Risk Review

Date: 2026-06-11

## Decision

- Dispatch `codex-aws-auth` -> `ontocode-aws-auth`.
- Dispatch `codex_aws_auth` -> `ontocode_aws_auth`.
- Scope is identity-only package/lib/Bazel/import rename.

## Direct Inventory

- Cargo metadata direct reverse dependency: `ontocode-model-provider`.
- Active direct refs before dispatch: 9.
- Refs are confined to root workspace metadata, `aws-auth` manifest/Bazel identity, and Amazon Bedrock model-provider dependency/imports.

## OntoIndex Impact

- MCP impact is miswired to repo `OntoIndex`; CLI fallback was used.
- `Struct:ontocode-rs/aws-auth/src/lib.rs:AwsAuthContext`: LOW, 2 impacted nodes, 1 direct, 0 affected processes.
- `Struct:ontocode-rs/aws-auth/src/lib.rs:AwsAuthConfig`: LOW, 5 impacted nodes, 2 direct, 0 affected processes.
- `Struct:ontocode-rs/aws-auth/src/lib.rs:AwsRequestToSign`: LOW, 4 impacted nodes, 2 direct, 0 affected processes.
- `Enum:ontocode-rs/aws-auth/src/lib.rs:AwsAuthError`: LOW, 0 impacted nodes, 0 direct, 0 affected processes.

## Guardrails

- Preserve AWS SDK credential-provider loading.
- Preserve profile, region, and service resolution semantics.
- Preserve SigV4 signing behavior, auth headers, and session-token behavior.
- Preserve retryability classification.
- Preserve Amazon Bedrock auth/provider behavior in `ontocode-model-provider`.
- Preserve telemetry, env/config/wire/generated names, persisted state, and the existing `aws-auth` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-aws-auth --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_aws_auth|codex-aws-auth`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Model

- Use `gpt-5.4-mini` high because `gpt-5.3-codex-spark` reached usage limit.
