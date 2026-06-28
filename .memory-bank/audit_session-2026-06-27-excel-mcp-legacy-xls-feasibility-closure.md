# Excel MCP Legacy `.xls` Feasibility Closure

Date: 2026-06-27

Scope:
- Close the design-only feasibility step for legacy `.xls` support.

OntoIndex and code evidence used:
- `inspect_workbook_with_display_path` in `ontocode-rs/ext/excel/src/backend.rs` opens workbooks with `zip::ZipArchive`, proving the current owner is ZIP/OpenXML-first.
- `ExcelInspectionTool.handle` still routes through the same offline workbook-inspection owner.
- `ontocode-rs/ext/excel/src/tool.rs` currently validates `excel.inspect_workbook` paths only for `.xlsx`, `.xlsm`, and `.xlsb`.
- current tests prove `.xlsb` is only partially supported today, which blocks any claim that `.xls` would be a trivial parity follow-up.

Donor evidence used:
- donor tests explicitly mark legacy `.xls` as metadata-unavailable when no managed reader exists
- donor `.xlsb` work uses a separate managed branch, reinforcing that legacy binary formats are distinct owner problems rather than extension-flag toggles

Decision:
- accept a feasibility-only `.xls` note
- keep `.xls` implementation blocked
- treat conversion-first or native inspect-only as the only future-approved categories
- reject hand-rolled BIFF/OLE parsing as the default path

Queue effect:
- legacy `.xls` feasibility is closed for the first reopen step
- large-workbook XML budget policy becomes the next active design-only task
- no implementation-worker dispatch is opened
