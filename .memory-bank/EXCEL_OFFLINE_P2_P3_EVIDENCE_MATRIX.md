# Excel Offline P2 P3 Evidence Matrix

## Status
Closed as the smallest unblock step for the remaining offline Excel follow-ons.

This file does not reopen implementation by itself. It only proves:
- what current owners already answer
- what package-level metadata is present in real workbooks
- which exact unanswered questions would justify reopening `P2` or `P3`

## Sources
- Current workbook owner:
  - `ontocode-rs/ext/excel/src/backend.rs`
  - `ontocode-rs/ext/excel/src/tool.rs`
- Current Power Query owner:
  - `ontocode-rs/ext/excel/src/powerquery_extract.rs`
  - `ontocode-rs/ext/excel/src/powerquery_review_bundle.rs`
- Sample workbooks:
  - `tmp/excel/in2sql_dotNet_addin/tools/WorkbookArtifactExtractor/examples/pq_test.xlsm`
  - `tmp/excel/samples/Unleashing-Power-Query-for-Data-Scientists-Reshape-Merge-Clean-at-Scale.xlsx`
  - `tmp/excel/samples/Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm`

## OntoIndex Owner Check
- `inspect_workbook_with_display_path` remains the high-blast workbook owner
- `extract_powerquery_queries_from_workbook` remains the lower-blast Power Query owner

## P2 Evidence: Pivot Report Metadata

### What current `inspect_workbook` already answers
- whether the workbook has pivot artifacts at all
- whether the workbook has connections at all
- bounded marker summaries and sample part paths

### What the real sample package proves exists
From `Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm`:
- `xl/pivotTables/pivotTable1.xml` through `pivotTable7.xml` exist
- `xl/pivotCache/pivotCacheDefinition1.xml` through `pivotCacheDefinition9.xml` exist
- `xl/connections.xml` contains Data Model and query connections
- `xl/workbook.xml` contains workbook-level `pivotCache cacheId` entries
- pivot cache definitions expose `cacheSource type="external" connectionId="8"`
- pivot table definitions expose report names such as `PivotTable2`, `PivotTable1`, `PivotTable3`
- pivot table definitions expose `cacheId` and sheet-local `location ref`

### What current `inspect_workbook` does not answer
- which PivotTable maps to which cache id
- whether a specific pivot is Data Model / OLAP backed
- which connection id backs a specific pivot cache
- which source range or source name backs a specific pivot
- whether any stored MDX text exists for a specific pivot report

### P2 result
- There is a real package-level metadata gap.
- There is still no consumer proof yet.

### Exact P2 reopen questions
- “Which PivotTable uses the Data Model?”
- “What cache id and connection id back PivotTable X?”
- “What source range or source name backs PivotTable X?”
- “Does PivotTable X have stored MDX in the workbook package?”

If a real downstream task asks one of those questions, `P2` is justified.

## P3 Evidence: Detailed Workbook Connections

### What current Power Query outputs already answer
From `extract_powerquery_queries_from_workbook`:
- `connection_name`
- `location`
- `command_preview`
- extracted query text
- lexical references
- bundle artifact paths and normalized copies
- bounded worksheet-load targets when package routing can prove them:
  - `ExternalData_*` name
  - worksheet table name
  - sheet name
  - target range

### What the simple real sample proves current outputs already cover
From `pq_test.xlsm`:
- `xl/connections.xml` contains Mashup OLE DB connections with workbook locations such as:
  - `CSV File table`
  - `DimDate`
  - `Excel File table`
- those connections also contain SQL-style command previews such as `SELECT * FROM [DimDate]`

This means current Power Query extraction already answers the basic question:
- “What workbook query name and connection preview back this extracted M query?”

### What the richer real samples prove exists but current outputs do not expose
From `Unleashing-Power-Query-for-Data-Scientists-Reshape-Merge-Clean-at-Scale.xlsx`:
- `xl/connections.xml` contains worksheet-load query connections:
  - `Query - CSV Files`
  - `Query - Customers`
  - `Query - Support Tickets`
  - `Query - Transactions`
- `xl/queryTables/queryTable1.xml` through `queryTable4.xml` expose `connectionId` values that map those queries into worksheet query tables
- `xl/tables/table1.xml` through `table4.xml` expose `tableType="queryTable"` plus table identities such as:
  - `Support_Tickets`
  - `Transactions`
  - `Customers`
  - `CSV_Files`
- `xl/workbook.xml` defined names expose exact sheet targets for those loads, for example:
  - `ExternalData_3` -> `'Customers'!$A$1:$G$11`
  - `ExternalData_4` -> `Transactions!$A$1:$C$61`

This proves a bounded offline routing path exists for:
- query connection -> queryTable -> table -> worksheet range

From `Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm`:
- `xl/connections.xml` contains:
  - Data Model connections with `model="1"`
  - `ThisWorkbookDataModel`
  - query connections like `Query - Customers`, `Query - Products`, `Query - Sales`
- `xl/workbook.xml` contains `x15:modelTable` entries tying:
  - `Sales` -> `Query - Sales`
  - `Customers` -> `Query - Customers`
  - `Products` -> `Query - Products`
  - `Regions` -> `Query - Regions`
- pivot cache definitions point back to connection id `8`
- pivot tables expose concrete `cacheId` values and sheet-local `location ref`
- `xl/model/item.data` and pivot cache metadata show `Sales` in the model and pivot surface even though the simple connection view alone does not explain full consumer routing

### What current Power Query outputs do not answer
- deeper table/query-specific PivotTable usage such as measures, fields, stored MDX, or a package-proven claim that PivotTable X consumes only query Y

### P3 result
- There is a real package-level metadata gap.
- Current outputs already cover the basic query-name / location / preview case.
- Current outputs now also cover the bounded worksheet-table / worksheet-range routing case when package evidence is present.
- Current outputs now also cover bounded Data Model table routing and Data-Model-wide pivot consumers when package evidence is present.
- The remaining package-level metadata gap is no longer a Power Query extractor gap; it is detailed pivot-report semantics and stays under `P2` unless a narrower package-proof owner emerges.

### Exact P3 reopen questions
- “Which specific Data Model field, measure, MDX item, or table-specific payload proves PivotTable X consumes query Y rather than the Data Model as a whole?”

If a real downstream task asks that question, challenge whether it belongs to `P2` pivot metadata before reopening `P3`.

## Decision
- `P2` stays closed until a real consumer asks one of the exact pivot-report questions above.
- broader `P3-A` is closed.
- broader `P3-B` is closed for bounded Data Model table routing and Data-Model-wide pivot consumers in the Power Query owner.
- No dependency-ready `P3` implementation task remains.
