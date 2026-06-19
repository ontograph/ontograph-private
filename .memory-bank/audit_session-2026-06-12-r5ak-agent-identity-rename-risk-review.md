# R5AK Agent Identity Rename Risk Review

Date: 2026-06-12

## Candidate

- `codex-agent-identity` -> `ontocode-agent-identity`
- `codex_agent_identity` -> `ontocode_agent_identity`

## Inventory

- Cargo metadata direct reverse dependencies: `ontocode-login`, `ontocode-model-provider`
- Active direct refs: 17
- Ref locations: root workspace metadata, agent-identity manifest/Bazel/internal test string, login dependency/import/test usage, and model-provider dependency/import usage.

## OntoIndex Impact

- `Function:ontocode-rs/agent-identity/src/lib.rs:authorization_header_for_agent_task`: LOW, 3 impacted, 3 direct, 1 module, 0 processes.
- `Function:ontocode-rs/agent-identity/src/lib.rs:register_agent_task`: LOW, 1 impacted, 1 direct, 1 module, 0 processes.
- `Function:ontocode-rs/agent-identity/src/lib.rs:generate_agent_key_material`: LOW, 9 impacted, 1 direct, 1 module, 0 processes.
- `Function:ontocode-rs/agent-identity/src/lib.rs:decode_agent_identity_jwt`: HIGH, 12 impacted, 3 direct, 4 modules, 0 processes.
- `Function:ontocode-rs/agent-identity/src/lib.rs:fetch_agent_identity_jwks`: HIGH, 10 impacted, 1 direct, 3 modules, 0 processes.

## Decision

- Proceed as an identity-only package/lib/Bazel/import rename.
- The HIGH impact is accepted only because JWT/JWKS validation, signing, registration, login, and model-provider auth behavior must remain unchanged.

## Guardrails

- Preserve JWT issuer/audience/kid/JWKS validation, raw plan alias mapping, signing/decryption/key generation, task-registration request/response behavior, ABOM shape, URL construction, auth header construction, login auth storage/manager behavior, and model-provider auth behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `agent-identity` directory path.
- Do not print or store private keys, JWTs, auth headers, cookies, or credentials in logs, tests, memory-bank, or final output.
- Verify with agent-identity package tests, focused login agent-identity tests, focused model-provider auth tests, fmt, Bazel lock checks, active-source stale-reference search, metadata count, diff check, and OntoIndex CLI fallback verification.
