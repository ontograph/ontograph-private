# Excel Advanced Phase 2 Fixture Proof

## Goal
Verify the output shape, edge resolution, and fail-closed rules for `excel.scan_sheet_formulas_dependency` and `excel.generate_slider_query_package` before any implementation lands.

## Input Worksheet XML Fixture
A synthetic `xl/worksheets/sheet1.xml` containing:
- Clean sequential calculation: `B1=A1*2`, `C1=B1+5`
- A circular dependency: `A2=B2`, `B2=A2`
- An unsupported volatile reference: `D1=OFFSET(A1,1,1)`

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <sheetData>
    <row r="1">
      <c r="A1"><v>10</v></c>
      <c r="B1"><f>A1*2</f><v>20</v></c>
      <c r="C1"><f>B1+5</f><v>25</v></c>
      <c r="D1"><f>OFFSET(A1,1,1)</f></c>
    </row>
    <row r="2">
      <c r="A2"><f>B2</f></c>
      <c r="B2"><f>A2</f></c>
    </row>
  </sheetData>
</worksheet>
```

## Expected `excel.scan_sheet_formulas_dependency` Output
```json
{
  "sheetName": "Sheet1",
  "nodes": [
    {
      "cell": "B1",
      "formula": "A1*2",
      "dependencies": ["A1"],
      "hasCycle": false,
      "isSupported": true
    },
    {
      "cell": "C1",
      "formula": "B1+5",
      "dependencies": ["B1"],
      "hasCycle": false,
      "isSupported": true
    },
    {
      "cell": "D1",
      "formula": "OFFSET(A1,1,1)",
      "dependencies": ["A1"],
      "hasCycle": false,
      "isSupported": false,
      "unsupportedReason": "volatile_function_offset"
    },
    {
      "cell": "A2",
      "formula": "B2",
      "dependencies": ["B2"],
      "hasCycle": true,
      "isSupported": true
    },
    {
      "cell": "B2",
      "formula": "A2",
      "dependencies": ["A2"],
      "hasCycle": true,
      "isSupported": true
    }
  ],
  "cyclesDetected": [
    ["A2", "B2"]
  ]
}
```

## Expected `excel.generate_slider_query_package` Output
The package must group supported calculations and isolate blockages:
- `manifest.json`: List of generated queries, parameters, and skipped/blocked formulas.
- `queries/sheet1_prepared.sql`: Sequential calculations represented as prepared SQL columns.
- `queries/sheet1_blocked.json`: Detailed records of skipped circular/volatile formulas.

### Generated `queries/sheet1_prepared.sql`
```sql
-- Source: Sheet1, cells B1, C1
-- Confidence: high

WITH base_source AS (
    SELECT *
    FROM raw."sheet1"
)
SELECT
    *,
    (col_a * 2) AS col_b,
    ((col_a * 2) + 5) AS col_c
FROM base_source;
```

### Generated `manifest.json`
```json
{
  "packageName": "sheet1_slider_package",
  "generatedQueries": [
    {
      "name": "sheet1_prepared",
      "type": "prepared_columns",
      "sqlPath": "queries/sheet1_prepared.sql"
    }
  ],
  "blockedFormulas": [
    {
      "cell": "D1",
      "formula": "OFFSET(A1,1,1)",
      "reason": "volatile_function_offset"
    },
    {
      "cell": "A2",
      "formula": "B2",
      "reason": "circular_dependency"
    },
    {
      "cell": "B2",
      "formula": "A2",
      "reason": "circular_dependency"
    }
  ]
}
```

## Fail-Closed Rules
1. Any sheet containing volatile functions or circular references must still generate the `prepared` query for unrelated clean sub-graphs (e.g. B1/C1), but must explicitly flag the blocked cells in the package manifest.
2. If a clean sub-graph depends on a blocked cell, that sub-graph must also be flagged as blocked (e.g. if `E1=D1+1`, E1 is blocked because D1 uses `OFFSET`).
