# Claude Parked Row 129 Review

Date: 2026-06-20

## Decision

Row 129 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 129 says MCP teaching/demo server is documentation/plugin work.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 129 proposes adding a compare-tools MCP prompt under docs/MCP with a prompt fixture.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- The parked ADR challenge says rows 122, 123, 128-130, and 145-147 must not become an MCP browser, source explorer, teaching server, command debugger, or second diagnostics surface.
- Workspace search found no current `compare-tools` prompt owner or failing docs/MCP prompt fixture.
- OntoIndex search returned existing MCP approval/tool metadata and configuration tests, not a docs/MCP compare-tools prompt owner.
- No fresh bug, regression, security, safety, product evidence, or concrete single-owner test gap was found.

## Outcome

No implementation dispatch. Tool migration review guidance remains outside core unless an accepted docs/MCP owner and concrete fixture gap are approved.
