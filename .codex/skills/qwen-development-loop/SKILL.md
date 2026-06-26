---
name: qwen-development-loop
description: Bounded code-development loop for candidate rows from QWEN_CODE_2000_USEFUL_APPROACHES_REVIEW.md
---

# Qwen Code 2000 Development Loop

Use this skill to process KEEP-CANDIDATE rows from `.memory-bank/QWEN_CODE_2000_USEFUL_APPROACHES_REVIEW.md` safely. You must loop through all available candidates until the entire list is processed.

## Prerequisites

1. Identify the next unprocessed candidate(s) from `.memory-bank/QWEN_CODE_2000_MANAGER_LOOP_TRACKING.md`.
2. Update the tracking file to mark the candidate as `in progress` before starting it.
3. Use OntoIndex to inspect the current owner and impact.

## Rules

- **Continuous Processing:** Do not stop after one candidate. Process the next available candidate immediately after closing the current one. Continue until all 50 tasks are processed.
- **Code Proof:** Before coding, prove a concrete owner-local gap: failing behavior, missing compatibility test, or explicit product requirement.
- **No-Gap Fallback:** If no gap is proven, mark the candidate COVERED/NO-DISPATCH in tracking and move directly to the next candidate.
- **Architectural Guardrails:** Do not create side stacks, public APIs, config/schema changes, daemon owners, or broad refactors. Keep changes strictly inside the existing owner.
- **Test-First:** Prefer adding a focused regression test first. Change production code only if that test exposes a real bug or missing behavior.
- **Minimal Patch:** One candidate, one owner, one test surface.
- **Indexing:** Use single OntoIndex refreshes only; no parallel build/index processes.

## Required Sub-Agent Delegation

- **senior-reviewer:** Request `claude-sonnet-4-6`, fallback `gpt-5.4-mini`. Challenges whether the candidate is worth code work.
- **implementation-worker:** Request `gemini-3.5-flash-low` and omit `reasoning_effort`, fallback `gpt-5.3-codex-spark`, `gpt-5.4-mini`. Writes the smallest failing/regression test in the existing owner test suite. If the test fails, applies the smallest production fix.
- **verification-worker:** Request `gpt-5.4-mini`. Checks changed files, tests, scope, and missing evidence.

## Verification Steps (per candidate)

1. Run `just fmt` in `ontocode-rs`.
2. Run the narrowest relevant test, e.g. `CARGO_BUILD_JOBS=8 just test -p <changed-crate>`.
3. If changes pass verification (or if the candidate is closed as COVERED/NO-DISPATCH without changes), the manager finalizes the row in the tracker, runs one OntoIndex refresh, and immediately starts the next candidate.
