# Claude Parked Row 167 Review

Date: 2026-06-20

## Decision

Row 167 stays parked.

## Source

- ADR row 167: `Partial` / `Non-core` / `NARROW` / `Compact status lines may help but need UI owner.`
- Donor row 167: `Add clickable file refs in TUI. | TUI markdown/file links | Improves review navigation. | Markdown snapshot.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-rs/tui/src/markdown_render.rs` already owns local file-link display, destination-based labels, path normalization, cwd-relative display, and width-aware markdown rendering.
- `ontocode-rs/tui/src/markdown_render_tests.rs` already covers local file-link hiding, decoding, line/range/hash suffixes, cwd behavior, and the file-link snapshot.
- `ontocode-rs/tui/src/history_cell/messages.rs` preserves session cwd so local file links render consistently during transcript reflow.
- `ontocode-rs/tui/src/terminal_hyperlinks.rs` currently treats OSC8/link metadata as web-link behavior; making local file refs clickable would require new terminal hyperlink or open-file navigation semantics.

## Outcome

No implementation dispatch. No exactly-one existing TUI markdown/file-link rendering snapshot gap was found. Clickable local file refs would add terminal hyperlink/open-file behavior, navigation semantics, editor integration, or UI surface beyond this parked row. No Rust tests were run because no product code changed.
