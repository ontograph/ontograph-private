# ADR: Gemini CLI Donor Ideas For Context, Tools, Agents, And Evals

## Status

Accepted for planning - keep-only idea inventory. No implementation is authorized by this ADR.
Each proposal must use the dispatch owner table below before implementation.

Deferred, narrow, and rejected donor ideas were moved to
[Gemini CLI Donor Deferred TODO](ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_DEFERRED_TODO.md).

## Date

2026-06-16

## Context

`tmp/gemini-cli-main` was downloaded as a no-history main-branch snapshot of `google-gemini/gemini-cli` for donor review. The useful material is not provider/runtime code. The useful material is product shape, test shape, and workflow shape around:

- context and memory
- file/search/tool behavior
- MCP, hooks, and extensions
- agents and subagents
- automation, release, and evals

Ontocode already has owners for these concerns. Donor ideas must extend existing owners instead of creating parallel systems.

## Decision

Use Gemini CLI as an idea donor for focused follow-on planning. Do not port Gemini CLI subsystems wholesale.

Accepted donor ideas may become future work only when a later implementation card names:

- the existing Ontocode owner
- exact behavior
- impact analysis target
- bounded data contract
- redaction rules
- compatibility tests

## Donor Evidence

| Area | Gemini donor evidence |
| --- | --- |
| Context and memory | `packages/core/src/context/`, `packages/core/src/services/memoryService.ts`, `packages/core/src/services/sessionScratchpadUtils.ts`, `evals/auto_memory_contract.eval.ts`, `evals/hierarchical_memory.eval.ts`, `integration-tests/context-fidelity.test.ts` |
| File/search/tool behavior | `packages/core/src/tools/`, `packages/sdk/src/tool.ts`, `evals/frugalReads.eval.ts`, `evals/frugalSearch.eval.ts`, `integration-tests/read_many_files.test.ts`, `integration-tests/write_file.test.ts`, `integration-tests/parallel-tools.test.ts` |
| MCP/hooks/extensions | `packages/core/src/mcp/`, `packages/core/src/hooks/`, `packages/cli/src/commands/mcp/`, `packages/cli/src/commands/extensions/`, `integration-tests/mcp-resources.test.ts`, `integration-tests/hooks-system.test.ts`, `integration-tests/extensions-install.test.ts` |
| Agents and subagents | `packages/core/src/agents/`, `packages/a2a-server/src/agent/`, `evals/subagents.eval.ts`, `evals/subtask_delegation.eval.ts`, `evals/generalist_delegation.eval.ts` |
| Automation, release, evals | `evals/`, `integration-tests/*.responses`, `.github/workflows/`, `docs/issue-and-pr-automation.md`, `docs/release-confidence.md` |

## Review Notes

The retained ideas are useful, but the ADR must stay traceable to exact donor examples and must not smuggle in Gemini runtime architecture. The donor codebase is TypeScript-first and includes full provider, policy, extension, A2A, and telemetry stacks. Ontocode should borrow test shapes, fixture contracts, and product behavior names, then implement any accepted slice inside existing Rust owners.

## Donor Links By Area

### Context And Memory

- [context manager](../tmp/gemini-cli-main/packages/core/src/context/contextManager.ts)
- [context compression service](../tmp/gemini-cli-main/packages/core/src/context/contextCompressionService.ts)
- [memory context manager](../tmp/gemini-cli-main/packages/core/src/context/memoryContextManager.ts)
- [memory service](../tmp/gemini-cli-main/packages/core/src/services/memoryService.ts)
- [session scratchpad utilities](../tmp/gemini-cli-main/packages/core/src/services/sessionScratchpadUtils.ts)
- [auto-memory contract eval](../tmp/gemini-cli-main/evals/auto_memory_contract.eval.ts)
- [auto-memory modes eval](../tmp/gemini-cli-main/evals/auto_memory_modes.eval.ts)
- [hierarchical memory eval](../tmp/gemini-cli-main/evals/hierarchical_memory.eval.ts)
- [memory persistence eval](../tmp/gemini-cli-main/evals/memory_persistence.eval.ts)
- [context fidelity integration test](../tmp/gemini-cli-main/integration-tests/context-fidelity.test.ts)
- [context compression integration test](../tmp/gemini-cli-main/integration-tests/context-compress-interactive.test.ts)

### File, Search, And Tool Behavior

- [SDK tool wrapper](../tmp/gemini-cli-main/packages/sdk/src/tool.ts)
- [SDK tool integration test](../tmp/gemini-cli-main/packages/sdk/src/tool.integration.test.ts)
- [frugal reads eval](../tmp/gemini-cli-main/evals/frugalReads.eval.ts)
- [frugal search eval](../tmp/gemini-cli-main/evals/frugalSearch.eval.ts)
- [grep search eval](../tmp/gemini-cli-main/evals/grep_search_functionality.eval.ts)
- [tool output masking eval](../tmp/gemini-cli-main/evals/tool_output_masking.eval.ts)
- [read-many-files integration test](../tmp/gemini-cli-main/integration-tests/read_many_files.test.ts)
- [write-file integration test](../tmp/gemini-cli-main/integration-tests/write_file.test.ts)
- [parallel-tools integration test](../tmp/gemini-cli-main/integration-tests/parallel-tools.test.ts)
- [tool error recovery fixture](../tmp/gemini-cli-main/packages/sdk/test-data/tool-error-recovery.json)
- [tool catchall error fixture](../tmp/gemini-cli-main/packages/sdk/test-data/tool-catchall-error.json)

### MCP, Hooks, And Extensions

- [MCP OAuth provider](../tmp/gemini-cli-main/packages/core/src/mcp/mcp-oauth-provider.ts)
- [MCP token storage](../tmp/gemini-cli-main/packages/core/src/mcp/oauth-token-storage.ts)
- [MCP command list](../tmp/gemini-cli-main/packages/cli/src/commands/mcp/list.ts)
- [MCP resource integration test](../tmp/gemini-cli-main/integration-tests/mcp-resources.test.ts)
- [MCP cyclic schema test](../tmp/gemini-cli-main/integration-tests/mcp_server_cyclic_schema.test.ts)
- [hook system](../tmp/gemini-cli-main/packages/core/src/hooks/hookSystem.ts)
- [hook runner](../tmp/gemini-cli-main/packages/core/src/hooks/hookRunner.ts)
- [hook registry](../tmp/gemini-cli-main/packages/core/src/hooks/hookRegistry.ts)
- [hook system integration matrix](../tmp/gemini-cli-main/integration-tests/hooks-system.test.ts)
- [extension manager](../tmp/gemini-cli-main/packages/cli/src/config/extension-manager.ts)
- [extension install command](../tmp/gemini-cli-main/packages/cli/src/commands/extensions/install.ts)
- [extension reload test](../tmp/gemini-cli-main/integration-tests/extensions-reload.test.ts)

### Agents And Subagents

- [agent scheduler](../tmp/gemini-cli-main/packages/core/src/agents/agent-scheduler.ts)
- [agent loader](../tmp/gemini-cli-main/packages/core/src/agents/agentLoader.ts)
- [agent registry](../tmp/gemini-cli-main/packages/core/src/agents/registry.ts)
- [acknowledged agents](../tmp/gemini-cli-main/packages/core/src/agents/acknowledgedAgents.ts)
- [local subagent protocol](../tmp/gemini-cli-main/packages/core/src/agents/local-subagent-protocol.ts)
- [remote subagent protocol](../tmp/gemini-cli-main/packages/core/src/agents/remote-subagent-protocol.ts)
- [A2A task model](../tmp/gemini-cli-main/packages/a2a-server/src/agent/task.ts)
- [A2A race-condition test](../tmp/gemini-cli-main/packages/a2a-server/src/agent/race-condition.test.ts)
- [subagents eval](../tmp/gemini-cli-main/evals/subagents.eval.ts)
- [subtask delegation eval](../tmp/gemini-cli-main/evals/subtask_delegation.eval.ts)
- [generalist delegation eval](../tmp/gemini-cli-main/evals/generalist_delegation.eval.ts)

### Automation, Release, And Evals

- [eval harness README](../tmp/gemini-cli-main/evals/README.md)
- [eval test helper](../tmp/gemini-cli-main/evals/test-helper.ts)
- [validation fidelity eval](../tmp/gemini-cli-main/evals/validation_fidelity.eval.ts)
- [snapshot fidelity eval](../tmp/gemini-cli-main/evals/snapshot_fidelity.eval.ts)
- [API resilience test](../tmp/gemini-cli-main/integration-tests/api-resilience.test.ts)
- [API resilience response fixture](../tmp/gemini-cli-main/integration-tests/api-resilience.responses)
- [integration test helper](../tmp/gemini-cli-main/integration-tests/test-helper.ts)
- [release confidence docs](../tmp/gemini-cli-main/docs/release-confidence.md)
- [issue and PR automation docs](../tmp/gemini-cli-main/docs/issue-and-pr-automation.md)
- [agent-session drift workflow](../tmp/gemini-cli-main/.github/workflows/agent-session-drift-check.yml)
- [deflake workflow](../tmp/gemini-cli-main/.github/workflows/deflake.yml)
- [release promote workflow](../tmp/gemini-cli-main/.github/workflows/release-promote.yml)

## Donor Pattern Examples

These examples are intentionally Rust-shaped sketches for Ontocode. They are not copied Gemini implementation code.

### Context Fidelity Test Shape

Borrow from [context-fidelity.test.ts](../tmp/gemini-cli-main/integration-tests/context-fidelity.test.ts): test that large or compressed context preserves the important user-visible facts.

```rust
#[test]
fn ide_context_keeps_active_file_and_selection_after_compaction() {
    let rendered = render_prompt_context(&ide_context_with_selection("src/lib.rs", "fn selected() {}"));

    assert!(rendered.contains("src/lib.rs"));
    assert!(rendered.contains("fn selected() {}"));
    assert!(rendered.len() <= MAX_IDE_CONTEXT_BYTES);
}
```

### Frugal Read Eval Shape

Borrow from [frugalReads.eval.ts](../tmp/gemini-cli-main/evals/frugalReads.eval.ts): make waste visible by counting broad reads.

```rust
#[test]
fn small_symbol_question_uses_targeted_reads() {
    let run = run_agent_fixture("what does render_prompt_context do?");

    assert!(run.file_reads <= 3);
    assert!(run.used_search_before_large_read);
    assert!(!run.read_entire_large_files);
}
```

### Model-Visible Tool Error Shape

Borrow from [tool.ts](../tmp/gemini-cli-main/packages/sdk/src/tool.ts) and [tool-error-recovery.json](../tmp/gemini-cli-main/packages/sdk/test-data/tool-error-recovery.json): only intentional, redacted errors reach the model.

```rust
enum ToolErrorVisibility {
    Hidden,
    ModelVisible,
}

struct ToolFailure {
    visibility: ToolErrorVisibility,
    redacted_message: String,
}
```

### MCP Resource Fixture Shape

Borrow from [mcp-resources.test.ts](../tmp/gemini-cli-main/integration-tests/mcp-resources.test.ts): use a tiny deterministic MCP server fixture.

```rust
#[tokio::test]
async fn mcp_resources_are_listed_and_read_with_stable_order() {
    let server = TestMcpServer::with_resources(["repo://one", "repo://two"]);

    let listed = list_mcp_resources(&server).await?;
    assert_eq!(listed, vec!["repo://one", "repo://two"]);
}
```

### Hook Matrix Shape

Borrow from [hooks-system.test.ts](../tmp/gemini-cli-main/integration-tests/hooks-system.test.ts): test each hook event separately and with one multi-event fixture.

```rust
#[test]
fn hook_error_is_bounded_and_does_not_leak_secret() {
    let output = run_hook_fixture("post_tool_use", "TOKEN=secret");

    assert!(output.error.is_some());
    assert!(!output.model_visible_text.contains("secret"));
}
```

### Subagent Delegation Eval Shape

Borrow from [subagents.eval.ts](../tmp/gemini-cli-main/evals/subagents.eval.ts): verify both use and non-use of delegation.

```rust
#[test]
fn simple_task_does_not_spawn_subagent() {
    let run = run_agent_fixture("rename this local variable in one file");

    assert_eq!(run.spawned_subagents, 0);
}
```

### Golden Response Fixture Shape

Borrow from Gemini `.responses` fixtures such as [api-resilience.responses](../tmp/gemini-cli-main/integration-tests/api-resilience.responses): keep agent integration tests deterministic.

```text
fixture:
  user: "run the small task"
  model_events:
    - tool_call: "read_file"
    - tool_result: "bounded fixture text"
    - final: "done"
```

## Existing Ontocode Owners

| Concern | Existing owner |
| --- | --- |
| Model-visible context | `ontocode-rs/context-fragments/`, `ontocode-rs/core/src/context/`, `ontocode-rs/tui/src/ide_context/` |
| Memory/state | `ontocode-rs/state/`, memory-bank files, existing session/state services |
| File tools and search | `ontocode-rs/file-system/`, `ontocode-rs/exec-server/`, `ontocode-rs/core/src/tools/`, existing search/MCP tool paths |
| MCP | `ontocode-rs/rmcp-client/`, `ontocode-rs/codex-mcp/`, `ontocode-rs/mcp-server/` |
| Hooks | `ontocode-rs/hooks/`, `ontocode-rs/core/src/hook_runtime.rs` |
| Extensions/skills/plugins | existing `core-skills`, `core-plugins`, plugin and skill loaders |
| Agents/subagents | existing core session, agent control, multi-agent orchestration, app-server thread flow |
| Automation/evals | existing Rust tests, snapshot tests, CI workflows, memory-bank tracking |

## Retained Keep Proposals

### Context And Memory

1. Add context-fidelity regression tests for active file, selected range, and user prompt rendering.
2. Add a context compression failure-mode test: empty compression, failed compression, and successful compression.
3. Add a tool-output masking check before any tool output becomes model-visible context.
4. Add tests or narrow existing telemetry assertions for prompt/context token budget counts. Do not report content.

### File, Search, And Tool Behavior

5. Add frugal-read evals that fail when the agent reads too many files for a small task.
6. Add frugal-search evals that prefer targeted `rg`/symbol search over broad scans.
7. Add read-many-files guard tests for bounded output and stable ordering.
8. Add write-file tests for overwrite, create, missing parent, and permission denial behavior.
9. Add model-visible tool error typing tests, and extend existing error types only where tests prove a gap.
10. Add tool error recovery golden fixtures for visible error, hidden error, and confirmation-required cases.

### MCP, Hooks, And Extensions

11. Add MCP resource list/read integration tests with deterministic fixture servers.
12. Add cyclic MCP schema tests to guard schema traversal.
13. Add MCP prompt/tool conflict tests so duplicate names resolve predictably.
14. Add MCP OAuth/token storage redaction tests.
15. Add hook lifecycle tests for before/after model, before/after tool, notification, and stop events.
16. Add hook error-handling tests that prove hook failure cannot leak secrets or corrupt the turn.
17. Add hook sequential-execution tests for ordering and timeout behavior.

### Agents And Subagents

18. Add subagent delegation evals for when a task should and should not be delegated.
19. Add local subagent protocol tests for request, response, cancellation, and error reporting.
20. Add subagent acknowledgement tests around existing trust boundaries before adding any new acknowledgement behavior.
21. Add codebase-investigator role evals that require evidence paths, not broad narration.

### Automation, Release, And Evals

22. Add golden `.responses`-style fixtures for deterministic agent integration tests.
23. Add API-resilience fixtures for retryable errors, quota, and malformed responses.
24. Add interactive-hang tests for terminal/TUI flows.
25. Add snapshot-fidelity evals for transcript and UI rendering.
26. Add agent-session drift check to catch stale prompts, memory, and generated workflow artifacts.

## Dispatch Owner Table

| Proposal | Dispatch owner |
| --- | --- |
| 1 | `ontocode-rs/core/tests/common/context_snapshot.rs`, `ontocode-rs/core/tests/suite/additional_context.rs`, `ontocode-rs/tui/src/ide_context/prompt.rs` |
| 2 | `ontocode-rs/core/tests/suite/compact.rs`, `ontocode-rs/core/tests/suite/compact_remote.rs`, `ontocode-rs/core/src/session/turn.rs` |
| 3 | existing redaction and model-visible output owners; no new masking layer |
| 4 | `ontocode-rs/core/src/session/turn.rs`; counts only |
| 5-6 | `ontocode-rs/core/tests/suite/search_tool.rs`, existing tool-search tests |
| 7-8 | `ontocode-rs/exec-server/tests/file_system.rs` |
| 9-10 | `ontocode-rs/core/tests/common/responses.rs`, existing protocol/tool tests |
| 11-13 | `ontocode-rs/core/src/tools/handlers/mcp_resource_tests.rs`, `ontocode-rs/codex-mcp/src/connection_manager_tests.rs` |
| 14 | existing login/auth and MCP redaction tests; no new credential store |
| 15-17 | `ontocode-rs/core/tests/suite/hooks.rs`, `ontocode-rs/hooks/src/engine/mod_tests.rs` |
| 18-21 | `ontocode-rs/core/src/tools/handlers/multi_agents_tests.rs`, `ontocode-rs/core/tests/suite/subagent_notifications.rs` |
| 22-23 | `ontocode-rs/core/tests/common/responses.rs`, `ontocode-rs/core/tests/common/test_codex.rs` |
| 24-25 | existing TUI/core snapshot suites only when UI or transcript behavior changes |
| 26 | memory-bank/script checks; no Rust runtime changes |

## Blocked Scope

This ADR does not authorize:

- a second provider registry, provider router, model catalog, credential store, MCP manager, hook registry, shell runtime, or scheduler
- importing Gemini CLI code directly into Rust crates
- adding a new public app-server API, config schema, SDK, extension runtime, or model-visible context fragment without a later ADR
- executing Gemini commands, extensions, hooks, or skills discovered in the donor repo
- copying Gemini's A2A server as a new Ontocode runtime
- copying Gemini telemetry, policy, or extension systems wholesale
- adding vector memory, embeddings, or search reranking from Gemini donor material

## First Slice Recommendation

Start with test/eval-shaped imports because they are low-risk and improve confidence without new runtime surface:

1. context-fidelity tests
2. context compression failure-mode tests
3. model-visible tool error tests
4. golden response fixtures for deterministic agent behavior

## Implementation Rules

- Keep each slice under the dispatch owner table above.
- Prefer tests and fixtures before runtime changes.
- Use existing redaction and bounded context helpers.
- Do not add public config, app-server APIs, SDK surfaces, or persistent formats without a separate compatibility ADR.
- Run OntoIndex/GitNexus impact before editing any Rust symbol.
- If the slice touches Rust, run scoped `just fmt`, `just fix -p <project>`, and `just test -p <project>` per repository rules.

## Final Recommendation

Use Gemini CLI as a checklist donor, not as an architecture donor. The best value is in its eval coverage, context fidelity checks, hook/MCP integration matrices, and subagent delegation tests. Runtime ideas should wait until those tests reveal an actual Ontocode gap.
