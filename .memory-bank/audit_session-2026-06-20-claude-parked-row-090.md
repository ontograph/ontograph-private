# Claude Parked Row 090 Review

Date: 2026-06-20

## Decision

Row 090 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 090 says to keep this only as a bounded compression rule.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 090 proposes suppressing speculation when a command changes directories under `execpolicy` / shell parser.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `ontocode-rs/shell-command/src/parse_command.rs` already has `cd` / directory-change parser coverage and uses `cd` as path context for following command summaries.
- `ontocode-rs/shell-command/src/command_safety/is_safe_command.rs` already treats `cd` as a safe command and covers shell-wrapper safety cases.
- `ontocode-rs/core/src/exec_policy.rs` already accounts for `cd /some/folder && ...` path-context behavior.
- No speculation suppression owner exists in shell parser or execpolicy.

## Outcome

No implementation dispatch. Adding speculation suppression would introduce runtime speculation behavior rather than hardening an existing parser/execpolicy failing test.
