# Oh My Pi Row 83 Coverage Closeout

Date: 2026-06-19

Row 83: same-model retry loop guard tests.

Result: covered.

Evidence:
- Added `pre_sampling_compact_does_not_run_for_same_model` in `ontocode-rs/core/tests/suite/compact.rs`.
- Existing `pre_sampling_compact_runs_on_switch_to_smaller_context_model` still covers the switch-to-smaller-model path.
- Focused verification passed with `CARGO_BUILD_JOBS=8 just test -p ontocode-core pre_sampling_compact_does_not_run_for_same_model pre_sampling_compact_runs_on_switch_to_smaller_context_model`.

Notes:
- The change stays owner-local in the compact/session test surface.
- Row 82 remains blocked by the remote pre-turn compaction blocker and was not modified here.
