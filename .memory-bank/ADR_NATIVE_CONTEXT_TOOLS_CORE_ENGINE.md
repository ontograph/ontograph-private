# ADR: Native Context Tools In Core Engine

## Status

Accepted - C0 default-on shell output reduction

## Date

2026-06-15

## Context

The completed `LEAN_CTX_PRE_JUNIOR_PROJECT_PLAN.md` intentionally stopped at one repository-only Python helper. It did not port lean-ctx runtime behavior into Ontocode.

The next question is whether Ontocode should implement native equivalents for the useful parts:

- bounded file reads
- compact text search
- shell execution wrapping
- output compression
- cache/archive storage
- session or knowledge persistence
- background workers
- Rust runtime ownership

OntoIndex review found existing Ontocode owners that already cover part of this space:

- `ontocode-rs/core/src/tools/mod.rs` owns tool output formatting through `format_exec_output_for_model` and `format_exec_output_str`.
- `ontocode-rs/utils/output-truncation/src/lib.rs` owns token/byte-budget truncation helpers.
- `ontocode-rs/core/src/user_shell_command.rs` already formats shell command records for model context.
- `ontocode-rs/core/tests/suite/hooks.rs` already verifies shell-command hook/context behavior.
- OntoIndex remains the external code-intelligence/search owner and should not be duplicated inside core.

This ADR combines prior Solution 4 and Solution 2:

- Solution 4: use OntoIndex for code intelligence and search where possible.
- Solution 2: add the smallest native Rust tool/read/shell/compression layer where Ontocode already owns the execution path.

## Decision

Implement a native Ontocode context-tools layer, but keep it inline with existing core engine owners.

This is not a lean-ctx port. Do not vendor, import, shell out to, or depend on donor lean-ctx code. Reuse only the ideas: bounded reads, compact output, explicit safety guards, deterministic summaries, and optional evidence pointers.

Challenge result:

- Do not add `context_read`, `context_search`, or `context_shell` as model-visible tools in the first slice.
- Turn deterministic output reducers on by default before the existing truncation path.
- Keep final truncation as the hard budget guard after semantic reduction.
- Do not add a config flag in C0; add one only if a real compatibility issue appears.
- Treat bounded read as a second slice that must use the active environment filesystem and permission profile boundary.
- Keep exact search deferred until OntoIndex plus existing exact shell search proves insufficient.
- Keep persistence, archive pointers, context fragments, and background workers blocked until redaction and evidence owners are explicitly named.

## Architecture

### Owner Map

| Capability | Owner | Rule |
|---|---|---|
| File read summaries | Existing environment filesystem / permission-profile boundary first; new small helper only behind that boundary | Do not read through raw `std::fs` in model-visible paths. |
| Text search | Prefer OntoIndex for semantic/code queries; use `rg`-style plain search only as local fallback | Do not create a second static code graph. |
| Shell execution wrapper | Existing shell tool handler/runtime path | Do not add a second shell launcher, shell tool, or sandbox path. |
| Output compression | Existing shell/tool output formatting before generic truncation | Profile reducers must preserve failures before `utils/output-truncation` applies the final budget. |
| Model-visible summaries | Existing context-fragment architecture only | Hard caps are mandatory. |
| Session-local cache | Core session/tool runtime, memory-only at first | No persistence in first implementation. |
| Persistent archive/evidence | Existing state/runtime evidence owners only after separate stage approval | Store bounded metadata and references, not raw unlimited output. |
| Background workers | Existing runtime/task orchestration only after separate stage approval | No worker system in the first slice. |

### Initial Tool Surface

Do not expose a new model-visible tool surface in the first implementation slice.

If later slices expose internal behavior, they must go through existing Ontocode tool planning, handler lifecycle, permission, telemetry, and tests. They must not add a new registry.

Deferred placeholder names:

- `context_read`: read a file with bounded modes.
- `context_search`: plain text search with bounded output, optionally recommending OntoIndex for semantic/code queries.
- `context_shell`: run shell through the existing shell handler with compact output and write-command guardrails.

These names remain blocked until a stage card proves model-visible exposure is needed. Internal helpers should extend existing tool behavior first.

## First Implementation Slice

Keep the first Rust slice small.

### C0: Shell Output Reducers

First implementation slice. Extend existing shell/tool output formatting, not shell execution.

Add deterministic reducers for common noisy outputs before final generic truncation:

- Rust build/test
- Python unittest
- `rg`
- `git status`
- JSON-ish logs
- generic long logs

Default-on rule:

- Shell/tool output first passes through the deterministic reducer.
- Reducers only return changed text for recognized noisy patterns and keep small/unmatched output unchanged.
- Existing generic truncation always runs after reduction.
- Exit code, timeout state, duration, and existing shell execution behavior remain unchanged.

Acceptance:

- Failing build/test output keeps actionable errors, file paths, line numbers, and failing test names.
- Successful verbose output is reduced.
- Exit code, timeout state, and duration remain visible.
- Existing shell handler/runtime behavior is unchanged.
- Existing truncation tests still pass, with new reducer-specific tests added.

### C1: Bounded Read

Second implementation slice. Add one native bounded read path behind the active environment filesystem and permission profile boundary:

- `full` with hard byte/token cap
- `lines:start-end`
- `summary` with file metadata, first meaningful lines, and truncation notice

No raw `std::fs` in model-visible paths. No AST, no semantic cache, no compression archive.

Acceptance:

- Large files are capped.
- Binary files are rejected or summarized safely.
- Paths outside approved workspace and permission boundaries are rejected.
- Output includes truncation metadata.

### C2: Bounded Read Tool Exposure Review

Decide whether bounded read should remain internal or become model-visible.

No implementation should expose a new tool until this review is complete.

Acceptance:

- Existing tool planning, permission, telemetry, and tests are named.
- Model-visible name and default output budget are approved.
- Context-fragment impact is reviewed if output enters model context outside normal tool output.

### C3: Plain Search

Keep deferred. Add a bounded plain search wrapper only if existing exact search plus OntoIndex is not enough.

Rules:

- For code-meaning queries, prefer OntoIndex.
- For exact text search, use local scan or `rg`-equivalent implementation through existing command paths.
- Output must be capped by match count, line width, and file count.

Acceptance:

- No new graph/index store.
- No background indexer.
- No donor dependency.

## Advanced Additions

These are explicitly not part of the first slice.

### A1: Compression Profiles

Add named profiles for common outputs:

- `rust-test`
- `rust-build`
- `python-test`
- `search`
- `git-status`
- `json`
- `generic-log`

Each profile is a deterministic reducer with tests. No LLM summarization.

### A2: Session-Local Read/Search Cache

Cache only within the active session:

- key: path/query/mode plus file metadata
- invalidation: mtime/size/hash when cheap
- scope: memory only

This reduces repeated model-context bloat without creating persistent state.

### A3: Evidence Archive Pointers

Blocked until redaction and operational evidence owners are named. For very large outputs, a later slice may store a bounded local artifact and return a small pointer:

- artifact id
- command/path/query
- timestamp
- byte count
- redaction status
- preview

Do not persist raw secrets. Do not put raw archive content into model context.

### A4: Context Fragment Bridge

Allow compact read/search/shell summaries to enter model context only through existing `ContextualUserFragment` paths.

Rules:

- hard cap per fragment
- memory-exclusion handling
- no raw large output
- no side-channel context injection

### A5: OntoIndex-Aware Search Routing

When OntoIndex is fresh:

- semantic/code-owner/impact questions route to OntoIndex reports
- exact text queries route to native bounded search

When OntoIndex is missing or stale:

- exact text search can still work
- semantic/code-intelligence search reports unavailable/stale rather than guessing

### A6: Persistent Operational Evidence Records

Only after the evidence backbone ADR is implemented:

- store compact facts, not raw logs
- include source, command, redaction state, and retention metadata
- support readiness summaries and task closure evidence

This is operational evidence, not knowledge persistence and not a second memory system.

### A7: Background Compression Workers

Only after synchronous compression is proven too slow:

- background jobs may compress/archive large outputs after the model receives a safe preview
- failures must not break normal tool execution
- worker state must be observable and bounded

## Rejected

- Vendoring lean-ctx.
- Depending on lean-ctx runtime, MCP, cache, session, knowledge, or compression code.
- Creating a second shell launcher.
- Creating a second static search/index engine.
- Creating a second context injection path.
- Persisting raw shell output by default.
- Adding persistent knowledge/session storage under this ADR.
- Adding model-visible `context_*` tool names in the first slice.
- Adding new app-server APIs in the first slice.
- Adding a large new crate before proving `core/src/tools` plus `utils/output-truncation` cannot hold the first slice.

## Behavior Changes

### For Users

- First slice only: shell results shown to the model become shorter and more actionable by default for recognized noisy outputs.
- Small or unrecognized shell output remains on the existing truncation path.
- Large file reads become safer only after C1 lands.
- Search output changes only if a later C3 slice is approved.

### For Agents

- Agents get less raw output and more structured summaries.
- Repeated reads/searches may become cheaper only after a later session-local cache stage.
- Shell execution policy does not change in C0.
- OntoIndex-backed questions become explicitly separated from exact text search.

### For Developers

- Core output formatting becomes more important and needs focused regression tests.
- Shell, read, and search behavior must be tested through existing tool and core suite paths.
- No rebuild is required for ADR-only work, but Rust implementation slices will require `just fmt`, scoped `just test -p ...`, and likely `just fix -p ...`.

## Implementation Stages

1. ADR and owner review only.
2. C0 shell output reducers through existing formatting path.
3. C1 bounded read behind environment filesystem and permission boundaries.
4. C2 bounded read exposure review.
5. C3 bounded exact search only if still needed after OntoIndex routing review.
6. A1 compression profiles.
7. A2 session-local cache.
8. A3 evidence archive pointers.
9. A4 context fragment bridge.
10. A5 OntoIndex-aware search routing.
11. A6/A7 persistence and background workers only after separate approval.

## Verification Requirements

Each implementation slice must include:

- OntoIndex impact check for edited symbols.
- Tests for cap enforcement where the slice introduces a cap.
- Tests for failure-output preservation.
- Tests for no secret-looking values in summaries.
- Tests proving no raw large output enters model context.
- Scoped `just test -p ...` for changed Rust crates.

## Open Questions

- Should the first model-visible names be `context_read/search/shell`, or should these remain internal helpers behind existing tool names?
- Should bounded read live in `core/src/tools` or a tiny utility crate used by both core and app-server later?
- What is the maximum default model-visible budget per read/search/shell result?
- Which redaction owner should run before archive pointer creation?
