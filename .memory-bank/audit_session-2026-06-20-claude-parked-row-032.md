name: Claude Parked Row 032 Review
desc: Row 032 stays parked because security-review plugin commands are non-core workflow surface, not one existing-owner test gap
type: audit_session
date: 2026-06-20

# Claude Parked Row 032 Review

## Decision

Row 032 remains parked. No promotion packet.

## Evidence

- Parked ADR row 032 says the idea is useful as workflow docs or skill, not runtime core.
- Donor row 032 asks for a security-review plugin-backed command in `core-skills` / `core-plugins`.
- Gemini pre-junior review removes Claude command/plugin overlap from Gemini scope.
- Oh My Pi pre-junior review explicitly blocks the related Claude security-review skill/hook surface.
- OntoIndex reports `ontocode-rs/skills/src/lib.rs` owns `system_cache_root_dir` and `install_system_skills`; the file is 170 lines.
- Worker review found `SlashCommand::Review` maps to `open_review_popup()` in the existing TUI review path, not a separate security-review command owner.
- Existing review, guardian, core-skills, and core-plugins tests cover nearby loading and guardrail behavior.

## Closure

A dedicated `security-review` plugin command would introduce new workflow surface. This plan only promotes one existing-owner failing test gap, so row 032 stays parked.
