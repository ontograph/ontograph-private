# R5E Core Rename Risk Review

Date: 2026-06-11

## Candidate

- Approved slice: `codex-core` -> `ontocode-core`.
- Approved crate import rename: standalone `codex_core` -> `ontocode_core`.
- Keep directory path: `ontocode-rs/core`.

## Scope

- Direct textual scope: 143 `codex-core` package refs, 775 standalone `codex_core` crate refs, 280 files.
- This is the final Stage 5 implementation slice after `core-api`, client transport, and API client were accepted.

## OntoIndex Evidence

- `Struct:ontocode-rs/core/src/session/mod.rs:Codex`: CRITICAL risk, 41 impacted nodes, 21 direct, 3 affected execution flows, 7 modules.
- Affected flows include unified exec runtime, shell runtime, and apply-patch handler paths.
- `Struct:ontocode-rs/core/src/client.rs:ModelClient`: LOW risk, 0 impacted nodes.
- `Struct:ontocode-rs/core/src/session/session.rs:Session`: LOW risk, 3 impacted nodes, 1 direct.

## Allowed Changes

- Rename Cargo package identity from `codex-core` to `ontocode-core`.
- Rename standalone Rust crate identity/imports from `codex_core` to `ontocode_core`.
- Update Bazel crate identity, workspace dependency keys, lockfiles, direct dependent manifests/imports, and internal tests.
- Update README/internal package-identity references only when they describe the crate identity.

## Explicit Non-Scope

- Do not rename `codex-core-plugins` or `codex-core-skills`.
- Do not rename protocol/generated crate/package/schema names.
- Do not rename public commands, CLI surface names, package-manager runtime names, or public SDK names.
- Do not rename telemetry/product strings, OTEL service names, persisted state keys, rollout/session data, or env/config keys.
- Do not change session, tool, model-client, shell, unified-exec, guardian, MCP, or apply-patch runtime behavior.
- Do not rename the `ontocode-rs/core` directory.

## Required Verification

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-core --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-cli --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-exec --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server --no-tests=pass`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Stale search for active standalone `codex_core` and package `codex-core` refs under `ontocode-rs`; classify intentional leftovers separately from `codex_core_plugins` and `codex_core_skills`.
- `git diff --check`.
- OntoIndex scoped diff verification if MCP is usable, otherwise CLI `detect-changes --repo codex`.

## Decision

- Dispatch one worker for R5E only.
- Stage 5 closes only after R5E manager acceptance.
