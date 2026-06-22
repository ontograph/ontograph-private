# Claude Parked Row 109 Review

Date: 2026-06-20

## Decision

Row 109 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 109 says bridge transport should not duplicate MCP/app-server.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 109 proposes bridge session spawn modes under `app-server` / exec-server with a config enum test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- Existing owners already cover remote-control enable/status and app-server remote-control transport lifecycle.
- Exec-server already has local/remote environment selection and environment TOML coverage.
- App-server thread start already accepts cwd and runtime workspace root overrides, but no bridge-specific session-spawn mode config owner or failing enum test gap was found.
- No fresh bug, regression, security, safety, or product evidence was found.

## Outcome

No implementation dispatch. Bridge session-spawn modes would add transport/config surface without ADR-backed demand or a concrete owner-local test gap.
