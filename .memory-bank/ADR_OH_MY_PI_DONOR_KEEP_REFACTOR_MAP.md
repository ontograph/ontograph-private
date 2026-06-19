# ADR: Oh My Pi Donor Keep Refactor Map

Status: proposed planning ADR

Date: 2026-06-16

Source: `OH_MY_PI_DONOR_200_SOLUTIONS_CHALLENGE.md`

Decision: keep the accepted Oh My Pi donor ideas only as targeted tests, fixtures, small owner-local refactors, and offline scripts. Do not add a second read/search/tool/runtime/provider/MCP/hook/context/memory/agent stack.

OntoIndex basis: repo `codex` was indexed on 2026-06-16. Checks covered file/search/edit/apply-patch, context/compaction, MCP, hooks, skills/plugins, agents/jobs, provider streaming, TUI/session, security/redaction, and release/eval automation. Hot files found by OntoIndex, such as `ontocode-rs/hooks/src/engine/discovery.rs`, `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`, `ontocode-rs/codex-mcp/src/connection_manager.rs`, and `ontocode-rs/tui/src/app/session_lifecycle.rs`, should not grow casually; prefer sibling tests or extracted helper modules when implementation is needed.

## Challenge Review

- Accept this ADR only as a refactor/test map, not as a 127-item work order.
- Current path check passed for the listed owner files; `ontocode-rs/core/src/context/` is the context owner.
- Hot-file guard: OntoIndex reports `hooks/src/engine/discovery.rs` at 1087 LOC, `codex-mcp/src/connection_manager.rs` at 823 LOC, `tui/src/app/session_lifecycle.rs` at 831 LOC, and `core/src/tools/handlers/agent_jobs.rs` at 763 LOC. Add sibling tests or small helper modules; do not grow these files unless the edit is trivial.
- Bundle duplicate security guards instead of implementing them one-by-one: 57, 60, 72, 76, and 188 are one negative-capability policy bundle.
- Bundle MCP/OAuth redaction rows 127 and 187 into one redaction test surface.
- Treat memory rows 91-99 as memory-bank and `memories` constraints unless a real implementation gap is found.
- Treat automation rows 191-200 as offline scripts/checklists only. They are not runtime work.
- Row 128 stays conditional: add MCP reconnect/backoff tests only if the current MCP manager already exposes retry/backoff state. Do not copy the app-server websocket backoff pattern into MCP just to satisfy this donor row.
- This ADR owns the duplicate donor-hardening work removed from `GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_PRE_JUNIOR_PROJECT_PLAN.md`: compaction failure/cap tests, model-visible tool error tests, and deterministic response fixture helpers.

## Owner Map

- `FS`: `ontocode-rs/exec-server/src/environment_path.rs`, `ontocode-rs/file-system/`, `ontocode-rs/core/src/tools/`, `ontocode-rs/core/tests/suite/search_tool.rs`.
- `PATCH`: `ontocode-rs/apply-patch/`, `ontocode-rs/core/tests/suite/apply_patch_cli.rs`, `ontocode-rs/core/tests/suite/shell_serialization.rs`, `ontocode-rs/core/tests/common/responses.rs`.
- `IDE`: `ontocode-rs/tui/src/ide_context.rs`, `ontocode-rs/tui/src/ide_context/`, `ontocode-rs/tui/src/chatwidget/ide_context.rs`, existing app-server editor/thread context.
- `SHELL`: `ontocode-rs/core/src/shell.rs`, `ontocode-rs/exec-server/`, `ontocode-rs/exec/tests/suite/approval_policy.rs`, shell-related core suite tests.
- `CTX`: `ontocode-rs/core/src/session/turn.rs`, `ontocode-rs/core/src/compact.rs`, `ontocode-rs/core/src/session/turn_context.rs`, `ontocode-rs/core/src/context_manager/`, `ontocode-rs/core/src/context/`, `ontocode-rs/prompts/src/compact.rs`.
- `MEM`: `.memory-bank/`, `ontocode-rs/memories/`, `ontocode-rs/memories/write/src/prompts.rs`, `ontocode-rs/memories/write/src/prompts_tests.rs`, existing external-agent detector ADRs.
- `HOOK`: `ontocode-rs/hooks/`, especially tests/sibling modules around `ontocode-rs/hooks/src/engine/discovery.rs`, plus `ontocode-rs/core/src/hook_runtime.rs`.
- `MCP`: `ontocode-rs/codex-mcp/src/connection_manager.rs`, `ontocode-rs/rmcp-client/`, `ontocode-rs/mcp-server/`, `ontocode-rs/core/src/mcp_tool_call_tests.rs`.
- `EXT`: `ontocode-rs/core-skills/`, `ontocode-rs/core-plugins/`, `ontocode-rs/ext/extension-api/`, plugin metadata analytics.
- `AGENT`: `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`, `ontocode-rs/core/src/tools/handlers/multi_agents*`, `ontocode-rs/core/src/agent/`, `ontocode-rs/core/src/thread_manager.rs`, `ontocode-rs/state/src/runtime/agent_jobs.rs`.
- `GIT`: existing git/session test harnesses only. No git virtual filesystem.
- `PROV`: `ontocode-rs/model-provider/`, `ontocode-rs/model-provider-info/`, `ontocode-rs/codex-client/`, `ontocode-rs/codex-api/`, `ontocode-rs/protocol/`, `ontocode-rs/prompts/`.
- `TUI`: `ontocode-rs/tui/src/app/session_lifecycle.rs`, `ontocode-rs/tui/src/chatwidget/`, `ontocode-rs/tui/src/bottom_pane/`, app-server thread/session processors.
- `SEC`: `ontocode-rs/provider-auth/`, `ontocode-rs/login/`, `ontocode-rs/prompts/src/permissions_instructions_tests.rs`, `ontocode-rs/exec/tests/suite/approval_policy.rs`, shared diagnostics/redaction paths.
- `AUTO`: `scripts/`, `.github/workflows/`, `sdk/python/tests/`, `ontocode-rs/core/tests/common/`, eval/release fixtures.

## Per-Proposal Refactor Map

| ID | Best place | Suitable/useful check |
|---:|---|---|
| 2 | `FS`: add bounded directory/read output fixtures near `file-system` or core read/search tests. | Suitable as a cap test only; do not add a new read abstraction. |
| 3 | `FS`: read/search tests that assert deterministic multi-result ordering. | Useful because stable model-visible output reduces churn. |
| 6 | `FS`: directory output fixture in existing file-system/core tool tests. | Suitable if it stays a fixture; no new directory renderer stack. |
| 11 | `FS`: document/test current search fallback behavior near `search_tool.rs`. | Useful as drift protection for existing native search. |
| 13 | `FS`: search snippet cap tests in `search_tool.rs` or existing formatter tests. | Suitable and useful; protects model context. |
| 15 | `FS`: `search_tool.rs` gitignore fixtures. | Suitable; cheap regression coverage. |
| 16 | `FS`: `search_tool.rs` case sensitivity fixtures. | Suitable; low-risk. |
| 17 | `FS`: `search_tool.rs` glob include/exclude fixtures. | Suitable; low-risk. |
| 20 | `AUTO`: offline eval under `scripts/` or existing eval fixtures. | Useful only as eval evidence, not runtime behavior. |
| 22 | `AUTO`: benchmark notes/scripts comparing hash-anchor ideas to current patch flow. | Suitable as benchmark only. |
| 23 | `PATCH`: `apply_patch_cli.rs`/`shell_serialization.rs` stale-edit failure wording tests. | Useful; improves model recovery without new patch format. |
| 26 | `PATCH`: `apply-patch` parser tests and CLI harness. | Suitable; ambiguity rejection is owner-local. |
| 27 | `PATCH`: `apply-patch` parser malformed-input tests. | Suitable; direct hardening. |
| 30 | `AUTO`: small edit eval under `scripts/` or core fixtures. | Useful as wrong-file guard, not a runtime feature. |
| 31 | `PATCH`: `shell_serialization.rs` custom apply_patch call tests. | Suitable; exact existing home. |
| 32 | `PATCH`: `apply_patch_cli.rs` failure matrix. | Useful; strengthens current owner. |
| 33 | `PATCH`: `core/tests/common/responses.rs` helpers plus patch tests. | Suitable if it reuses existing structured helpers. |
| 34 | `PATCH`: `apply_patch_cli.rs` move overwrite rejection. | Suitable; destructive edge guard. |
| 35 | `PATCH`: `apply_patch_cli.rs` move/new-dir behavior tests. | Suitable; no new tool surface. |
| 36 | `PATCH`: `shell_serialization.rs` failure output cap tests. | Useful for context safety. |
| 37 | `PATCH`: keep one harness in `apply_patch_cli.rs`. | Suitable as a constraint; prevents duplicate harnesses. |
| 38 | `PATCH`: parse-failure no-write test in `apply_patch_cli.rs`. | Essential and useful. |
| 39 | `PATCH`: large patch output truncation tests. | Useful; prevents context flooding. |
| 40 | `PATCH`: changed-file verification in patch suite. | Suitable; narrow validation. |
| 41 | `IDE`: current TUI/app-server IDE context tests, plus OntoIndex impact gate docs. | Suitable only as preflight evidence before rename automation. |
| 43 | `IDE`: references-before-edit test/eval using existing LSP/OntoIndex context. | Useful for risk evidence. |
| 45 | `IDE`: parity tests between `tui/src/ide_context*` and app-server editor context. | Suitable; avoids duplicated IDE state. |
| 46 | `IDE`: editor selection empty/oversize tests. | Useful and cheap. |
| 47 | `IDE`: large selection truncation tests in TUI/app-server IDE context. | Suitable; protects context caps. |
| 48 | `IDE`: existing tool error conversion path. | Useful only if it normalizes through current errors. |
| 49 | `IDE`: explicit LSP-to-search fallback test. | Suitable; no new fallback owner. |
| 50 | `IDE`: OntoIndex impact gate docs/tests for rename workflows. | Useful; required before symbol edits. |
| 57 | `SEC`: negative attach policy test/docs. | Suitable as guard; no debugger implementation. |
| 60 | `SEC`: disabled-by-default debugger/browser policy note. | Useful as negative policy only. |
| 62 | `SHELL`: permission/approval fixtures in exec/core shell tests. | Suitable; validates existing command risk logic. |
| 63 | `SHELL`: approval prompt tests in permission UI/prompt owners. | Useful; keeps reason visible. |
| 64 | `SHELL`: missing-CWD tests in shell/exec-server suite. | Suitable and cheap. |
| 66 | `SEC`: diagnostics/redaction tests. | Useful; prevents path leakage. |
| 67 | `AUTO`: failed-command loop eval under `scripts/`. | Suitable as offline eval. |
| 68 | `SHELL`: timeout/cancel tests in shell runtime suite. | Useful for existing command runtime. |
| 69 | `FS`: docs/test rule preferring native file/search helpers over shell wrappers. | Suitable as contributor guidance. |
| 70 | `SEC`: environment redaction fixture in exec/security tests. | Useful; protects secrets. |
| 72 | `SEC`: negative notebook-execution guard in policy docs/tests. | Suitable; no notebook runtime. |
| 76 | `SEC`: code-exec tool-call negative guard. | Useful only as policy/test. |
| 81 | `CTX`: overflow retry tests around `session/turn.rs`. | Suitable; current owner. |
| 82 | `CTX`: compaction/session overflow fixtures. | Useful; direct owner coverage. |
| 83 | `CTX`: same-model retry loop guard tests. | Suitable; prevents runaway behavior. |
| 85 | `CTX`: compaction request shape snapshots. | Useful; protects prompt cache stability. |
| 86 | `CTX`: bounded compaction failure event tests. | Suitable; no new event pipeline. |
| 88 | `CTX`: hard cap tests for context items. | Required by project rules; useful. |
| 89 | `CTX`: prompt cache stability tests. | Suitable; high value for repeated turns. |
| 90 | `CTX`: reinjection-after-summary tests. | Useful; current compaction owner. |
| 91 | `MEM`: `.memory-bank/` and `memories` owner notes. | Suitable; explicitly rejects new memory service. |
| 92 | `MEM`: memory scope rules in memory-bank/memories tests. | Useful; project-scoped default. |
| 93 | `MEM`: ADR text retaining automatic extraction as deferred. | Suitable as guardrail. |
| 94 | `MEM`/`SEC`: memory redaction tests. | Useful and necessary. |
| 96 | `MEM`: bounded synopsis tests in memories/write. | Suitable; context safety. |
| 97 | `MEM`: citation requirement tests/docs. | Useful; keeps memory auditable. |
| 98 | `MEM`: raw-log rejection tests/docs. | Suitable; prevents noisy memory. |
| 99 | `MEM`: external-agent detector ADR route. | Useful; prevents new importer stack. |
| 103 | `HOOK`: hook/rule config parser tests. | Suitable; warn-only invalid regex behavior. |
| 104 | `HOOK`: glob-gated selector tests. | Useful; fits config selectors. |
| 106 | `CTX`: bounded context fragment guard. | Suitable only if rule text later enters context. |
| 110 | `SEC`: redaction tests for rule/hook diagnostics. | Useful. |
| 111 | `HOOK`: sibling tests around hook discovery. | Suitable; avoid growing `discovery.rs`. |
| 112 | `HOOK`: startup fixture with one good and one bad hook. | Useful. |
| 113 | `HOOK`: hook schema fixture tests. | Suitable. |
| 114 | `HOOK`: hook timeout tests. | Useful; current runtime concern. |
| 116 | `SEC`/`HOOK`: hook secret redaction tests. | Useful and necessary. |
| 117 | `HOOK`: deterministic hook order tests. | Suitable. |
| 118 | `HOOK`: hook stdout/stderr cap tests. | Useful for context safety. |
| 119 | `HOOK`: trust prompt tests. | Suitable; current hook trust model. |
| 120 | `HOOK`: config layer merge tests. | Useful; current config owner. |
| 121 | `MCP`: `codex-mcp` connection lifecycle tests. | Strongly suitable; first-slice item. |
| 122 | `MCP`: partial server failure fixtures. | Useful and realistic. |
| 124 | `MCP`: current connection manager/error status mapping. | Suitable if it stays in existing MCP owner. |
| 125 | `MCP`: fixture MCP servers for resources. | Useful; test-only. |
| 126 | `MCP`: prompt/tool name conflict tests. | Suitable; protects tool surface. |
| 127 | `SEC`/`MCP`: OAuth/token redaction tests. | Required; useful. |
| 128 | `MCP`: reconnect/backoff status tests. | Conditional: suitable only if current MCP manager exposes retry/backoff state. |
| 130 | `MCP`: tool schema cycle guard tests. | Useful; protects model/tool schema path. |
| 131 | `EXT`: extension/plugin detector tests. | Suitable; no new loader. |
| 132 | `EXT`: `core-plugins` cache invalidation tests. | Useful; current plugin manager owner. |
| 133 | `EXT`: per-path plugin/extension load error tests. | Suitable; keep per-path, no global abort. |
| 135 | `EXT`: `core-skills` manifest fixtures. | Useful. |
| 137 | `EXT`: declared permission tests. | Suitable; avoids implicit extension powers. |
| 139 | `EXT`: bundled skill enable/disable tests. | Useful; current skills owner. |
| 141 | `AGENT`: multi-agent typed result tests. | Suitable. |
| 144 | `AGENT`: subagent progress snapshots. | Useful; current status paths. |
| 145 | `AGENT`: `agent_jobs.rs` job aggregation tests or extracted helper tests. | Suitable; avoid growing the already-large file. |
| 146 | `AGENT`: `state/src/runtime/agent_jobs.rs` cancel semantics tests. | Useful; current state owner. |
| 148 | `AGENT`: strict structured-result parsing tests. | Suitable; no new agent protocol. |
| 150 | `AGENT`: orphan cleanup tests around agent runtime/state. | Useful; current cancellation owner. |
| 151 | `AUTO`: review verdict fixtures in prompt/eval tests. | Suitable as fixture only. |
| 155 | `GIT`: existing commit/session test harness. | Useful; no git VFS. |
| 156 | `GIT`: commit heuristic tests. | Suitable; lockfile guard only. |
| 161 | `PROV`: provider streaming terminal mapping tests. | Useful; current provider/client owner. |
| 162 | `PROV`: failed/canceled/error stream fixtures. | Suitable. |
| 163 | `PROV`: stop reason normalization tests. | Useful; provider owner. |
| 164 | `PROV`: usage extraction fixtures. | Suitable; no new telemetry path. |
| 166 | `AUTO`: prompt adjustment eval before adding prompt branches. | Useful as gate. |
| 167 | `PROV`: protocol/tool-conversion channel routing tests. | Suitable. |
| 168 | `PROV`: oversized tool schema compaction tests. | Useful; protects tool prompt size. |
| 170 | `PROV`: provider-unavailable fallback tests. | Suitable. |
| 171 | `TUI`: session listing snapshots. | Useful; current TUI/session owner. |
| 172 | `TUI`: fork/resume behavior tests in session lifecycle owners. | Suitable; avoid growing `session_lifecycle.rs` logic. |
| 173 | `TUI`: handoff guard tests. | Useful and small. |
| 175 | `PATCH`/`TUI`: codemod preview/accept guideline and tests where existing preview exists. | Suitable as principle; no new preview stack. |
| 176 | `TUI`: approval prompt snapshots. | Useful; current UI surface. |
| 179 | `TUI`: transcript snapshots. | Suitable. |
| 180 | `TUI`: install/version diagnostics in current diagnostics/version owners. | Useful. |
| 181 | `SEC`: central redaction checklist in ADR/tests. | Suitable and useful. |
| 182 | `SEC`: approval mode matrix tests. | Useful. |
| 183 | `SEC`: critical-command reason tests. | Suitable; permission surface. |
| 184 | `SEC`: remote-fetch-execute risk fixture. | Useful. |
| 185 | `SEC`: denial fixtures for host/credential risk. | Suitable. |
| 187 | `SEC`/`MCP`: MCP/OAuth redaction tests. | Required; useful. |
| 188 | `SEC`: browser-control negative guard. | Suitable; no browser runtime. |
| 190 | `SEC`: artifact path redaction tests. | Useful. |
| 191 | `AUTO`: edit benchmark script/fixture. | Suitable as offline tool. |
| 192 | `AUTO`: small-edit analyzer script. | Useful; no runtime code. |
| 195 | `AUTO`: release notes generator tests. | Suitable; release automation only. |
| 196 | `AUTO`: version spoof tests, especially SDK/release fixtures. | Useful. |
| 197 | `AUTO`: CI concurrency checks. | Suitable. |
| 198 | `AUTO`: native build/signing checklist. | Useful as docs/checklist only. |
| 199 | `AUTO`: session stats analyzer script. | Suitable as offline script. |
| 200 | `AUTO`: deflake/retry report script. | Useful as offline CI/eval helper. |

## First Slice

1. MCP lifecycle hardening: 121, 122, 124, 127, 130. Row 128 is conditional.
2. Apply-patch safety: 23, 26, 27, 31-40.
3. Context cap and compaction safety: 81-83, 85, 86, 88-90.
4. Hook load, trust, and redaction: 103, 104, 111-120.
5. Agent job cleanup and structured results: 141, 144-146, 148, 150.

## Non-Goals

- No DAP/browser/notebook execution runtime.
- No `pr://`, `issue://`, `agent://`, `skill://`, or `conflict://` resource scheme.
- No persistent Python/Bun worker.
- No second MCP lifecycle, hook matcher, provider registry, memory service, tool runtime, or shell runtime.
- No public API/config/schema change from this ADR alone.

## Verification Before Implementation

- Before editing any Rust symbol, run OntoIndex impact on that exact symbol.
- Add one focused test or eval for each implemented row.
- If implementation touches config schemas, app-server API, SDK behavior, or user-visible TUI, follow the repository-specific schema/snapshot rules.
- For hot files listed in Challenge Review, prefer adding or extending sibling test files first.
