# ADR: Donor Tool Proposals Consolidation

## Status

Challenged - narrow core-extension guidance only

## Date

2026-06-21

## Inputs

- [Claude tools review](../tools_review.md) from donor [tmp/claude-code-main/src/tools](../tmp/claude-code-main/src/tools)
- [Qwen tools challenge](../tmp/qwen-tools-challenged.md) from donor [tmp/qwen-code/packages/core/src/tools](../tmp/qwen-code/packages/core/src/tools)
- [RTK tools challenge](../tmp/rtk-main) from donor [tmp/rtk-main](../tmp/rtk-main)
- [Haft tools challenge](../tmp/haft-main) from donor [tmp/haft-main](../tmp/haft-main)
- [BitGN ECOM1 exoskeleton review](../tmp/bitgn-ecom1-exoskeleton-review.md) from donor [tmp/bitgn-ecom1-exoskeleton-main](../tmp/bitgn-ecom1-exoskeleton-main)

Gate: keep only proposals that add real core capability or narrowly extend an existing Ontocode owner.

## OntoIndex Challenge Evidence

OntoIndex repo `codex` was fresh at `8b61fd0dbfa32aa9e4a00ef15930e5b4fcb9119f` with a dirty worktree.

- `final_output_json_schema` already exists in protocol serialization paths; any structured-output work must extend that path.
- Ontocode already passes `final_output_json_schema` into the native prompt request as `output_schema` with strict mode; Qwen's synthetic `structured_output` tool is useful as behavioral evidence, not as a direct tool-stack import.
- Hosted web/search tool planning is already owned by `core/src/tools/spec_plan.rs` and `hosted_model_tool_specs`; any fetch work must be a bounded sibling of hosted web search.
- Agent-job execution, cancellation checks, terminal status, recovery, and finalization are already owned by `core/src/tools/handlers/agent_jobs.rs` and `state/src/runtime/agent_jobs.rs`.
- The BitGN ECOM1 exoskeleton donor validates the "model proposes, code disposes" harness shape already present in `apply_patch`, Guardian/exec policy, tool output handling, and bounded `ContextualUserFragment` injection.

## Decision

Keep four accepted/narrowed candidates. Everything else is verification-only until a specific failing workflow proves a current owner gap.

| Candidate | Sources | Decision | Existing owner | Boundary |
| --- | --- | --- | --- | --- |
| Structured final output enforcement | Qwen [syntheticOutput.ts](../tmp/qwen-code/packages/core/src/tools/syntheticOutput.ts), [syntheticOutput.test.ts](../tmp/qwen-code/packages/core/src/tools/syntheticOutput.test.ts), [config.ts](../tmp/qwen-code/packages/cli/src/config/config.ts), [nonInteractiveCli.ts](../tmp/qwen-code/packages/cli/src/nonInteractiveCli.ts), [telemetry/types.ts](../tmp/qwen-code/packages/core/src/telemetry/types.ts) | KEEP, first slice closed | protocol, session, final-output handling, telemetry | Closed slice validates non-null `final_output_json_schema` updates on the existing session path. Structured payload redaction and schema-conformance diagnostics remain parked until a concrete current-owner failing test proves the gap. Do not add a normal `structured_output` tool. |
| Guarded web fetch | Claude [WebFetchTool](../tmp/claude-code-main/src/tools/WebFetchTool/WebFetchTool.ts) | KEEP, senior-narrowed | hosted web search / network policy | Do not add a parallel Rust fetcher or unsupported hosted `web_fetch` tool. Current hosted `web_search` already carries guarded search/open-page/find-in-page actions through provider capability, config, standalone-web, and network-policy gates. Future dedicated `web_fetch` work requires a supported provider/API surface and must reuse the same owner. |
| Evidence ledger for tool results | BitGN [evidence_ledger.py](../tmp/bitgn-ecom1-exoskeleton-main/evidence_ledger.py), [doc_autocite.py](../tmp/bitgn-ecom1-exoskeleton-main/doc_autocite.py), [ARCHITECTURE.md](../tmp/bitgn-ecom1-exoskeleton-main/articles/ARCHITECTURE.md) | KEEP | context fragments, tool output handling, session facts | Carry bounded metadata for files read, symbols touched, tests run, policy checks, and source references through existing `ContextualUserFragment` and tool output paths. No separate citation runtime. |
| Deterministic final answer verifier | BitGN [answer_formatter.py](../tmp/bitgn-ecom1-exoskeleton-main/answer_formatter.py), [manager_verification.py](../tmp/bitgn-ecom1-exoskeleton-main/manager_verification.py), [ARCHITECTURE.md](../tmp/bitgn-ecom1-exoskeleton-main/articles/ARCHITECTURE.md) | KEEP, narrowed slice closed | session finalization, turn diff tracker, test evidence | Closed slice emits bounded warnings when final assistant answers claim tests, policy checks, or source changes without matching recorded turn evidence. Exact file/command/failure/approval verification remains parked until a current-owner failing test proves the gap. No second formatter model or parallel model loop. |

## Verification-Only Candidates

These are not active feature work. They may become owner-local tests or small fixes only after a concrete current gap is reproduced.

| Candidate | Sources | Allowed action |
| --- | --- | --- |
| Bounded file read/search safety | Qwen [read-file.ts](../tmp/qwen-code/packages/core/src/tools/read-file.ts), [priorReadEnforcement.ts](../tmp/qwen-code/packages/core/src/tools/priorReadEnforcement.ts) | Owner-local tests/fixes for per-turn read evidence or generated-file warnings only. |
| Agent job lifecycle hardening | Qwen [task-stop.ts](../tmp/qwen-code/packages/core/src/tools/task-stop.ts), [task-update.ts](../tmp/qwen-code/packages/core/src/tools/task-update.ts), [task-list.ts](../tmp/qwen-code/packages/core/src/tools/task-list.ts), [agent.ts](../tmp/qwen-code/packages/core/src/tools/agent/agent.ts) | Tests/fixes for idempotent cancellation, monotonic terminal status, and bounded final summary inside existing agent-job owners only. |
| MCP/auth/discovery diagnostics | Qwen [mcp-tool.ts](../tmp/qwen-code/packages/core/src/tools/mcp-tool.ts), [mcp-discovery-timeout.ts](../tmp/qwen-code/packages/core/src/tools/mcp-discovery-timeout.ts), [mcp-workspace-budget.ts](../tmp/qwen-code/packages/core/src/tools/mcp-workspace-budget.ts), [mcp-status.ts](../tmp/qwen-code/packages/core/src/tools/mcp-status.ts) | Add partial-discovery reporting, bounded per-server discovery/call budgets, or auth-status handoff only if current MCP status loses that evidence. |
| Tool search internal reason text | Qwen [tool-search.ts](../tmp/qwen-code/packages/core/src/tools/tool-search.ts), [tool-registry.ts](../tmp/qwen-code/packages/core/src/tools/tool-registry.ts) | Add disabled/source reason metadata only if a test proves current `tool_search` text is incomplete. |
| Hook output safety | Qwen hooks [package](../tmp/qwen-code/packages/core/src/hooks), especially [stopHookCap.ts](../tmp/qwen-code/packages/core/src/hooks/stopHookCap.ts) and [hookRunner.ts](../tmp/qwen-code/packages/core/src/hooks/hookRunner.ts) | Tests/fixes for post-tool truncation/redaction, capped hook-added context, and explicit hook failure behavior on existing hook/context/tool owners. |
| Tool preflight harness hardening | BitGN [security_preflight.py](../tmp/bitgn-ecom1-exoskeleton-main/security_preflight.py), [checkout_preflight.py](../tmp/bitgn-ecom1-exoskeleton-main/checkout_preflight.py), [refund_preflight.py](../tmp/bitgn-ecom1-exoskeleton-main/refund_preflight.py) | Tests/fixes for current Guardian, exec policy, and `apply_patch` preflight only when an unsafe or confusing tool-call path is reproduced. |
| Native sub-agent role harness | BitGN [dispatch_planner.py](../tmp/bitgn-ecom1-exoskeleton-main/dispatch_planner.py), [task_classifier.py](../tmp/bitgn-ecom1-exoskeleton-main/task_classifier.py), SwarmForge [handoff-protocol.md](../tmp/swarm-forge-main/swarmforge/handoff-protocol.md) | Prompt/schema refinements for existing `spawn_agent` roles only. No tmux, file queues, worktree scheduler, or parallel task runtime. |
| Tool output reducers | BitGN [runtime_calls.py](../tmp/bitgn-ecom1-exoskeleton-main/runtime_calls.py), RTK [src/core/filter.rs](../tmp/rtk-main/src/core/filter.rs) | Owner-local reducers for noisy shell/tool output only when they preserve raw evidence access and stay under existing output caps. |
| Exoskeleton review pipeline | BitGN [manager_verification.py](../tmp/bitgn-ecom1-exoskeleton-main/manager_verification.py), [scripts/generate_runs_report.py](../tmp/bitgn-ecom1-exoskeleton-main/scripts/generate_runs_report.py) | Graph-aware review may assemble deterministic facts first, then let the model reason over bounded facts. Must reuse existing review, impact, test-gap, and diff-verification tools. |
| Structured-output compatibility fallback | Qwen [syntheticOutput.ts](../tmp/qwen-code/packages/core/src/tools/syntheticOutput.ts), [nonInteractiveCli.ts](../tmp/qwen-code/packages/cli/src/nonInteractiveCli.ts) | Only if a supported provider cannot honor native output schema. The fallback must remain internal to the final-output owner, be always visible only for schema turns, avoid permission/tool-budget side effects, and preserve the native result/redaction contract. |

## Implementation Order

Single-mode execution only: implement, format, test, review, and close one
candidate before starting the next. Do not run parallel sub-agents or
overlapping build/test commands for this ADR; use `CARGO_BUILD_JOBS=1`.

1. Qwen structured final output enforcement, because it extends an existing protocol contract. Closed first slice: schema shape validation on the native `final_output_json_schema` path. Redaction and conformance diagnostics stay parked without a failing current-owner test.
2. BitGN evidence ledger for tool results, because it improves factual final answers without adding a new runtime.
3. BitGN deterministic final answer verifier, because it prevents unsupported claims. Closed narrowed slice: bounded warnings for unsupported test, policy-check, and source-change claims. Exact file/command/failure/approval verification stays parked without a failing current-owner test.
4. Claude guarded fetch closes by senior-narrowed existing-owner proof: hosted
   `web_search` already supports guarded open-page/find-in-page fetch-style
   actions behind provider/config/standalone-web gates. Do not add a parallel
   `web_fetch` tool until a supported provider/API surface exists.
5. Stop. Do not dispatch verification-only candidates until a failing current-owner test or runtime bug is identified.

## Qwen Structured Output-Specific Keep

The donor mechanism uses a synthetic terminal tool because that codebase needs a function-call-shaped final answer contract. Ontocode already has a native `final_output_json_schema` contract, so keep the behaviors and map them to existing owners.

| Proposal | Source | Decision |
| --- | --- | --- |
| Strict schema input validation | Qwen [config.ts](../tmp/qwen-code/packages/cli/src/config/config.ts) | Keep: reject empty, unreadable, oversized, non-object, object-incompatible, or un-compilable schemas before the turn starts. |
| Structured payload redaction | Qwen [telemetry/types.ts](../tmp/qwen-code/packages/core/src/telemetry/types.ts), [syntheticOutput.ts](../tmp/qwen-code/packages/core/src/tools/syntheticOutput.ts) | Keep: preserve success/duration/decision metrics while removing final structured payloads from telemetry, resume, logs, and UI mirrors. |
| Native conformance diagnostics | Qwen [nonInteractiveCli.ts](../tmp/qwen-code/packages/cli/src/nonInteractiveCli.ts) | Keep: when a schema-constrained turn returns plain text or invalid structured output, surface a bounded diagnostic with turn count and preview instead of silently accepting prose. |
| Terminal synthetic-tool behavior | Qwen [syntheticOutput.ts](../tmp/qwen-code/packages/core/src/tools/syntheticOutput.ts), [tool-registry.test.ts](../tmp/qwen-code/packages/core/src/tools/tool-registry.test.ts), [permission-manager.test.ts](../tmp/qwen-code/packages/core/src/permissions/permission-manager.test.ts) | Keep only as fallback design notes for providers lacking native schema output. Default Ontocode implementation stays on `final_output_json_schema` and must not expose a normal user-facing `structured_output` tool. |

## RTK-Specific Keep

The RTK review confirms the prior `ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md` direction. No new architecture owner is added.

| Proposal | Source | Decision |
| --- | --- | --- |
| Shell output compression (C0 reducers) | RTK [src/core/filter.rs](../tmp/rtk-main/src/core/filter.rs), [src/cmds](../tmp/rtk-main/src/cmds) | Keep only as bounded CLI output reduction (e.g., squashing test runner spam or package manager progress bars) integrated into the existing shell formatting path. Validates the Native Context Tools Core Engine ADR. |
| Token saving telemetry | RTK [src/analytics/gain.rs](../tmp/rtk-main/src/analytics/gain.rs) | Keep only to surface token reduction metrics in existing telemetry/turn context. |

## Haft-Specific Keep

| Proposal | Source | Decision |
| --- | --- | --- |
| Parity/Claim Vocabulary | Haft [internal/artifact/parity_schema.go](../tmp/haft-main/internal/artifact/parity_schema.go), [internal/fpf](../tmp/haft-main/internal/fpf) | Keep only as semantic terminology (claims, evidence, parity) for structuring diagnostic and test output. |

## BitGN Exoskeleton-Specific Keep

The donor pattern is useful as a harness discipline, not as a new framework: the model dispatches intent, while existing Rust owners validate, bound, execute, and verify.

| Proposal | Source | Decision |
| --- | --- | --- |
| Model proposes, code disposes | BitGN [ARCHITECTURE.md](../tmp/bitgn-ecom1-exoskeleton-main/articles/ARCHITECTURE.md), [agent.py](../tmp/bitgn-ecom1-exoskeleton-main/agent.py), [main.py](../tmp/bitgn-ecom1-exoskeleton-main/main.py) | Keep as architectural guidance for tools: model output is an intent proposal; Rust tool owners perform validation, policy, bounds, and execution. |
| Bounded evidence before prose | BitGN [evidence_ledger.py](../tmp/bitgn-ecom1-exoskeleton-main/evidence_ledger.py), [submission_refs.py](../tmp/bitgn-ecom1-exoskeleton-main/submission_refs.py) | Keep as a session/context requirement: final prose must be backed by bounded facts already recorded by tool and session owners. |
