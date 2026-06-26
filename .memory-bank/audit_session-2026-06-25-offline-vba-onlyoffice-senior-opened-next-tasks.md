# Offline VBA To ONLYOFFICE Senior Opened Next Tasks

Date: 2026-06-25

## Scope

Open the next bounded senior queue after the `SFT-*` closure.

## Decision

Open only the three tasks that close the remaining evidence gap:

- get real workbook-derived VBA text into workspace-local scope so the Excel extraction tools can operate on it directly
- rerun the placeholder shallow-syntax check against those real extracts instead of only the cached sample files
- keep one tiny trigger scoreboard so later manager loops can answer "still closed or now real?" without re-reading every note

## Guardrails

- no implementation slice is reopened by this note
- no parser dependency work is opened
- no runtime ONLYOFFICE validation is opened
- no public `excel.translate` work is opened
- no OntoIndex refresh is scheduled from this note
