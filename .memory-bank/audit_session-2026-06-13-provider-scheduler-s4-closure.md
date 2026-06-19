# Provider Scheduler S4 Closure

Date: 2026-06-13
Status: accepted

## Scope

- `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md` `S4-A` and `S4-B`
- Private scheduler core in `ontocode-rs/model-provider`

## Landed

- Private `model-provider::schedule` core
- Deterministic policies:
  - `RoundRobin`
  - `Priority`
  - `Failover`
  - `StickySession`
- Sticky-session reset boundary:
  - explicit `sticky_session_reset`
- Sticky failover:
  - previous sticky selection is reused only while still eligible
  - scheduler falls back to the next eligible candidate when the sticky choice becomes suppressed or otherwise ineligible

## Architecture decision

- Scheduler remains private to `model-provider`.
- No public config, app-server API, or SDK surface was added.
- Scheduler consumes normalized credential summaries and refresh diagnostics; it does not own provider runtime logic.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-model-provider`
- `git diff --check`
- `ontoindex analyze`

## Next slice

- `S5-A` internal provider-auth adapter contract extraction
