# Excel Legacy `.xls` Feasibility Note

## Status

Feasibility-only. No `.xls` implementation, dependency change, tool-surface change, or fixture approval is opened by this note.

## Date

2026-06-27

## Context

Current `ext/excel` is built around ZIP/OpenXML inspection. OntoIndex and code inspection show:

- `inspect_workbook_with_display_path` opens the workbook with `zip::ZipArchive` and classifies only OpenXML-derived package shapes
- `excel.inspect_workbook` path validation currently accepts only `.xlsx`, `.xlsm`, and `.xlsb`
- `excel.read_sheet_preview`, `excel.inspect_sheet_formulas`, and `excel.export_sheet_to_csv` still support only `.xlsx` and `.xlsm` in this stage
- `.xlsb` already proves that package-level detection does not imply full worksheet-parity support

The runtime dependency surface in `ontocode-rs/ext/excel/Cargo.toml` is `zip`, `quick-xml`, and `ovba`. The `cfb` crate exists only as a dev-dependency for test-fixture construction, not as a runtime workbook reader.

Donor evidence from `tmp/excel/in2sql_dotNet_addin` is also cautionary: its tests explicitly mark legacy `.xls` as unsupported when no managed reader exists, while `.xlsx`, `.xlsm`, and `.xlsb` have separate managed paths.

This means `.xls` is not a small extension of the current owner. It is a second workbook-container and parsing problem.

## Feasibility Verdict

Legacy `.xls` support is feasible only as a narrow, inspect-first reopen. It is not currently justified as preview/export/formula/translation parity work.

The smallest acceptable future scope is:

- inspect-only metadata
- one real `.xls` artifact
- explicit owner boundary inside offline `ext/excel`
- no automatic extension of preview, export, formula inspection, SQL planning, VBA translation, or workbook mutation

## Options Reviewed

### Option A: Keep `.xls` unsupported

Pros:

- zero owner churn
- no new runtime parser
- preserves the current OpenXML-only trust model

Cons:

- user must convert `.xls` files outside Ontocode before inspection

Manager verdict:

- still a valid default
- cheapest correct option until real `.xls` demand is proven

### Option B: External conversion path, then reuse current OpenXML tools

Shape:

- use a separate conversion companion or explicit user-provided converted artifact
- feed the resulting `.xlsx` or `.xlsm` into the existing `ext/excel` tools

Pros:

- preserves the current Rust owner and tool semantics
- avoids adding a second parser stack to `ext/excel`
- naturally keeps `.xls` reopen scope inspect-first and artifact-driven

Cons:

- conversion provenance must be explicit
- some legacy workbook semantics may be normalized or lost during conversion
- requires a separate operational story outside the current extension

Manager verdict:

- preferred first reopen path if real `.xls` demand appears
- keep it outside the current bounded tool surface until provenance and artifact rules are written down

### Option C: Native `.xls` inspect-only runtime path inside `ext/excel`

Shape:

- add a runtime compound-file / BIFF reader path
- limit the first slice to inspect-only metadata

Pros:

- keeps the user inside one offline Rust tool family
- could preserve legacy workbook evidence without conversion

Cons:

- adds a second parser/container owner to an extension that is currently ZIP/OpenXML-first
- requires new bounded-read policies for compound-file sectors and BIFF record walks
- does not justify automatic parity with preview/export/formula tooling
- current repo has no runtime `.xls` reader; `cfb` is only used in tests

Manager verdict:

- possible, but only after explicit dependency approval and a separate inspect-only contract
- do not open this as a “small follow-up”

### Option D: Hand-rolled BIFF/OLE parser in this repo

Manager verdict:

- rejected
- this is architecture drift and parser theater unless there is long-term product demand plus fixture-backed proof that an external dependency cannot satisfy the inspect-only need

## Recommended Reopen Path

If `.xls` is reopened, use this order:

1. prove real demand with at least one user-relevant `.xls` fixture
2. decide whether conversion-first or native inspect-only is required
3. if native is required, approve a runtime dependency strategy before any tool work
4. reopen only `excel.inspect_workbook`-level metadata first
5. require a second senior-review pass before preview/export/formula parity is even discussed

## Hard Boundaries

Any first `.xls` reopen must keep these boundaries:

- inspect-only first
- no silent conversion
- no promise of sheet preview parity
- no promise of formula metadata parity
- no SQL planning
- no graph extraction
- no workbook mutation
- no live Excel or COM fallback hidden inside `ext/excel`

## Minimal Artifact Requirements Before Code

Implementation should stay blocked until the reopen pack contains:

- one real `.xls` workbook fixture
- expected metadata output for that fixture
- explicit statement whether the chosen path is conversion-first or native inspect-only
- dependency and licensing review for any new runtime reader
- byte/record budget policy for compound-file and BIFF reads

## Senior Challenge Outcome

The current codebase is not one careful `if extension == "xls"` away from legacy support.

The professional move is to treat `.xls` as a separate container/read-path decision, reopen inspect-only at most, and prefer conversion-first unless native legacy evidence is genuinely required.
