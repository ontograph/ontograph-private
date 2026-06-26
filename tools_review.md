# Tools Review

Donor: `tmp/claude-code-main/src/tools/`

Gate: keep only ideas that add core capability or narrowly extend an existing Ontocode owner. Do not add duplicate tool stacks, second schedulers, second task systems, or model-facing controls for settings/auth unless the current owner already needs that surface.

OntoIndex baseline used:

- `codex` index is fresh at `8b61fd0dbfa32aa9e4a00ef15930e5b4fcb9119f`.
- Existing MCP resources are already implemented in `ontocode-rs/core/src/tools/handlers/mcp_resource_spec.rs`.
- Existing multi-agent tools are already implemented in `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`.
- Existing web search is already implemented in `ontocode-rs/core/src/tools/hosted_spec.rs`.
- Existing shell/PowerShell selection is already implemented in `ontocode-rs/core/src/shell.rs`.
- Existing structured user-input handling is covered by `ontocode-rs/core/tests/suite/request_user_input.rs`.

## Keep

1. **WebFetchTool**
   - Keep as a guarded `web_fetch` extension beside existing hosted web search.
   - Must reuse existing web-search/network policy, redaction, timeout, size caps, and permission handling.
   - Do not add a browser runtime, crawler, cache, or parallel web stack.

2. **FileReadTool / GlobTool / GrepTool**
   - Keep as bounded filesystem context tools only if they reuse existing filesystem, workspace-root, sandbox, and context-cap owners.
   - Scope is read/search ergonomics and lower shell dependence.
   - Do not add a separate permission model or broad file-write API.

3. **LSPTool**
   - Keep only as read-only code-navigation support.
   - Prefer existing OntoIndex/GitNexus evidence paths or the current LSP owner if one is already active.
   - Do not add a second indexer, daemon, or mutable refactor surface.

4. **BriefTool**
   - Keep only as a read-only session/context brief over existing compaction, resume, and rollout summaries.
   - No new memory store, no autonomous memory writes, no unbounded transcript injection.

5. **McpAuthTool**
   - Keep only as an MCP auth status/diagnostic handoff that extends existing MCP auth status surfaces.
   - No model-executed OAuth login flow and no credential mutation from a tool call.

## Reject

Reject the rest from this donor list for now:

- **Already covered:** `AgentTool`, `BashTool`, `PowerShellTool`, `EnterPlanModeTool`, `ExitPlanModeTool`, `FileEditTool`, `FileWriteTool`, `ListMcpResourcesTool`, `ReadMcpResourceTool`, `MCPTool`, `SendMessageTool`, `SkillTool`, `TodoWriteTool`, `ToolSearchTool`, `WebSearchTool`.
- **Duplicate or non-core task system:** `TaskCreateTool`, `TaskGetTool`, `TaskListTool`, `TaskUpdateTool`, `TaskOutputTool`, `TaskStopTool`.
- **New scheduler/runtime surface:** `RemoteTriggerTool`, `ScheduleCronTool`, `SleepTool`, `REPLTool`.
- **Niche or blocked product surface:** `NotebookEditTool`, `EnterWorktreeTool`, `ExitWorktreeTool`, `TeamCreateTool`, `TeamDeleteTool`.
- **Protocol/config risk without a proven owner gap:** `AskUserQuestionTool`, `ConfigTool`, `SyntheticOutputTool`.

## Next Slice

Smallest useful implementation slice: `web_fetch` only. It is the clearest new core capability and can be bounded by existing web-search/network policy. Everything else should wait for a specific failing workflow or owner-local test gap.
