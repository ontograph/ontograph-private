# Offline VBA To ONLYOFFICE Micro Task Closure

Date: 2026-06-25

## Scope

Run a bounded manager loop over the currently opened `SMT-*` micro-tasks using OntoIndex-backed preflight plus the staged VBA extracts and current ONLYOFFICE audit notes.

## Roles

- manager: current session
- senior-reviewer: handled by manager locally because this is a bounded evidence and formatting pass
- implementation-worker: not dispatched because no implementation slice reopened
- verification-worker: handled by manager locally because this is documentation and queue-state verification only

## Evidence

- OntoIndex `gn_ensure_fresh` reports `codex` fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2`.
- Dirty-worktree caveat remains active with `dirtyFileCount=265` and `scopeConfidence=medium`.
- The staged VBA extracts already contain stable provenance lines:
  - `tmp/vba-samples/tabell.vba`
  - `tmp/vba-samples/essbase.vba`
  - `tmp/vba-samples/mylo.vba`
- Existing audit notes already show repeated redaction patterns for:
  - passwords
  - workbook protection
  - business-sensitive workbook and sheet references
  - server or URL literals
- The current plan and closure notes already define the core Slice 0 decision fields:
  - candidate family
  - redacted snippet
  - source provenance
  - status bucket
  - reopen or no-reopen outcome

## SMT-1 Provenance Header Normalization

Decision:

- `SMT-1` is closed.

Canonical header shape:

- first line may keep the existing `olevba` banner when present
- required provenance line:
  - `FILE: tmp/vba-samples/<workbook-file-name>`
- recommended first module marker immediately after the header block:
  - `VBA MACRO <module-name>`
- extraction date is not required inside the extract because repository history already tracks the artifact update date

Why this is enough:

- the current extracts already carry the only provenance field needed for Slice 0 review
- adding more header metadata would increase churn without improving reopen decisions

## SMT-2 Redaction Placeholder Lexicon

Decision:

- `SMT-2` is closed.

Accepted placeholder lexicon:

| Sensitive source | Placeholder shape |
| --- | --- |
| password literal or password constant | `<redacted>` inside code snippets, or `REDACTED_PASSWORD` in test-style literals |
| username or login literal | `REDACTED_USER` |
| workbook name when business-specific | `Workbook("<redacted-workbook>")` |
| sheet name when business-specific | `Sheets("<redacted-sheet>")` |
| server, host, or URL literal | `REDACTED_HOST` or `https://<redacted-host>/...` |
| business label, department, or domain text | `<redacted-label>` |

Rules:

- prefer placeholders that preserve code shape
- keep placeholders stable across notes
- redact before promoting snippets into ADR or audit docs
- do not redact technical family names such as `.FormulaLocal` or `.ColorIndex`

## SMT-3 Slice 0 Trigger Card Template

Decision:

- `SMT-3` is closed.

Accepted one-card template:

```md
### Trigger Card: <candidate-family>

- source provenance: `FILE: tmp/vba-samples/<workbook-file-name>`
- redacted snippet: `<one minimal snippet>`
- existing operation-family match: `<yes: SetCellFormula | no>`
- status: `<supported | absent | semantics-blocked | out of scope>`
- reopen recommendation: `<yes | no>`
- note: `<one-sentence reason>`
```

Why this is enough:

- it fits in one screen
- it matches the current manager-loop decision fields
- it avoids building a second intake process

## Final Queue Decision

No implementation-worker dispatch is justified from these closures.

Current queue result:

- provenance handling is standardized enough
- redaction placeholders are now explicit
- the next Slice 0 review can be recorded in one short card

What is still not open:

- `A2`
- `B1`
- `C2`
- `E3`
- any new `C1` slice beyond the already closed `.FormulaLocal` variant
