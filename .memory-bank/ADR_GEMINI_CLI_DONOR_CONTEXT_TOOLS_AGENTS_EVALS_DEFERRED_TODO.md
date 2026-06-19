# ADR: Gemini CLI Donor Deferred TODO For Context, Tools, Agents, And Evals

## Status

Deferred TODO inventory. These ideas are not approved for implementation by
[Gemini CLI Donor Context/Tools/Agents/Evals](ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS.md).

Use this file as a parking lot for donor ideas that need a narrower owner, a later ADR, or deletion.

## Deferred Or Narrowed Ideas

### Context And Memory

1. Narrow: memory update tests proving persisted project memory changes are deliberate and small. Best home: `ontocode-rs/memories/write/` tests.
2. Narrow: hot-start context cache tests around unchanged context fragments. Best home: existing context fragment/cache owner only if a cache already exists.
3. Narrow: tool-output distillation only for large outputs crossing existing caps. Best home: output truncation/spill owners.
4. Narrow: IDE context parity tests between app-server/TUI context and editor context. Best home: `ontocode-rs/tui/src/ide_context/` plus app-server tests.
5. Defer: bounded session scratchpad. Best home if revived: existing `state/` or session state, with hard caps.
6. Defer: hierarchical memory lookup rules for repo, workspace, and user layers. Best home if revived: config/memory-bank boundary after a separate ADR.

### File, Search, And Tool Behavior

7. Narrow: parallel-tool tests where independent reads may run concurrently. Best home: existing tool execution/router tests.
8. Narrow: file edit location evals for wrong file or wrong range. Best home: existing edit tool tests.
9. Narrow: shell-efficiency evals for command count and needless repetition. Best home: existing shell/exec tests.
10. Narrow: unsafe-cloning/file-copy evals for accidental broad copy behavior. Best home: file-system/exec tests.

### MCP, Hooks, And Extensions

11. Narrow: hook input-modification tests with explicit caps and redaction. Best home: existing hooks tests.
12. Narrow: extension manifest detector tests for names, permissions, and declared commands only. Best home: `core-plugins` and `core-skills`.
13. Defer: extension install/reload tests. Requires a later ADR for extension mutation semantics.

### Agents And Subagents

14. Narrow: generalist-agent fallback tests for tasks that do not match a specialist. Best home: agent role/config tests if fallback behavior already exists.
15. Narrow: CLI-help delegation evals routing help requests to docs/tool help instead of model guessing. Best home: existing help/tool-search routing tests.
16. Defer: remote subagent protocol tests. Requires an approved remote invocation protocol.
17. Defer: agent scheduler concurrency tests for pending approval and race conditions. Best home if revived: `thread_manager` and agent control.
18. Defer: skill-extraction evals. Best home if revived: `core-skills`, after thresholds are defined.

### Automation, Release, And Evals

19. Narrow: eval categories for answer-vs-act, plan-mode, tool-output masking, and validation fidelity. Best home: existing core suite/eval docs, not a new framework.
20. Defer: release-confidence checklist automation around tests, docs drift, and package artifacts. Best home if revived: `.github/workflows/` or scripts.
21. Defer: PR size labeling and stable comment update workflow. Best home if revived: GitHub workflow only.
22. Defer: deflake workflow for known flaky tests with bounded retry and reporting. Best home if revived: CI workflow/test tooling.

## Rejected For Core Runtime

1. Reject for now: issue triage automation as product/runtime behavior. Keep it external.
2. Reject for now: A2A-style task lifecycle/server work. Keep only as compatibility reading unless a later protocol ADR approves it.

## Revival Rule

Move an item back to the main ADR only when it has:

- a concrete current-codebase owner
- a test-first implementation shape
- no new parallel runtime, scheduler, memory layer, extension lifecycle, or public API
