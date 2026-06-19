# R5AT Keyring Store Rename Closure

## Scope

Accepted the identity-only residual crate rename:

- `codex-keyring-store` -> `ontocode-keyring-store`
- `codex_keyring_store` -> `ontocode_keyring_store`

## Preserved Behavior

- OS keyring service/account/value behavior, credential load/save/delete semantics, error wrapping/messages, mock credential behavior, login credential storage behavior, RMCP OAuth credential behavior, secrets keyring behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `keyring-store` directory path stayed unchanged.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-keyring-store --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login storage`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client oauth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-secrets --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_keyring_store|codex-keyring-store`
- Cargo metadata residual count: 27 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Notes

- Active old keyring-store identity refs are clean.
- OntoIndex reports the known broad high-risk dirty tree from the accumulated rename program.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
