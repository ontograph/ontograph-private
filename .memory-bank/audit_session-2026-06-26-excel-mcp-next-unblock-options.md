# Excel MCP Next Unblock Options

Date: 2026-06-26

Scope:
- Remaining Excel MCP queue after `N2-N4` closure and `N5` proof-contract creation.

Issue fixed:
- `EXCEL-MCP-N5` had a proof contract but no concrete junior-safe unblock options.
- Superseded: the original fixture-first path was later blocked by senior review.
- Current guidance is design/ADR only before any graph code or fixture.

OntoIndex evidence:
- Current Excel tests already have `write_zip_fixture` and `write_zip_fixture_bytes`.
- Formula inventory and Power Query extraction tests already build tiny OpenXML-style fixtures.
- Reusing those helpers is enough; no new fixture framework or graph service is justified.

Superseded clarification:
- A later senior-review pass blocked `N5-A`.
- Fixture-first graph proof is not valid before an approved graph schema, Rust type, parser, and output contract exist.
- The current allowed next action is design-only: write a text graph-schema ADR or leave `N5` closed.

Former recommendation, now blocked:
- `N5-A`: create one tiny synthetic workbook fixture plus expected edge output.
- This is now rejected because it would create graph-shaped test data without real approved edge extraction.

Other valid unblock options:
- `N5-ADR`: text graph schema/output-contract ADR only.
- `N5-HOLD`: keep workbook graph extraction closed until explicit user demand accepts the new architecture surface.
- `N1-A`: isolate or explicitly exclude unrelated dirty worktree changes before global verification.
- `N6-A`: draft live Excel companion ADR only if live mutation/chart/VBA demand remains explicit.

Rejected shortcuts:
- Do not mine real samples as the first graph proof.
- Do not ship empty-edge graph output.
- Do not implement formula-to-SQL, formula rewrite, workbook mutation, or recalculation as part of this queue.
