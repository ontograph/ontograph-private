---
name: Ontocode Agent Rules
description: Compact implementation rules for agents working in this repository
type: reference
---

# Ontocode Agent Rules

This is a compact memory-bank reference. Full rules remain in `../AGENTS.md`.

## Required Tooling

- Use lean-ctx for shell/read/search/tree whenever possible.
- Use `lean-ctx -c "<command>"` for compressed shell output when MCP `ctx_shell` is blocked or when commands match lean-ctx compression rules.
- Use `rg`/`rg --files` through lean-ctx for searches.
- Use GitNexus context/impact before code edits.
- Run GitNexus `detect_changes` before close-out/commit.

## GitNexus Rules

- Before editing any function, method, type, or module owner, run GitNexus impact on the target.
- Record direct callers, affected processes, affected modules, and risk.
- If risk is HIGH or CRITICAL, report the blast radius and narrow the seam before editing.
- Do not rename symbols with broad find-and-replace; use GitNexus rename/impact analysis.

## Rust Workflow

- Use `just test`, not `cargo test`.
- Run `just fmt` in `codex-rs/` after Rust changes.
- Run scoped tests for changed crates.
- Ask before running full workspace `just test`.
- Do not kill Rust commands by PID; slow Rust builds and locks are expected.
- If `codex-cli` hits linker OOM, use:

```bash
CARGO_BUILD_JOBS=1 CARGO_PROFILE_TEST_DEBUG=0 RUSTFLAGS='-Cdebuginfo=0' just test -p codex-cli <filter>
```

## Architecture Reuse

Before provider, auth, MCP, hooks, shell, session/context, diagnostics, or external-agent import work:

- Reuse existing architecture first.
- Do not create duplicate provider factories, provider registries, model catalogs, runtime stream abstractions, capability resolvers, OAuth parsers, credential stores, redactors, MCP status pipelines, hook registries, shell launchers, policy evaluators, or context injection paths.
- Add new modules only when existing owners would become too large or mix unrelated concepts.
- Prefer existing test harnesses and fixtures.

## Security And Context

- Diagnostics must not expose tokens, cookies, auth headers, keychain paths, raw credentials, API keys, or OAuth secrets.
- Add no-secret assertions for security-sensitive diagnostics.
- Anything injected into model context must use bounded context fragments with hard caps.
- Treat any model-context item over 1k tokens as requiring manual review.

## Ontocode Rename

- `Ontocode` / `ontocode` is the target identity for rename/migration work.
- Preserve compatibility shims for external integrations, persisted state, CLI commands, APIs, config keys, package names, rollout/session data, and user files.
- Never broad find-and-replace names.

## Tracking Discipline

- Before starting any project-plan task, update `CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md`.
- Include status, GitNexus context, reuse anchors, owner, test plan, and next action.
- After completion, record verification and GitNexus detect result.
