# R5AK Agent Identity Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-agent-identity` -> `ontocode-agent-identity`.
- Accepted `codex_agent_identity` -> `ontocode_agent_identity`.
- Scope was identity-only package/lib/Bazel/import rename.

## Preserved Surfaces

- JWT issuer/audience/kid/JWKS validation and raw plan alias mapping.
- Signing, decryption, key generation, task-registration request/response behavior, and ABOM shape.
- URL construction and auth header construction.
- Login auth storage/manager behavior and model-provider auth behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the `agent-identity` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-agent-identity --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login agent_identity`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider auth`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_agent_identity|codex-agent-identity`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5AK.
- Active old refs are clean in `ontocode-rs`.
- Cargo metadata reports 36 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5AK-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
