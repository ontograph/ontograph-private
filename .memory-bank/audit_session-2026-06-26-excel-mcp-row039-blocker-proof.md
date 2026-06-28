# Excel MCP Row 039 Blocker Proof

Date: 2026-06-26

Scope:
- Close the design-only proof step for row `039` array/dynamic-array conversion semantics.

Donor evidence used:
- `Tests/SqlEngine.Tests/Program.cs` proves array constants, dynamic-array formulas such as `FILTER(...)`, and external workbook references are parsed as explicit unsupported nodes rather than converted.
- The same donor tests prove malformed formulas return blocker-able parse results without throwing.
- ADR/doc text for the donor planner keeps volatile functions such as `INDIRECT`, `OFFSET`, `RAND`, `NOW`, `TODAY`, `CELL`, and `INFO` in the deferred/blocker set.
- Donor planner tests also pin spill-capable `XLOOKUP(...)` as a manual blocker rather than a generated SQL plan.

Decision:
- Treat array constants, spill-capable dynamic-array formulas, external workbook references, volatile functions, and malformed formulas as fail-closed blocker categories for any future worksheet-formula planner work.
- Do not reopen row `039` as automatic conversion semantics.

Queue effect:
- row `040` becomes the next active formula-track design task
- no implementation-worker dispatch is opened
