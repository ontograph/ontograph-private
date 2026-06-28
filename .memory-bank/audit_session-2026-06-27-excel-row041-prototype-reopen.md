# Excel Row 041 Prototype Reopen

## Date

2026-06-27

## Scope

Manager decision after the row `041` synthetic positive fixture review.

## Decision

Accepted option `2`: the synthetic positive workbook is sufficient to reopen a prototype-only `041A` path.

This is not a production reopen.

## Prototype Boundary

Prototype scope is limited to:

- offline `ontocode-rs/ext/excel`
- read-only dry-run only
- exact textual match replacement only
- existing defined names only
- workbook-scoped names only in the first slice unless sheet scope is proven unambiguous
- no apply path
- no mutation

## What Opened

One valid next implementation task is now open:

- prototype dry-run implementation for the synthetic fixture and blocker workbooks

## What Remains Gated

Production-level reopen still wants:

- one real user workbook
- one real user reason for the rewrite
- one concrete mapping file from that workbook

## Recommended Next Task

Implement the smallest `041A` prototype:

- input: workbook path, sheet selector, mapping file
- matching: exact textual match only
- target names: existing names only
- output: structured dry-run diff per formula
- fail closed on external links, `#REF!`, scope ambiguity, R1C1, and unsupported cases
