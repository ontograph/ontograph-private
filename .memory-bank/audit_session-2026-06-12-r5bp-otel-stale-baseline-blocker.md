# R5BP OTEL Stale Baseline Blocker

Date: 2026-06-12

## Decision

R5BP is blocked and not accepted.

## Evidence

- Worker verification renamed exact OTEL surfaces and passed OTEL checks, but reported a restored baseline with 119 remaining `codex-*` packages.
- Manager validation confirmed the regression with Cargo metadata and found previously accepted package identities restored to stale `codex-*` names.
- The accepted baseline before R5BP had 6 remaining `codex-*` packages; after accepting OTEL the target residual set should be exactly `ontocode-app-server-protocol`, `codex-extension-api`, `codex-protocol`, `codex-state`, and `codex-tools`.

## Recovery

- Start R5BP-U1 before any new dispatch.
- Recover accepted package/lib/Bazel/import identities with an explicit mapping.
- Preserve public commands, config keys, wire/generated names, persisted state, telemetry/product strings, and directory paths.
- Revalidate metadata count, stale references, lockfiles, formatting, targeted package tests/checks, diff hygiene, and OntoIndex change detection before accepting OTEL.
