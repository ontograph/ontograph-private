# ADR: Gemini CLI Interop And Tool Extension Review

## Status

Consolidated - historical source evidence; dispatch via `ADR_EXTERNAL_AGENT_INTEROP_DETECTORS_CONSOLIDATION.md`

## Date

2026-06-07

## Context

The original version of this ADR stored 400 Gemini CLI inspired tool-extension candidates. After GitNexus review, most of those candidates are not Gemini-specific. They either extend existing Ontocode code owners, duplicate prior ADR proposals, or are not natural core functionality.

This ADR now owns only Gemini CLI interop/import work: detecting Gemini CLI configuration, memory/context files, commands, skills, extensions, MCP servers, auth configuration, ignore files, and producing a redacted migration report.

## Consolidation Override

This ADR is no longer an independent dispatch plan. Its retained Gemini CLI Stage 0 requirements are consolidated into [External-Agent Interop Detector Consolidation](ADR_EXTERNAL_AGENT_INTEROP_DETECTORS_CONSOLIDATION.md).

If this ADR conflicts with the consolidation ADR, the consolidation ADR wins.

Dispatch rule:

- Do not implement `scripts/onto_gemini_cli_interop.py` as a separate detector stack unless it is a compatibility wrapper around the shared external-agent interop detector contract.
- Use the shared report envelope, redaction rules, source-specific Gemini requirements, and blocked-scope rules from the consolidation ADR.
- Keep this file as upstream/source evidence and historical disposition for the original Gemini CLI review.

## Gemini CLI Source Evidence

Upstream implementation references:

- Tool registry and definitions: <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/tool-registry.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/definitions/coreTools.ts>
- File tools: <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/read-file.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/read-many-files.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/glob.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/grep.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/edit.ts>
- Shell/sandbox tools: <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/shell.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/shellBackgroundTools.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/utils/sandbox.ts>
- MCP implementation: <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/mcp-client-manager.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/list-mcp-resources.ts>, <https://github.com/google-gemini/gemini-cli/tree/main/packages/core/src/mcp>
- Command loaders: <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/services/CommandService.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/services/FileCommandLoader.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/services/McpPromptLoader.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/services/SkillCommandLoader.ts>
- Settings, policy, trusted folders, extensions: <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/config/settings.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/config/policy.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/config/trustedFolders.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/config/extension-manager.ts>
- Auth and OAuth: <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/config/auth.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/code_assist/oauth2.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/code_assist/oauth-credential-storage.ts>
- Memory/context/checkpointing: <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/config/memory.ts>, <https://github.com/google-gemini/gemini-cli/tree/main/packages/core/src/context>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/utils/autoMemory.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/utils/sessions.ts>
- Agents/A2A: <https://github.com/google-gemini/gemini-cli/tree/main/packages/core/src/agents>, <https://github.com/google-gemini/gemini-cli/tree/main/packages/a2a-server/src>
- Web tools: <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/web-fetch.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/web-search.ts>
- Tracker/todos: <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/write-todos.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/core/src/tools/trackerTools.ts>
- Headless JSON output: <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/nonInteractiveCli.ts>, <https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/utils/jsonoutput.ts>
- ACP/IDE integration: <https://github.com/google-gemini/gemini-cli/tree/main/packages/cli/src/acp>

Documentation references:

- Tools reference: <https://www.geminicli.com/docs/reference/tools>
- Commands reference: <https://www.geminicli.com/docs/reference/commands>
- Configuration reference: <https://www.geminicli.com/docs/reference/configuration>
- MCP docs: <https://www.geminicli.com/docs/tools/mcp-server>
- Checkpointing: <https://www.geminicli.com/docs/cli/checkpointing>
- Plan mode: <https://www.geminicli.com/docs/cli/plan-mode>
- Skills: <https://www.geminicli.com/docs/cli/skills>
- Policy engine: <https://www.geminicli.com/docs/reference/policy-engine>

## Ontocode Source Evidence

GitNexus source links for similar existing code:

- Native provider descriptors and runtime engines: [descriptor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/model-provider/src/descriptor.rs:7), [provider.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/model-provider/src/provider.rs:108), [client.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/client.rs:1584)
- Model-provider diagnostics/API exposure: [config_processor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/request_processors/config_processor.rs:170)
- Model-visible tool planning and extension tools: [spec_plan.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:160), [extension_tools.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/extension_tools.rs:38), [registry_tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/registry_tests.rs:136)
- Web-search extension example: [extension.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/web-search/src/extension.rs:128)
- MCP manager, status, OAuth, resources: [connection_manager.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/codex-mcp/src/connection_manager.rs:105), [mcp/mod.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/codex-mcp/src/mcp/mod.rs:317), [oauth.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/rmcp-client/src/oauth.rs:58), [session/mcp.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/session/mcp.rs:238)
- External-agent config import and detection: [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/config/external_agent_config.rs:166), [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/request_processors/external_agent_config_processor.rs:78), [external_agent_config_tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/config/external_agent_config_tests.rs:38)
- External-agent migration helpers: [lib.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/external-agent-migration/src/lib.rs:102)
- Context fragments: [fragment.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/context-fragments/src/fragment.rs:46), [contextual_user_message.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/context/contextual_user_message.rs:46)
- Shell and policy: [shell.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/shell.rs:59), [exec_policy.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/exec_policy.rs:631)

Prior ADR links:

- Native Gemini provider engine work belongs in [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:523).
- Generic tool/workflow proposals belong in [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:126).
- Provider extensibility boundaries belong in [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1).
- Non-natural items are moved to [ADR_GEMINI_CLI_TOOL_EXTENSIONS_LEFTIES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_GEMINI_CLI_TOOL_EXTENSIONS_LEFTIES.md:1).

## GitNexus Challenge

GitNexus confirms the kept Gemini CLI work is closest to external-agent config detection/import, not model-provider runtime work:

- [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/config/external_agent_config.rs:260) owns detection of external-agent migrations.
- [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/request_processors/external_agent_config_processor.rs:391) owns app-server import request processing.
- [lib.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/external-agent-migration/src/lib.rs:1578) has existing migration tests for MCP config precedence.
- [provider.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/model-provider/src/provider.rs:108) is a provider runtime seam, but Gemini CLI interop must not edit it.

GitNexus impact results gathered before this challenge:

| Symbol | File | Risk | Challenge result |
|---|---|---|---|
| `ExternalAgentConfigService::detect_migrations` | `ontocode-rs/app-server/src/config/external_agent_config.rs` | LOW | Candidate future owner, but Stage 0 must not edit it yet. |
| `ExternalAgentConfigRequestProcessor::import_external_agent_config` | `ontocode-rs/app-server/src/request_processors/external_agent_config_processor.rs` | LOW | App-server import path exists; no new API without later ADR. |
| `ProviderRuntimeEngine::from_provider_engine` | `ontocode-rs/model-provider/src/provider.rs` | LOW | Out of scope for Gemini CLI interop; provider work stays in native provider ADR. |

Challenge outcome:

- The ADR is too broad if all active points `381-400` are treated as implementable.
- Stage 0 may only detect Gemini CLI artifacts and produce a redacted dry-run report.
- Real import, auth mapping, MCP mutation, context injection, command execution, skill/extension activation, app-server API changes, and provider runtime changes are blocked until separate task cards are added.
- GitNexus source graph shows low impact for inspected owners, but app-server/external-agent behavior is guarded primarily by tests and compatibility expectations, so implementation must be test-first.

## Decision

Keep only Gemini CLI interop/import detection items in this ADR. Remove all generic tool, provider runtime, MCP, shell, context, Git/GitHub, telemetry, IDE, release, enterprise, eval, and package proposals from this ADR.

Delegated items are not deleted from the project; they are now owned by either prior ADRs or the lefties file.

Only the lowest-agent Stage 0 task is approved. All import/mutation/execution items remain backlog labels and are blocked.

## Lowest-Agent Affordability Decision

The previous Stage 0 still required too much interpretation. A low-capability coding agent must not decide how to implement Gemini CLI import, how to map MCP config, how to parse custom commands, or how to integrate with app-server.

Affordable Stage 0 is reduced to one repository-only Python script with one command and a fixed JSON output schema.

Approved implementation scope:

- edit at most `scripts/onto_gemini_cli_interop.py`
- optional tests may be added only in `scripts/tests/test_onto_gemini_cli_interop.py`
- read only the target root passed by `--root`
- no writes anywhere
- no Rust changes
- no app-server changes
- no GitNexus-required code-symbol edits
- no dependencies outside Python standard library
- no network calls
- no command execution from detected files

If the implementation needs any other file, stop and create a senior-review task.

## Lowest-Agent Stage 0 Task

Implement exactly this command:

```bash
python3 scripts/onto_gemini_cli_interop.py detect --root <path>
```

The command must print JSON to stdout with this exact top-level shape:

```json
{
  "root": "<absolute path>",
  "gemini_config_dir": {"exists": true, "path": "<path-or-null>"},
  "memory_files": [{"path": "<path>", "bytes": 123}],
  "command_files": [{"path": "<path>", "bytes": 123}],
  "skill_files": [{"path": "<path>", "bytes": 123}],
  "extension_files": [{"path": "<path>", "bytes": 123}],
  "mcp_files": [{"path": "<path>", "bytes": 123}],
  "auth_files": [{"path": "<path>", "bytes": 123, "redacted": true}],
  "ignore_files": [{"path": "<path>", "bytes": 123}],
  "summary": {
    "memory_files": 0,
    "command_files": 0,
    "skill_files": 0,
    "extension_files": 0,
    "mcp_files": 0,
    "auth_files": 0,
    "ignore_files": 0
  },
  "readiness": "dry_run_only"
}
```

Detection rules:

- `.gemini/` directory: detect by path only.
- `GEMINI.md`: include path and byte size only.
- `.gemini/commands/**`: include file paths and byte sizes only.
- `.gemini/skills/**` and `skills/**`: include file paths and byte sizes only.
- `.gemini/extensions/**` and `extensions/**`: include file paths and byte sizes only.
- MCP config candidates: include `.gemini/settings.json`, `.gemini/config.yaml`, and files under `.gemini/mcp/**` by path and byte size only.
- Auth candidates: include `.gemini/settings.json`, `.gemini/config.yaml`, and files with names containing `auth`, `oauth`, `token`, `credential`, `key`, or `service-account`; never print file content.
- `.geminiignore`: include path and byte size only.

Error behavior:

- missing `--root`: print usage to stderr and exit non-zero.
- root does not exist or is not a directory: print clear error to stderr and exit non-zero.
- unreadable files should be skipped with an `"errors"` array entry containing path and reason.
- malformed JSON/YAML must not fail the command because Stage 0 is path/size detection only.

Redaction rule:

- Never print file contents.
- Never print environment variable values.
- Never print detected token/key substrings.
- Auth findings must only include `path`, `bytes`, and `redacted: true`.

Verification command:

```bash
python3 scripts/onto_gemini_cli_interop.py detect --root /tmp/some-fixture
```

If tests are added, use only temporary directories and Python standard library `unittest`.

## Original Point Disposition

| Original points | Disposition | Similar core solution | Prior ADR destination | Architecture decision |
|---|---|---|---|---|
| 1-20 | Removed from current; delegated | `ProviderEngine`, `ProviderDescriptor`, native runtime selection | `ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md` | Extend existing `GeminiGenerateContent` seam; no second provider registry. |
| 21-40 | Removed from current; delegated | login/RMCP OAuth stores and redaction | `ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md` and provider extensibility ADR | Gemini API-key first; OAuth/Vertex requires separate auth-store task card. |
| 41-50, 53-60 | Removed from current; delegated | config loading, external-agent config migration | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Treat as repo-only config/import diagnostics unless app-server API is explicitly approved. |
| 51-52 | Moved to lefties | TUI/theme surfaces exist but are not core provider/import work | lefties | UI theme/terminal setup is polish, not core architecture. |
| 61-72, 74-80 | Removed from current; delegated | bounded context fragments and memory-bank docs | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Context additions must use `ContextualUserFragment` with hard caps. |
| 73 | Moved to lefties | memory exists, but autonomous mutation is risky | lefties | Auto-memory mutation requires separate safety ADR. |
| 81-100 | Removed from current; delegated | core tool spec, handlers, extension registry | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | No new tool registry; model-visible tools require tool schema/lifecycle tests. |
| 101-120 | Removed from current; delegated | file tools and diff/patch handling already exist | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Keep as analysis/checklist proposals; do not duplicate file APIs. |
| 121-139 | Removed from current; delegated | shell handler, sandbox, exec policy | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Runtime permission behavior requires GitNexus impact and tests. |
| 140 | Moved to lefties | shell UI polish, not core | lefties | Defer non-core shell UI. |
| 141-160 | Removed from current; delegated | `McpConnectionManager`, RMCP OAuth, MCP status/resource APIs | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Extend existing MCP owners only. |
| 161-167, 170-180 | Removed from current; delegated | slash-command/tool/status concepts overlap generic workflow tooling | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Commands must be repo-only first; model-visible/app-server commands need separate ADR. |
| 168-169, 172 | Moved to lefties | docs/editor/IDE UI surfaces are not core here | lefties | Defer documentation/editor UI commands. |
| 181-200 | Removed from current; delegated | tracker/todo proposals overlap existing tracking/memory-bank ADR | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Implement as repository-only tracking tools first. |
| 201-220 | Removed from current; delegated | external-agent import/session and multi-agent owners exist | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Keep as workflow prompts unless runtime owner is approved. |
| 221-224, 235, 239 | Moved to lefties | public GitHub automation and workflow changes are outside core | lefties | Requires separate CI/security/release governance. |
| 225-234, 236-238, 240 | Removed from current; delegated | dirty-worktree/diff/test summaries overlap generic tooling | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Keep as repo-only Git/diff diagnostics. |
| 241-254, 257-260 | Removed from current; delegated | telemetry/stats overlap diagnostics and test summaries | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Telemetry export requires privacy/opt-in review. |
| 255-256 | Moved to lefties | dashboards/export UI not core | lefties | Defer dashboard/export surfaces. |
| 261-265, 267-268, 275-280 | Removed from current; delegated | ACP/IDE/terminal diagnostics overlap generic integration proposals | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Keep as diagnostic plans only. |
| 266, 269-274 | Moved to lefties | UI/editor/terminal polish | lefties | Defer non-core UI behavior. |
| 281-300 | Removed from current; delegated | shell/MCP/provider policy checks overlap generic policy tooling | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Extend existing policy evaluator; no second policy engine. |
| 301-320 | Removed from current; delegated | web-search extension already exists | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Extend web-search extension, not core provider/import ADR. |
| 321-330, 332-340 | Removed from current; delegated | skills/extensions overlap generic extension tooling | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Use existing extension registry and bounded context rules. |
| 331 | Moved to lefties | marketplace/release is not core | lefties | Defer marketplace/release work. |
| 341-360 | Removed from current; delegated | tests/evals overlap generic test tooling | `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` | Keep as test-planning proposals. |
| 361-380 | Moved to lefties | packaging/release/enterprise/admin docs are not Gemini interop core | lefties | Requires separate product/release/enterprise ADR. |
| 381-400 | Kept in current ADR | external-agent config detection/import, migration helpers | current ADR | Implement as redacted Gemini CLI interop detection and dry-run import only. |

## Active Candidate Tools

Only original points 381-400 remain in this ADR, but not all are approved for implementation.

| Point | Tool | Status | Challenge |
|---|---|---|---|
| 381 | `onto_gemini_cli_config_detector` | Approved implementation | Read-only detection of `.gemini/` and config candidate paths only. |
| 382 | `onto_gemini_cli_memory_detector` | Approved implementation | Detect `GEMINI.md` path and byte size only; do not inject context. |
| 383 | `onto_gemini_cli_command_detector` | Derived report field | Inventory only inside the fixed JSON report; do not execute or import commands. |
| 384 | `onto_gemini_cli_skill_detector` | Derived report field | Inventory only inside the fixed JSON report; do not activate skills. |
| 385 | `onto_gemini_cli_extension_detector` | Derived report field | Inventory only inside the fixed JSON report; do not install or activate extensions. |
| 386 | `onto_gemini_cli_mcp_detector` | Derived report field | Inventory only inside the fixed JSON report; do not mutate MCP config. |
| 387 | `onto_gemini_cli_auth_detector` | Derived report field | Redacted presence/path/byte-size only; never read or print secret values. |
| 388 | `onto_gemini_cli_ignore_detector` | Derived report field | Inventory only inside the fixed JSON report; no ignore-rule rewrite. |
| 389 | `onto_gemini_cli_import_dry_run` | Approved implementation | Redacted report only; no writes. |
| 390 | `onto_gemini_cli_import_plan` | Blocked | Requires Stage 0 output schema and senior review. |
| 391 | `onto_gemini_cli_import_hooks` | Blocked | Hooks import must extend external-agent migration tests first. |
| 392 | `onto_gemini_cli_import_mcp` | Blocked | MCP import must extend existing MCP config owner; no second MCP registry. |
| 393 | `onto_gemini_cli_import_context` | Blocked | Context import requires bounded-fragment/memory-bank decision. |
| 394 | `onto_gemini_cli_import_commands` | Blocked | Command execution/import requires command-surface ADR. |
| 395 | `onto_gemini_cli_import_skills` | Blocked | Skill import requires extension/skill activation policy. |
| 396 | `onto_gemini_cli_import_extensions` | Blocked | Extension import requires extension manager policy and tests. |
| 397 | `onto_gemini_cli_import_auth` | Blocked | Auth import requires credential-store ADR and redacted sample evidence. |
| 398 | `onto_gemini_cli_import_lefties` | Derived report field | Classification only inside the fixed JSON report; no mutation outside report. |
| 399 | `onto_gemini_cli_import_report` | Derived report field | Redacted report generated from detection output. |
| 400 | `onto_gemini_cli_interop_readiness` | Derived report field | Always report `readiness: "dry_run_only"` in Stage 0. |

Stage 0 implementation labels: 3.

Stage 0 derived report fields: 9.

Blocked count: 8.

## Architecture Decisions For Kept Items

- Reuse external-agent config detection/import owners first: [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/config/external_agent_config.rs:166), [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/request_processors/external_agent_config_processor.rs:78), [external_agent_config_tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/config/external_agent_config_tests.rs:38).
- Stage 0 must not plug into external-agent import services yet; it is a standalone repository script only.
- Later Gemini CLI detection may become a new import source plugged into existing external-agent import services, not a standalone migration framework.
- First implementation must be dry-run only, redacted, and repository-script based.
- Auth detection must report presence, mode, and source path only; it must not copy API keys, OAuth tokens, service account files, or keychain entries.
- MCP import must map Gemini CLI MCP config to existing MCP config shapes; do not add a second MCP registry.
- Context import must map `GEMINI.md`/project context to memory-bank or existing instruction surfaces; any model-context injection must use bounded `ContextualUserFragment`.
- Commands/skills/extensions detection may produce a migration report, but actual command/extension execution requires a separate ADR.
- Stage 0 output should be a stable JSON or markdown report with counts, source paths, and redacted findings.
- Stage 0 must not modify `.memory-bank`, app-server config, MCP config, provider config, credentials, hooks, commands, skills, extensions, or sessions.

## Recommended First Slice

Implement only the lowest-agent Stage 0 task above.

Minimum first command:

```bash
python3 scripts/onto_gemini_cli_interop.py detect --root <path>
```

Constraints:

- read-only
- redacted output
- no Rust runtime changes
- no app-server API changes
- no model-visible tools
- no credential import
- no MCP runtime mutation
- no writes to target project
- no network calls
- no command execution from detected Gemini files

Required first-slice tests:

- detects `.gemini` directory without reading secret values
- detects `GEMINI.md` and reports path/size only
- detects command, skill, extension, MCP, auth, and ignore artifacts by path and byte size
- auth entries contain only `path`, `bytes`, and `redacted: true`
- missing or invalid root exits non-zero with a clear error

## Guardrails

- Do not add a new provider registry.
- Do not add a new tool registry.
- Do not add a new MCP manager.
- Do not add a new credential broker.
- Do not add a new policy engine.
- Do not add public config keys or app-server APIs without a separate ADR and compatibility tests.
- Before editing any code symbol, run GitNexus impact and report the blast radius.
- Do not implement blocked points `390-397` until Stage 0 report output exists and a follow-up ADR/task card defines exact import behavior.
