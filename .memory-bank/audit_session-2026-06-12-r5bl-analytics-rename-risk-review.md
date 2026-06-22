# R5BL Analytics Rename Risk Review

Date: 2026-06-12

## Slice

- Rename `codex-analytics` -> `ontocode-analytics`.
- Rename Rust crate refs `codex_analytics` -> `ontocode_analytics`.
- Identity-only scope: package metadata, library crate name, Bazel target/deps, imports, and lockfiles.

## OntoIndex

- MCP `mcp__ontoindex` is still not wired to `/opt/demodb/_workfolder/ontocode`; use local OntoIndex CLI for this repo.
- `accepted_line_fingerprints_from_unified_diff`: HIGH, 9 impacted nodes, 4 direct, 4 modules.
- `accepted_line_repo_hash_for_cwd`: HIGH, 6 impacted nodes, 1 direct, 4 modules.
- `fingerprint_hash`: HIGH, 11 impacted nodes, 2 direct, 4 modules.
- `AnalyticsEventsClient`, `AnalyticsReducer`, and `TrackEventRequest`: ambiguous/UNKNOWN.
- `AnalyticsFact`: LOW.

## Guardrails

- Do not change analytics queueing/sending behavior, reducer state transitions, event serialization shapes, JSON field names, timestamps, accepted-line fingerprint normalization/hash behavior, repo hash behavior, app/plugin/hook/guardian/review/turn/subagent telemetry mapping, app-server client metadata mapping, dedupe behavior, config/runtime metadata semantics, env/config/wire/generated names, telemetry/product strings, persisted state, or the existing `analytics` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-analytics --no-tests=pass`
- Focused analytics client/reducer/accepted-line tests.
- Focused core telemetry checks or compile.
- App-server/app-server-client metadata compile or focused checks.
- Feedback/model-provider telemetry compile checks as applicable.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_analytics|codex-analytics`.
- Cargo metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
