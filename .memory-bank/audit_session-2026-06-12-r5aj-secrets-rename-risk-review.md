# R5AJ Secrets Rename Risk Review

Date: 2026-06-12

## Candidate

- `codex-secrets` -> `ontocode-secrets`
- `codex_secrets` -> `ontocode_secrets`

## Inventory

- Cargo metadata direct reverse dependencies: `codex-memories-write`
- Active direct refs: 5
- Ref locations: root workspace metadata, memories-write dependency/import usage, and secrets manifest/Bazel identity.

## OntoIndex Impact

- `Function:ontocode-rs/secrets/src/sanitizer.rs:redact_secrets`: LOW, 7 impacted, 3 direct, 2 modules, 0 processes.
- `Struct:ontocode-rs/secrets/src/lib.rs:SecretsManager`: LOW, 0 impacted, 0 processes.
- `Impl:ontocode-rs/secrets/src/lib.rs:SecretsManager`: LOW, 0 impacted, 0 processes.
- `Struct:ontocode-rs/secrets/src/lib.rs:SecretName`: LOW, 0 impacted, 0 processes.
- `Struct:ontocode-rs/secrets/src/local.rs:LocalSecretsBackend`: LOW, 0 impacted, 0 processes.
- `Impl:ontocode-rs/secrets/src/local.rs:LocalSecretsBackend`: LOW, 0 impacted, 0 processes.
- `Function:ontocode-rs/secrets/src/lib.rs:compute_keyring_account`: HIGH, 9 impacted, 2 direct, 4 modules, 0 processes.

## Decision

- Proceed as an identity-only package/lib/Bazel/import rename.
- The HIGH impact is accepted only because keyring service/account derivation, encrypted file format, redaction behavior, and memories-write behavior must remain unchanged.

## Guardrails

- Preserve redaction regex behavior, encrypted local secrets file format/version/path, keyring service/account derivation, passphrase generation/loading, atomic writes, deletion/listing semantics, key validation, and memories-write redaction behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `secrets` directory path.
- Do not print or store raw secrets, tokens, keychain paths, or private values in logs, tests, or memory-bank entries.
- Verify with secrets package tests, memories-write redaction checks, fmt, Bazel lock checks, active-source stale-reference search, metadata count, diff check, and OntoIndex CLI fallback verification.
