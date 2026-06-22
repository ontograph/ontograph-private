# Claude Parked Row 168 Review

Date: 2026-06-20

## Decision

Row 168 stays parked.

## Source

- ADR row 168: `Partial` / `Non-core` / `NARROW` / `Diff display should reuse existing TUI diff patterns.`
- Donor row 168: `Add structured diff component. | TUI | Better edit review. | Snapshot test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-rs/tui/src/diff_render.rs` already owns structured diff rendering through `DiffSummary`, `create_diff_summary`, `FileChange` `Renderable`, and `render_change`.
- Existing diff snapshots cover add, delete, update, rename, multiple files, long-line wrapping, syntax highlighting, ANSI/theme behavior, and gallery layouts.
- The `/diff` flow already routes through `AppEvent::DiffResult` and the same `DiffSummary` render path.
- External-agent config migration owns its own prompt/screen rendering and snapshot coverage rather than requiring a new generic diff component.

## Outcome

No implementation dispatch. No exactly-one existing TUI diff-rendering owner-local snapshot gap was found; adding a structured diff component would introduce a new abstraction, edit-review surface, protocol/API shape, or second renderer. No Rust tests were run because no product code changed.
