# R3E-U1 App Server Skills Fixture Unblock

Date: 2026-06-10

## Problem

- Broad `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server` fails one test: `suite::v2::turn_start::turn_start_emits_thread_scoped_warning_notification_for_trimmed_skills`.
- Actual warning reports 14 additional omitted skills.
- Existing fixture expects 7 additional omitted skills.

## OntoIndex Evidence

- `turn_start_emits_thread_scoped_warning_notification_for_trimmed_skills` impact: LOW, 0 upstream impacted nodes.
- OntoIndex repo path: `/opt/demodb/_workfolder/ontocode`.

## Scope

- Diagnose and fix the fixture or test setup so the app-server suite is deterministic in the current repository skill inventory.
- Prefer asserting invariant warning behavior over hard-coding an environment-sensitive skill count.
- Do not change production skills budget semantics, warning delivery, thread scoping, config warning behavior, app-server protocol, or model-visible context behavior unless the worker proves production behavior is wrong.

## Result

- Updated `ontocode-rs/app-server/tests/suite/v2/turn_start.rs` to parse the omitted-skill count from the warning and assert it is positive instead of expecting a fixed inventory-sensitive count.
- Preserved exact warning invariants for the 2% skills context budget prefix, description removal text, omitted-skills suffix, thread scoping, and trimmed model-visible skill body assertions.
- No production code or app-server wire/protocol behavior changed.

## Required Verification

- Focused app-server test for the failing fixture.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `git diff --check`.
- Scoped OntoIndex `gn_verify_diff`.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server turn_start_emits_thread_scoped_warning_notification_for_trimmed_skills` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server` passed: 810 tests passed, 1 skipped.
- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `git diff --check` passed.
- Scoped OntoIndex `gn_verify_diff` passed for `ontocode-rs/app-server/tests/suite/v2/turn_start.rs` and `turn_start_emits_thread_scoped_warning_notification_for_trimmed_skills`.
