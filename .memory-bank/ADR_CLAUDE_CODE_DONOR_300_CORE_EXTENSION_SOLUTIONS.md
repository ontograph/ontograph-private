---
name: Claude Code Donor 300 Core Extension Solutions
description: Senior unblock plan for the keep-only Claude Code donor 300 core-extension rows
type: adr
date: 2026-06-21
status: accepted
---

# ADR: Claude Code Donor 300 Core Extension Solutions

Authority:
- `tmp/claude-code-main-300-tools-for-ontocode-challenged.md`
- `CLAUDE_CODE_DONOR_300_CORE_EXTENSION_TRACKING.md`

## Decision

Unblock the remaining `KEEP` rows as owner-local regression bundles. Donor code is reference evidence only. Do not import Claude Code runtimes, tool registries, plugin marketplaces, scheduled tasks, REPL/eval tools, or parallel task systems.

## Accepted Bundles

| Bundle | Rows | Solution | Owner | Stop Condition |
| --- | --- | --- | --- | --- |
| `CLAUDE300-R1` | `001`, `003`, `004`, `005`, `008`, `011`, `012`, `015`, `016`, `026`-`031`, `049`, `053` | Harden current multi-agent and agent-job behavior with focused request-shape, profile, lineage, resume, output cap, cancel, and state-transition tests. Production edits only if an existing-owner test fails. | `ontocode-rs/core/src/tools/handlers/multi_agents*`, `ontocode-rs/core/src/tools/handlers/agent_jobs*`, `ontocode-rs/state/src/runtime/agent_jobs.rs` | New agent runtime, task DB, alias registry, or scheduler. |
| `CLAUDE300-R2` | `061`, `064`-`076`, `079`, `081`-`084`, `087`-`089`, `093`-`100` | Add missing shell/PowerShell policy and parser regressions for command classification, destructive warnings, read-only validation, path/cwd/env semantics, and package/network approval. | `ontocode-rs/core/src/exec_policy*`, `ontocode-rs/shell-command/src/command_safety/*`, `ontocode-rs/execpolicy/*`, shell runtime tests | New shell tool, policy engine, or sandbox mode. |
| `CLAUDE300-R3` | `101`, `103`, `104`, `106`, `111`, `115`, `118`, `121`, `123`, `125`, `149`, `239`, `241`, `242` | Verify bounded context/file/search/LSP/attachment/compaction behavior. Add only cap/order/shape regressions inside current owners. | `ontocode-rs/core/src/context/*`, existing file/search/apply-patch/compaction handlers, LSP/context owners | New context injector, file transport, or LSP replacement. |
| `CLAUDE300-R4` | `151`, `153`, `155`, `156`, `159`, `163`, `164`, `166`, `169`, `190`, `195`, `197`, `201` | Strengthen MCP/resource/auth/tool-discovery/skills/plugins/config behavior through current MCP, plugin, tool-search, and config owners. | `ontocode-rs/codex-mcp/*`, `ontocode-rs/rmcp-client/*`, `ontocode-rs/core/src/tools/handlers/mcp_resource*`, `ontocode-rs/tools/src/tool_search.rs`, `ontocode-rs/core-plugins/*` | New MCP manager, command stack, marketplace flow, or public cache API. |
| `CLAUDE300-R5` | `054`, `218`, `226`-`228`, `231`, `233`, `235`, `236`, `261`, `262`, `265`, `266`, `286`, `288`, `290`, `296`-`300` | Add minimal redacted diagnostics/review/web/pacing/plan-mode/model/auth regressions in existing owners. Keep slash-command behavior tied to existing config/auth/review surfaces. | Existing diagnostics, review, web, plan-mode, sandbox, model picker, reasoning, and auth/login owners | GitHub automation, new command framework, unredacted support bundles, or public API changes. |

## Verification

Each worker must:

- update `CLAUDE_CODE_DONOR_300_CORE_EXTENSION_TRACKING.md` before closure;
- run OntoIndex context/impact before production symbol edits;
- keep changes inside the accepted owner;
- prefer tests over production code when current behavior already exists;
- run scoped tests for touched crates with `CARGO_BUILD_JOBS=8`;
- run `CARGO_BUILD_JOBS=8 just fmt` after Rust edits;
- report exact files changed and tests run.
