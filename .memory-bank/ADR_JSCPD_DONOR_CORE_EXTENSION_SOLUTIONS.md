---
name: jscpd Donor Core Extension Solutions
description: Senior unblock plan for the five accepted jscpd donor core-extension rows
type: adr
date: 2026-06-21
status: accepted
---

# ADR: jscpd Donor Core Extension Solutions

Authority:
- `ADR_JSCPD_DONOR_CORE_EXTENSION_REVIEW.md`
- `JSCPD_DONOR_CORE_EXTENSION_TRACKING.md`

## Decision

Unblock the accepted jscpd donor rows as owner-local regression coverage. The
donor value is duplicate/noise prevention in generated text paths, not a native
duplicate detector.

## Accepted Slices

| Slice | Rows | Solution | Owner | Stop Condition |
| --- | --- | --- | --- | --- |
| `JSCPD-R1` | `JSC-31`, `JSC-39` | Add apply-patch regression/adversarial fixtures for duplicate or malformed generated patch diagnostics. Fix only owner-local formatter behavior if a fixture fails. | `ontocode-rs/core/tests/suite/apply_patch_cli.rs` | Any generic duplicate detector, scanner, or production parser rewrite. |
| `JSCPD-R2` | `JSC-32` | Add hook output spill tests proving repeated hook text stays bounded, preserves one recovery-path attribution, and keeps full text in the spill file. | `ontocode-rs/hooks/src/output_spill_tests.rs` | New hook reporter, hook discovery flow, or model-visible unbounded output. |
| `JSCPD-R3` | `JSC-33` | Add a static prompt guard scoped to `protocol` base instructions, checking only high-signal duplicated headings or section blocks. | `ontocode-rs/protocol/src/models.rs` or a sibling protocol test file | Repo-wide markdown clone scanning. |
| `JSCPD-R4` | `JSC-40` | Close through the smallest existing model-visible/generated-output golden or snapshot owner if a concrete gap is found; otherwise mark covered by existing snapshots. | Existing output owner tests only | New generic report registry or reporter pipeline. |

## Rejected Paths

- SQLite/in-memory tracking is not needed for this ADR. The task state is small,
  repo-local, and already represented by the memory-bank tracking file.
- No MCP service, REST API, CI clone threshold, background scanner, or
  donor-style report format is approved.
- No code belongs in large orchestration owners such as `session/turn.rs`,
  `tools/spec_plan.rs`, or hook discovery unless a failing owner-local test
  proves a current defect there.

## Verification

Each dispatched slice must:

- update `JSCPD_DONOR_CORE_EXTENSION_TRACKING.md` before starting and after
  closure;
- use OntoIndex context/impact for any production symbol edit;
- run scoped tests for the touched crate;
- run `just fmt` after Rust edits;
- refresh/check OntoIndex after closure.
