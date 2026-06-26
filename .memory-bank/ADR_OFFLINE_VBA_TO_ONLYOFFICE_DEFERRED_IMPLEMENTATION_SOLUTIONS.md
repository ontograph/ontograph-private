# ADR: Offline VBA To ONLYOFFICE Deferred Implementation Solutions

Date: 2026-06-25

## Status

Proposal only. No new implementation approval.

## Context

The current accepted state is:

- Stage 0 target contract is complete.
- Stage 1 analyzer is complete.
- Stage 2 fail-closed preview translator is complete.
- Stage 3 workbook-assisted review is complete as `excel.review_vba_onlyoffice_workbook`.

The current owner remains `ontocode-rs/ext/excel`. The current explicit Excel surface is still a set of direct tools rather than a router or workflow engine:

- workbook inspection / preview / export
- VBA extraction
- ONLYOFFICE analyzer
- fail-closed ONLYOFFICE preview emission
- workbook-assisted review wrapper
- source-first VBA-to-M preview

Current code evidence:

- `excel.review_vba_onlyoffice_workbook` already routes safe modules through the existing analyzer and preview translator; it is not an independent preview emitter.
- the registered Excel extension surface is still explicit tools only; there is no current duplicate internal tool-selection path.
- runtime execution, parser dependency, and generic translation routing remain speculative until backed by concrete samples or a second sink.

This ADR proposes concrete implementation solutions for the remaining deferred areas without reopening rejected broad scope by accident.

## Real Sample Donor: `EssBaseWF.xlam`

A stronger real-world sample is available locally at:

- [EssBaseWF.xlam](/opt/YD/Downloads/Essbase.Danone/EssBaseWF.xlam)

Package evidence:

- the add-in is a real macro carrier with `xl/vbaProject.bin` size `628224`
- extracted modules include `mSVCSVGeneralLIB.bas` and `mGPN_2FA.bas`
- the workbook is not a narrow recorder-style sheet macro; it is an Excel add-in with ribbon callbacks, workbook mutation, shape metadata, comments, hyperlinks, and external Smart View interop

Examples that matter for product scope:

1. External add-in interop via `Declare` / `PtrSafe`

   Source evidence:

   - [olevba-extracted `mSVCSVGeneralLIB.bas`](/opt/YD/Downloads/Essbase.Danone/EssBaseWF.xlam)
   - representative lines from extraction:
     - `Public Declare PtrSafe Function HypConnect Lib "HsAddin" ...`
     - `Public Declare PtrSafe Function HypRetrieve Lib "HsAddin" ...`
     - `Public Declare PtrSafe Function HypExecuteCalcScript Lib "HsAddin" ...`

   Why it matters:

   - this sample contains roughly `400` `Declare` signatures
   - these are not translation candidates for ONLYOFFICE macros
   - they strengthen the existing fail-closed rule for external DLL / add-in calls
   - they do not justify a broader parser by themselves because the blocker is semantics, not syntax

2. Excel application-state toggling and sheet-level UI control

   Representative code from the extracted VBA:

   ```vb
   Application.ScreenUpdating = False
   Application.Calculation = xlCalculationManual
   Application.EnableEvents = False
   ActiveSheet.DisplayPageBreaks = False
   ActiveSheet.UsedRange.EntireRow.Hidden = False
   ```

   Why it matters:

   - these are real workbook automation operations
   - some may eventually map to a bounded spreadsheet-operation IR
   - many have no safe or useful ONLYOFFICE preview equivalent and should stay fail-closed

3. Workbook and worksheet mutation

   Representative code:

   ```vb
   Worksheets.Add(After:=Worksheets(Worksheets.Count)).Name = vNewSheetName
   Worksheets(vNewSheetName).Move After:=Worksheets(vOldSheetName)
   Worksheets(vOldSheetName).UsedRange.Copy
   Worksheets(vNewSheetName).Range("A1").PasteSpecial xlPasteValues
   Worksheets(vNewSheetName).Range("A1").PasteSpecial xlPasteFormulas
   ```

   Why it matters:

   - this is a better future-stage sample than toy `Range("A1").Value = ...`
   - it argues for operation-family grouping if workbook-copy support is ever reopened
   - it still does not require a public generic `excel.translate` surface

4. Shapes and text boxes used as hidden workbook state

   Representative code:

   ```vb
   ActiveSheet.Shapes.AddTextbox(...).Name = vNameOfTextBox
   ActiveSheet.Shapes(vNameOfTextBox).TextFrame.Characters.Text = vText
   For Each oTextBox In ActiveSheet.TextBoxes
       If InStr(UCase(oTextBox.Name), UCase(vNameOfTextBox)) > 0 Then
           oTextBox.Delete
       End If
   Next oTextBox
   ```

   Why it matters:

   - this is real spreadsheet-state mutation outside plain cells
   - it is a strong reason to keep translator claims narrow
   - it is also a reason not to let a parser dependency expand product scope by accident

5. Comments, named ranges, hyperlinks, and formula rewriting

   Representative code:

   ```vb
   Worksheets(vSheetName).Range(vCurrCellAdrees).AddComment "er"
   ActiveWorkbook.Names.Add Name:=getClearString(getNamedRange), RefersTo:=Worksheets(vNewSheetName).Range(vCurrCellAddress)
   Worksheets(vCurrSheetName).Hyperlinks.Add Anchor:=Worksheets(vCurrSheetName).Range(vCurrCellAddress), Address:="", SubAddress:=vCurrNamedRange
   ```

   Why it matters:

   - this is exactly the kind of non-trivial workbook-assisted flow that pushes beyond Stage 2 preview emission
   - it supports keeping workbook review and preview translation separate
   - it is a better argument for a small internal emit split than for runtime validation

6. Domain-specific formula parsing and rewriting

   Representative code from `mGPN_2FA.bas`:

   ```vb
   vArrFormula1 = Split(vCurrFormula, "HSGETVALUE(")
   vArrFormula2 = Split(vArrFormula1(1), ")")
   vArrFormula1 = Split(vArrFormula2(0), ",")
   ```

   Why it matters:

   - the sample has real formula-string parsing pressure
   - but the pressure is domain-specific to `HsGetValue`, not proof that a broad general VBA parser is the next smallest step
   - the first acceptable reopen from this sample would still be targeted augmentation, not a broad parser dependency

Net result:

- this add-in is valuable as a benchmark corpus
- it mostly strengthens current non-goals:
  - fail closed on external interop
  - keep explicit tools
  - do not promise broad VBA compatibility
- if any reopen happens from this sample, the first plausible ones are:
  - `B1` internal emit/module split under operation growth
  - `C1` targeted syntax augmentation for concrete blocked snippets
- it does not independently justify:
  - public `excel.translate`
  - runtime ONLYOFFICE execution
  - broad parser dependency

## Additional Real Workbook Samples: `/opt/YD/Temp/_w1`

Additional local samples are available in:

- `/opt/YD/Temp/_w1`

Current files:

- [Выдача спецодежды_без табельных.xlsm](/opt/YD/Temp/_w1/Выдача%20спецодежды_без%20табельных.xlsm)
- [Заявка на мыло.xlsm](/opt/YD/Temp/_w1/Заявка%20на%20мыло.xlsm)
- [Табель Макрос.xlsm](/opt/YD/Temp/_w1/Табель%20Макрос.xlsm)

Why this set matters:

- unlike `EssBaseWF.xlam`, these are operational business workbooks rather than an integration add-in
- all three are macro-enabled OpenXML workbooks with embedded `xl/vbaProject.bin`
- they give a second sample family:
  - workbook event handlers
  - sheet protection / visibility workflows
  - comment helpers
  - range-driven utility functions
  - user-facing business-sheet automation

### Workbook A: `Выдача спецодежды_без табельных.xlsm`

Observed sample shape:

- `Workbook_Open`
- repeated `Worksheets(...).Protect Password:=...`
- repeated `Sheets(...).Visible = True/False`
- large workbook with many sheets and comments/drawings parts
- package has `xl/vbaProject.bin` size `310272`

Representative code patterns:

```vb
Worksheets("Отделы").Protect Password:="111222333", UserInterfaceOnly:=True
Sheets("Отделы").EnableAutoFilter = True
Sheets("Отделы").Visible = False
```

Why it matters:

- this is real workbook lifecycle automation, not just a static module
- it strengthens event-handler and sheet-state examples for the analyzer
- the hard-coded password and protection semantics should remain fail-closed for ONLYOFFICE preview output

### Workbook B: `Заявка на мыло.xlsm`

Observed sample shape:

- `Workbook_BeforePrint`
- `Workbook_Open`
- `Worksheet_Change`
- `Worksheet_SelectionChange`
- active-sheet protection and sheet visibility toggles
- package has `xl/vbaProject.bin` size `47104`

Representative code patterns:

```vb
Private Sub Workbook_Open()
Sheets("СУП").Visible = xlHidden
Sheets("Заявка").Cells(6, "G") = "Структурное подразделение"
End Sub
```

Why it matters:

- this is a clean example of event-driven workbook logic
- it is useful for classifying supported read/write cell operations versus unsupported event semantics
- it argues for keeping workbook review separate from translation because the trigger context matters

### Workbook C: `Табель Макрос.xlsm`

Observed sample shape:

- helper-heavy standard module with range utilities
- comment helper:
  - `Public Sub AddComment(r, erc, com, er, typ)`
- array/string/date utilities
- range endpoint detection and day/hour aggregation routines
- package has `xl/vbaProject.bin` size `59904`

Representative code patterns:

```vb
Public Function FindEndRowRange(ran)
Public Function FindEndColumnRange(ran)
Public Sub AddComment(r, erc, com, er, typ)
Public Sub ChasNum(rangeName As String, addStep)
Public Sub DayNum(rangeName As String, addStep)
```

Why it matters:

- this is closer to general workbook business logic than the Smart View sample
- it gives realistic utility-style procedures without forcing external interop
- it is a better candidate set for narrow `C1` syntax augmentation if a concrete blocked snippet appears

### Combined Reading

These `_w1` files strengthen the benchmark corpus in a different way than `EssBaseWF.xlam`:

- `EssBaseWF.xlam` is strongest for:
  - external add-in interop
  - workbook copy flows
  - shapes/comments/hyperlinks
  - domain-specific formula rewriting
- `_w1` workbooks are strongest for:
  - `Workbook_Open` / workbook event procedures
  - sheet protection and visibility choreography
  - cell writes inside event handlers
  - range/comment helper utilities

Net effect on deferred options:

- they improve future test-fixture quality
- they strengthen the case for narrow analyzer examples and fail-closed classification
- they still do not independently justify:
  - public `excel.translate`
  - runtime ONLYOFFICE execution
  - broad parser dependency

If a reopen comes from this set, the most plausible first step is still `C1` targeted augmentation for a specific blocked syntax case, not a broad parser or new public routing surface.

### Corpus Classification Matrix

Use the current local sample corpus like this:

| Sample | Strongest evidence class | Good future use | Not a reopen trigger by itself |
| --- | --- | --- | --- |
| `EssBaseWF.xlam` | external interop, workbook mutation, shapes/comments/hyperlinks, formula rewriting | fail-closed analyzer fixtures, workbook-operation classification, future emit-family tests | broad parser dependency, runtime executor, public `excel.translate` |
| `Выдача спецодежды_без табельных.xlsm` | workbook-open setup, mass sheet protection, visibility choreography | fail-closed event/workbook-lifecycle fixtures | parser reopen, runtime reopen |
| `Заявка на мыло.xlsm` | workbook and worksheet events, direct cell writes inside event handlers | classify event context vs translatable cell operations | parser reopen unless a blocked syntax case is isolated outside event semantics |
| `Табель Макрос.xlsm` | general utility procedures, comments, range helpers, file-system automation | best candidate source for future `C1` syntax-gap fixtures | broad parser dependency unless several isolated blocked syntax cases accumulate |

A practical reading:

- `EssBaseWF.xlam` is the best negative-pressure donor:
  - it proves why external `Declare ... Lib` and workbook-side state should stay fail-closed
- `Заявка на мыло.xlsm` is the best event-context donor:
  - it proves why worksheet event bodies are not equivalent to plain source-first translation input
- `Табель Макрос.xlsm` is the best narrow parser donor:
  - it has the highest chance of yielding a future redacted `C1` syntax fixture without dragging in external interop

### Good Fixture Candidates

If the project later needs redacted fixtures, prefer examples in this order:

1. `Табель Макрос.xlsm`
   - range helpers
   - comment writes
   - string / array helpers
   - bounded `CreateObject("Scripting.FileSystemObject")` fail-closed examples
2. `Заявка на мыло.xlsm`
   - event-handler gating examples
   - row hide/show and range clear/write examples
3. `EssBaseWF.xlam`
   - shapes, hyperlinks, named ranges, workbook-copy flows
   - external `HsAddin` declarations as explicit unsupported examples
4. `Выдача спецодежды_без табельных.xlsm`
   - sheet protection / visibility mass-setup examples

This ordering is intentional:

- start with the smallest workbook logic that is closest to general VBA
- keep external interop and workbook-lifecycle automation as negative fixtures unless product scope changes

### Redaction Rules For Future Fixtures

Do not lift raw workbook snippets into tests unchanged.

The current sample corpus already contains sensitive or business-specific material such as:

- hard-coded passwords:
  - `Password:="111222333"`
  - `Const Pass = "..."`
  - `vCurrPasswordLine = "..."`
- user/environment reads:
  - `Environ("UserName")`
- business-specific workbook, sheet, and department names
- vendor-specific external library names and credentials surfaces:
  - `Lib "HsAddin"`
  - `vtUserName`, `vtPassword`

When deriving future fixtures:

1. Replace literal passwords with neutral placeholders.
   - example: `Password:="REDACTED_PASSWORD"`
2. Replace usernames and environment-dependent values with neutral placeholders.
   - example: `Environ("USER_PLACEHOLDER")`
3. Replace organization-specific workbook and sheet names with generic names unless the name itself is the subject under test.
   - example: `Sheets("SheetA")`, `Sheets("Lookup")`
4. Keep external library names only when the purpose of the fixture is to prove fail-closed unsupported interop.
5. Preserve the syntactic shape that matters for parsing.
   - redact values, not control-flow or call shape
6. Prefer the smallest snippet that still proves the classification.
   - one utility function
   - one event-handler fragment
   - one unsupported external declaration block

Good redacted fixture shapes:

```vb
Private Sub Workbook_Open()
    Sheets("Lookup").Visible = xlHidden
    Sheets("Main").Cells(6, "G") = "LABEL"
End Sub
```

```vb
Public Sub AddComment(r, c, txt)
    Cells(r, c).AddComment
    Cells(r, c).Comment.Text txt
End Sub
```

```vb
Public Declare PtrSafe Function ExternalCall Lib "UNSUPPORTED_LIB" () As Long
```

Bad fixture harvesting:

- copying full workbook-open routines with real passwords or staff names
- copying raw Smart View declaration blocks wholesale
- copying long business data loops when one 5-10 line fragment proves the same parser or analyzer point

## Idea Donors

These proposals are informed by the checked-out ONLYOFFICE donor repositories and their documented layering, not by a requirement to copy their architecture directly:

- ONLYOFFICE spreadsheet macro recorder evidence: `https://github.com/ONLYOFFICE/sdkjs`
  - pinned local evidence was captured from `tmp/onlyoffice/sdkjs`
- ONLYOFFICE desktop frontend shell: `https://github.com/ONLYOFFICE/DesktopEditors`
  - local donor checkout: `tmp/onlyoffice/desktop-apps`
- ONLYOFFICE web-apps frontend shell: `https://github.com/ONLYOFFICE/web-apps`
  - local donor checkout: `tmp/onlyoffice/web-apps`

Challenge:

- `sdkjs` is the real donor for macro preview shape and recorder behavior.
- `DesktopEditors` and `web-apps` are useful for ecosystem context only, not as implementation owners for the Excel extension.
- No external parser donor repository is accepted by this ADR yet; parser work stays sample-driven first.

## Decision Frame

Any reopened item should still satisfy these constraints:

- stay inside `ontocode-rs/ext/excel`
- preserve the analyzer-first fail-closed contract
- keep public tools explicit unless there is proven discovery pressure
- avoid a second orchestration stack
- keep runtime and environment coupling out unless a new local-only trust boundary clearly requires it

## Areas

The remaining deferred areas are:

1. static ONLYOFFICE preview validator
2. internal IR / module split
3. broad VBA parser dependency
4. public `excel.translate`
5. runtime ONLYOFFICE validation

## Option Set

### A. Static ONLYOFFICE Preview Validator

#### A1. Public explicit validator tool

Add:

- `excel.validate_onlyoffice_macro_preview`

Inputs:

- `macro_value`
- optional `function_body`

Behavior:

- verify the approved IIFE wrapper shape
- verify only the currently approved `Api.*` call patterns appear
- reject `macrosArray` writes
- reject filesystem / network / process / dynamic-exec patterns
- reject oversized or malformed preview payloads

Good:

- simple user-facing second check
- no runtime dependency
- cheap to test

Bad:

- duplicates part of the current translator gate
- adds a public tool before a new sink is proven

Use when:

- previews are being passed around or persisted outside the current translator/reviewer flow

#### A2. Internal validator only

Do not add a public tool.

Instead:

- move preview-shape checks into a shared internal helper only after a second independent preview sink or emitter exists
- keep `excel.review_vba_onlyoffice_workbook` as a caller of the existing translator while it remains a read-only wrapper

Good:

- avoids public surface growth
- removes duplicated preview-shape logic later if the emitter grows

Bad:

- no standalone user-facing validation capability

Use when:

- a second internal sink starts duplicating preview-shape checks

#### Recommendation

Prefer no reopen by default. If reopened, use `A2` only when preview-shape logic is duplicated by a second independent sink or emitter. The current workbook-review wrapper does not satisfy that trigger because it already routes through the translator. Promote `A1` only if a new user-visible sink exists that the current fail-closed translator no longer fully protects.

### B. Internal IR / Module Split

#### B1. Lightweight emit module split

Split current translator internals into:

- `vba_onlyoffice_emit.rs`
- keep analyzer output shape unchanged

Behavior:

- only the line-emission and preview-assembly logic moves
- no new IR type yet

Good:

- smallest refactor
- reduces file growth
- keeps public contracts unchanged

Bad:

- analyzer/emitter coupling remains mostly intact

Use when:

- operation count is growing but not exploding

#### B2. Small internal spreadsheet IR

Add:

- `vba_onlyoffice_ir.rs`
- `vba_onlyoffice_emit.rs`

Behavior:

- analyzer lowers supported operations into a tiny internal IR
- translator emits ONLYOFFICE preview from IR
- public JSON still stays unchanged

Good:

- cleaner seam for future validation and tests
- better long-term emitter hygiene

Bad:

- more rewrite risk than B1
- easy to overbuild before more operations actually land

Use when:

- several new operation families are about to be added and operation-summary emission is getting brittle

#### Recommendation

Prefer no reopen by default. `B1` is the first acceptable refactor only under concrete operation-growth or file-growth pressure. Escalate to `B2` only after `B1` no longer keeps the emitter simple.

### C. Broad VBA Parser Dependency

#### C1. Targeted parser augmentation

Keep the current hand-rolled parser and add bounded support for specific missing constructs:

- line continuations
- declaration variants
- selected control-flow forms that are already in product scope

Good:

- smallest diff
- no new dependency
- keeps product claims narrow

Bad:

- parser debt remains manual

Use when:

- a few concrete user samples are blocked by shallow syntax gaps

#### C2. Private grammar-backed parser adapter

Add a VBA parser dependency behind a private adapter layer:

- no direct dependency leakage into tool contracts
- still fail closed on unsupported semantics

Good:

- better source understanding ceiling
- cleaner future IR work

Bad:

- larger dependency and maintenance surface
- license and update review required
- easy to drift into broader product claims

Use when:

- multiple in-scope user samples cannot be handled safely by targeted augmentation

Required gates before dispatch:

- name the parser candidate and maintenance owner
- confirm license compatibility
- add a redacted sample corpus that demonstrates repeated C1 failure
- prove output stays bounded and fail-closed on unsupported semantics
- keep dependency types out of public tool payloads

#### C3. Parser sidecar / offline preprocess path

Use a separate offline parser helper binary or local adapter that produces bounded structured output for the existing analyzer.

Good:

- isolates parser churn from main logic
- can be tested independently

Bad:

- introduces a second moving part
- easy to become an accidental side stack

Use when:

- dependency isolation matters more than simplicity

#### Recommendation

Prefer `C1`. Only consider `C2` after several concrete blocked samples and all dependency gates above are satisfied. Treat `C3` as effectively rejected unless dependency isolation becomes a hard constraint, because it risks creating a second parsing stack.

### D. Public `excel.translate`

#### D1. Keep rejected publicly

Do nothing.

Good:

- simplest
- preserves explicit tool semantics
- avoids monolith regression

Bad:

- discovery remains manual

Use when:

- explicit tools are still understandable enough

#### D2. Internal router only

Add a private helper that routes between:

- VBA -> M preview
- Power Query -> SQL preview
- VBA -> ONLYOFFICE JS preview

Public tools remain unchanged.

Good:

- shared implementation only when duplicate internal callers exist
- no public contract churn

Bad:

- limited direct user value

Use when:

- internal call paths start duplicating tool selection logic

#### D3. Public facade tool

Add:

- `excel.translate`

Behavior:

- selects one explicit translation path
- reports which explicit tool path it used

Good:

- friendlier discovery

Bad:

- highest risk of recreating the rejected monolith
- mixed trust/validation semantics in one public entry point

Use when:

- tool discoverability is proven to be a persistent user problem

#### Recommendation

Keep `D1` now. Use `D2` only if internal duplication becomes measurable in current code; do not build it for hypothetical future UX. Treat `D3` as blocked unless tool discoverability becomes a proven user problem with evidence that explicit-tool help is insufficient.

### E. Runtime ONLYOFFICE Validation

#### E1. Local experimental runner

Blocked. This is not a dispatchable tool proposal until a stable repo-local harness exists.

Possible future tool name:

- `excel.validate_onlyoffice_macro_runtime_experimental`

Behavior:

- execute preview macros in a sandboxed local harness
- return bounded pass/fail diagnostics
- never persist workbook mutations by default

Good:

- highest confidence on runtime compatibility

Bad:

- environment-sensitive
- hardest to keep deterministic
- largest maintenance burden

Use when:

- static guarantees are no longer enough for a real workflow

#### E2. Fixture-based runtime replay tests only

Do not add a public tool.

Instead:

- create repo-local runtime proof fixtures or replay harness tests for development-only validation

Good:

- no public surface
- improves maintainer confidence

Bad:

- no direct user-facing validation

Use when:

- maintainers need stronger regression confidence but runtime validation is not worth exposing

#### E3. Snapshot contract drift checker

Do not execute macros.

Instead:

- compare generated preview output against recorder-derived known-good fixtures

Good:

- cheap
- deterministic

Bad:

- still not real runtime execution

Use when:

- the main need is drift detection, not runtime proof

#### Recommendation

Prefer `E3` first if stronger confidence is needed. Use `E2` for maintainer-only checks. Keep `E1` blocked and non-dispatchable until a stable repo-local harness exists; otherwise it is just a second runtime stack in disguise.

## Recommended Reopen Order

If any deferred area is reopened, use this order:

1. static preview validation only if it protects a new sink
2. lightweight emit-module split if operation count starts growing
3. targeted parser augmentation only against concrete blocked samples
4. internal router only if duplicate tool-selection logic appears
5. runtime drift checks before any public runtime validator

Current status: none of these triggers is satisfied by the existing Stage 3 workbook-review flow.

## Non-Recommendations

Do not do these as one change:

- parser dependency plus product-scope expansion
- public `excel.translate` plus new translation families
- runtime execution plus workbook rewrite
- public validator plus generic router
- large IR rewrite before operation-growth pressure exists

## Recommendation

The default path is still the lazy one:

- keep the current surface as-is
- reopen only one deferred area at a time
- choose the smallest option in that area that solves a concrete new problem

Current best default per area:

- static validator: no reopen, otherwise `A2`
- IR split: no reopen, otherwise `B1`
- parser dependency: `C1`
- public `excel.translate`: `D1`
- runtime validation: `E3`
