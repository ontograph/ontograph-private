name: Gemini CLI Donor Context/Tools/Agents/Evals Pre-Junior Project Plan
desc: Junior-safe, test-first plan for the first approved slices from ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS.md
type: project_plan
date: 2026-06-16
status: closed-no-dispatch

# Gemini CLI Donor Context/Tools/Agents/Evals Pre-Junior Project Plan

## Goal

Close the pre-junior implementation plan for
[ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS.md](ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS.md).

OntoIndex-backed review found no remaining Gemini-specific pre-junior slice
that is both new and a core functionality extension. Future Gemini donor work
needs a fresh manager card with one failing core regression test before
implementation.

## Closure Decision

No tasks are currently dispatchable from this plan.

The previously retained context-fidelity slice is already covered by existing
TUI/core tests:

- `ontocode-rs/tui/src/ide_context/prompt.rs` covers active file, selected
  content, selected ranges, prompt delimiter/user request injection, truncation,
  and open-tab bounds.
- `ontocode-rs/tui/src/chatwidget/tests/composer_submission.rs` covers hiding
  IDE prompt context from displayed user-message text.
- `ontocode-rs/core/tests/suite/additional_context.rs` covers model-visible
  additional context and user-message preservation.

The previous compression, model-visible tool error, and golden response fixture
items are duplicated by
[ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md](ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md)
and must be dispatched from that ADR if still needed. Claude Code donor items
classified as `NARROW`, `DEFER`, or `REJECT` are parked in
[ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md)
and are not dispatchable from this Gemini plan.

Everything else in the Gemini ADR is backlog until a manager creates a separate
task that names a current-codebase owner, proves the behavior is missing, and
extends existing core functionality.

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

## Revival Rule

Move work back out of this closure only when all of these are true:

- OntoIndex context/impact identifies a current core owner and missing behavior.
- The task extends existing core functionality rather than adding duplicate
  coverage, TUI-only formatting, or a parallel runtime.
- The card includes one failing core regression test shape before runtime
  changes.
- The task stays under the Gemini ADR dispatch owner table or a newer accepted
  ADR that supersedes it.

## Stop Conditions

Stop and ask for review if a task requires:

- new runtime architecture
- new public API/config/SDK/schema
- new persisted state
- a new context fragment
- provider/auth/MCP/hook/shell/scheduler changes outside the listed owner files
- changes over roughly 300 lines
