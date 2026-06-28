# Excel Large-Workbook XML Budget Policy Note

## Status

Policy-only. No budget change, warning downgrade, or tool behavior change is implemented by this note.

## Date

2026-06-27

## Context

Current `ext/excel` already enforces explicit bounded-read limits.

In `ontocode-rs/ext/excel/src/backend.rs`:

- `MAX_PACKAGE_PART_COUNT = 4096`
- `MAX_XML_ENTRY_BYTES = 1 MiB`
- `MAX_XML_SCAN_ENTRIES = 128`
- `MAX_XML_SCAN_BYTES = 8 MiB`

The current tests prove these are hard-stop failures today:

- too many package parts
- oversized required XML entry
- too many XML entries scanned
- too many total XML bytes scanned

The current preview and formula-inspection paths also carry their own bounded limits:

- worksheet XML cap for preview
- shared-strings XML cap for preview
- workbook/worksheet/styles XML caps for formula inspection
- bounded row/column and formula-count truncation warnings

This means the codebase already distinguishes two policy classes:

- hard-stop package/read safety failures
- bounded output truncation warnings after a successful bounded read

The unresolved question is narrow: if a future large workbook exceeds optional inspection budgets, which paths may ever degrade to warnings instead of fatal errors?

## Policy Verdict

Keep hard-stop behavior for required package structure and required XML reads.

Only optional carrier scans may ever be considered for warning-only downgrade, and only after a real failing workbook artifact is reviewed.

## Required Hard-Stop Categories

These must stay fatal:

- archive cannot be opened as the expected workbook container
- package part count exceeds the top-level safety cap
- required workbook metadata entries exceed byte budgets
- required workbook metadata reads exceed shared XML scan budgets before the minimal workbook shape is decoded
- selected worksheet XML exceeds preview or formula-inspection byte budgets
- corrupted or undecodable required XML entries

In practice, this means at least these current paths remain hard-stop:

- `[Content_Types].xml` / workbook classification failure when needed
- `xl/workbook.xml` for `.xlsx` / `.xlsm`
- selected worksheet XML for preview/export/formula inspection
- any archive-level corruption or declared-size mismatch

These failures mean the tool cannot honestly claim a bounded answer.

## Optional Warning-Candidate Categories

These may become warning-only in a future reopen, but are not approved yet:

- formula-marker scans across many worksheet XML parts during `inspect_workbook`
- Power Query carrier scans across many `customXml/item*.xml` entries
- optional connection-marker reads
- optional marker summaries whose samples can be truncated without hiding required workbook identity

If any of these are downgraded in the future, the result must carry explicit incompleteness warnings such as:

- marker scan skipped after budget cap
- formula presence may be incomplete
- Power Query detection may be incomplete

Silent downgrade is not allowed.

## Not Acceptable As Warning-Only

The following must not degrade to warnings:

- missing or unreadable required workbook topology
- corrupt selected worksheet reads
- ambiguous workbook format classification
- broken path between selected sheet metadata and selected sheet content
- any condition that would make preview/export/formula output appear complete when it is not

## Reopen Requirements Before Code

Any budget-policy implementation stays blocked until the reopen pack contains:

- one real large-workbook artifact that currently fails
- classification of the exact failing path
- proof that the failing path is optional rather than required
- proposed warning text and incompleteness semantics
- explicit statement that required workbook parts remain fatal

Synthetic tests are enough to justify the current policy note, but not enough to justify a behavior change.

## Recommended Future Shape

If reopened, use this order:

1. classify the failing artifact as required-read or optional-scan
2. if required-read, keep fatal behavior and close with no code
3. if optional-scan, add warning-only downgrade in `inspect_workbook` first
4. do not touch preview/export/formula-inspection fatal paths in the same change
5. require focused tests proving both preserved fatal paths and new warning semantics

## Senior Challenge Outcome

The wrong move would be to “make large workbooks work” by turning structural read failures into soft warnings.

The only defensible downgrade candidate is optional carrier detection after the minimal workbook shape is already known. Everything else stays fail-closed until a real workbook proves otherwise.
