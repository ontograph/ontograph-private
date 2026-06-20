# Claude Parked Row 137 Review

Date: 2026-06-20

## Decision

Row 137 stays parked.

## Source

- ADR row 137: New / Conditional / DEFER / custom command marketplace is speculative.
- Donor row 137: execute shell substitutions in prompt commands with an allowlist under the command layer.

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent tool surface; triage used the available `gpt-5.4-mini` fallback.
- Slash command ownership is static TUI command metadata and dispatch, not markdown prompt-command execution.
- Shell execution already goes through shell-command handlers and exec policy.
- Existing shell parser tests intentionally reject variable and command substitutions in safety-sensitive parsing paths.
- Existing exec policy tests already cover prefix allow rules; no failing allowlist gap tied to prompt commands was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
