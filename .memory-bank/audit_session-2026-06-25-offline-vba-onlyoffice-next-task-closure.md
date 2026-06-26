# Offline VBA To ONLYOFFICE Next Task Closure

Date: 2026-06-25

## Scope

Run a bounded manager loop over the currently opened `SNT-*` tasks using OntoIndex-backed preflight plus workspace-local corpus evidence.

## Roles

- manager: current session
- senior-reviewer: handled by manager locally because this is a bounded evidence pass
- implementation-worker: not dispatched because no implementation slice reopened
- verification-worker: handled by manager locally because this is documentation and queue-state verification only

## Evidence

- OntoIndex `gn_ensure_fresh` reports `codex` fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2`.
- Dirty-worktree caveat remains active with `dirtyFileCount=262` and `scopeConfidence=medium`.
- Workspace-local corpus already exists under `tmp/vba-samples/`:
  - workbook files:
    - `EssBaseWF.xlam`
    - `Выдача спецодежды_без табельных.xlsm`
    - `Заявка на мыло.xlsm`
    - `Табель Макрос.xlsm`
  - extracted module text:
    - `essbase.vba`
    - `mylo.vba`
    - `tabell.vba`
- The extracted module text carries source provenance headers:
  - `tabell.vba` declares `FILE: tmp/vba-samples/Табель Макрос.xlsm`
  - `essbase.vba` declares `FILE: tmp/vba-samples/EssBaseWF.xlam`
  - `mylo.vba` declares `FILE: tmp/vba-samples/Заявка на мыло.xlsm`

## SNT-1 Workspace-Local Corpus Staging Gate

Decision:

- `SNT-1` is closed.

Minimum staging rule:

- treat `tmp/vba-samples/*.vba` as the preferred workspace-local review surface
- use workbook files in `tmp/vba-samples/` only as provenance anchors or for explicit re-extraction, not as the default review artifact
- do not create additional raw workbook copies when an extracted module text file already exists
- redact secrets and business labels in quoted literals before any snippet is promoted into ADR or audit notes
- if a fresh workbook is needed later, stage either:
  - extracted module text only, preferred
  - or a single workbook copy plus immediate extraction, followed by snippet-level redaction before documentation reuse

Why this is enough:

- the workspace already has both provenance anchors and extracted text
- extracted module text is cheaper to search, cite, and redact than raw workbook payloads

## SNT-2 Real-Workbook Placeholder Recheck

Families rechecked against workspace-local workbook-derived extracts:

- `.Value2`
- `.FormulaR1C1`
- `.NumberFormatLocal`
- `.ColumnWidth`

Result:

| Family | Real-workbook extract result | Note |
| --- | --- | --- |
| `.Value2` | confirmed absent | no hit in `tmp/vba-samples/*.vba` |
| `.FormulaR1C1` | confirmed absent | no hit in `tmp/vba-samples/*.vba` |
| `.NumberFormatLocal` | confirmed absent | only literal `NumberFormat` appears |
| `.ColumnWidth` | confirmed absent | no hit in `tmp/vba-samples/*.vba` |

Decision:

- `SNT-2` is closed.
- This supersedes the earlier sample-cache-only phrasing because the checked extracts are explicitly tied to staged real workbook files in workspace scope.
- No `C1` reopen is justified from these four placeholder families.

## SNT-3 Tiny Trigger Scoreboard

| Family | Current status | Last evidence source | Next action if status changes |
| --- | --- | --- | --- |
| `.FormulaLocal` literal target variant | supported | `tabell.vba`; shipped tests and `C1` closure | none |
| `.Value2` | absent | workspace-local workbook-derived extracts | reopen only if a redacted real snippet appears |
| `.FormulaR1C1` | absent | workspace-local workbook-derived extracts | reopen only if a redacted real snippet appears |
| `.NumberFormatLocal` | absent | workspace-local workbook-derived extracts | reopen only if a redacted real snippet appears |
| `.ColumnWidth` | absent | workspace-local workbook-derived extracts | reopen only if a redacted real snippet appears |
| `.Font.ColorIndex` | semantics-blocked | `tabell.vba` | reopen only with deterministic palette mapping proof |
| `.Interior.ColorIndex` | semantics-blocked | `tabell.vba`, `essbase.vba` | reopen only with deterministic palette mapping proof |
| `.RowHeight` | semantics-blocked | `essbase.vba` | reopen only with a recorder-grounded row-dimension target contract |
| dynamic formula concatenation | semantics-blocked | `essbase.vba` | reopen only with bounded expression-rewrite proof |
| shape or text frame writes | out of scope | `essbase.vba` | reopen only with a new approved target contract |
| workbook `Visible` or `Protect` flows | out of scope | `mylo.vba`, `essbase.vba` | reopen only with a new approved workbook-state slice |

Decision:

- `SNT-3` is closed.
- The scoreboard is small enough to use as the next loop's front door.

## Final Queue Decision

No implementation-worker dispatch is justified from these closures.

Current queue result:

- the workspace-local staging question is resolved
- the placeholder syntax backlog is now confirmed absent against real workbook-derived extracts
- the remaining families are either already supported, semantics-blocked, or out of scope

What is still not open:

- `A2`
- `B1`
- `C2`
- `E3`
- any new `C1` slice beyond the already closed `.FormulaLocal` variant
