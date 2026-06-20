# Claude Parked Row 193 Review

Date: 2026-06-20

## Decision

Row 193 stays parked.

## Source

- ADR row 193: `Partial | Conditional | NARROW | Skill loading diagnostics can extend core-skills.`
- Donor row 193: `Add auth test script. | login/auth | Faster auth flow verification. | Auth script smoke.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-rs/cli/tests/login.rs` already covers stdin API-key login and invalid access-token rejection at the CLI level.
- `ontocode-rs/login/src/auth/auth_tests.rs` already covers auth.json overwrite, access-token-only writes, invalid/unsigned JWT rejection, and missing auth.json behavior.
- `ontocode-rs/login/tests/suite/login_server_e2e.rs` already covers end-to-end login flow persistence.
- Additional login/auth suites cover device-code login, logout/revoke, refresh, and storage behavior.
- `ontocode-rs/core-skills/src/model.rs` already carries structured skill-load errors through `SkillLoadOutcome`.
- `ontocode-rs/core-skills/src/render.rs` already carries `AvailableSkills.warning_message`, and render/loader tests cover truncation, omission warnings, invalid metadata, and missing frontmatter diagnostics.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
