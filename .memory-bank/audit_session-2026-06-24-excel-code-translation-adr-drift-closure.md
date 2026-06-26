# Excel Code Translation ADR Drift Closure

## Scope

Close the open audit findings against [ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md](ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md) by aligning the ADR with the implemented `ext/excel` tool surface and the closed tracking ledger.

## Findings Closed

1. The ADR's "current tools" section still described the pre-implementation Excel surface.
2. The ADR's stage plan still read as pending even though Stage 1A and 1B were already implemented and verified.
3. The ADR was missing implementation constraints discovered during review and verification of the PowerQuery slice.

## Changes Made

- Rewrote the ADR context/current-state sections so they distinguish the original baseline from the current implemented Excel translation surface.
- Marked Stage 1A and Stage 1B as implemented, `excel.translate_vba_to_sql_preview` as deferred, and `excel.review_translation_candidates` as rejected/deferred.
- Added the verified conservative-translation constraints for unsupported `Table.SelectRows` predicates, bounded custom-XML warning accumulation, and `has_power_query` preservation on corrupted `DataMashup` payloads.

## Verification

- OntoIndex freshness check at HEAD remained non-stale during the closure pass.
- The implemented tool surface stayed aligned with `ontocode-rs/ext/excel/src/extension.rs` registration and the existing extension tool-list test.
- Final scope verification was run against the markdown-only diff.
