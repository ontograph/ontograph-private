# Oh My Pi Donor Non-Keep Proposals

Status: moved from `OH_MY_PI_DONOR_200_SOLUTIONS_CHALLENGE.md`.

Date: 2026-06-16

Scope: `NARROW`, `DEFER`, and `REJECT` proposals from the Oh My Pi donor review. These are not accepted implementation work.

## Verdicts

- `NARROW`: useful only after reducing scope to the existing owner. Do not import the donor architecture.
- `DEFER`: potentially useful, but needs a separate ADR, public API decision, or existing owner gap.
- `REJECT`: not suitable for this codebase now; it creates a second stack or speculative runtime.

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

## Non-Keep Proposals

| ID | Verdict | Home | Suitability challenge |
|---:|---|---|---|
| 1 | NARROW | FS | Useful as a behavior matrix; do not turn read into a universal resource loader. |
| 4 | NARROW | FS | Only adopt if current multi-read semantics already need this edge case. |
| 5 | NARROW | FS | Useful, but selector support must reuse existing syntax/OntoIndex paths. |
| 7 | DEFER | FS | Only if archive reads already exist or become a real user workflow. |
| 8 | DEFER | FS | SQLite reads are a new resource class; needs owner approval. |
| 9 | DEFER | FS | URL/PDF conversion is a public behavior change. |
| 10 | NARROW | FS | Good error-shape idea if it matches existing tool result protocol. |
| 12 | DEFER | FS | Ranking fields may affect public/protocol output. |
| 14 | DEFER | FS | OntoIndex-enriched grouping is useful but cross-owner. |
| 18 | DEFER | FS | Virtual search schemes need read-resource approval first. |
| 19 | NARROW | FS | Add only if current telemetry path already has these counters. |
| 21 | NARROW | PATCH | Hash anchors are useful only as stale-edit checks, not a new patch language. |
| 24 | DEFER | PATCH | Preview staging changes tool behavior; needs UX/protocol decision. |
| 25 | DEFER | PATCH | Tree-sitter block edits need syntax-owner proof first. |
| 28 | DEFER | PATCH | Warning channels may change tool output contract. |
| 29 | DEFER | PATCH | Matched-span metadata may be protocol-visible. |
| 42 | DEFER | IDE | `willRenameFiles` is only useful if current LSP owner supports it. |
| 44 | NARROW | IDE | Good only if diagnostics already flow after writes. |
| 51 | DEFER | DBG | Debugger integration has no current owner. |
| 52 | DEFER | DBG | Keep as future ADR requirement only. |
| 53 | DEFER | DBG | Redaction rule is valid, but debugger scope is not approved. |
| 54 | DEFER | DBG | Cap rule is valid, but debugger scope is not approved. |
| 55 | DEFER | DBG | Timeout/cancel belongs in a future debugger ADR. |
| 56 | REJECT | DBG | Crash-triage workflow docs are speculative here. |
| 58 | DEFER | DBG | Transcript fixtures only matter after debugger approval. |
| 59 | REJECT | DBG | Debugger fallback docs add noise without debugger support. |
| 61 | DEFER | SHELL | Persistent shells are a runtime design change. |
| 65 | NARROW | SHELL | Artifact spill is useful only through current output reducer. |
| 71 | NARROW | NB | Only notebook-as-text; no execution. |
| 73 | DEFER | NB | Cell marker format needs explicit notebook edit design. |
| 74 | DEFER | NB | Round-trip tests need approved notebook edit support. |
| 75 | REJECT | NB | Persistent Python/Bun worker is a second runtime. |
| 77 | DEFER | NB | Permission profile matters only after execution ADR. |
| 78 | DEFER | NB | Output spill belongs to execution ADR. |
| 79 | DEFER | NB | Kernel cancel semantics need runtime approval. |
| 80 | DEFER | TUI | Notebook diff UI is not current scope. |
| 84 | DEFER | CTX | Injected rules are not approved; keep bounded-fragment rule. |
| 87 | NARROW | CTX | Add only to existing telemetry. |
| 95 | NARROW | MEM | Age policy only if existing memory lifecycle supports it. |
| 100 | NARROW | MEM | Drift checker should be a small script/check, not a service. |
| 101 | DEFER | PROV | Stream abort/retry rules are a streaming architecture change. |
| 102 | NARROW | HOOK | Regex lint is useful only in existing hook/prompt config. |
| 105 | DEFER | HOOK | AST-grep rule matching needs syntax-owner approval. |
| 107 | NARROW | HOOK | Diagnostics source ids are useful if current diagnostics support them. |
| 108 | DEFER | PROV | Retry budgets matter only after stream-abort approval. |
| 109 | DEFER | PROV | Loop guard belongs to deferred stream-rule design. |
| 115 | NARROW | HOOK | Only if hooks already request approval. |
| 123 | NARROW | MCP | Synthetic config errors must use existing status path. |
| 129 | NARROW | MCP | Provenance only if current metadata supports it. |
| 134 | DEFER | EXT | `skill://` creates a resource scheme. |
| 136 | DEFER | EXT | Marketplace validation only if marketplace owner is active. |
| 138 | DEFER | EXT | Install/reload lifecycle needs proof of current gap. |
| 140 | NARROW | EXT | Provenance only through existing tool metadata. |
| 142 | DEFER | AGENT | Isolated worktrees are a filesystem/session design change. |
| 143 | DEFER | AGENT | `agent://` creates a resource scheme. |
| 147 | NARROW | AGENT | Cost/duration display only if current progress model has fields. |
| 149 | REJECT | AGENT | Peer coordination is speculative and adds agent complexity. |
| 152 | DEFER | AGENT | Reviewer subagents depend on typed-result maturity. |
| 153 | NARROW | GIT | Use existing read/search surfaces; no git filesystem. |
| 154 | DEFER | GIT | Atomic commit splitting is a workflow feature. |
| 157 | DEFER | GIT | `pr://` is a new resource scheme. |
| 158 | DEFER | GIT | `issue://` is a new resource scheme. |
| 159 | DEFER | GIT | `conflict://` is a new resource scheme. |
| 160 | DEFER | GIT | Conflict writes need separate owner/design. |
| 165 | NARROW | PROV | Prompt snapshots only in existing prompt/protocol owner. |
| 169 | NARROW | PROV | Telemetry only through current telemetry path. |
| 174 | NARROW | TUI | Job status rendering only if current job state already exists. |
| 177 | NARROW | TUI | Theme sync only if current theme generation exists. |
| 178 | NARROW | TUI | Keybinding drift only if keymaps are generated/configured. |
| 186 | NARROW | SEC | Clipboard/copy guard only if current command policy can express it. |
| 189 | DEFER | SEC | URL reads are not approved; keep as future guard. |
| 193 | NARROW | AUTO | Tool IO normalization should stay a tiny script/fixture helper. |
| 194 | NARROW | AUTO | Prompt rewrite checker only for generated prompt artifacts. |

## Rejected Center Of Gravity

Do not let this donor review become a plan for DAP, browser control, notebook execution, persistent language workers, virtual URL schemes, peer agent coordination, or a second memory/tool/runtime stack.
