# Claude Parked Row 143 Review

Date: 2026-06-20

## Decision

Row 143 stays parked.

## Source

- ADR row 143: Partial / Conditional / NARROW / command telemetry should be compact and opt-in.
- Donor row 143: add `/add-dir` with scoped workspace roots under config / permissions with a permission root test.

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent tool surface; triage used the available `gpt-5.4-mini` fallback.
- TUI already has `/sandbox-add-read-dir <absolute-directory-path>`.
- Additional directory warnings already explain when `--add-dir` is ignored under restrictive permissions.
- Config and permissions owners already cover profile workspace roots, runtime workspace roots, and scoped `:workspace_roots` materialization.
- App-server tests already cover command-cwd root scoping and turn-start workspace-root rebinding.
- No single failing owner-local permission-root test/doc gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
