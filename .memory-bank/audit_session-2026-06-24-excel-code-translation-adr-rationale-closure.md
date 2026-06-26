# Excel Code Translation ADR Rationale Closure

## Scope

Close the final open audit items against [ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md](ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md) after the first drift-remediation pass.

## Findings Closed

1. Proposal A still described the pre-implementation `ext/excel` owner state as if workbook source extraction did not exist yet.
2. Proposal B still described the owner as workbook-first only in present tense, even though source-first translation tools are now implemented.

## Changes Made

- Rewrote the Proposal A challenge text into explicit historical rationale.
- Rewrote the Proposal B "why this should move earlier" bullets into explicit historical rationale.

## Verification

- OntoIndex freshness remained non-stale at HEAD during the cleanup pass.
- The implemented tool surface remained aligned with `ontocode-rs/ext/excel/src/extension.rs` registration and the extension tool-list test.
- Final scope verification passed for the markdown-only diff.
