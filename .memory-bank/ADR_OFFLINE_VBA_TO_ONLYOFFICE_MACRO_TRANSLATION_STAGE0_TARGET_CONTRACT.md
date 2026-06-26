# Stage 0 Target Contract Capture: Offline VBA To ONLYOFFICE Macro Translation

Status: checked

## Source Pin

- ONLYOFFICE source repo: `https://github.com/ONLYOFFICE/sdkjs.git`
- ONLYOFFICE source commit: `72b0421c0bbf9d01eed9cf14834ae47eb2df1b50`
- Local evidence: [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js)

## Evidence Anchors

- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:243) through [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:253) show the persisted IIFE wrapper and `macrosArray` / `current` payload update path.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:465) through [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:468) seed spreadsheet macros with `Api.GetActiveSheet()` and `Api.GetActiveWorkbook()`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1462) through [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1547) cover active-cell writes and selection formatting emitters.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1640) through [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1705) show later spreadsheet actions that stay deferred from the first slice.

## First-Slice Contract

### Macro Wrapper Shape

The preview output for the first slice must be a paste-ready ONLYOFFICE macro value in IIFE form, with the inner body kept separately for review and diffing.

```javascript
(function()
{
    let worksheet = Api.GetActiveSheet();
    let workbook = Api.GetActiveWorkbook();
    // emitted body here
})();
```

The checked contract is:

- `macro_value` is the full IIFE string
- `function_body` is the bounded inner JavaScript body
- the first slice does not emit the surrounding `macrosArray` payload

### Supported First-Slice Call Catalog

Only the following recorder-grounded spreadsheet calls are admitted in the first slice. The table includes root `Api.*` calls and the recorder methods reached from those roots:

| Call | Role in the first slice |
| --- | --- |
| `Api.GetActiveSheet()` | Root worksheet handle for spreadsheet macros |
| `Api.GetActiveWorkbook()` | Root workbook handle for spreadsheet macros |
| `Api.GetSelection()` | Selection root for formatting operations |
| `worksheet.GetActiveCell()` | Active-cell root for direct value and formula writes |
| `Api.CreateColorFromRGB(...)` | Color construction for selection fill and font color |
| `Api.GetSelection().SetBold(...)` | Boolean font toggle |
| `Api.GetSelection().SetItalic(...)` | Boolean font toggle |
| `Api.GetSelection().SetFontSize(...)` | Font-size formatting |
| `Api.GetSelection().SetFontName(...)` | Font-name formatting |
| `Api.GetSelection().SetFontColor(...)` | Text color formatting |
| `Api.GetSelection().SetBackgroundColor(...)` | Fill color formatting |
| `Api.GetSelection().SetNumberFormat(...)` | Number-format formatting |
| `Api.GetSelection().SetWrap(...)` | Wrap formatting |
| `Api.GetSelection().SetAlignHorizontal(...)` | Horizontal alignment formatting |
| `Api.GetSelection().SetAlignVertical(...)` | Vertical alignment formatting |
| `worksheet.GetActiveCell().SetValue(...)` | Direct value write |
| `worksheet.GetActiveCell().SetFormulaArray(...)` | Direct formula write |

## Example VBA To ONLYOFFICE Pairs

### Example 1: Value And Formula Writes

```vb
Sub FillCell()
    ActiveCell.Value = "Ready"
    ActiveCell.Formula = "=SUM(A1:A3)"
End Sub
```

```javascript
(function()
{
    let worksheet = Api.GetActiveSheet();
    let workbook = Api.GetActiveWorkbook();
    worksheet.GetActiveCell().SetValue("Ready");
    worksheet.GetActiveCell().SetFormulaArray("=SUM(A1:A3)");
})();
```

### Example 2: Selection Formatting

```vb
Sub FormatSelection()
    Selection.Font.Bold = True
    Selection.Font.Italic = False
    Selection.Font.Size = 12
    Selection.NumberFormat = "0.00"
End Sub
```

```javascript
(function()
{
    let worksheet = Api.GetActiveSheet();
    let workbook = Api.GetActiveWorkbook();
    Api.GetSelection().SetBold(true);
    Api.GetSelection().SetItalic(false);
    Api.GetSelection().SetFontSize("12");
    Api.GetSelection().SetNumberFormat("0.00");
})();
```

### Example 3: Brush-Up Formatting With Colors

```vb
Sub PaintSelection()
    Selection.Font.Name = "Arial"
    Selection.Font.Color = RGB(10, 20, 30)
    Selection.Interior.Color = RGB(200, 210, 220)
End Sub
```

```javascript
(function()
{
    let worksheet = Api.GetActiveSheet();
    let workbook = Api.GetActiveWorkbook();
    Api.GetSelection().SetFontName("Arial");
    Api.GetSelection().SetFontColor(Api.CreateColorFromRGB(10, 20, 30));
    Api.GetSelection().SetBackgroundColor(Api.CreateColorFromRGB(200, 210, 220));
})();
```

## Deferred Operations

Observed in the recorder, but deferred from the first slice until analyzer and IR coverage are proven:

- merge and unmerge
- sort
- auto filter
- range select
- comments
- hyperlinks
- images
- shapes
- chart insertion
- paste and clear actions
- font increase and decrease shortcuts
- border mutation variants
- `Api.Format(...)`-style conversion helpers

## Non-Scope

The Stage 0 contract does not admit or promise:

- runtime execution against ONLYOFFICE
- workbook mutation in Ontocode
- a generic `excel.translate` surface
- a second Excel workflow stack
- full VBA compatibility
- event procedures
- `Function` procedures with return semantics
- `On Error`
- COM automation
- external DLL calls
- user forms
- shell, file, or network I/O
- late-bound object access
- dynamic invocation patterns such as `CallByName`

## Bounds And Redaction Expectations

The first implementation slices that consume this contract must stay bounded and redacted:

- cap source input size
- cap emitted JavaScript size
- cap procedure count
- cap warning and blocker count
- cap literal passthrough before a value can appear in output
- redact connection strings
- redact passwords and token-looking literals
- redact authorization headers
- redact local keychain or credential-store paths
- redact full local filesystem paths when they appear in VBA literals
- redact URLs with embedded credentials

No raw secret value should survive into `macro_value`, `function_body`, warnings, blockers, or debug text.

## Drift-Check Expectations

The checked fixture must fail fast if the upstream recorder contract drifts:

- the pinned repository or commit changes
- the IIFE wrapper shape changes
- `macrosArray` / `current` persistence changes
- the supported first-slice call catalog changes
- a deferred operation is accidentally promoted into the first slice without an explicit artifact update

The drift check is static and repo-local. It should compare the captured contract against the pinned recorder evidence, not against a live ONLYOFFICE runtime.
