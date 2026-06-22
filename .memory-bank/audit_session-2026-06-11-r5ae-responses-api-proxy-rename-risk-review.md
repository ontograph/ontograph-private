# R5AE Responses API Proxy Rename Risk Review

Date: 2026-06-11

## Candidate

- `ontocode-responses-api-proxy` -> `ontocode-responses-api-proxy`
- `codex_responses_api_proxy` -> `ontocode_responses_api_proxy`

## Current Inventory

- Cargo metadata direct reverse dependency: `ontocode-cli`.
- Active refs: 29.
- Ref scope: root workspace metadata, CLI dependency/import/call sites, proxy manifest/Bazel identity, proxy main/lib imports, README/npm public surfaces, and dump-test compatibility string.

## OntoIndex CLI Fallback Impact

- `Function:ontocode-rs/responses-api-proxy/src/lib.rs:run_main`: LOW, 0 impacted nodes, 0 affected processes.
- `forward_request`: LOW, 1 impacted node, 0 affected processes.
- `Args`: LOW, 0 impacted nodes, 0 affected processes.
- `Function:ontocode-rs/responses-api-proxy/src/main.rs:main`: LOW, 0 impacted nodes, 0 affected processes.

## Guardrails

- Only Cargo package/lib/Bazel/import identity may change.
- Preserve API-key stdin handling.
- Preserve request forwarding and auth header construction.
- Preserve dump behavior, server-info behavior, HTTP shutdown behavior, and process hardening.
- Preserve public `ontocode-responses-api-proxy` binary name.
- Preserve npm package/bin names and README/config examples.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and folder path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-responses-api-proxy --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli responses-api-proxy`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_responses_api_proxy|ontocode-responses-api-proxy`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Decision

- Approved as R5AE only because it is a bounded identity-only Cargo package/lib/import rename with one direct dependent.
- Public binary/npm/docs/config compatibility names must not be removed or renamed in this slice.
- Work must run on fallback `gpt-5.4-mini` after Spark usage-limit fallback and record that fallback in output/tracking.
