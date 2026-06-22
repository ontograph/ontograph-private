# Oh My Pi Donor 200 Solutions Challenge

Status: accepted `KEEP` subset from `OH_MY_PI_DONOR_200_SOLUTIONS_REVIEW.md`. Non-keep proposals moved to `OH_MY_PI_DONOR_200_SOLUTIONS_NON_KEEP.md`.

Date: 2026-06-16

Method: OntoIndex repo `codex` was fresh on 2026-06-16. Queries checked current homes for file/search/edit/apply-patch, context/compaction/memory, MCP/hooks/plugins/skills, agents/jobs, provider streaming, TUI/session, security/redaction, and release/eval automation.

## Verdict

`KEEP`: useful now as a focused test, fixture, doc, or small refactor in an existing owner. This file keeps only those proposals.

## Current Homes

- `FS`: `ontocode-rs/file-system/`, `ontocode-rs/exec-server/`, `ontocode-rs/core/src/tools/`, `ontocode-rs/core/tests/suite/search_tool.rs`.
- `PATCH`: `ontocode-rs/apply-patch/`, `ontocode-rs/core/tests/suite/apply_patch_cli.rs`, `ontocode-rs/core/tests/suite/shell_serialization.rs`.
- `IDE`: existing TUI/app-server IDE context and LSP-facing helpers.
- `SHELL`: `ontocode-rs/core/src/shell.rs`, `ontocode-rs/exec-server/`, permission and hook test suites.
- `NB`: no execution home; only file/notebook-as-text handling is acceptable.
- `CTX`: `ontocode-rs/core/src/session/turn.rs`, `ontocode-rs/core/src/compact.rs`, `ontocode-rs/core/src/context_manager/`, `ontocode-rs/core/src/context/`.
- `MEM`: `.memory-bank/`, `ontocode-rs/memories/`, existing external-agent detector ADRs.
- `HOOK`: `ontocode-rs/hooks/`, `ontocode-rs/core/src/hook_runtime.rs`, hook config and hook tests.
- `MCP`: `ontocode-rs/codex-mcp/`, `ontocode-rs/rmcp-client/`, `ontocode-rs/mcp-server/`, `ontocode-rs/core/src/mcp_tool_call_tests.rs`.
- `EXT`: `ontocode-rs/core-skills/`, `ontocode-rs/core-plugins/`, `ontocode-rs/ext/extension-api/`, plugin metadata paths.
- `AGENT`: `ontocode-rs/core/src/tools/handlers/multi_agents*`, `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`, `ontocode-rs/core/src/agent/`, `ontocode-rs/core/src/thread_manager.rs`, `ontocode-rs/state/src/runtime/agent_jobs.rs`.
- `GIT`: current git/session/test harnesses only; no new git virtual filesystem.
- `PROV`: `ontocode-rs/model-provider/`, `ontocode-rs/model-provider-info/`, `ontocode-rs/codex-client/`, `ontocode-rs/codex-api/`, `ontocode-rs/protocol/`, prompts/tool conversion tests.
- `TUI`: `ontocode-rs/tui/src/app/session_lifecycle.rs`, `ontocode-rs/tui/src/chatwidget/`, app-server thread/session processors.
- `SEC`: `ontocode-rs/provider-auth/`, `ontocode-rs/login/`, shared redaction/diagnostics, protocol permission profiles.
- `AUTO`: `scripts/`, `.github/workflows/`, `sdk/python/tests/`, `ontocode-rs/core/tests/common/`, eval/release fixtures.

## Kept Proposals

| ID | Verdict | Home | Suitability challenge |
|---:|---|---|---|
| 2 | KEEP | FS | Good bounded-output test; fits current context caps. |
| 3 | KEEP | FS | Stable output ordering is useful and cheap. |
| 6 | KEEP | FS | Good directory output fixture. |
| 11 | KEEP | FS | Documentation/test of existing fallback is useful. |
| 13 | KEEP | FS | Snippet caps directly protect model context. |
| 15 | KEEP | FS | Gitignore behavior should be locked by tests. |
| 16 | KEEP | FS | Low-cost search fixtures. |
| 17 | KEEP | FS | Low-cost glob fixtures. |
| 20 | KEEP | AUTO | Useful eval; no runtime feature required. |
| 22 | KEEP | AUTO | Use as benchmark only. |
| 23 | KEEP | PATCH | Clear stale-edit recovery is useful. |
| 26 | KEEP | PATCH | Good fallback and ambiguity coverage. |
| 27 | KEEP | PATCH | Parser hardening belongs in apply-patch tests. |
| 30 | KEEP | AUTO | Wrong-file edit eval is high value. |
| 31 | KEEP | PATCH | Existing custom tool-call tests are the right home. |
| 32 | KEEP | PATCH | Failure matrix strengthens current owner. |
| 33 | KEEP | PATCH | Structured assertions already match test harness style. |
| 34 | KEEP | PATCH | Good destructive edge-case guard. |
| 35 | KEEP | PATCH | Good move behavior guard. |
| 36 | KEEP | PATCH | Bounded failure output is context safety. |
| 37 | KEEP | PATCH | Correct: one harness, no duplicate test stack. |
| 38 | KEEP | PATCH | No-write-on-parse-failure is essential. |
| 39 | KEEP | PATCH | Large output truncation belongs here. |
| 40 | KEEP | PATCH | Changed-file verification is useful and narrow. |
| 41 | KEEP | IDE | Useful preflight test before any rename automation. |
| 43 | KEEP | IDE | References-before-edit is useful risk evidence. |
| 45 | KEEP | IDE | TUI/app-server editor parity is valuable. |
| 46 | KEEP | IDE | Selection edge cases are cheap and useful. |
| 47 | KEEP | IDE | Large-selection truncation protects context. |
| 48 | KEEP | IDE | Normalize through existing tool errors. |
| 49 | KEEP | IDE | Explicit fallback is useful. |
| 50 | KEEP | IDE | OntoIndex blast-radius gate is suitable for rename workflows. |
| 57 | KEEP | SEC | Keep as a negative guard: no auto-attach. |
| 60 | KEEP | SEC | Good negative policy: disabled by default. |
| 62 | KEEP | SHELL | Risk classifier fixtures are useful. |
| 63 | KEEP | SHELL | Approval reasons fit existing permission UI/tests. |
| 64 | KEEP | SHELL | Missing-CWD tests are cheap. |
| 66 | KEEP | SEC | Path redaction belongs in diagnostics/redaction tests. |
| 67 | KEEP | AUTO | Failed-command loop eval is useful. |
| 68 | KEEP | SHELL | Timeout/cancel tests fit current command runtime. |
| 69 | KEEP | FS | Good rule: prefer native helpers over shell wrappers. |
| 70 | KEEP | SEC | Environment redaction fixture is useful. |
| 72 | KEEP | SEC | Negative guard is useful. |
| 76 | KEEP | SEC | Good negative guard. |
| 81 | KEEP | CTX | Overflow retry tests fit session turn owner. |
| 82 | KEEP | CTX | Directly targets current compaction/session path. |
| 83 | KEEP | CTX | Loop guard is useful. |
| 85 | KEEP | CTX | Snapshotting request shape prevents cache churn. |
| 86 | KEEP | CTX | Bounded failure events are suitable. |
| 88 | KEEP | CTX | Hard cap tests are required by project rules. |
| 89 | KEEP | CTX | Prompt cache stability is useful. |
| 90 | KEEP | CTX | Reinjection tests fit compaction behavior. |
| 91 | KEEP | MEM | Correct: use memory-bank/current memory owners. |
| 92 | KEEP | MEM | Project scope is the right default. |
| 93 | KEEP | MEM | Correct challenge: defer automatic extraction. |
| 94 | KEEP | MEM | Redaction before memory write is essential. |
| 96 | KEEP | MEM | Bounded synopsis is context safety. |
| 97 | KEEP | MEM | Citation requirement is useful. |
| 98 | KEEP | MEM | Prevent raw-log memory dumps. |
| 99 | KEEP | MEM | Correct: reuse detector ADRs. |
| 103 | KEEP | HOOK | Invalid regex should warn, not break startup. |
| 104 | KEEP | HOOK | Glob-gated rule tests fit existing config selectors. |
| 106 | KEEP | CTX | Good bounded-context guard if rules ever exist. |
| 110 | KEEP | SEC | Redaction test is useful. |
| 111 | KEEP | HOOK | Partial load errors fit hook discovery. |
| 112 | KEEP | HOOK | Good startup fixture. |
| 113 | KEEP | HOOK | Schema fixtures fit existing hook tests. |
| 114 | KEEP | HOOK | Timeout tests are useful. |
| 116 | KEEP | SEC | Hook redaction tests are necessary. |
| 117 | KEEP | HOOK | Deterministic order is useful. |
| 118 | KEEP | HOOK | Output caps protect context. |
| 119 | KEEP | HOOK | Trust prompt tests fit current hook trust model. |
| 120 | KEEP | HOOK | Config layer merge tests fit hook config. |
| 121 | KEEP | MCP | Strong first-slice candidate. |
| 122 | KEEP | MCP | Partial server failure is useful and realistic. |
| 124 | KEEP | MCP | Unified error map belongs in current MCP owner. |
| 125 | KEEP | MCP | Fixture servers are the right test shape. |
| 126 | KEEP | MCP | Prompt/tool conflicts need coverage. |
| 127 | KEEP | SEC | OAuth/token redaction is required. |
| 128 | KEEP | MCP | Reconnect/backoff status tests are useful. |
| 130 | KEEP | MCP | Schema cycle guards are useful. |
| 131 | KEEP | EXT | Detector tests fit extension/plugin owners. |
| 132 | KEEP | EXT | Cache invalidation is current plugin-owner work. |
| 133 | KEEP | EXT | Per-path load errors are useful. |
| 135 | KEEP | EXT | Skill manifest fixtures are useful. |
| 137 | KEEP | EXT | Declared permission tests are useful. |
| 139 | KEEP | EXT | Enable/disable tests fit current skills owner. |
| 141 | KEEP | AGENT | Typed result tests are useful. |
| 144 | KEEP | AGENT | Progress snapshots fit current agent status paths. |
| 145 | KEEP | AGENT | Job aggregation maps to `agent_jobs.rs`. |
| 146 | KEEP | AGENT | Cancel semantics fit state/runtime owner. |
| 148 | KEEP | AGENT | Strict structured parsing is useful. |
| 150 | KEEP | AGENT | Orphan cleanup fits current cancellation/runtime owner. |
| 151 | KEEP | AUTO | Review verdict fixtures are useful. |
| 155 | KEEP | GIT | Invalid commit state tests are useful. |
| 156 | KEEP | GIT | Lockfile heuristic guard is useful. |
| 161 | KEEP | PROV | Streaming terminal mapping tests are useful. |
| 162 | KEEP | PROV | Failed/canceled/error fixtures are useful. |
| 163 | KEEP | PROV | Stop reason normalization fits provider owner. |
| 164 | KEEP | PROV | Usage extraction fixtures are useful. |
| 166 | KEEP | AUTO | Prompt adjustment evals are useful before adding branches. |
| 167 | KEEP | PROV | Tool channel routing tests fit protocol/tool conversion. |
| 168 | KEEP | PROV | Oversized schema compaction is useful. |
| 170 | KEEP | PROV | Provider unavailable fallback tests are useful. |
| 171 | KEEP | TUI | Session listing snapshots fit TUI/session owner. |
| 172 | KEEP | TUI | Fork/resume behavior fits session owners. |
| 173 | KEEP | TUI | Handoff guard is small and useful. |
| 175 | KEEP | PATCH | Preview/accept principle is useful for codemods. |
| 176 | KEEP | TUI | Approval prompt snapshots are useful. |
| 179 | KEEP | TUI | Transcript snapshots are useful. |
| 180 | KEEP | TUI | Install/version diagnostics fit existing diagnostics. |
| 181 | KEEP | SEC | Redaction checklist is useful. |
| 182 | KEEP | SEC | Approval matrix tests are useful. |
| 183 | KEEP | SEC | Critical-command reason tests fit permission surface. |
| 184 | KEEP | SEC | Remote-fetch-execute fixture is useful. |
| 185 | KEEP | SEC | Denial fixtures are useful. |
| 187 | KEEP | SEC | MCP/OAuth redaction tests are necessary. |
| 188 | KEEP | SEC | Negative browser-control guard is useful. |
| 190 | KEEP | SEC | Artifact path redaction tests are useful. |
| 191 | KEEP | AUTO | Edit benchmark is useful and non-runtime. |
| 192 | KEEP | AUTO | Small-edit analyzer is useful. |
| 195 | KEEP | AUTO | Release notes generator tests fit release automation. |
| 196 | KEEP | AUTO | Version spoof checks fit release automation. |
| 197 | KEEP | AUTO | CI concurrency checks are useful. |
| 198 | KEEP | AUTO | Checklist-only native build/signing is the right scope. |
| 199 | KEEP | AUTO | Session stats analysis is useful as an offline script. |
| 200 | KEEP | AUTO | Deflake/retry report is useful as an offline script. |

## First Slice

1. MCP partial-failure tests: items 121, 122, 124, and 127.
2. Apply-patch safety tests: items 23, 27, and 30-40.
3. Context caps and overflow tests: items 81-83 and 85-90.
4. Hook partial-load/redaction tests: items 111-114 and 116-120.
5. Agent job cleanup tests: items 141, 145, 146, 148, and 150.

This keeps the useful donor surface as tests, caps, redaction, and lifecycle hardening in existing owners.
