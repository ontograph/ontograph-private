---
name: jscpd Donor Core Extension Tracking
description: Dispatch and verification ledger for the accepted jscpd donor core-extension rows
type: tracking
date: 2026-06-21
status: complete
---

# jscpd Donor Core Extension Tracking

Authority:
- `ADR_JSCPD_DONOR_CORE_EXTENSION_REVIEW.md`
- `ADR_JSCPD_DONOR_CORE_EXTENSION_SOLUTIONS.md`

## Manager Rules

- Update this file before starting each slice.
- Keep every change inside the existing owner named by the ADR.
- Do not add a duplicate detector, scanner daemon, report registry, MCP service,
  REST API, or SQLite tracking store.
- Refresh/check OntoIndex after each accepted slice.

## Dispatch Queue

| Slice | Rows | Status | Owner / Write Scope | Assigned Model | Verification |
| --- | --- | --- | --- | --- | --- |
| `JSCPD-R1` | `JSC-31`, `JSC-39` | closed | `ontocode-rs/core/tests/suite/apply_patch_cli.rs` | `claude-sonnet-4-6` | 7 named apply-patch tests pass locally |
| `JSCPD-R2` | `JSC-32` | closed | `ontocode-rs/hooks/src/output_spill_tests.rs` (+ minimal fix in `output_spill.rs`) | `claude-sonnet-4-6` | 123/123 ontocode-hooks tests pass locally |
| `JSCPD-R3` | `JSC-33` | closed | `ontocode-rs/protocol/src/models_prompt_tests.rs` (new sibling) + `ontocode-rs/protocol/src/models.rs` (module declaration) | `claude-sonnet-4-6` | 235/235 tests pass (`just test -p ontocode-protocol`); `just fmt` clean |
| `JSCPD-R4` | `JSC-40` | covered-no-dispatch | existing generated-output snapshot/golden owner if a concrete gap exists | manager-local | OntoIndex found existing guardian/model-visible layout snapshots; no concrete missing output owner gap |

## Event Log

- 2026-06-21: Senior unblock accepted four narrow slices covering the five active
  ADR rows. Tracking opened before dispatch. Available sub-agent model list does
  not include `gemini-3.1-flash-lite`, `gemini-3.5-flash-low`,
  `gemini-pro-agent high`, `gemini-3-flash-agent`, or
  `gpt-5.3-codex-spark`; use the first exact requested model currently
  available, `claude-sonnet-4-6`, then `gpt-5.4-mini` as needed.
- 2026-06-21: Marked `JSCPD-R1`, `JSCPD-R2`, and `JSCPD-R3` in progress before
  dispatch.
- 2026-06-21: JSCPD-R2 done. Added three new tests in `output_spill_tests.rs`:
  `repeated_large_hook_text_stays_bounded`, `spilled_output_has_exactly_one_recovery_path`,
  and `duplicate_hook_text_spill_files_contain_full_text`. A minimal fix was
  required in `output_spill.rs` (`spilled_hook_output_preview`) to account for
  the `formatted_truncate_text` header and truncation-marker overhead that
  caused the token budget to overrun by ~6–13 tokens. All 123 ontocode-hooks
  tests pass.
- 2026-06-21: Closed `JSCPD-R4` as `covered-no-dispatch`. OntoIndex points to
  existing generated-output/model-visible snapshots in `core/src/guardian` and
  `core/tests/suite/model_visible_layout`; no new generic golden-report task was
  accepted.
- 2026-06-21: Closed `JSCPD-R3`. Added `models_prompt_tests.rs` as a sibling
  test file with two heading/section-block duplication guards for
  `BASE_INSTRUCTIONS_DEFAULT`. Wired via `#[cfg(test)] #[path = "models_prompt_tests.rs"]`
  module declaration appended to `models.rs`. 235/235 protocol tests pass.
  No production symbols touched; write scope stayed within allowed boundaries.
- 2026-06-21: JSCPD-R1 closed. Added 7 focused tests to `apply_patch_cli.rs`: 2 for JSC-31 (single-failure sentinel dedup, two-consecutive-failures independent diagnostics) and 5 for JSC-39 (duplicate file header, empty hunk body, dot-segment path, very long filename, leading commentary). All 7 pass via `just test -p ontocode-core`. No production code touched.
- 2026-06-21: Manager verification complete. Local tests passed:
  `CARGO_BUILD_JOBS=8 just test -p ontocode-core apply_patch_failure_diagnostic_appears_exactly_once apply_patch_two_consecutive_failures_produce_independent_diagnostics apply_patch_duplicate_file_header_in_single_patch apply_patch_hunk_with_no_diff_lines_is_rejected apply_patch_path_with_embedded_dot_segments_is_handled apply_patch_very_long_filename_does_not_panic apply_patch_with_leading_commentary_before_begin_patch`,
  `CARGO_BUILD_JOBS=8 just test -p ontocode-hooks`, and
  `CARGO_BUILD_JOBS=8 just test -p ontocode-protocol`. `CARGO_BUILD_JOBS=8
  just fmt` completed. OntoIndex remains fresh at HEAD but medium confidence
  because the worktree has unrelated uncommitted changes.
