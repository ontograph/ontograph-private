name: Oh My Pi Donor Keep Refactor Map Pre-Junior Project Plan
desc: Junior-safe, test-first first slice from ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md
type: project_plan
date: 2026-06-16
status: challenged

# Oh My Pi Donor Keep Refactor Map Pre-Junior Project Plan

## Goal

Implement only the first low-risk, test-first slice from
[ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md](ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md).

This is not a 127-item work order. Each stage must add or extend the smallest
test in the existing owner. If existing coverage already proves the behavior,
record that and do not change code.

## Challenge Review

- Do not dispatch a full stage to one pre-junior worker. Dispatch one ADR row, one owner, and one failing test at a time.
- Package names verified from `Cargo.toml`: `ontocode-core`, `ontocode-mcp`, `ontocode-hooks`, `ontocode-state`, and `ontocode-apply-patch`.
- `ontocode-rs/core/src/context/` exists; `ontocode-rs/core/src/ctx/` does not.
- Hot owner files are already large. Add or extend sibling tests first, and extract a helper only when the helper is reused by the same patch.
- Schema-cycle, redaction, cancellation, and compaction work must stay test-first. If a test cannot be expressed with existing owner APIs, stop for senior review.
- Claude Code donor rows parked as `NARROW`, `DEFER`, or `REJECT` are not pre-junior scope. This plan may keep only the narrower Oh My Pi accepted test rows and must not reopen the broader Claude MCP, hook, context/cache, command/debug, UI, release, eval, plugin, or agent-runtime ideas.

## Claude Code Consolidation Review

Do not dispatch these duplicate or broader Claude Code parked areas from this Oh My Pi pre-junior plan:

- MCP/resource/debug overlap: Claude rows 122, 123, 128-130, and 145-147.
- Context/compaction/prompt-cache overlap: Claude rows 073, 084, 089, 090, 094, 095, and 187.
- Hook overlap: Claude rows 097 and 101-104.
- Agent/job/session overlap: Claude rows 057-059 and 148-150.
- UI/release/eval/plugin overlap: Claude rows 161-180 and 181-200.

The retained Oh My Pi rows below are allowed only as test-first hardening of existing owners. If an implementation needs new runtime state, new prompt-cache behavior, a new MCP browser/debugger, a second hook registry, a new eval framework, or agent protocol/persistence changes, stop and move it back to the relevant ADR.

## Approved First Slice

1. MCP lifecycle hardening: ADR rows 121, 122, 124, 127, and 130.
2. Apply-patch safety: ADR rows 23, 26, 27, and 31-40.
3. Context cap and compaction safety: ADR rows 81-83, 85, 86, and 88-90.
4. Hook load, trust, and redaction: ADR rows 103, 104, and 111-120.
5. Agent job cleanup and structured results: ADR rows 141, 144-146, 148, and 150.

Row 128 is conditional. Do not implement MCP reconnect/backoff tests unless the
current MCP manager already exposes retry/backoff state.

## Non-Goals

- Do not import Oh My Pi code.
- Do not add public API, config, SDK, schema, persisted state, or new context fragments.
- Do not add DAP, browser control, notebook execution, virtual URI schemes, or a persistent language worker.
- Do not create a second MCP lifecycle, hook matcher, provider registry, memory service, tool runtime, shell runtime, or fixture framework.
- Do not grow hot files casually: `hooks/src/engine/discovery.rs`, `codex-mcp/src/connection_manager.rs`, `tui/src/app/session_lifecycle.rs`, and `core/src/tools/handlers/agent_jobs.rs`.

## Stage 0: Preflight

Read:

- [ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md](ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md)
- [OH_MY_PI_DONOR_200_SOLUTIONS_NON_KEEP.md](OH_MY_PI_DONOR_200_SOLUTIONS_NON_KEEP.md)
- [ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md)
- `ontocode-rs/core/tests/common/responses.rs`
- `ontocode-rs/core/tests/common/test_codex.rs`

Checks:

- Run OntoIndex context or impact on the exact owner symbol before editing Rust.
- Record the chosen owner file in the task note.
- Pick exactly one ADR row from the stage, not the whole stage.
- Stop if the task needs a new runtime concept.

Acceptance:

- Owner file is named.
- ADR row is named.
- No code changed in Stage 0.

## Stage 1: MCP Lifecycle Hardening

Owner:

- `ontocode-rs/codex-mcp/src/connection_manager.rs`
- `ontocode-rs/core/src/mcp_tool_call_tests.rs`
- Existing RMCP/MCP fixture helpers

Task:

- Add tests for partial server failure, unified MCP error mapping, MCP/OAuth redaction, and schema cycle guards.
- Prefer test-only fixture servers.
- Do not add new MCP lifecycle state unless a failing test proves the current owner cannot express the case.
- Do not implement Claude parked MCP rows 122, 123, 128-130, or 145-147 here. No MCP source browser, teaching server, command debugger, explorer UI, or new metadata surface.
- Pre-junior dispatch limit: choose one of rows 121, 122, 124, 127, or 130 per patch.

Acceptance:

- One MCP server can fail without hiding healthy servers.
- Redacted diagnostics never expose tokens or OAuth material.
- Tool schema cycles fail safely.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

If touching `codex-mcp/` directly, run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-mcp
```

## Stage 2: Apply-Patch Safety

Owner:

- `ontocode-rs/apply-patch/`
- `ontocode-rs/core/tests/suite/apply_patch_cli.rs`
- `ontocode-rs/core/tests/suite/shell_serialization.rs`
- `ontocode-rs/core/tests/common/responses.rs`

Task:

- Add tests for stale-edit recovery, ambiguous line fallback, malformed patch rejection, create/update/move/delete failures, no-write parse failure, large output truncation, and exact changed-file verification.
- Reuse existing apply-patch harnesses and response helpers.
- Pre-junior dispatch limit: choose one row or one tightly coupled parser failure group per patch.

Acceptance:

- Malformed patches do not write files.
- Failure output is bounded and model-visible only where intended.
- Move/delete edge cases are deterministic.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

If touching `apply-patch/` directly, run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-apply-patch
```

## Stage 3: Context And Compaction Safety

Owner:

- `ontocode-rs/core/src/session/turn.rs`
- `ontocode-rs/core/src/compact.rs`
- `ontocode-rs/core/src/session/turn_context.rs`
- `ontocode-rs/core/tests/suite/compact.rs`
- `ontocode-rs/core/tests/suite/compact_remote.rs`

Task:

- Add tests for overflow retry, same-model retry loop guard, compaction request shape stability, bounded compaction failure events, hard context caps, existing prompt-cache stability, and reinjection after summary.
- Prefer existing compact snapshot tests.
- Do not change compaction behavior unless the failing test proves a real gap.
- Do not implement Claude parked context/cache rows 073, 084, 089, 090, 094, 095, or 187 here. No new context fragment, diagnostics-only context mutation, speculative cache, or golden-prompt/eval asset.
- Pre-junior dispatch limit: choose one compaction behavior per patch.

Acceptance:

- Tests do not require a live provider.
- Context items stay capped.
- Compaction failures are bounded and distinguishable.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

## Stage 4: Hook Load, Trust, And Redaction

Owner:

- `ontocode-rs/hooks/`
- `ontocode-rs/core/src/hook_runtime.rs`
- Existing hook tests and schema fixtures

Task:

- Add tests for invalid regex warning, glob-gated selectors, partial hook load errors, startup with one bad hook and one good hook, hook schema fixtures, hook timeout, hook order, output caps, trust prompts, config layer merge, and secret redaction.
- Prefer sibling tests. Do not grow `hooks/src/engine/discovery.rs` unless the edit is trivial.
- Do not implement Claude parked hook rows 097 or 101-104 here. No second hook registry, no new hook policy layer, and no model-context hook output beyond existing capped behavior.
- Pre-junior dispatch limit: choose one hook behavior per patch.

Acceptance:

- A bad hook does not disable all valid hooks.
- Hook output is bounded.
- Secrets are redacted.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

If touching `hooks/` directly, run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-hooks
```

## Stage 5: Agent Job Cleanup And Structured Results

Owner:

- `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents*`
- `ontocode-rs/core/src/agent/`
- `ontocode-rs/state/src/runtime/agent_jobs.rs`

Task:

- Add tests for typed subagent results, progress snapshots, job aggregation, cancellation, strict structured-result parsing, and orphan cleanup.
- Prefer sibling test files or extracted helpers. Do not grow `agent_jobs.rs` unless the edit is trivial.
- Pre-junior dispatch limit: choose one agent behavior per patch.
- Do not change agent protocol or persisted state shape.
- Do not implement Claude parked agent/session rows 057-059 or 148-150 here. No scheduler changes, parent/child job model, new session command behavior, or persisted-state expansion.

Acceptance:

- Canceled or failed subagents are cleaned up.
- Job progress remains deterministic.
- Structured-result parsing rejects prose when JSON is required.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

If touching `state/` directly, run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Closure Checklist

- Run `just fmt` in `ontocode-rs` after code changes.
- Run `CARGO_BUILD_JOBS=8 just fix -p <changed-package>` before finalizing Rust changes.
- Run the smallest relevant package test listed in the changed stage.
- If any TUI surface changes, also run `CARGO_BUILD_JOBS=8 just test -p ontocode-tui` and inspect snapshots.
- Update this file status only after tests pass.

## Stop Conditions

Stop and ask for review if a task requires:

- new runtime architecture
- new public API/config/SDK/schema
- new persisted state
- new model-visible context fragment
- changes outside the listed owner files
- changes over roughly 300 lines
