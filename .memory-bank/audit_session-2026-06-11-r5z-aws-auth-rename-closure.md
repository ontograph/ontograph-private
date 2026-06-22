# R5Z AWS Auth Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-aws-auth` -> `ontocode-aws-auth`.
- Accepted `codex_aws_auth` -> `ontocode_aws_auth`.
- Scope was identity-only package/lib/Bazel/import rename.

## Preserved Surfaces

- AWS SDK credential-provider loading.
- Profile, region, and service resolution semantics.
- SigV4 signing behavior.
- Auth header and session-token behavior.
- Retryability classification.
- Amazon Bedrock auth/provider behavior.
- Telemetry, env/config/wire/generated names, persisted state, and folder path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-aws-auth --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_aws_auth|codex-aws-auth`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5Z.
- Active old refs are clean in `ontocode-rs`.
- Cargo metadata reports 47 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5Z-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
