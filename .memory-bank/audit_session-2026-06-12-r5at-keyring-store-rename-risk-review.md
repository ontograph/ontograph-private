# R5AT Keyring Store Rename Risk Review

## Decision

Dispatch R5AT as an identity-only residual package rename:

- `codex-keyring-store` -> `ontocode-keyring-store`
- `codex_keyring_store` -> `ontocode_keyring_store`

## OntoIndex Impact

- `KeyringStore`: LOW, 2 impacted implementors, no affected processes.
- `DefaultKeyringStore`: LOW, 0 impacted nodes, no affected processes.
- `MockKeyringStore`: ambiguous between struct and impl; direct inventory confines usage to tests and dependent test helpers.

## Direct Inventory

- Root workspace metadata and keyring-store manifest/Bazel identity.
- Dependent manifests/imports in `login`, `rmcp-client`, and `secrets`.

## Guardrails

- Preserve OS keyring service/account/value behavior.
- Preserve credential load/save/delete semantics.
- Preserve error wrapping/messages and mock credential behavior.
- Preserve login credential storage behavior.
- Preserve RMCP OAuth credential behavior.
- Preserve secrets keyring/local behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `keyring-store` directory path.

## Verification Required

- `CARGO_BUILD_JOBS=8 just test -p ontocode-keyring-store --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login storage`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client oauth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-secrets --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_keyring_store|codex-keyring-store`
- Cargo metadata residual count, expected 27 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`
