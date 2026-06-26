# Offline VBA To ONLYOFFICE Follow-Up Task Closure

Date: 2026-06-25

## Scope

Run a bounded manager loop over the currently opened `SFT-*` follow-up tasks using OntoIndex where available and direct-source or extracted-corpus fallback where OntoIndex coverage is partial.

## Roles

- manager: current session
- senior-reviewer: handled by manager locally because this is a bounded evidence pass
- implementation-worker: not dispatched because no implementation slice reopened
- verification-worker: handled by manager locally because this is documentation and queue-state verification only

## Evidence

- OntoIndex `gn_ensure_fresh` reports `codex` fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2`.
- Dirty-worktree caveat remains active with `dirtyFileCount=260` and `scopeConfidence=medium`.
- OntoIndex cleanly resolves:
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
- OntoIndex still does not provide clean file coverage for the ONLYOFFICE-specific Excel files, so this pass uses direct-source fallback for:
  - `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs`
  - `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`
  - `ontocode-rs/ext/excel/src/vba_onlyoffice_workbook_review.rs`
- Corpus evidence reviewed from extracted local samples:
  - `tmp/vba-samples/tabell.vba`
  - `tmp/vba-samples/essbase.vba`
  - `tmp/vba-samples/mylo.vba`

## SFT-1 Shallow Syntax Family Presence Check

Families checked:

- `.Value2`
- `.FormulaR1C1`
- `.NumberFormatLocal`
- `.ColumnWidth`

Result:

| Family | Current corpus result | Note |
| --- | --- | --- |
| `.Value2` | absent from current extracted corpus | no redacted trigger snippet available |
| `.FormulaR1C1` | absent from current extracted corpus | no redacted trigger snippet available |
| `.NumberFormatLocal` | absent from current extracted corpus | literal `NumberFormat` exists, but not the local variant |
| `.ColumnWidth` | absent from current extracted corpus | no redacted trigger snippet available |

Decision:

- `SFT-1` is closed.
- None of these families becomes a real `C1` candidate from the current corpus.

## SFT-2 Utility-Style Fixture Pack From `Табель Макрос`

Review-ready redacted snippet pack:

| Snippet label | Redacted shape | Classification | Duplicate or novel |
| --- | --- | --- | --- |
| `tabell-formulalocal-variable-rhs` | `Cells(r, erc).FormulaLocal = er` | semantics-blocked | duplicate of already-reviewed family |
| `tabell-font-colorindex-write` | `Cells(r, erc).Font.ColorIndex = 2` | semantics-blocked | novel fixture for blocked ledger |
| `tabell-fill-colorindex-write-a` | `Cells(r, erc).Interior.ColorIndex = 41` | semantics-blocked | novel fixture for blocked ledger |
| `tabell-fill-colorindex-write-b` | `Cells(r, erc).Interior.ColorIndex = 3` | semantics-blocked | duplicate of same family |
| `tabell-protect-password` | `Sheets(sh).Protect Password:=<redacted>` | out of scope | duplicate of blocked workbook-protection family |

Decision:

- `SFT-2` is closed.
- `Табель Макрос` remains useful as a compact utility-style evidence source, but the current extracted fixtures still do not open a new syntax-only slice.

## SFT-3 Semantics-Blocked Boundary Ledger

| Family | Why current owner cannot absorb it as `C1` | Missing semantic owner or assumption | Deferred slice pointer |
| --- | --- | --- | --- |
| `.Font.ColorIndex` | current analyzer and emitter understand direct RGB-like formatting intent, not Excel palette-index semantics | palette index to concrete ONLYOFFICE color mapping contract | none proven yet |
| `.Interior.ColorIndex` | same boundary as text color; integer palette indices are not shallow syntax aliases | palette-index normalization layer with deterministic mapping | none proven yet |
| `.RowHeight` | not an existing supported operation family and not part of pinned first-slice recorder contract | new row-dimension semantic plus ONLYOFFICE target proof | no approved slice |
| dynamic formula concatenation | requires expression evaluation or safe symbolic rewrite, not just property-name parsing | formula-expression IR or constrained evaluator | not `C1`; would point beyond current deferred queue |
| shape or text frame writes | leaves the worksheet/selection formatting owner and enters shape APIs | new target contract for shapes/text objects | out of current scope |
| `Visible` and `Protect` | workbook, sheet, and protection choreography are workflow semantics, not direct cell-formatting syntax | workbook-state model and non-trivial safety policy | out of current scope |

Decision:

- `SFT-3` is closed.
- The recurring blocked families stay blocked for semantic reasons, not because the parser missed a narrow alias.

## SFT-4 Direct-Source Owner Map For ONLYOFFICE Excel Flow

Minimal review path:

- entrypoint:
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `install(...)` wires the three ONLYOFFICE-related tools into the Excel extension
- analyzer owner:
  - `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs`
  - `analyze_vba_onlyoffice_migration(...)`
- emitter owner:
  - `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`
  - `translate_vba_to_onlyoffice_js_preview(...)`
  - `translate_analyzed_vba_to_onlyoffice_js_preview(...)`
- workbook composition owner:
  - `ontocode-rs/ext/excel/src/vba_onlyoffice_workbook_review.rs`
  - `review_vba_onlyoffice_workbook(...)`
- canonical regression area:
  - `ontocode-rs/ext/excel/src/tests.rs`
  - analyzer coverage around `1151` to `1367`
  - translator coverage around `1481` to `1809`
  - workbook review coverage around `2702` to `2857`

Decision:

- `SFT-4` is closed.
- Future deferred-loop review should start from this source path instead of trying to rediscover ownership through partial index coverage.

## SFT-5 Minimum `C1` Reopen Proof Pack

Accepted minimum checklist:

- one redacted snippet that survives without business-sensitive literals
- explicit statement of the candidate family
- proof that the existing operation family already exists
- proof that no new semantic owner is required
- one positive analyzer or translator test target
- one adjacent fail-closed test target
- one explicit non-scope statement naming what is still excluded

Decision:

- `SFT-5` is closed.
- This is short enough to reuse and does not add a second process layer.

## Final Queue Decision

No implementation-worker dispatch is justified from these follow-up closures.

Current queue result:

- the evidence queue is tighter
- the fake shallow-syntax backlog is smaller
- the blocked families are now tied to concrete semantic reasons

What is still not open:

- `A2`
- `B1`
- `C2`
- `E3`
- any new `C1` slice beyond the already closed `.FormulaLocal` variant
