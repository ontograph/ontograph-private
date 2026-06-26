# Excel MCP N5-A Blocked

Date: 2026-06-26

Scope:
- Bounded manager loop on remaining tasks from `EXCEL_MCP_2000_USEFUL_SOLUTIONS_REVIEW.md`.
- Candidate task: `N5-A` fixture-first workbook graph proof.

Senior-reviewer verdict:
- `N5-A` is blocked.
- A fixture-only expected-edge test would introduce a workbook graph concept without an approved graph schema, Rust type, parser, or output contract.
- That would repeat the donor anti-pattern already called out as graph theater: graph-shaped artifacts without real edge extraction.

Implementation-worker result:
- Stopped after senior block.
- No files changed by the worker.
- No tests run by the worker.

Manager decision:
- No Rust implementation dispatch.
- Removed the local experimental fixture test before closure.
- `N5` remains design-only until a graph schema/output-contract ADR is approved.

Allowed next actions:
- Write a text design note or ADR for workbook graph node types, edge types, source OpenXML parts, warning/blocker rules, and output contract.
- Keep `N5` closed until explicit user demand accepts the new graph architecture surface.

Rejected:
- No fixture-first test.
- No expected-edge JSON in `tests.rs`.
- No graph type, parser skeleton, public tool, private extractor, formula evaluation, SQL generation, formula rewrite, workbook mutation, or live Excel work.
