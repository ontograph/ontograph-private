# R5E Core Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-core` -> `ontocode-core`.
- Accepted standalone Rust crate import rename `codex_core` -> `ontocode_core`.
- Preserved `codex-core-plugins`, `codex-core-skills`, protocol/generated names, public command names, telemetry/product strings, persisted state, config/env compatibility, and the existing `core` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-core --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `git diff --check`
- Stale-reference search for standalone `codex_core`, `codex-core`, and new `ontocode_core` / `ontocode-core` identities.
- OntoIndex CLI fallback: `detect-changes --repo codex`.

## Findings

- No standalone `codex_core` crate imports remain under `ontocode-rs`.
- Remaining `codex-core` strings are intentional compatibility/docs/tests plus `codex-core-plugins` and `codex-core-skills`.
- `ontocode-core` package/lib identity is present in Cargo metadata.
- `ontocode-core` had one realtime conversation test marked flaky by nextest after retry; the package test run completed successfully.
- OntoIndex CLI fallback reports high risk across the broad dirty rename tree; this is expected for the accumulated staged rename work and is not isolated to R5E.

## Decision

R5E is accepted. Stage 5 core/shared implementation is complete. R5B protocol/generated-schema rename remains blocked by separate ADR; R6 cleanup/command-reference work is the next eligible staged review.
