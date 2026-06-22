# Lean-ctx Pre-Junior Donor Preflight

Date: 2026-06-15

Donor source checked:
- files read: `LEAN-CTX.md`, `skills/lean-ctx/SKILL.md`, `rust/src/tools/ctx_read/mod.rs`, `rust/src/tools/ctx_search.rs`, `rust/src/tools/ctx_shell.rs`
- reviewer-only context skimmed: `rust/src/server/registry.rs`, `rust/src/server/call_tool.rs`, `rust/src/tools/ctx_session.rs`, `rust/src/tools/ctx_knowledge/mod.rs`, `rust/src/shell/compress/engine.rs`

Patterns reused:
- resolve the repo root once
- read UTF-8 markdown/text deterministically
- return bounded summaries instead of raw dumps
- print clear errors with stable exit codes
- keep the script read-only and stdlib-only

Patterns explicitly rejected:
- MCP tool registration and dispatch
- session or knowledge persistence
- compression/archive storage
- shell execution wrappers
- graph, cache, or background-worker mechanics

Why this remains one read-only repository script:
- the approved slice only needs local memory-bank inspection and link validation
- no donor runtime service, cache, or tool registry is required
- the script will not shell out, write files, or call OntoIndex/GitNexus/lean-ctx
