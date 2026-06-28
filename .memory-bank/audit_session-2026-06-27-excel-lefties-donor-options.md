# Excel Lefties Donor Options

## Decision

No implementation dispatch was reopened from the `tmp/excel` donor review.

The donor pass narrowed the useful families to:

- `tmp/excel/in2sql_dotNet_addin` for offline parser, graph, SQL, and possible later `.xlsb` direction
- `tmp/excel/mcp-server-excel` for future live named-range and `Formula2` semantics only

Other Excel MCP donors remain secondary read-only metadata or CRUD references and do not justify reopening the lefties loop by themselves.

## Canonical Options

### Option A

Stay offline-only in `ontocode-rs/ext/excel`.

Use donor ideas only for grammar, bounded planning, graph evidence, and later feasibility checks.

### Option B

If live Excel work is ever reopened, keep it in a separate companion owner and reuse only the live semantics from `mcp-server-excel`.

### Option C

Prefer a planner/executor split if live work opens later:

- this repo owns dry-run and proof
- a future companion owns apply and `Formula2`

## Exact Reopen Gates

- Phase 4C: reopen only with a fresh approved fixture pack proving target-column resolution and same-grain criteria/range alignment for optional aggregate planning
- Phase 5 / Phase 6: reopen only with a separate accepted live-owner contract

## Why

This matches the current code owners:

- formula inspection feeds named-range dry-run
- workbook graph stays an offline evidence surface
- Power Query extraction already sits in its own read-only owner

The donor review therefore refined the future implementation shape, but did not create a new active task.
