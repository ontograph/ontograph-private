# Subagent Task: Qwen Code 2000 Loop - AGENT-01-03

## Context
Row 203: "agent job loop recovers running items after restart".
Production implementation exists in `ontocode-rs/core/src/tools/handlers/agent_jobs.rs:recover_running_items`.
No focused regression test currently validates that a restarted job loop effectively recovers running items via `resume_agent_from_rollout`.

## Requirements
1. Add a single failing/regression test `agent_job_loop_recovers_running_items_after_restart` in `ontocode-rs/core/tests/suite/agent_jobs.rs`.
2. Do not change production code unless the test actually exposes a gap in `recover_running_items` or the `AgentControl` integration.
3. Keep changes minimal.
