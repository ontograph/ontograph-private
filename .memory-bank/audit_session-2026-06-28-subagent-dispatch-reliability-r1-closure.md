# Sub-Agent Dispatch Reliability R1 Closure

Date: 2026-06-28
Status: prompt-only-closure

## Scope

Close the first implementation-ready slice from `ADR_SUBAGENT_DISPATCH_RELIABILITY.md` using current source plus OntoIndex.

## OntoIndex Evidence

- `gn_ensure_fresh` reported the `codex` index fresh at `5edde24a78efe0f10bc710936dfa228427ab7fd1` with a dirty worktree.
- `evaluate_operational_evidence_task_closure` impact was MEDIUM, but current source reads showed it is only exercised by its own runtime tests.
- `resolve_requested_spawn_agent_model` impact was LOW, and current source already proved exact-id fail-closed behavior plus `inherit` and `fast`.

## Manager Decision

Implement only the prompt-enforcement part of `R1`.

Why:

- It is the only live owner that currently affects bounded manager-loop behavior.
- Extending `OperationalEvidenceTaskClosureResult` right now would be dead weight because no current production path consumes it.
- `R2` is already covered.
- `R3` and `R4` do not have a current runtime reproduction strong enough to justify code.

## Implemented

Updated `ontocode-rs/protocol/src/prompts/base_instructions/default.md` so required manager-loop roles must be reported as exactly one of:

- dispatched, with effective model;
- not dispatched, with exact blocker;
- intentionally skipped, only when the loop contract allowed it.

The prompt also now forbids implying delegated execution when no worker spawned and forbids claiming a fallback model unless the loop contract allowed fallback.

## Verification

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-protocol`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

## Follow-Up Gates

- `SDR-R3`: keep pending and start only with a narrow source audit that proves where a structured runtime role signal can reuse the existing preferred-worker order without creating a second router.
- `SDR-R4`: keep pending and start only with a replay/forwarding path inventory plus a failing test that proves a namespace drop outside `Prompt::get_formatted_input()`.

No other implementation-ready task was proven in this first pass, but the remaining slices stay pending rather than blocked.
