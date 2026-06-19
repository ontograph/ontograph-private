---
name: Native Context Tools Core Engine Tracking
description: Dispatch and verification ledger for the C0 shell-output reducer slice
type: tracking
date: 2026-06-15
status: done
---

# Native Context Tools Core Engine Tracking

Authority:

- [ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md](ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md)
- [NATIVE_CONTEXT_TOOLS_CORE_ENGINE_PROJECT_PLAN.md](NATIVE_CONTEXT_TOOLS_CORE_ENGINE_PROJECT_PLAN.md)

Scope: C0 shell-output reducers only. Do not add `context_*` tools, search wrappers, shell launchers, caches, archive stores, background workers, app-server APIs, config, dependencies, or donor lean-ctx code.

## Dispatch Rules

- Update this file before starting each task.
- Use OntoIndex impact before editing any target symbol.
- Refresh OntoIndex after each accepted implementation task.
- Keep reducers deterministic and private under existing `ontocode-core` tool formatting owners.
- Preserve shell approval, sandbox, timeout, and execution behavior.

## Task Ledger

| ID | Task | Status | Notes |
| --- | --- | --- | --- |
| NC-0 | Baseline impact | done | `format_exec_output_str` CRITICAL: direct callers include shell handler, user-shell command, user shell task, exec event emitter, and timeout test. `formatted_truncate_text` CRITICAL: shared truncation utility with non-shell callers. `ToolEmitter::format_exec_output_for_model` LOW: direct caller is `ToolEmitter::finish`. No hard blocker; narrow NC-1 to no-op seam before reducer behavior. |
| NC-1 | No-op reducer seam | done | Added private `core/src/tools/output_reducer.rs` no-op `Cow` seam and wired both `format_exec_output_for_model` and `format_exec_output_str` through it. `events.rs` unchanged because `ToolEmitter` delegates to `super::format_exec_output_for_model`. Focused Cargo unit test passed; `just test -p ontocode-core output_reducer` is blocked by existing `test-binary-support` PATH alias collision before test execution. |
| NC-2 | Rust output reducer | done | Implemented fixture-first Rust/Cargo output reducer in `output_reducer.rs`. Preserves compiler errors, file paths/line numbers, failed test names, and failed summary; shortens verbose successful Rust output. Focused Cargo reducer tests passed after manager rerun. |
| NC-3 | Python unittest reducer | done | Added deterministic Python unittest reducer in `output_reducer.rs`. Preserves failing test names, traceback headers/targets, and final `FAILED`/`OK` line; summarizes long dot runs. Focused reducer tests passed. |
| NC-4 | `rg` shell-output reducer | done | Added command-output-only `rg` reducer. Long plain `path:line:match` / `path:line:col:match` output keeps first matches and reports omitted match count; non-`rg` colon text remains unchanged. Focused reducer tests passed. |
| NC-5 | Git status reducer | done | Added large `git status --short` reducer with status counts for modified/untracked/deleted/renamed and other porcelain categories. Small status and non-status output stay unchanged. Focused reducer tests passed. |
| NC-6 | Generic log reducer | done | Moved reducer tests to sibling `output_reducer_tests.rs`, added deterministic long-log reducer that keeps first/last edge lines plus obvious error lines, and reports omitted-line count without LLM summarization. Scoped Cargo test and diff check passed. |
| NC-7 | Redaction and budget guard | done | Added token-like fixtures across generic log, git status, rg, Python unittest dots, and Rust success reducers; proved no token-like duplication and verified final truncation still applies after reduction. Scoped reducer test and diff check passed. |
| NC-8 | Final verification | done | `just fmt` passed. `cargo test -p ontocode-core --lib output_reducer -- --nocapture` passed with 14 tests. The prior `test-binary-support` PATH alias collision is fixed in `ontocode-arg0`; `just test -p ontocode-arg0` passed. Full `just test -p ontocode-core` passed with a fresh `TMPDIR` after avoiding a reused temp root polluted by `.codex`. `just fix -p ontocode-core` completed with existing warnings. Scoped and full `git diff --check` passed. Audit note added. |
| NC-9 | Default-on reducer wiring | done | ADR updated to accept C0 default-on shell output reduction. OntoIndex marks `format_exec_output_str` and `format_exec_output_for_model` DANGEROUS because they are shared public formatting APIs; scope stayed limited to `reduce -> existing truncation` with no new config/tool/shell launcher. Focused reducer and caller formatting tests passed. |

## Current State

- Plan reviewed and narrowed on 2026-06-15.
- NC-0 baseline impact is complete.
- CRITICAL risk is acknowledged for shared formatting helpers; implementation must stay staged, fixture-first, and no-op before behavior changes.
- NC-1 no-op reducer seam is complete.
- NC-2 Rust output reducer is complete.
- NC-3 Python unittest reducer is complete.
- NC-4 `rg` shell-output reducer is complete.
- NC-5 git status reducer is complete.
- `output_reducer.rs` is now below the repo's large-module threshold after moving the test module into sibling `output_reducer_tests.rs`.
- NC-6 generic log reducer is complete.
- NC-7 redaction and budget guard is complete.
- NC-8 final verification is complete.
- NC-9 default-on reducer wiring is complete; shell/tool formatting now reduces recognized noisy output before existing final truncation by default.
