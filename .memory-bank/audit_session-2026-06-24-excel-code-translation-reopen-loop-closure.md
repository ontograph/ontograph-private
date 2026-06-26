# Excel Code Translation Reopen Loop Closure

## Scope

Close the reopened deferred and rejected tasks from [ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md](ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md) after a fresh OntoIndex-backed senior review.

## Review Outcomes

- `excel.translate_vba_to_sql_preview`: deferred again
- `excel.review_translation_candidates`: rejected again
- `excel.translate_vba_to_onlyoffice_javascript`: rejected again
- `excel.translate` monolith with mixed modes and workflow side effects: rejected again
- artifact-save, zip-export, and query-create tool surfaces: rejected again

## Reasoning

The current `ext/excel` owner already exposes explicit read-only extraction and translation primitives that cover the proven workflow shape.

No new repeated-usage evidence justified promoting `excel.translate_vba_to_sql_preview` from a convenience pipeline candidate into a required bounded tool.

No narrower contract was proven for `excel.review_translation_candidates` that avoided recreating the broad donor workflow bundle.

No concrete donor/runtime contract was established for VBA to OnlyOffice JavaScript translation.

No architecture evidence justified replacing the explicit tool family with a generic `excel.translate` monolith or side-effect workflow surfaces.

## Result

The reopened loop closed without code changes. The prior Stage 1 implementation remains the accepted active surface.
