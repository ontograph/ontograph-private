# Qwen Code 2000 Useful Approaches Review

Status: distinct core-functionality review artifact, OntoIndex-grounded, no implementation dispatch.
Date: 2026-06-22
Donor source: `tmp/qwen-code`
Current repo: Ontocode independent fork, not official OpenAI/Azure.

## Senior Challenge

The donor repository is mostly a TypeScript/ACP/daemon/WebUI/Desktop stack. For Ontocode, the remaining useful rows are only those that can extend existing Rust core behavior: agent jobs, shell/sandbox policy, hooks, context/compaction, or operational evidence.

Rejected by default: installer/release packaging, terminal UI rendering, snapshots/docs-only rows, parallel daemon owners, JS bridge state, desktop/webui/channel products, new public APIs without ADR, native non-OpenAI OAuth/provider stacks, and broad MCP pooling unless a current Rust owner has a proven failing path.

## Review Challenge Findings

- The previous 600-row version was not 600 implementation candidates; it was 50 behavior seeds expanded through 12 review lenses.
- Lens rows such as `AGENT` plus `MCP` or `SHELL` crossed owner boundaries and would create duplicate or misleading dispatch tasks.
- This artifact now keeps one row per distinct behavior seed. Use the family, owner, and action as the dispatch unit; apply redaction, bounds, compatibility, failure, resume, cancellation, single-path, owner, state, MCP, agent, and shell checks during implementation review instead of dispatching them as separate tasks.

## OntoIndex Owner Evidence

- Index freshness: `gn_ensure_fresh(repo=codex)` reported indexed commit `1d91f6ba20637508c8087e308bb49ed520011f2f`, dirty worktree caveat, and no stale index.
- Agent jobs owner: `ontocode-rs/core/src/tools/handlers/agent_jobs.rs::run_agent_job_loop`, with state calls in `ontocode-rs/state/src/runtime/agent_jobs.rs`.
- Shell/sandbox owner: `ontocode-rs/core/src/tools/runtimes/unified_exec.rs::run` plus runtime sandbox helpers and `shell-command` safety checks.
- Hook output owner: `ontocode-rs/hooks/src/output_spill.rs::HookOutputSpiller` and `core/src/hook_runtime.rs`.
- Context/compaction owner: `ontocode-rs/core/src/session/turn.rs::run_turn`, `run_pre_sampling_compact`, and `compact_remote_v2.rs`.
- Operational evidence owner: `ontocode-rs/state/src/runtime/operational_evidence_import.rs` and `operational_evidence.rs`.

## Verdict Counts

- KEEP-CANDIDATE: 50

`KEEP-CANDIDATE` means retained for owner-local review. It does not mean dispatchable implementation work without a fresh failing behavior or missing compatibility test.

## Family Counts

- AGENT: 10
- CONTEXT: 10
- EVIDENCE: 10
- HOOK: 10
- SHELL: 10

## Dispatch Rule

Do not dispatch rows mechanically. Dispatch only a distinct row when it points to a current Rust core owner and a concrete failing behavior or missing compatibility test. Keep all implementation inside the listed owner; reject side stacks, parallel daemon owners, and public API/config/schema expansion unless a separate ADR approves that surface.

## Distinct Core Functionality Approaches

| # | Approach | Verdict | Donor Evidence | Current Owner | Useful Action |
|---:|---|---|---|---|---|
| 203 | `AGENT-01-03` | KEEP-CANDIDATE | `packages/acp-bridge agent/session lifecycle + sdk subagents tests` | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs; state/runtime/agent_jobs.rs` | agent job loop recovers running items after restart |
| 223 | `AGENT-02-03` | KEEP-CANDIDATE | `packages/acp-bridge agent/session lifecycle + sdk subagents tests` | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs; state/runtime/agent_jobs.rs` | agent job cancellation marks pending items deterministically |
| 243 | `AGENT-03-03` | KEEP-CANDIDATE | `packages/acp-bridge agent/session lifecycle + sdk subagents tests` | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs; state/runtime/agent_jobs.rs` | agent job progress stays bounded for many items |
| 263 | `AGENT-04-03` | KEEP-CANDIDATE | `packages/acp-bridge agent/session lifecycle + sdk subagents tests` | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs; state/runtime/agent_jobs.rs` | agent job CSV export caps result JSON |
| 283 | `AGENT-05-03` | KEEP-CANDIDATE | `packages/acp-bridge agent/session lifecycle + sdk subagents tests` | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs; state/runtime/agent_jobs.rs` | agent job final summary survives resume |
| 303 | `AGENT-06-03` | KEEP-CANDIDATE | `packages/acp-bridge agent/session lifecycle + sdk subagents tests` | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs; state/runtime/agent_jobs.rs` | subagent completion notification includes stable path |
| 323 | `AGENT-07-03` | KEEP-CANDIDATE | `packages/acp-bridge agent/session lifecycle + sdk subagents tests` | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs; state/runtime/agent_jobs.rs` | stale active items are reaped without killing unrelated threads |
| 343 | `AGENT-08-03` | KEEP-CANDIDATE | `packages/acp-bridge agent/session lifecycle + sdk subagents tests` | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs; state/runtime/agent_jobs.rs` | worker prompt carries task scope without raw history dump |
| 363 | `AGENT-09-03` | KEEP-CANDIDATE | `packages/acp-bridge agent/session lifecycle + sdk subagents tests` | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs; state/runtime/agent_jobs.rs` | agent model override is explicit in spawn metadata |
| 383 | `AGENT-10-03` | KEEP-CANDIDATE | `packages/acp-bridge agent/session lifecycle + sdk subagents tests` | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs; state/runtime/agent_jobs.rs` | status subscription wakes waiters without busy polling |
| 603 | `SHELL-01-03` | KEEP-CANDIDATE | `.qwen/design/session-shell-permission-policy.md + run_shell_command.test.ts` | `ontocode-rs/core/src/tools/runtimes/unified_exec.rs; shell-command command_safety` | shell command policy inheritance is covered for child agents |
| 623 | `SHELL-02-03` | KEEP-CANDIDATE | `.qwen/design/session-shell-permission-policy.md + run_shell_command.test.ts` | `ontocode-rs/core/src/tools/runtimes/unified_exec.rs; shell-command command_safety` | direct shell paths preserve denied read restrictions |
| 643 | `SHELL-03-03` | KEEP-CANDIDATE | `.qwen/design/session-shell-permission-policy.md + run_shell_command.test.ts` | `ontocode-rs/core/src/tools/runtimes/unified_exec.rs; shell-command command_safety` | PowerShell commands get UTF-8 profile-safe wrapping |
| 663 | `SHELL-04-03` | KEEP-CANDIDATE | `.qwen/design/session-shell-permission-policy.md + run_shell_command.test.ts` | `ontocode-rs/core/src/tools/runtimes/unified_exec.rs; shell-command command_safety` | dangerous command detection covers destructive literals |
| 683 | `SHELL-05-03` | KEEP-CANDIDATE | `.qwen/design/session-shell-permission-policy.md + run_shell_command.test.ts` | `ontocode-rs/core/src/tools/runtimes/unified_exec.rs; shell-command command_safety` | safe command detection rejects ambiguous chained writes |
| 703 | `SHELL-06-03` | KEEP-CANDIDATE | `.qwen/design/session-shell-permission-policy.md + run_shell_command.test.ts` | `ontocode-rs/core/src/tools/runtimes/unified_exec.rs; shell-command command_safety` | sandbox environment strips managed proxy secrets when needed |
| 723 | `SHELL-07-03` | KEEP-CANDIDATE | `.qwen/design/session-shell-permission-policy.md + run_shell_command.test.ts` | `ontocode-rs/core/src/tools/runtimes/unified_exec.rs; shell-command command_safety` | working-directory snapshots survive shell wrapper injection |
| 743 | `SHELL-08-03` | KEEP-CANDIDATE | `.qwen/design/session-shell-permission-policy.md + run_shell_command.test.ts` | `ontocode-rs/core/src/tools/runtimes/unified_exec.rs; shell-command command_safety` | command output truncation leaves recovery path |
| 763 | `SHELL-09-03` | KEEP-CANDIDATE | `.qwen/design/session-shell-permission-policy.md + run_shell_command.test.ts` | `ontocode-rs/core/src/tools/runtimes/unified_exec.rs; shell-command command_safety` | timeout/cancel paths terminate owned process only |
| 783 | `SHELL-10-03` | KEEP-CANDIDATE | `.qwen/design/session-shell-permission-policy.md + run_shell_command.test.ts` | `ontocode-rs/core/src/tools/runtimes/unified_exec.rs; shell-command command_safety` | single-build-mode guidance remains documented and enforced by command examples |
| 803 | `HOOK-01-03` | KEEP-CANDIDATE | `hook-integration + hooks command tests` | `ontocode-rs/hooks/src/output_spill.rs; core/src/hook_runtime.rs` | large hook output spills to file with bounded preview |
| 823 | `HOOK-02-03` | KEEP-CANDIDATE | `hook-integration + hooks command tests` | `ontocode-rs/hooks/src/output_spill.rs; core/src/hook_runtime.rs` | repeated hook output stays bounded |
| 843 | `HOOK-03-03` | KEEP-CANDIDATE | `hook-integration + hooks command tests` | `ontocode-rs/hooks/src/output_spill.rs; core/src/hook_runtime.rs` | spilled hook output exposes exactly one recovery path |
| 863 | `HOOK-04-03` | KEEP-CANDIDATE | `hook-integration + hooks command tests` | `ontocode-rs/hooks/src/output_spill.rs; core/src/hook_runtime.rs` | hook completion events do not inject unbounded text |
| 883 | `HOOK-05-03` | KEEP-CANDIDATE | `hook-integration + hooks command tests` | `ontocode-rs/hooks/src/output_spill.rs; core/src/hook_runtime.rs` | hook permission mode follows current session policy |
| 903 | `HOOK-06-03` | KEEP-CANDIDATE | `hook-integration + hooks command tests` | `ontocode-rs/hooks/src/output_spill.rs; core/src/hook_runtime.rs` | turn-stop hooks record structured start/completion events |
| 923 | `HOOK-07-03` | KEEP-CANDIDATE | `hook-integration + hooks command tests` | `ontocode-rs/hooks/src/output_spill.rs; core/src/hook_runtime.rs` | hook transcript paths are normalized before display |
| 943 | `HOOK-08-03` | KEEP-CANDIDATE | `hook-integration + hooks command tests` | `ontocode-rs/hooks/src/output_spill.rs; core/src/hook_runtime.rs` | after-agent legacy hook remains compatibility-only |
| 963 | `HOOK-09-03` | KEEP-CANDIDATE | `hook-integration + hooks command tests` | `ontocode-rs/hooks/src/output_spill.rs; core/src/hook_runtime.rs` | hook failure text is redacted before model context |
| 983 | `HOOK-10-03` | KEEP-CANDIDATE | `hook-integration + hooks command tests` | `ontocode-rs/hooks/src/output_spill.rs; core/src/hook_runtime.rs` | subagent hook context is explicit and capped |
| 1003 | `CONTEXT-01-03` | KEEP-CANDIDATE | `auto-compaction-threshold-redesign.md; compact-mode docs; memory diagnostics plans` | `ontocode-rs/core/src/session/turn.rs; compact_remote_v2.rs; context_manager` | pre-sampling compaction runs before oversized request |
| 1023 | `CONTEXT-02-03` | KEEP-CANDIDATE | `auto-compaction-threshold-redesign.md; compact-mode docs; memory diagnostics plans` | `ontocode-rs/core/src/session/turn.rs; compact_remote_v2.rs; context_manager` | remote compaction keeps retained messages bounded |
| 1043 | `CONTEXT-03-03` | KEEP-CANDIDATE | `auto-compaction-threshold-redesign.md; compact-mode docs; memory diagnostics plans` | `ontocode-rs/core/src/session/turn.rs; compact_remote_v2.rs; context_manager` | compacted history discards before truncating |
| 1063 | `CONTEXT-04-03` | KEEP-CANDIDATE | `auto-compaction-threshold-redesign.md; compact-mode docs; memory diagnostics plans` | `ontocode-rs/core/src/session/turn.rs; compact_remote_v2.rs; context_manager` | context-window warning uses current token estimates |
| 1083 | `CONTEXT-05-03` | KEEP-CANDIDATE | `auto-compaction-threshold-redesign.md; compact-mode docs; memory diagnostics plans` | `ontocode-rs/core/src/session/turn.rs; compact_remote_v2.rs; context_manager` | manual compact retry has same-model loop guard |
| 1103 | `CONTEXT-06-03` | KEEP-CANDIDATE | `auto-compaction-threshold-redesign.md; compact-mode docs; memory diagnostics plans` | `ontocode-rs/core/src/session/turn.rs; compact_remote_v2.rs; context_manager` | image stripping respects model capability |
| 1123 | `CONTEXT-07-03` | KEEP-CANDIDATE | `auto-compaction-threshold-redesign.md; compact-mode docs; memory diagnostics plans` | `ontocode-rs/core/src/session/turn.rs; compact_remote_v2.rs; context_manager` | context fragments implement hard token caps |
| 1143 | `CONTEXT-08-03` | KEEP-CANDIDATE | `auto-compaction-threshold-redesign.md; compact-mode docs; memory diagnostics plans` | `ontocode-rs/core/src/session/turn.rs; compact_remote_v2.rs; context_manager` | additional context from hooks is recorded through bounded owner |
| 1163 | `CONTEXT-09-03` | KEEP-CANDIDATE | `auto-compaction-threshold-redesign.md; compact-mode docs; memory diagnostics plans` | `ontocode-rs/core/src/session/turn.rs; compact_remote_v2.rs; context_manager` | turn context does not rewrite prior history |
| 1183 | `CONTEXT-10-03` | KEEP-CANDIDATE | `auto-compaction-threshold-redesign.md; compact-mode docs; memory diagnostics plans` | `ontocode-rs/core/src/session/turn.rs; compact_remote_v2.rs; context_manager` | overflow errors produce actionable bounded diagnostics |
| 1203 | `EVIDENCE-01-03` | KEEP-CANDIDATE | `async-memory-recall and runtime diagnostics docs` | `ontocode-rs/state/src/runtime/operational_evidence*.rs; core/context operational evidence` | operational evidence import rejects raw artifact text |
| 1223 | `EVIDENCE-02-03` | KEEP-CANDIDATE | `async-memory-recall and runtime diagnostics docs` | `ontocode-rs/state/src/runtime/operational_evidence*.rs; core/context operational evidence` | operational evidence stores provenance not secrets |
| 1243 | `EVIDENCE-03-03` | KEEP-CANDIDATE | `async-memory-recall and runtime diagnostics docs` | `ontocode-rs/state/src/runtime/operational_evidence*.rs; core/context operational evidence` | evidence by provenance upsert is idempotent |
| 1263 | `EVIDENCE-04-03` | KEEP-CANDIDATE | `async-memory-recall and runtime diagnostics docs` | `ontocode-rs/state/src/runtime/operational_evidence*.rs; core/context operational evidence` | read evidence output is bounded before model context |
| 1283 | `EVIDENCE-05-03` | KEEP-CANDIDATE | `async-memory-recall and runtime diagnostics docs` | `ontocode-rs/state/src/runtime/operational_evidence*.rs; core/context operational evidence` | diagnostic artifact parser rejects malformed payloads |
| 1303 | `EVIDENCE-06-03` | KEEP-CANDIDATE | `async-memory-recall and runtime diagnostics docs` | `ontocode-rs/state/src/runtime/operational_evidence*.rs; core/context operational evidence` | evidence summaries cite artifact source paths |
| 1323 | `EVIDENCE-07-03` | KEEP-CANDIDATE | `async-memory-recall and runtime diagnostics docs` | `ontocode-rs/state/src/runtime/operational_evidence*.rs; core/context operational evidence` | evidence context fragment has stable size cap |
| 1343 | `EVIDENCE-08-03` | KEEP-CANDIDATE | `async-memory-recall and runtime diagnostics docs` | `ontocode-rs/state/src/runtime/operational_evidence*.rs; core/context operational evidence` | evidence import path handles missing files cleanly |
| 1363 | `EVIDENCE-09-03` | KEEP-CANDIDATE | `async-memory-recall and runtime diagnostics docs` | `ontocode-rs/state/src/runtime/operational_evidence*.rs; core/context operational evidence` | runtime diagnostics avoid token/cookie/header values |
| 1383 | `EVIDENCE-10-03` | KEEP-CANDIDATE | `async-memory-recall and runtime diagnostics docs` | `ontocode-rs/state/src/runtime/operational_evidence*.rs; core/context operational evidence` | memory recall remains read-only unless ADR-approved |
