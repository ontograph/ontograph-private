# Excel MCP Senior Unblock

Date: 2026-06-26
Scope: `.memory-bank/EXCEL_MCP_2000_USEFUL_SOLUTIONS_REVIEW.md`

## Decision

Opened one implementation-ready task:

- `EXCEL-MCP-U1`: harden `excel.inspect_workbook` for donor workbooks whose Power Query `customXml/item*.xml` parts are UTF-16. Current `backend.rs` reads optional XML entries with `read_to_string`, so `tmp/excel/in2sql_dotNet_addin/tools/WorkbookArtifactExtractor/examples/pq_test.xlsx` and `.xlsm` fail with invalid UTF-8 before inspection can return package markers.

Opened two evidence-only tasks:

- `EXCEL-MCP-U2`: create a minimal workbook proof for validation metadata rows `010-012`.
- `EXCEL-MCP-U3`: create a minimal workbook proof for formula inventory row `037`.

## Still Blocked

- Rows `038-040`: formula AST / formula-to-SQL requires a separate ADR and validation contract.
- Row `041`: named-range rewrite remains blocked; read-only named-range inspection may be proposed later with fixtures.
- Row `042`: live `Formula2` work belongs to a separate live Excel companion ADR.
- Rows `043-044`: workbook graph work requires real edge extraction proof.

## Verification

- Reproduced `excel.inspect_workbook` failure on the donor `.xlsx` and `.xlsm`.
- Confirmed the failing `customXml/item1.xml` starts with UTF-16LE BOM and declares `encoding="utf-16"`.
- Confirmed OntoIndex module ownership for `ontocode-rs/ext/excel/src/backend.rs`.
