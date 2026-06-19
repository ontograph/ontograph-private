# R5Z AWS Auth Rename Worker Verification

Date: 2026-06-11

Status: worker verification complete

Summary:
- Implemented the identity-only rename `codex-aws-auth` -> `ontocode-aws-auth` and `codex_aws_auth` -> `ontocode_aws_auth`.
- Kept AWS SDK credential-provider loading, profile/region/service semantics, SigV4 signing behavior, auth header/session-token behavior, retryability classification, and Amazon Bedrock auth/provider behavior unchanged.
- Updated the workspace package metadata, the aws-auth library crate name, the Bazel crate name, the model-provider dependency/import surface, and the lockfile package identity entries.

Verification:
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-aws-auth --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n "codex_aws_auth|codex-aws-auth" ontocode-rs`
- `cargo metadata --format-version 1 --no-deps`
- `git diff --check`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

Residual refs:
- No active-source `codex_aws_auth` or `codex-aws-auth` refs remain under `ontocode-rs`.
- Cargo metadata now reports 47 remaining `codex-*` workspace packages.

Model note:
- Requested Spark was unavailable/usage-limited; this worker ran on fallback `gpt-5.4-mini`.
