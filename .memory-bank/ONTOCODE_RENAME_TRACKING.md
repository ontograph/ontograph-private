# Ontocode Rename Execution Tracking

Source plan: `ONTOCODE_RENAME_PROJECT_PLAN.md`

## Status Key

- `pending`
- `in_progress`
- `blocked`
- `done`
- `deferred`

## Task Queue

| ID | Task | Scope | Status | Notes |
| --- | --- | --- | --- | --- |
| T1 | Surface inventory and compatibility matrix | New Markdown doc | done | Added `ONTOCODE_RENAME_SURFACE_MATRIX.md` |
| T2 | User-facing branding doc updates | `README.md`, `ontocode-rs/README.md`, `sdk/python/README.md`, package descriptions as appropriate | done | Updated user-facing branding while preserving commands and package identities |
| T3 | `ontocode` CLI alias implementation and tests | `ontocode-rs/cli` and related tests/build glue | done | Wrapper now resolves `CARGO_BIN_EXE_codex` before sibling fallback; direct alias verification confirmed env-path dispatch and `ONTOCODE_CLI_COMMAND_NAME=ontocode` propagation. Full `just test -p codex-cli` remains blocked by stale/stalled shared-target nextest workers |
| T4 | Config/env dual-read compatibility | config loader, env resolution, tests | done | `ONTOCODE_HOME` precedence and default home resolution implemented in `ontocode-rs/utils/home-dir` with targeted tests |
| T5 | Persisted-state migration and rollback design | Design doc / implementation if feasible | done | Added `ONTOCODE_PERSISTED_STATE_MIGRATION.md` |
| T6 | Package identity migration design | Design doc | done | Added `ONTOCODE_PACKAGE_IDENTITY_MIGRATION.md` |
| T7 | Protocol/integration alias inventory | Design doc / audit | done | Added `ONTOCODE_PROTOCOL_INTEGRATION_INVENTORY.md` |
| T8 | Optional internal crate rename decision | Decision doc | deferred | Explicitly out of scope for the completed public-surface rename; only revisit as a separate breaking-change program |
| T9 | Python SDK public-surface Ontocode sweep | `sdk/python` docs, examples, tests, non-generated source | done | Public docs/examples/tests now prefer `Ontocode*`; repair loop cleared, runtime behavior test passes, and the signature test remains blocked only by local Python 3.10 lacking `tomllib` |
| T10 | TypeScript SDK public-surface Ontocode sweep | `sdk/typescript` docs, samples, tests, non-generated source | done | Docs, samples, tests, and public-facing SDK text now prefer `Ontocode*`; local `npm exec -- tsup` and targeted Jest suites passed |
| T11 | Remaining codex-named surfaces disposition pass | generated SDK models, wire identifiers, internal Rust types/crates | done | Added `ONTOCODE_REMAINING_SURFACES_DISPOSITION.md`; remaining non-SDK `codex` surfaces are now explicitly preserved or deferred by policy |
| T12 | Tighten Ontocode CLI canonicalization | CLI help/display, docs, examples, cli tests | done | CLI help/display now prefers Ontocode when invoked via alias; docs and tests updated |
| T13 | Packaging alias implementation | npm, python, native runtime packaging | done | npm, Python, and native runtime packaging now install both `codex` and `ontocode` binaries |
| T14 | Internal helper rename (Stage 4) | ontocode-exec, ontocode-exec-server, sandbox helpers, etc. | in_progress | Adding aliases for internal helpers to support `ontocode-*` names |
| T15 | Optional package rename (Stage 3) | @openai/ontocode, etc. | deferred | Only start after release tooling supports dual publish |

## Dispatch Log

| Step | Tracking Update | Action |
| --- | --- | --- |
| 1 | Created tracker and seeded queue | Ready for first dispatch wave |
| 2 | Marked `T2` in progress | Stage 1 branding docs scoped to user-facing copy only |
| 2 | Marked T3 in progress | Implement `ontocode` CLI alias and parity tests |
| 3 | Marked T1 done | Surface matrix reviewed and accepted |
| 4 | Marked `T2` done | Branding docs updated; technical identifiers preserved |
| 5 | Marked T5/T6/T7 in progress | Dispatching design-task wave while T3 continues |
| 6 | Marked T5 done | Persisted-state migration policy reviewed and accepted |
| 7 | Marked T6 done | Package identity migration design reviewed and accepted |
| 8 | Marked T7 done | Protocol/integration inventory reviewed and accepted |
| 9 | Marked T4 in progress | Dispatching config/env compatibility slice while T3 continues |
| 10 | Updated T4 before implementation | Manager continuing home/env resolution slice in `ontocode-rs/utils/home-dir` |
| 11 | Marked T4 done | Home/env resolution now prefers `ONTOCODE_HOME`, preserves `CODEX_HOME`, and covers default-home precedence with tests |
| 12 | T3 still in progress | Partial implementation reported; manager requesting wrapper-based rework and full verification |
| 13 | Updated T3 before fallback verification | Recorded stalled full-crate nextest runs; manager proceeding with targeted alias verification and repair loop |
| 14 | T3 failure recorded before repair | Direct `ontocode_alias` test execution failed because the wrapper could not locate a sibling `codex` binary in `target/debug` |
| 15 | Marked T3 done after repair | Wrapper now honors `CARGO_BIN_EXE_codex`; isolated verification confirmed the repaired path and preserved `ontocode` command identity |
| 16 | Added T9/T10/T11 before new dispatch wave | Queue split into Python SDK sweep, TypeScript SDK sweep, and manager disposition pass for generated/wire/internal surfaces |
| 17 | Marked T9/T10 in progress before dispatch | Spawning parallel workers with disjoint write scopes in `sdk/python` and `sdk/typescript` |
| 18 | Marked T11 in progress before manager policy pass | Remaining generated, wire, and internal `codex` identifiers being classified before any further rename attempt |
| 19 | Marked T11 done | Added remaining-surface disposition doc; generated, wire, package, telemetry, and internal Rust names are now explicitly classified as preserve, version, or defer |
| 20 | Marked T10 done after manager verification | Local TypeScript build and targeted Jest runs passed after the worker sweep |
| 21 | Marked T9 done after manager verification | Python runtime behavior test passed; public API signature test remains blocked by Python 3.10 missing `tomllib`, not by rename changes |
| 22 | Reopened T9 after formatter failure | `just fmt` found Python syntax errors in edited example/test files; manager is repairing and will rerun formatting/verification |
| 23 | Re-closed T9 after repair | `just fmt` now passes, Python runtime behavior test still passes, and the only remaining Python verification blocker is the expected Python 3.10 `tomllib` gap |
| 24 | Recorded Option 1 closeout | Rename program closed at the public-surface boundary; all remaining `codex` identifiers are now explicitly preserved or deferred by policy |
| 25 | Marked T12 done | Tightened CLI canonicalization and updated docs/tests |
| 26 | Marked T13 done | npm, Python, and native runtime packaging now install both `codex` and `ontocode` binaries |
| 27 | Marked T14 in progress | Dispatching internal helper alias implementation (Stage 4) |
