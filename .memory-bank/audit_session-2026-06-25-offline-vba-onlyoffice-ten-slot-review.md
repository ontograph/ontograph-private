# Offline VBA To ONLYOFFICE Ten Slot Review

Date: 2026-06-25

## Scope

Bounded follow-on manager loop after the `.FormulaLocal` `C1` closure.

The request was to open ten more tasks and continue until the queue was exhausted. This pass interpreted that as ten narrow review slots, not ten speculative implementation tasks.

## Decision

One slot was justified and already implemented in the same workstream.

The other nine slots were closed without code because they fail the current ADR trigger gates.

## Candidate Slots

| Slot | Candidate | Evidence | Result |
| --- | --- | --- | --- |
| 1 | `.FormulaLocal` target variant | `tmp/vba-samples/tabell.vba` `Cells(r, erc).FormulaLocal = er` | accepted and implemented as narrow `C1` |
| 2 | `.FormulaR1C1` target variant | Microsoft VBA docs only, no local corpus hit | closed, no local trigger |
| 3 | `.Value2` target variant | Microsoft VBA docs only, no local corpus hit | closed, no local trigger |
| 4 | `.NumberFormatLocal` target variant | Microsoft VBA docs only, no local corpus hit | closed, no local trigger |
| 5 | `.ColumnWidth` assignment | no current corpus hit | closed, no local trigger |
| 6 | `.Interior.ColorIndex` palette fill | `tmp/vba-samples/tabell.vba` | blocked by color-palette semantics, not syntax |
| 7 | `.Font.ColorIndex` palette text color | analyzer limitation review, no current safe mapping | blocked by color-palette semantics, not syntax |
| 8 | `.RowHeight` assignment | `tmp/vba-samples/essbase.vba` | blocked by range/root/emitter semantics, not syntax |
| 9 | dynamic formula concatenation | `tmp/vba-samples/essbase.vba` `Formula = "=" & vNewName & "!A1"` | blocked by nonliteral runtime semantics |
| 10 | shape/control `.Text` or workbook `Visible` / `Protect` flows | `tmp/vba-samples/essbase.vba`, `tmp/vba-samples/mylo.vba` | out of current spreadsheet-recorder subset |

## Outcome

No further dispatchable tasks remain after the `.FormulaLocal` slice.

The queue is closed again. The next valid reopen still requires one fresh Slice 0 trigger with concrete local evidence.
