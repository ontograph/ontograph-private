# Excel Lefties Plan

## Date

2026-06-27

## Summary

Created `EXCEL_LEFTIES_IMPLEMENTATION_PROJECT_PLAN.md` to turn the remaining Excel lefties into one bounded implementation order.

The plan keeps a strict owner split:

- offline parsing, graph, and SQL stay in `ontocode-rs/ext/excel`
- live `Formula2`, array-formula, and named-range apply work stay in a separate live companion owner

## Planned order

1. contracts and fixtures
2. worksheet formula AST
3. formula metadata sidecar and pattern normalization
4. graph completion
5. bounded formula-to-SQL
6. live named-range apply
7. live `Formula2` and array-formula writes

## Donor conclusions captured

- `tmp/excel/in2sql_dotNet_addin` is the main donor for offline AST, SQL, `.xlsb`, and lineage ideas
- `tmp/excel/mcp-server-excel` is the main donor for live mutation semantics
- broad donor stack porting remains rejected
