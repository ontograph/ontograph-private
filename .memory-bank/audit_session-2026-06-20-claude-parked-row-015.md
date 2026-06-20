name: Claude Parked Row 015 Review
desc: Row 015 stays parked because notebook scope conflicts with blocked donor scope and embedded-search detection is already covered by existing tool-search/fallback owners
type: audit_session
date: 2026-06-20

# Claude Parked Row 015 Review

## Decision

Row 015 stays parked. No promotion packet was written.

## Evidence

- Parked ADR row 015 says: `Notebook-style output is useful only as protocol/UI surface.`
- Donor source row 015 says: `Add embedded-search capability detection` in `exec-server` / `core/src/tools/spec_plan.rs`.
- The two sources are mismatched. The notebook reading conflicts with the Oh My Pi plan rule: do not add notebook execution, virtual URI schemes, DAP, browser control, or persistent language workers.
- The embedded-search reading duplicates existing active work: Claude KEEP row 006 already owns tool search/suggest improvements in `core/src/tools`, and Oh My Pi row 11 tracks native search fallback behavior near `search_tool.rs`.
- OntoIndex reports `ontocode-rs/core/src/tools/spec_plan.rs` public API as `build_tool_router`, `search_tool_enabled`, and `tool_suggest_enabled`; the file is 996 lines.
- OntoIndex impact for `search_tool_enabled` is HIGH, with 13 impacted nodes across 4 modules and direct callers including `add_collaboration_tools`, `append_tool_search_executor`, `prepend_code_mode_executors`, `append_extension_tool_executors`, and `built_tools`.
- Existing `spec_plan_tests.rs` coverage already proves `tool_search` is hidden without model search capability, shown when deferred tools exist, and used to defer v1 multi-agent tools when search is available.

## Closure

The row is not reducible to exactly one existing-owner failing test gap without reopening tool-search architecture or notebook/protocol UI scope. It remains parked.
