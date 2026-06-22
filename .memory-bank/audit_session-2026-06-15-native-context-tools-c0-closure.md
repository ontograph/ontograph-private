---
name: Native Context Tools C0 Closure
description: Closure note for native shell-output reducers inside ontocode-core formatting
type: audit_session
date: 2026-06-15
status: done-with-harness-blocker
---

# Native Context Tools C0 Closure

Authority:

- [ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md](ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md)
- [NATIVE_CONTEXT_TOOLS_CORE_ENGINE_PROJECT_PLAN.md](NATIVE_CONTEXT_TOOLS_CORE_ENGINE_PROJECT_PLAN.md)
- [NATIVE_CONTEXT_TOOLS_CORE_ENGINE_TRACKING.md](NATIVE_CONTEXT_TOOLS_CORE_ENGINE_TRACKING.md)

## Accepted Scope

- Added a private `output_reducer` seam under existing `ontocode-core` tool formatting.
- Wired both `format_exec_output_for_model` and `format_exec_output_str` through the reducer before final truncation.
- Added deterministic reducers for:
  - Rust/Cargo build and test output.
  - Python unittest output.
  - `rg` shell output only.
  - large `git status --short` style output.
  - generic long logs.
- Moved reducer tests to `output_reducer_tests.rs` after the implementation approached the large-module threshold.
- Added redaction/budget guards proving token-like strings are not duplicated and final truncation still applies after reduction.

## Guardrails Preserved

- No `context_*` tools.
- No search wrapper or search index.
- No shell launcher, approval, sandbox, timeout, or execution behavior changes.
- No dependencies.
- No cache, archive store, persistence, app-server API, config, protocol, or donor lean-ctx code.

## Verification

Passed:

- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 cargo test -p ontocode-core --lib output_reducer -- --nocapture`
  - Result: `14 passed; 0 failed`.
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-core`
  - Result: completed with existing warnings.
- `cd /opt/demodb/_workfolder/ontocode && git diff --check -- <touched files>`
- `cd /opt/demodb/_workfolder/ontocode && git diff --check`

Blocked by existing harness issue:

- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-core`
  - Fails before test execution when `test-binary-support` cannot create arg0 PATH aliases: `File exists`.
  - The focused reducer test path passes and exercises the accepted changes.

## Final State

All NC-0 through NC-8 tasks are complete for the C0 reducer slice. The only residual item is the pre-existing `test-binary-support` PATH alias collision affecting broad `just test -p ontocode-core`.
