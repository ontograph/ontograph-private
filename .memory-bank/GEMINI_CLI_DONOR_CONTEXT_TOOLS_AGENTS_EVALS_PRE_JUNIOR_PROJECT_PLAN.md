name: Gemini CLI Donor Context/Tools/Agents/Evals Pre-Junior Project Plan
desc: Junior-safe, test-first plan for the first approved slices from ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS.md
type: project_plan
date: 2026-06-16
status: narrowed

# Gemini CLI Donor Context/Tools/Agents/Evals Pre-Junior Project Plan

## Goal

Implement only the first low-risk, test-first slices from
[ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS.md](ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS.md).

This plan is written for pre-junior implementers. Every task must stay in the
ADR dispatch owner table, add tests before runtime changes, and stop when the
existing owner already has adequate coverage.

## Approved First Slice

Only this Gemini-specific item remains approved for pre-junior dispatch:

1. context-fidelity tests

The previous compression, model-visible tool error, and golden response fixture
items are duplicated by [ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md](ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md)
and must be dispatched from that ADR if still needed. Claude Code donor items
classified as `NARROW`, `DEFER`, or `REJECT` are parked in
[ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md)
and are not dispatchable from this Gemini plan.

Everything else in the Gemini ADR is backlog until a manager creates a separate task.

## Consolidated Duplicate Scope

Removed from this Gemini pre-junior plan:

- Context compression failure-mode tests. Covered by Oh My Pi ADR rows 81-83 and 85-90.
- Model-visible tool error tests. Covered by Oh My Pi ADR rows 36, 48, 63, 66, 70, 116, 127, 181-187, and 190 where relevant.
- Golden response fixtures for deterministic agent behavior. Covered by Oh My Pi ADR rows 31-33, 40, and the `AUTO` eval/script rows when fixture work is repeated.
- Claude Code context, memory, prompt-cache, speculation, hook, MCP, command, UI, release, plugin, and eval duplicates. Covered by the parked Claude Code donor rows, especially 066-078, 081-095, 097, 101-104, 106-130, 131-160, 161-180, and 181-200.

Do not dispatch these from this Gemini plan; use the Oh My Pi ADR owner map or the Claude Code parked ADR to avoid duplicate implementation.

## Non-Goals

- Do not import Gemini CLI code.
- Do not edit provider, auth, MCP, hook, shell, scheduler, or extension architecture.
- Do not add public config, app-server APIs, SDK surfaces, persistent formats, or model-visible context fragments.
- Do not add new telemetry fields unless a reviewer explicitly approves the exact existing telemetry owner.
- Do not build a new eval framework. Use existing Rust test and snapshot harnesses.

## Stage 0: Preflight

Purpose: make sure the implementer understands the current test homes before editing.

Read:

- [ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS.md](ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS.md)
- [ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_DEFERRED_TODO.md](ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_DEFERRED_TODO.md)
- [ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md](ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md)
- [ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md)
- `ontocode-rs/core/tests/common/context_snapshot.rs`
- `ontocode-rs/core/tests/suite/additional_context.rs`

Checks:

- Run OntoIndex context on the owner symbol or file named in the stage before editing Rust.
- Record the intended owner file in the task note before editing.
- If the intended change needs a new runtime concept, stop and ask for manager review.

Acceptance:

- The implementer can name the owner file for the slice.
- No files are changed in Stage 0.

## Stage 1: Context Fidelity Tests

Owner:

- `ontocode-rs/core/tests/common/context_snapshot.rs`
- `ontocode-rs/core/tests/suite/additional_context.rs`
- `ontocode-rs/tui/src/ide_context/prompt.rs`

Task:

- Add or extend the smallest test proving active file, selected range, and user prompt context render predictably.
- Prefer snapshot coverage if the output is already snapshot-shaped.
- Do not add a new context builder.
- Do not implement Claude Code parked rows 073, 084, 089, 090, 094, 095, or 187 here. This stage is only a narrow regression test for existing context rendering.

Acceptance:

- Test fails if active file, selection, or prompt text disappears.
- Test output is bounded and deterministic.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

If touching `tui/src/ide_context/*`, run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-tui
```

## Closure Checklist

- Run `just fmt` in `ontocode-rs`.
- Run `CARGO_BUILD_JOBS=8 just fix -p ontocode-core` if `ontocode-core` changed.
- Run `CARGO_BUILD_JOBS=8 just fix -p ontocode-tui` if `ontocode-tui` changed.
- Run `CARGO_BUILD_JOBS=8 just test -p ontocode-core` if `ontocode-core` changed.
- Run `CARGO_BUILD_JOBS=8 just test -p ontocode-tui` if `ontocode-tui` changed.
- Update this plan status only after tests pass.
- Do not accept snapshot updates blindly; inspect each `.snap.new` before accepting.

## Stop Conditions

Stop and ask for review if a task requires:

- new runtime architecture
- new public API/config/SDK/schema
- new persisted state
- a new context fragment
- provider/auth/MCP/hook/shell/scheduler changes outside the listed owner files
- changes over roughly 300 lines
