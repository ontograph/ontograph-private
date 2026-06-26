# Excel Code Translation Agent Tools Tracking

## Status

Bounded manager loop closed after reopened follow-up review

## Scope

Bounded manager loop for [ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md](ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md).

## Model routing

- senior-reviewer: requested `gpt-5.4-mini`
- implementation-worker: requested `gemini-3.5-flash-low`, fallback `gpt-5.3-codex-spark`, `gpt-5.4-mini`; only `gpt-5.4-mini` is currently available from the requested list
- verification-worker: requested `gpt-5.4`

## Tasks

| ID | Task | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| EXCEL-CT-SR1 | Challenge Stage 1A VBA slice against current `ext/excel` owner and reject scope creep | senior-reviewer | completed | Accepted as scoped on 2026-06-24. Keep `extract_vba_modules` read-only, `translate_vba_to_m_preview` source-first, and move both into new owner-local modules instead of growing `tool.rs`. |
| EXCEL-CT-I1 | Implement `excel.extract_vba_modules` | implementation-worker | completed | Added read-only workbook VBA extraction in `ontocode-rs/ext/excel/src/vba_extract.rs` with bounded module metadata/source output and relative-path tool coverage. |
| EXCEL-CT-I2 | Implement `excel.translate_vba_to_m_preview` | implementation-worker | completed | Added source-first heuristic preview in `ontocode-rs/ext/excel/src/vba_translate.rs` with bounded query/VBA output and explicit preview warnings. |
| EXCEL-CT-V1 | Verify Stage 1A scope, tests, and OntoIndex diff | verification-worker | completed | Focused tests passed, `just fix -p ontocode-excel-extension` passed, and file-scoped `gn_verify_diff` passed. Follow-up review findings on unsupported-pattern warnings and path-safe warnings were fixed on 2026-06-24. |
| EXCEL-CT-SR2 | Challenge Stage 1B PowerQuery slice after Stage 1A closure | senior-reviewer | completed | Accepted on 2026-06-24. Proceed only with `excel.extract_powerquery_queries` plus `excel.translate_powerquery_to_sql_preview`, using new owner-local modules and no combined workflow bundling. |
| EXCEL-CT-I3 | Implement `excel.extract_powerquery_queries` | implementation-worker | completed | Added read-only workbook Power Query extraction in `ontocode-rs/ext/excel/src/powerquery_extract.rs` with bounded UTF-16 `DataMashup` decoding, embedded-zip query extraction, connection metadata lookup, and relative-path tool coverage. |
| EXCEL-CT-I4 | Implement `excel.translate_powerquery_to_sql_preview` | implementation-worker | completed | Added source-first heuristic SQL preview in `ontocode-rs/ext/excel/src/powerquery_translate.rs` with direct `Value.NativeQuery` extraction, bounded table-pipeline translation, and deterministic unsupported-function warnings. |
| EXCEL-CT-V2 | Verify Stage 1B scope, tests, and OntoIndex diff | verification-worker | completed | `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed after Stage 1B, `just fix -p ontocode-excel-extension` passed, fresh `ontoindex analyze --skills --skip-agents-md` completed, scoped `gn_verify_diff` passed, and scoped `gn_test_gap` passed. |
| EXCEL-CT-SR3 | Reassess need for `excel.translate_vba_to_sql_preview` | senior-reviewer | completed | Closed deferred on 2026-06-24. The composed Stage 1A/1B primitives already cover the workflow without adding a new bounded capability, so the convenience pipeline remains unjustified. |
| EXCEL-CT-SR4 | Reassess need for `excel.review_translation_candidates` | senior-reviewer | completed | Closed rejected/deferred on 2026-06-24. The current owner-local surface proves the smaller primitives; a bundled review tool would reintroduce the broad donor workflow shape that this ADR narrowed away. |
| EXCEL-CT-SR5 | Review and challenge ADR drift against implemented Excel translation surface | senior-reviewer | completed | Completed on 2026-06-24 using OntoIndex-backed evidence. Open findings were narrowed to documentation drift plus one missing design-constraint capture. |
| EXCEL-CT-I5 | Update ADR to reflect implemented Stage 1A/1B status and closed later-stage decisions | implementation-worker | completed | Updated `ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md` so current state, stage outcomes, and recommended tool-set status now match the implemented `ext/excel` surface and tracker decisions. |
| EXCEL-CT-V3 | Verify audit-remediation diff and record closure note | verification-worker | completed | Added `audit_session-2026-06-24-excel-code-translation-adr-drift-closure.md`, updated memory index routing, and ran scoped OntoIndex diff verification for the markdown-only closure set. |
| EXCEL-CT-SR6 | Re-review ADR after audit remediation and isolate remaining stale rationale text | senior-reviewer | completed | Completed on 2026-06-24 using OntoIndex-backed code evidence. Remaining issues were narrowed to two rationale bullets that still described the pre-implementation owner state in present tense. |
| EXCEL-CT-I6 | Update stale Proposal A and Proposal B rationale text to historical tense | implementation-worker | completed | Updated the remaining rationale text in `ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS.md` so the document now distinguishes original decision-time reasoning from the current implemented tool surface. |
| EXCEL-CT-V4 | Verify second ADR drift cleanup and record closure note | verification-worker | completed | Added a follow-up closure note, updated memory index routing, and ran scoped OntoIndex diff verification for the markdown-only cleanup set. |
| EXCEL-CT-SR7 | Reopen deferred Stage 2 task and challenge whether `excel.translate_vba_to_sql_preview` still earns a dedicated contract | senior-reviewer | completed | Re-challenged on 2026-06-24 and closed deferred again. OntoIndex-backed review found no new repeated-usage proof or owner-local capability gap beyond the composed Stage 1A/1B primitives. |
| EXCEL-CT-I7 | Implement `excel.translate_vba_to_sql_preview` if senior review still accepts the bounded Stage 2 contract | implementation-worker | completed | Not started. Senior review re-closed the proposal as deferred, so no implementation surface was accepted. |
| EXCEL-CT-V5 | Verify Stage 2 scope, tests, and OntoIndex diff if `excel.translate_vba_to_sql_preview` lands | verification-worker | completed | Not run because Stage 2 implementation did not proceed after re-review. |
| EXCEL-CT-SR8 | Reopen `excel.review_translation_candidates` and re-challenge whether a bounded workbook review bundle can exist without recreating the donor workflow stack | senior-reviewer | completed | Re-challenged on 2026-06-24 and closed rejected again. No narrower owner-local contract was proven that avoided recreating the broad donor workflow shape. |
| EXCEL-CT-I8 | Implement `excel.review_translation_candidates` only if senior review narrows it to an owner-local bounded surface | implementation-worker | completed | Not started. Senior review did not accept a bounded contract for this bundled review surface. |
| EXCEL-CT-V6 | Verify `excel.review_translation_candidates` scope, tests, and OntoIndex diff if it lands | verification-worker | completed | Not run because implementation did not proceed after re-review. |
| EXCEL-CT-SR9 | Reopen `excel.translate_vba_to_onlyoffice_javascript` and re-challenge whether a real donor/runtime contract now exists | senior-reviewer | completed | Re-challenged on 2026-06-24 and closed rejected again. No concrete target runtime surface, allowed API set, supported VBA subset, or validation plan was found. |
| EXCEL-CT-I9 | Implement `excel.translate_vba_to_onlyoffice_javascript` only if senior review establishes a non-speculative bounded contract | implementation-worker | completed | Not started. The proposal remained speculative after re-review. |
| EXCEL-CT-V7 | Verify OnlyOffice-JS translation scope, tests, and OntoIndex diff if it lands | verification-worker | completed | Not run because implementation did not proceed after re-review. |
| EXCEL-CT-SR10 | Reopen the rejected generic `excel.translate` monolith and side-effect workflow surfaces for senior re-challenge | senior-reviewer | completed | Re-challenged on 2026-06-24 and closed rejected again. OntoIndex-backed review found no reason to replace the explicit read-only tool family with a monolith or side-effect workflow surfaces. |

## Manager notes

- 2026-06-24: OntoIndex freshness check passed at indexed HEAD `2e72a6d25e147f0619863e7721107b6f11a87fc2`; worktree is dirty, so all changes must stay file-scoped and conservative.
- 2026-06-24: Start with Stage 1A only: `excel.extract_vba_modules` plus `excel.translate_vba_to_m_preview`.
- 2026-06-24: Do not implement `excel.translate_vba_to_sql_preview` in the first slice; it remains a later convenience pipeline candidate.
- 2026-06-24: Keep translation-tool contracts out of the existing oversized `ontocode-rs/ext/excel/src/tool.rs` where practical; prefer new owner-local modules.
- 2026-06-24: Senior review accepted Stage 1A with no blocker findings. Required split: `vba_extract.rs` for workbook-side VBA extraction, `vba_translate.rs` for source-first VBA-to-M preview, and only registration/plumbing changes in `extension.rs` and shared `tool.rs`.
- 2026-06-24: Stage 1A code landed in `ontocode-rs/ext/excel` with focused tests passing and scoped OntoIndex diff verification passing. Dependency updates also refreshed `ontocode-rs/Cargo.lock` and `MODULE.bazel.lock`.
- 2026-06-24: Verification review flagged two in-scope issues and one dirty-worktree false positive. Fixed: deterministic unsupported-pattern warnings in `vba_translate.rs` and caller-facing path warnings in `vba_extract.rs`. Rejected as out-of-scope noise: unrelated app-server/workspace files from the pre-existing dirty checkout.
- 2026-06-24: Stage 1B senior review accepted the PowerQuery slice with no blocker findings. Required split: `powerquery_extract.rs` for workbook-side query extraction, `powerquery_translate.rs` for source-first SQL preview, and no bundled review/pipeline tool in this stage.
- 2026-06-24: Stage 1B landed in `ontocode-rs/ext/excel` with focused extraction/translation tests added for UTF-16 `DataMashup` workbooks, direct `Value.NativeQuery` SQL extraction, bounded table-pipeline translation, and unsupported-function warnings.
- 2026-06-24: Stage 1B validation passed with `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`, `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`, fresh `ontoindex analyze --skills --skip-agents-md`, scoped `gn_verify_diff`, and scoped `gn_test_gap`.
- 2026-06-24: Verification review then challenged three medium issues in the initial Stage 1B patch. Fixed: conservative `Table.SelectRows` predicate translation, bounded custom-XML scan/warning accumulation, and `has_power_query` preservation on corrupted `DataMashup` payloads. Added regression tests for corrupted payload detection and unsupported predicate fallback.
- 2026-06-24: Remaining proposal rows were closed without implementation: `excel.translate_vba_to_sql_preview` stays deferred as an unjustified convenience pipeline, and `excel.review_translation_candidates` stays rejected/deferred as an over-broad bundled workflow.
- 2026-06-24: Follow-up audit review reopened the ADR only for documentation drift. Closure updated the ADR's current-state wording, marked Stage 1A/1B implemented, recorded Stage 2/3 defer/reject outcomes, and pulled the verified PowerQuery guardrails back into the ADR.
- 2026-06-24: A second post-remediation review found two remaining stale rationale bullets still written in present tense. Closure changed those sections to explicit historical rationale so the ADR no longer contradicts the implemented extraction and source-first translation surface.
- 2026-06-24: Senior request reopened all deferred tasks from this ADR. Only one task qualified as deferred rather than rejected/deferred: `excel.translate_vba_to_sql_preview`. `excel.review_translation_candidates` remains closed because its last accepted state is rejected/deferred, not purely deferred.
- 2026-06-24: Senior request then reopened all rejected tasks from this ADR. The tracker now carries pending re-challenge work for the bundled review surface, the OnlyOffice JavaScript proposal, and the previously rejected monolith/side-effect workflow surfaces. Reopen means "challenge again", not "accept automatically".
- 2026-06-24: Reopened follow-up loop completed. OntoIndex-backed senior review found no new evidence to overturn the prior decisions: `excel.translate_vba_to_sql_preview` is deferred again, and the bundled review surface, OnlyOffice JavaScript translator, monolith, and side-effect workflow proposals are rejected again. No implementation or verification work proceeded beyond the review phase.
