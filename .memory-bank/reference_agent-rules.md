---
name: Ontocode Agent Rules
description: Compact implementation rules for agents working in this repository
type: reference
---

# Ontocode Agent Rules

This is a compact memory-bank reference. Full rules remain in `../AGENTS.md`.

## Required Tooling

- Use OntoIndex for code intelligence and impact checks.
- Use native `rg` / `rg --files` for searches.
- Use native shell commands directly.
- Run OntoIndex context/impact before code edits.
- Run OntoIndex `gn_verify_diff` before close-out/commit when the worktree
  scope is clean enough for repo-wide verification; otherwise use file-scoped
  verification and record the limitation.

## Optional Maintained Fork

- If the repo-local `ontocode-lean-ctx` plugin is installed and the backend
  from `third_party/lean-ctx-fork` is running with `LEANCTX_TOKEN`, use only
  `ctx_read`, `ctx_search`, and `ctx_summary` for bounded read-only retrieval.
- Do not use the maintained fork for shell execution, editing, session memory,
  or general code intelligence.
- OntoIndex remains the default code-intelligence path even when the plugin is
  available.

## OntoIndex Rules

- Before editing any function, method, type, or module owner, run OntoIndex
  impact on the target.
- Record direct callers, affected processes, affected modules, and risk.
- If risk is HIGH or CRITICAL, report the blast radius and narrow the seam before editing.
- Do not rename symbols with broad find-and-replace; use OntoIndex
  rename/impact analysis.

## Rust Workflow

- Use `just test`, not `cargo test`.
- Run `just fmt` in `ontocode-rs/` after Rust changes.
- Run scoped tests for changed crates.
- Ask before running full workspace `just test`.
- For compilation, build, packaging, install, or binary-run tasks, always give the user the exact command(s), the working directory or `--manifest-path`, and the expected binary or artifact path.
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
- For third-party tool migrations, remove external runtime dependencies. Adopt
  only the required legacy code into this repo as a repo-owned plugin or
  existing plugin/backend owner; do not depend on upstream checkouts, hosted
  services, package streams, or hidden downloads for normal use.

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
