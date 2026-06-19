# R5AJ Secrets Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-secrets` -> `ontocode-secrets`.
- Accepted `codex_secrets` -> `ontocode_secrets`.
- Scope was identity-only package/lib/Bazel/import rename.

## Preserved Surfaces

- Redaction regex behavior.
- Encrypted local secrets file format/version/path.
- Keyring service/account derivation and passphrase generation/loading.
- Atomic writes, deletion/listing semantics, and key validation.
- Memories-write redaction behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the `secrets` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-secrets --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-memories-write serializes_memory_rollout_redacts_secrets_before_prompt_upload`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_secrets|codex-secrets`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5AJ.
- Active old refs are clean in `ontocode-rs`.
- Cargo metadata reports 37 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5AJ-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
