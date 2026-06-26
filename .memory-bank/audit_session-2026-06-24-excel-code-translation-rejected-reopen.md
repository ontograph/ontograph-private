# Excel Code Translation Rejected Tasks Reopen

## Scope

Reopen the previously rejected Excel code-translation proposals from [ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md](ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md) as active senior-review items.

## Reopened Items

- `excel.review_translation_candidates`
- `excel.translate_vba_to_onlyoffice_javascript`
- `excel.translate` monolith with mixed modes and workflow side effects
- artifact-save, zip-export, and query-create tool surfaces

## Reasoning

The reopen request changes project scope, but it does not erase the earlier rejection logic. Each reopened item must pass a fresh senior review before implementation proceeds.

The bundled review surface was previously rejected because it mixed inspection, extraction, translation, and migration review into one broad workflow.

The OnlyOffice JavaScript proposal was previously rejected because no concrete donor/runtime contract existed.

The monolith and side-effect workflow surfaces were previously rejected because they bypassed the bounded explicit-tool direction that the implemented Excel owner now follows.

## Next Active Tasks

- Re-challenge the bundled review surface and narrow it if possible
- Re-challenge the OnlyOffice JavaScript proposal against concrete runtime evidence
- Re-challenge the monolith and side-effect workflow surfaces against the existing explicit-tool architecture
