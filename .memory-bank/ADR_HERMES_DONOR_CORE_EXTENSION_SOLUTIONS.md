---
name: Hermes Donor Core Extension Solutions
description: Senior unblock plan for the four accepted Hermes donor core-extension rows
type: adr
date: 2026-06-21
status: accepted
---

# ADR: Hermes Donor Core Extension Solutions

Authority:
- `tmp/hermes-agent-500-tools-for-ontocode-challenged.md`
- `HERMES_DONOR_CORE_EXTENSION_TRACKING.md`

## Decision

Unblock only the four implementation-queue rows from the challenged Hermes donor review. The donor value is adversarial regression coverage for existing Ontocode owners, not a Hermes-style tool/runtime import.

## Accepted Slices

| Slice | Row | Solution | Owner | Stop Condition |
| --- | --- | --- | --- | --- |
| `HERMES-R1` | `HERMES-KEEP-03` | Add one focused regression proving `spawn_agent` preserves the exact model string, function-call namespace, and allowed tool exposure when parent tool visibility is restricted. Edit production code only if the test fails. | `ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs`, `ontocode-rs/core/src/tools/spec_plan.rs` | Any new delegation runtime, model alias map, or parallel tool registry. |
| `HERMES-R2` | `HERMES-KEEP-05` | Check the existing tool-result/display path for repeated-failure hints. If absent, add one bounded diagnostic with a cap and one owner-local test. | Existing tool result classification/display owner after OntoIndex context/impact. | New guardrail framework, autonomous retry planner, or policy engine. |
| `HERMES-R3` | `HERMES-KEEP-17` | Add or verify one regression proving MCP/plugin connector reload or auth-change invalidates stale cached tools/connectors. Edit code only if stale state persists. | `ontocode-rs/core-plugins/src/manager.rs`, `ontocode-rs/core-plugins/src/loader.rs`, `codex-mcp` connector cache owner if touched. | New connector lifecycle manager or public cache API. |
| `HERMES-R4` | `HERMES-KEEP-20` | Add one long-running process cleanup/status regression at the existing exec/session shell boundary. Edit code only if a process survives expected cleanup or status loses state. | `ontocode-rs/exec-server/src`, existing core shell/session tests. | Hermes-style process registry or new process persistence surface. |

## Closed Rows

`HERMES-KEEP-01`, `02`, `04`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `18`, and `19` remain closed unless a concrete owner-local failing fixture reopens them.

## Verification

Each dispatched slice must:

- update `HERMES_DONOR_CORE_EXTENSION_TRACKING.md` before starting and after closure;
- use OntoIndex context/impact before production symbol edits;
- keep changes inside the accepted owner;
- prefer tests over production code when current behavior already exists;
- run scoped tests for the touched crate;
- run `CARGO_BUILD_JOBS=8 just fmt` after Rust edits;
- refresh/check OntoIndex after closure.
