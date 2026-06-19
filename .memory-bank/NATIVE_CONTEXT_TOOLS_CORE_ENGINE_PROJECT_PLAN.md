---
name: Native Context Tools Core Engine Project Plan
description: Pre-junior implementation plan for the challenged native context-tools ADR
type: project_plan
date: 2026-06-15
status: done
---

# Native Context Tools Core Engine Project Plan

## Goal

Implement only the first safe slice from [ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md](ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md): deterministic shell output reducers inside existing Ontocode core output formatting.

This is not a lean-ctx port. Do not add a new tool registry, shell launcher, model-visible `context_*` tool, search index, persistent cache, background worker, app-server API, or donor dependency.

## Source ADR

Authoritative source: [ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md](ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md)

Binding interpretation:

- First implementation slice is `C0: Shell Output Reducers`.
- C0 changes formatting only, not shell execution policy.
- C1 bounded read is a later slice and must use environment filesystem / permission boundaries.
- C2 exposure review, C3 search, cache, archive pointers, context fragments, persistence, and workers are blocked until separate approval.

## OntoIndex Evidence

OntoIndex was used before this plan.

Relevant existing owners:

- `ontocode-rs/core/src/tools/mod.rs`
  - owns `format_exec_output_str`
  - calls `utils/output-truncation`
- `ontocode-rs/core/src/tools/events.rs`
  - owns `ToolEmitter::format_exec_output_for_model`
  - owns one model-bound output formatting surface via `ToolEmitter::finish`
- `ontocode-rs/utils/output-truncation/src/lib.rs`
  - owns generic byte/token truncation
  - should remain generic
- `ontocode-rs/core/src/tools/handlers/shell.rs`
  - owns shell handler flow
  - must not gain a parallel launcher
- `ontocode-rs/core/tests/suite/truncation.rs`
  - existing model-output truncation coverage
- `ontocode-rs/core/src/tools/handlers/shell_tests.rs`
  - existing shell-handler behavior coverage

## Non-Goals

- Do not add `context_read`, `context_search`, or `context_shell`.
- Do not add model-visible tools.
- Do not change shell permission, approval, sandbox, timeout, or command execution behavior.
- Do not create a compression daemon, cache, archive store, or background worker.
- Do not add dependencies.
- Do not edit app-server APIs, schemas, config, or protocol.
- Do not copy donor lean-ctx code.
- Do not use raw `std::fs` for any future model-visible read path.

## Stage 0: Baseline And Impact

Purpose: make juniors inspect before editing.

Files to read:

- `ontocode-rs/core/src/tools/mod.rs`
- `ontocode-rs/utils/output-truncation/src/lib.rs`
- `ontocode-rs/core/tests/suite/truncation.rs`
- `ontocode-rs/core/src/tools/handlers/shell.rs`
- `ontocode-rs/core/src/tools/handlers/shell_tests.rs`

Required OntoIndex checks before code edits:

```text
impact: format_exec_output_str
impact: format_exec_output_for_model
related: formatted_truncate_text
```

Done when:

- Reviewer has impact summary for edited symbols.
- No HIGH/CRITICAL warning is ignored.
- Implementation target is chosen.

## Stage 1: Add Reducer Type And No-Op Path

Purpose: introduce the smallest formatting seam without behavior change.

Required new private module:

- `ontocode-rs/core/src/tools/output_reducer.rs`

Steps:

- Add a private reducer helper used before final generic truncation.
- Prove whether `format_exec_output_str` and `ToolEmitter::format_exec_output_for_model` share the reducer or intentionally stay separate.
- If they stay separate, document why both model-bound surfaces do not need the same reducer.
- The first commit may return the original content unchanged.
- Do not make it public unless tests require it.
- Do not touch shell execution.

Done when:

- Existing truncation tests still pass.
- Output is unchanged before reducer rules are added.

## Stage 2: Rust Build/Test Reducer

Purpose: reduce noisy successful Rust build/test output while preserving failures.

Rules:

- If output contains obvious Rust compiler errors, keep error blocks and file/line references.
- If output is a successful verbose build/test, summarize counts and keep final status.
- Keep exit code, timeout, and duration metadata from the existing formatter.
- Do not hide warnings unless the command succeeded and output exceeds the budget.

Suggested test fixtures:

- successful Cargo build output
- Rust compiler error with file path and line number
- failed Rust test output with test name

Done when:

- Failure fixtures retain actionable lines.
- Success fixture is shorter than input.

## Stage 3: Python Unittest And `rg` Shell-Output Reducers

Purpose: add two low-risk shell-output reducers after Rust behavior is covered.

Python unittest rules:

- Keep failing test names and traceback headers.
- Keep final `FAILED` or `OK` line.
- Summarize repeated successful dots.

`rg` shell-output rules:

- Cap match lines.
- Keep file paths and line numbers.
- Report omitted match count when truncated.
- Do not add `context_search`, a search wrapper, or a search index.

Done when:

- Python failure fixture keeps traceback target.
- `rg` fixture keeps first matches and omitted count.

## Stage 4: Git Status And Generic Log Reducers

Purpose: reduce common long but low-risk output.

Git status rules:

- Group by status code.
- Keep untracked/modified/deleted counts.
- Do not expand thousands of paths unless output is already small.

Generic log rules:

- Keep first lines, last lines, and obvious error lines.
- Report omitted line count.
- Do not run LLM summarization.

Done when:

- Large status fixture is compact.
- Generic log fixture keeps errors.

## Stage 5: Redaction And Model-Context Guard

Purpose: prove reducers do not make secret exposure worse.

Rules:

- Do not add a new redactor.
- Name the existing redaction owner if it is in the call path; otherwise declare redaction out of scope for this slice.
- Add tests with token-like strings for every reducer that stores, groups, reorders, or duplicates output.
- Prove reducers do not persist raw output.
- Prove reduced output still respects existing truncation budget.

Done when:

- No secret-looking fixture appears unexpectedly.
- Reduced output remains bounded.

## Stage 6: Verification And Documentation

Required commands from `ontocode-rs/`:

```bash
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p ontocode-core
CARGO_BUILD_JOBS=8 just fix -p ontocode-core
```

Additional focused checks:

```bash
cd /opt/demodb/_workfolder/ontocode
git diff --check -- <touched-files>
git diff --check
ontoindex analyze --skills --skip-agents-md
```

Done when:

- Tests pass.
- OntoIndex refresh succeeds.
- `.memory-bank/NATIVE_CONTEXT_TOOLS_CORE_ENGINE_TRACKING.md` records changed behavior and commands run.
- Audit note links final accepted state.

## Pre-Junior Task Cards

### NC-0 Baseline Impact

Owner: senior or supervised pre-junior.

Files:

- `.memory-bank/ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md`
- `.memory-bank/NATIVE_CONTEXT_TOOLS_CORE_ENGINE_PROJECT_PLAN.md`

Steps:

- Read the ADR.
- Run OntoIndex impact on `format_exec_output_str`.
- Run OntoIndex impact on `format_exec_output_for_model`.
- Record owner files and risk.
- Create or update `.memory-bank/NATIVE_CONTEXT_TOOLS_CORE_ENGINE_TRACKING.md` before dispatch.

Done when:

- A reviewer can see blast radius before edits.

### NC-1 No-Op Reducer Seam

Owner: pre-junior.

Files:

- `ontocode-rs/core/src/tools/mod.rs`
- `ontocode-rs/core/src/tools/events.rs`
- `ontocode-rs/core/src/tools/output_reducer.rs`

Steps:

- Add private reducer function.
- Call it before `formatted_truncate_text`.
- Wire or explicitly reject wiring for both known model-output surfaces.
- Return input unchanged.
- Add one unit test for unchanged output.

Done when:

- Existing output tests pass unchanged.

### NC-2 Rust Output Reducer

Owner: pre-junior.

Files:

- reducer file from NC-1
- focused test file near existing truncation tests

Steps:

- Add Rust success fixture.
- Add Rust compiler-error fixture.
- Implement minimal line filtering.

Done when:

- Success output is shorter.
- Failure output preserves file/line and error text.

### NC-3 Python Unittest Reducer

Owner: pre-junior.

Files:

- reducer file from NC-1
- focused test file

Steps:

- Add Python unittest fixtures.
- Keep reducers pattern-based and deterministic.

Done when:

- Fixtures pass.
- No command execution behavior changes.

### NC-4 `rg` Shell-Output Reducer

Owner: pre-junior.

Files:

- reducer file from NC-1
- focused test file

Steps:

- Add `rg` output fixture.
- Keep file paths and line numbers.
- Report omitted match count.
- Do not add a search wrapper or tool.

Done when:

- `rg` fixture keeps first matches and omitted count.

### NC-5 Git Status Reducer

Owner: pre-junior.

Files:

- reducer file from NC-1
- focused test file

Steps:

- Add large git-status fixture.
- Implement status grouping.

Done when:

- Output is bounded and still actionable.

### NC-6 Generic Log Reducer

Owner: pre-junior.

Files:

- reducer file from NC-1
- focused test file

Steps:

- Add generic log fixture with error lines.
- Implement first-line, last-line, and error-line summary.
- Do not run LLM summarization.

Done when:

- Generic log fixture keeps errors and omitted-line count.

### NC-7 Redaction And Budget Guard

Owner: senior.

Files:

- reducer file from NC-1
- focused test file

Steps:

- Add token-like fixture for each grouping/reordering reducer.
- Prove reducers do not persist raw output.
- Prove final output still respects existing truncation budget.

Done when:

- Secret-looking fixture text does not appear more often than in the input.
- Reduced output remains bounded.

### NC-8 Final Verification

Owner: manager.

Steps:

- Update `.memory-bank/NATIVE_CONTEXT_TOOLS_CORE_ENGINE_TRACKING.md`.
- Run format.
- Run scoped tests.
- Run fix.
- Run scoped `git diff --check -- <touched-files>`.
- Run full `git diff --check` if the dirty worktree allows it.
- Refresh OntoIndex.
- Update tracking/audit memory.

Done when:

- All accepted commands are recorded.

## Blocked Later Work

These are not part of this project plan:

- C1 bounded read implementation.
- C2 model-visible read exposure review.
- C3 bounded exact search wrapper.
- session-local cache.
- evidence archive pointers.
- context fragment bridge.
- persistent operational evidence records.
- background compression workers.

Create a new plan before starting any of those.

## Rejection Rules

Reject a patch if it:

- adds a model-visible `context_*` tool
- changes shell approval, sandbox, timeout, or execution behavior
- adds a new shell launcher
- adds a new search index
- adds a search wrapper or model-visible search tool
- adds dependencies
- persists raw output
- copies donor lean-ctx code
- adds app-server or config surface
- hides failing test/build errors
- removes file paths or line numbers from failure output
