# ADR: Crush-Inspired Tool Extensions Review

## Status

Challenged - Crush Interop Stage 0 Only

## Date

2026-06-07

## Context

Charmbracelet Crush is a Go-based terminal coding agent with multi-model provider config, project-scoped workspaces, LSP context, MCP transports, skills, hooks, permissions, background jobs, todos, and a rich TUI.

The first version of this ADR stored 400 candidate tools inspired by Crush. After GitNexus and prior-ADR review, most candidates are not Crush-specific. They either extend existing Ontocode owners, duplicate prior ADR proposals, or are product/UI polish that should not be implemented as core architecture.

This ADR now owns only Crush external-agent interop detection: finding Crush configuration/state files and producing a redacted dry-run report. Provider, MCP, hooks, shell, context, app-server, skills, and test harness ideas are delegated to existing owners or lefties.

## Crush Source Evidence

Reviewed upstream at commit `2030d97ec0deae1adc94d625fe46f0a14511a568`.

- README feature/config evidence: <https://github.com/charmbracelet/crush>
- Agent tool registry and context helpers: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/agent/tools/tools.go>
- Built-in tools: <https://github.com/charmbracelet/crush/tree/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/agent/tools>
- Config schema, providers, MCP, LSP, permissions, tools, hooks: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/config/config.go>
- Config loading, project-bounded lookup, env expansion, skills paths: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/config/load.go>
- LSP manager and auto-start behavior: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/lsp/manager.go>
- Skill discovery manager: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/skills/manager.go>
- Hook decision and input rewrite model: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/hooks/hooks.go>
- Permission service and persistent grants: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/permission/permission.go>
- Background shell jobs: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/shell/background.go>
- Todo tool: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/agent/tools/todos.go>
- Workspace frontend abstraction: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/workspace/workspace.go>
- Backend workspace sharing and presence: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/backend/backend.go>
- MCP resource listing tool: <https://github.com/charmbracelet/crush/blob/2030d97ec0deae1adc94d625fe46f0a14511a568/internal/agent/tools/list_mcp_resources.go>

## GitNexus Challenge Evidence

GitNexus source-owner review found existing Ontocode owners for the major Crush-inspired surfaces:

- Provider descriptors and runtime selection already exist in [descriptor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/model-provider/src/descriptor.rs:66), [provider.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/model-provider/src/provider.rs:201).
- Model-visible tool planning already flows through [spec_plan.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:190) and must not be duplicated by a Crush-style registry.
- MCP status/resources already have owners in [mcp/mod.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/codex-mcp/src/mcp/mod.rs:317) and [session/mcp.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/session/mcp.rs:238).
- Pre-tool hooks already exist in [hook_runtime.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/hook_runtime.rs:159), with tests in [hooks.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/tests/suite/hooks.rs:2372).
- App-server permissions already have v2 coverage in [request_permissions.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/tests/suite/v2/request_permissions.rs:23).
- Context injection must use bounded fragments in [fragment.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/context-fragments/src/fragment.rs:46) and [contextual_user_message.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/context/contextual_user_message.rs:46).
- Shell/policy work must extend [shell.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/shell.rs:59) and [exec_policy.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/exec_policy.rs:631).
- External-agent import work must extend [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/config/external_agent_config.rs:166), [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/request_processors/external_agent_config_processor.rs:78), and external-agent migration helpers.
- External-agent detection already has a concrete owner in [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/config/external_agent_config.rs:260); Stage 0 must not compete with it as a runtime path.

Challenge result:

- Do not implement the original 400 items as independent tools.
- Do not add another provider registry, MCP manager, tool registry, permission system, hook runner, shell job runtime, context injector, app-server workspace API, or credential broker from this ADR.
- Keep this ADR limited to a redacted Crush interop detector and dry-run report.
- Delegate all generic architecture work to prior ADRs and existing owners.

Second challenge result:

- The five active labels were still too granular for a lowest-capability agent.
- `onto_crush_config_detector`, `onto_crush_ignore_detector`, `onto_crush_auth_reference_detector`, and `onto_crush_import_dry_run` are one script/report task, not four tools.
- `.crushignore` is useful only as a report field; it is not a standalone migration feature.
- The ADR now permits only one implementation label and one test label.

## Prior ADR Delegation

- Provider and runtime-engine work goes to [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:1).
- Provider extensibility, credential importer, and external adapter boundaries go to [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1) and [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md:1).
- Generic operational tooling, GitNexus gates, memory-bank checks, diagnostics planning, and low-agent task-card rules go to [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1).
- Non-natural or product-polish items move to [ADR_CRUSH_TOOL_EXTENSIONS_LEFTIES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_CRUSH_TOOL_EXTENSIONS_LEFTIES.md:1).

## Original Point Disposition

This table covers every original point from `001` through `400`.

| Original points | Disposition | Similar solution in core / ADR | Architecture decision |
|---|---|---|---|
| 001-020 | Removed from current; delegated | `ProviderDescriptor`, `ProviderKind`, native provider ADR | Provider catalog, model switching, capability, pricing, and probe ideas must extend `model-provider`; no Crush-specific provider registry. |
| 021-040 | Kept only as Crush interop detection inputs | config loading and external-agent config import owners | Stage 0 may detect `crush.json`, `.crush.json`, config paths, and redacted field presence; no config merge or mutation. |
| 041-060 | Removed from current; delegated | MCP status/resources owners and lean-ctx ADR | MCP config validation/resource ideas must extend existing MCP owners; no second MCP transport manager. |
| 061-080 | Removed from current; delegated | existing skills extension and lean-ctx ADR | Skill discovery/catalog ideas must extend existing skill surfaces and bounded context rules. |
| 081-100 | Removed from current; delegated | LSP-related TUI/app context owners and lean-ctx ADR | LSP diagnostics are useful but need an LSP-specific task card; not Crush interop. |
| 101-120 | Removed from current; delegated | hooks runtime/tests and lean-ctx ADR | Hook decisions/input rewrites already have an owner; future changes need hook impact/tests. |
| 121-140 | Removed from current; delegated | app-server permission protocol/tests | Permission queues/grants are app-server/TUI concerns; no parallel permission service. |
| 141-160 | Removed from current; delegated | shell handler, sandbox, exec-policy owners | Background shell/job ideas require shell-runtime ADR and tests; not Crush import. |
| 161-168, 170-180 | Removed from current; delegated | core file tools, file tracking, path utilities | File view/write/search/tracking ideas are generic tool improvements; no Crush-specific ownership. |
| 169 | Kept only as derived report field | external-agent migration | Stage 0 may report `.crushignore` path/bytes inside the single detector output; no standalone tool, import, or rewrite. |
| 181-200 | Removed from current; delegated | session/todo/state planning in lean-ctx ADR | Todo/session ideas are generic workflow features; no Crush-specific implementation. |
| 201-220 | Removed from current; delegated | app-server v2, workspace/session APIs | Workspace/multi-client ideas require app-server ADR and compatibility tests. |
| 221-240 | Moved to lefties | TUI has existing status/dialog surfaces, but these are Crush UI polish | Do not implement terminal appearance/dialog polish from this ADR. |
| 241-260 | Removed from current; delegated | bounded context fragments and context rules | Context-path ideas must use `ContextualUserFragment` and hard caps. |
| 261-280 | Removed from current; delegated | diagnostics/support-bundle/redaction planning in lean-ctx ADR | Diagnostics are useful only as owner-specific slices with redaction tests. |
| 281-288, 293-300 | Removed from current; delegated | web-search extension and network policy surfaces | Network/fetch/download ideas belong to extensions or policy ADRs; not Crush import. |
| 289-292 | Moved to lefties | optional Sourcegraph integration is not natural core | Sourcegraph-specific integration needs a separate extension proposal if ever needed. |
| 301-310 | Kept only as derived report field | provider extensibility and credential-import ADRs | Stage 0 may detect auth-looking fields/paths inside the single detector output; no token import, refresh, or persistence. |
| 311-320 | Removed from current; delegated | provider/auth ADRs | API-key dialogs, provider probes, cloud credential detectors, and auth fixtures belong to provider/auth owners. |
| 321-330 | Removed from current; delegated | state/migrations/test planning | DB/storage ideas require state-specific ADR and migration tests. |
| 331-335 | Removed from current; delegated | file tracker/message history concepts | Read tracking/history is generic runtime behavior, not Crush interop. |
| 336-339 | Moved to lefties | project registry UX is not natural core here | Project registry/list/delete state is product workflow, not a Crush-derived architecture need. |
| 340 | Kept only as readiness field | external-agent migration | Stage 0 may print `readiness: "dry_run_only"` inside the single detector output; no state import. |
| 341-360 | Removed from current; delegated | app-server v2 protocol rules | Broad protocol APIs require separate ADR/schema/docs/tests. |
| 361-380 | Removed from current; delegated | safety/redaction/policy rules in AGENTS and lean-ctx ADR | Safety ideas are guardrails for owner-specific work, not tools from this ADR. |
| 381-398 | Removed from current; delegated | test harnesses and lean-ctx test planning | Test ideas become acceptance criteria only when an owner-specific task card exists. |
| 399 | Kept only as Crush interop test label | external-agent migration tests | Tests may cover the Stage 0 dry-run report only. |
| 400 | Removed from current; delegated | lean-ctx lowest-agent affordability rules | Affordability checks are already owned by the lean-ctx ADR. |

## Active Crush Interop Stage 0

Only these labels remain active in this ADR:

| Label | Derived original points | Status | Scope |
|---|---|---|---|
| `onto_crush_interop_detector` | 021-040, 169, 301-310, 340 | Approved implementation | One read-only detector/report command covering config files, ignore files, redacted auth references, and `readiness: "dry_run_only"`. |
| `onto_crush_interop_report_test` | 399 | Approved test | Test only fixed report shape, redaction, JSON parse failure handling, and invalid-root behavior. |

Active implementation labels: 1.

Active test labels: 1.

## Lowest-Agent Stage 0 Contract

Implementation, if dispatched later, must be small enough for a low-capability coding agent:

- Edit at most `scripts/onto_crush_interop.py`.
- Optional tests may be added only in `scripts/tests/test_onto_crush_interop.py`.
- Use Python standard library only.
- Read only the target root passed by `--root`.
- Do not write to the target root or project config.
- Do not modify Rust code.
- Do not add app-server APIs.
- Do not call network services.
- Do not execute commands discovered in Crush config.
- Do not parse or print credential values.

Command:

```bash
python3 scripts/onto_crush_interop.py detect --root <path>
```

Required JSON shape:

```json
{
  "root": "<absolute path>",
  "config_files": [{"path": "<path>", "bytes": 123}],
  "ignore_files": [{"path": "<path>", "bytes": 123}],
  "auth_references": [{"path": "<path>", "field": "<name-or-null>", "bytes": 123, "redacted": true}],
  "summary": {
    "config_files": 0,
    "ignore_files": 0,
    "auth_references": 0
  },
  "readiness": "dry_run_only"
}
```

Detection rules:

- Config candidates: `crush.json`, `.crush.json`, `.crush/crush.json`, files under `.crush/` ending in `.json`.
- Ignore candidates: `.crushignore`.
- Auth candidates: files or JSON field paths containing `auth`, `oauth`, `token`, `credential`, `api_key`, `key`, `copilot`, or `hyper`.
- JSON parse failure must not fail the command; include path-only error metadata and continue.
- Missing or invalid `--root` exits non-zero with a clear stderr message.

Redaction rules:

- Never print file contents.
- Never print environment variable values.
- Never print token/key substrings.
- Auth findings must include only `path`, `field`, `bytes`, and `redacted: true`.

## Follow-Up Requirements

Before any removed/delegated point can be implemented:

- Add a task card to the owning ADR.
- Link the existing source owner.
- Run GitNexus context and impact before code-symbol edits.
- Define exact files allowed to edit.
- Define exact verification command.
- Include compatibility, schema, snapshot, or redaction tests when the affected owner requires them.
