# Excel MCP Large-Workbook XML Budget Policy Closure

Date: 2026-06-27

Scope:
- Close the design-only large-workbook XML budget policy step.

OntoIndex and code evidence used:
- `XmlReadBudget.new` and `inspect_workbook_with_display_path` define explicit package-entry, XML-entry, and total-XML budgets in `ontocode-rs/ext/excel/src/backend.rs`.
- current preview and formula-inspection code already use separate per-entry byte caps and bounded truncation semantics.
- current tests prove package over-size and XML over-budget failures are fatal today, while row/column and formula-count caps produce warnings only after a successful bounded read.

Decision:
- accept a policy-only note
- keep required workbook reads and structural decode failures fatal
- allow only optional carrier scans to be considered as future warning-only downgrade candidates
- keep implementation blocked until a real failing workbook artifact proves an optional-scan case

Queue effect:
- no active design-only task remains in the current reopen order
- rows `041-044` remain dependency-gated or demand-gated
- no implementation-worker dispatch is opened
