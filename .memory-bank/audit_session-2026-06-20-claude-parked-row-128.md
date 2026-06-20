# Claude Parked Row 128 Review

Date: 2026-06-20

## Decision

Row 128 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 128 says diagnostic explorer UI can wait.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 128 proposes adding an architecture overview MCP prompt under docs/MCP with a prompt fixture.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `.memory-bank/project_architecture.md` and `.memory-bank/MEMORY.md` already provide the current architecture overview entry point.
- The parked ADR challenge says rows 122, 123, 128-130, and 145-147 must not become an MCP browser, source explorer, teaching server, command debugger, or second diagnostics surface.
- No concrete docs/MCP prompt owner, failing prompt fixture, or fresh bug/regression/security/safety/product evidence was found.
- Adding an MCP architecture prompt would create a new docs/MCP API or prompt surface and diagnostic explorer path rather than extending an existing core owner.

## Outcome

No implementation dispatch. Architecture orientation remains in memory-bank and OntoIndex-backed project context unless a concrete accepted docs/MCP owner and fixture gap is approved.
