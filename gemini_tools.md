# Gemini Tools Challenge

status: challenged-no-dispatch
source: `gemini_tools.md`
donor: `tmp/gemini-cli-main`
date: 2026-06-21

## Baseline

This file was a raw inventory of 31 Gemini CLI tool classes. It is not an
implementation plan. After OntoIndex review, none of the listed tools are a
new, dispatchable core-extension slice.

OntoIndex checks used:

- `ontocode-rs/core/src/tools/spec_plan.rs:add_tool_sources` already owns hosted
  web/image tools, shell/unified-exec tools, MCP resource/runtime tools,
  `request_user_input`, tool search, goal tools, collaboration tools, dynamic
  tools, and extension tools.
- `ontocode-rs/core/src/unified_exec/mod_tests.rs` already covers persistent
  unified exec sessions and timeout behavior; TUI exec tests cover late output
  and interrupt preservation.
- `ontocode-rs/core/src/tools/handlers/agent_jobs.rs:run_agent_job_loop` already
  owns agent-job execution, recovery, status, cancellation checks, and progress.
- `.memory-bank/GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_PRE_JUNIOR_PROJECT_PLAN.md`
  is closed no-dispatch: future Gemini donor work needs one fresh failing core
  regression and a named current owner.

## Keep

None.

## Reject

| Gemini tool | Reason |
|---|---|
| `AgentTool` | Duplicate of existing `spawn_agent`, collaboration tools, and agent-job runtime. |
| `MockTool` | Test helper only, not core functionality. |
| `WebSearchTool` | Existing hosted web-search owner already exists in `spec_plan`. |
| `WebFetchTool` | New raw fetch surface; not an extension of hosted web-search or network policy. |
| `UpdateTopicTool` | Session topic/labeling product surface, not a missing core owner. |
| `ActivateSkillTool` | Duplicate of existing skill/plugin/extension-tool loading. |
| `ReadManyFilesTool` | Duplicate file-read/context ingestion surface; current context/additional-context tests already cover bounded injection. |
| `ReadFileTool`, `GrepTool`, `RipGrepTool`, `GlobTool`, `LSTool` | Duplicate file/search tool stack. |
| `ShellTool` | Duplicate of shell/unified-exec ownership. |
| `ListBackgroundProcessesTool`, `ReadBackgroundOutputTool` | Existing unified-exec process lifecycle and TUI output handling already own this; no new failing gap named. |
| `DiscoveredTool`, `DiscoveredMCPTool` | Duplicate registry/MCP exposure stack. |
| `ReadMcpResourceTool`, `ListMcpResourcesTool` | Existing MCP resource tools already exist. |
| `AskUserTool` | Duplicate of `request_user_input`. |
| `EnterPlanModeTool`, `ExitPlanModeTool` | Mode switching is owned by collaboration mode/session flow; model-visible toggles add surface area. |
| `WriteTodosTool` | Local planning aid, not core. |
| `TrackerCreateTaskTool`, `TrackerUpdateTaskTool`, `TrackerGetTaskTool`, `TrackerListTasksTool`, `TrackerAddDependencyTool`, `TrackerVisualizeTool` | New tracker/task graph product surface; existing agent jobs own real task execution. |
| `GetInternalDocsTool` | Non-core docs lookup. Use existing docs/context owners if a concrete docs drift gap appears. |

## Revival Rule

Reopen only with one failing core regression test and one existing owner. Do not
dispatch another Gemini tool inventory as-is.
