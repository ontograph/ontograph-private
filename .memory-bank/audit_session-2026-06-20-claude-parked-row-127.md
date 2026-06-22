# Claude Parked Row 127 Review

Date: 2026-06-20

## Decision

Row 127 stays rejected.

## Source

- ADR row 127: `Existing | Non-core | REJECT | Grep wrappers are not core functionality.`
- Donor row 127: `Add search-source MCP endpoint using rg. | codex-mcp / search | Fast donor/current repo search. | Result cap test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Sub-agent `019ee53b-5d9c-7dd3-a199-fa81189ee4a3` recommended rejected and made no edits.
- `.memory-bank/ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md` keeps native context-tool work bounded to existing owners rather than new source-browsing MCP wrappers.
- `ontocode-rs/core/src/tools/handlers/tool_search.rs` already exposes bounded search with an explicit `limit` path and validates empty/zero cases.
- `ontocode-rs/core/src/tools/handlers/tool_search_spec.rs` identifies `tool_search` as the model-facing MCP tool discovery path.
- `ontocode-rs/file-search/src/lib.rs` and `ontocode-rs/file-search/src/main.rs` own repository search behavior, limits, and truncation warnings.
- No concrete existing-owner result-cap or search behavior gap was found.

## Validation

- `CARGO_BUILD_JOBS=8 just test -p ontocode-file-search` passed 15/15 tests in the worker workspace.

## Outcome

No implementation dispatch.
