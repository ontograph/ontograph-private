# Excel Code Translation Stage 2 Reopen

## Scope

Reopen the deferred Stage 2 task from [ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md](ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md) without reopening proposals that were explicitly rejected.

## Decision

- Reopened: `excel.translate_vba_to_sql_preview`
- Not reopened: `excel.review_translation_candidates`

## Reasoning

`excel.translate_vba_to_sql_preview` was previously deferred as a possible convenience pipeline over the implemented Stage 1A and 1B primitives. That makes it a valid reopen target for a senior-directed follow-up loop.

`excel.review_translation_candidates` was last classified as rejected/deferred because it reintroduced the broad donor workflow shape. Reopening it would override a rejection decision, not merely reopen deferred work.

## Next Active Tasks

- Senior re-challenge of the Stage 2 pipeline contract
- Implementation only if the bounded contract still holds
- Focused verification if implementation proceeds
