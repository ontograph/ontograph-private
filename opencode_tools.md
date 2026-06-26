# OpenCode Tools Review

## Verdict

Keep only one core-extension slice: a redacted, read-only OpenCode interop detector/report for local `.opencode` artifacts.

Everything else in the donor tool list is rejected as a direct core tool addition. These tools either already exist in Ontocode, belong to an existing owner, or would create a second runtime/tool/config surface.

OntoIndex check: index is fresh at `8b61fd0dbfa32aa9e4a00ef15930e5b4fcb9119f` with a dirty worktree. Relevant owners found:

- external-agent migration/import: `ExternalAgentConfigRequestProcessor.import`, `ExternalAgentConfigService.import`;
- tool planning/exposure: `core/src/tools/spec_plan.rs`;
- web/image hosted tools: `hosted_model_tool_specs`;
- shell/search parsing: `shell-command/src/parse_command.rs`;
- core utility/collaboration tools: `add_core_utility_tools`, `add_collaboration_tools`.

## Keep

| Candidate | Decision | Existing owner | Scope allowed |
| --- | --- | --- | --- |
| OpenCode interop detector/report | KEEP | external-agent migration / diagnostics boundary | Detect `.opencode` config, command/tool/skill/theme/plugin/MCP/provider/LSP metadata. Report names, paths, counts, key names, and redacted endpoint identities only. |
| OpenCode interop detector tests | KEEP | external-agent migration tests | Prove redaction, bounded output, deterministic report shape, invalid config handling, no remote fetch, and no executable import. |

## Reject Or Delegate

| Donor tool | Decision | Reason |
| --- | --- | --- |
| `WebSearchTool` | REJECT | Hosted web search already exists in core tool planning. |
| `GlobTool` | REJECT | File discovery is covered by shell/search tooling and extension tools; no separate glob core tool. |
| `LspTool` | REJECT | Runtime LSP tooling is blocked unless a later ADR proves it complements OntoIndex/GitNexus instead of duplicating code intelligence. Stage 0 may detect LSP metadata only. |
| `QuestionTool` | REJECT | Typed user input already has an owner; no OpenCode-specific question tool. |
| `TaskTool` | REJECT | Task/sub-agent behavior belongs to current session, agent-job, and collaboration owners. |
| `TodoWriteTool` | REJECT | Planning/todo state is already covered by existing plan/update surfaces; adding another state owner is not core extension. |
| `SkillTool` | REJECT | Skills already have plugin/skill owners. Stage 0 may detect skill files only; no import/execution. |
| `WebFetchTool` | REJECT | Web fetch/search belongs to hosted/dynamic tool owners or extensions; no duplicate OpenCode tool. |
| `PlanExitTool` | REJECT | Plan-mode control is existing session/TUI behavior, not a new tool surface. |
| `WriteTool` | REJECT | File writes already flow through existing edit/shell/apply-patch tooling and policies. |
| `GrepTool` | REJECT | Search is already covered by shell parsing, `rg`, and code-intelligence tooling. |
| `ReadTool` | REJECT | File reads are existing shell/tool functionality; no new read tool. |
| `ApplyPatchTool` | REJECT | Apply-patch already exists with parser and failure coverage. |
| `InvalidTool` | REJECT | Invalid-tool handling is protocol/runtime error behavior, not a standalone core feature. |
| `ShellTool` | REJECT | Shell execution, sandboxing, and approvals already have dedicated owners. |
| `EditTool` | REJECT | Editing belongs to existing patch/write tooling and policy gates. |

## Accepted Follow-Up Contract

If implemented, keep it as the existing ADR already says:

- detect only local `.opencode` metadata;
- do not import sessions, agents, commands, tools, skills, providers, plugins, MCP servers, permissions, themes, credentials, or prompts;
- do not execute shell commands, plugins, tools, auth hooks, MCP servers, LSPs, package installs, or remote config fetches;
- do not add model-visible tools, app-server APIs, config schema keys, provider registries, MCP managers, permission engines, plugin runtimes, or session stores;
- never print credentials, tokens, authorization headers, cookies, database URLs, prompt bodies, command bodies, plugin code, or raw config secrets.

Useful output is a deterministic dry-run report. Anything beyond that needs a fresh ADR with user-facing value and compatibility tests.
