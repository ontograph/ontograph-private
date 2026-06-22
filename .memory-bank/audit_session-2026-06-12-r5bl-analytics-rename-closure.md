# R5BL Analytics Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-analytics` -> `ontocode-analytics` and `codex_analytics` -> `ontocode_analytics`.
- Scope stayed identity-only: package metadata, library crate name, Bazel target/deps, imports, tests, and lockfiles.
- Preserved analytics queueing/sending behavior, reducer state transitions, event serialization shapes, JSON field names, timestamps, accepted-line fingerprint normalization/hash behavior, repo hash behavior, app/plugin/hook/guardian/review/turn/subagent telemetry mapping, app-server client metadata mapping, dedupe behavior, config/runtime metadata semantics, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `analytics` directory path.

## Verification

- Worker verification passed for `ontocode-analytics`, focused analytics client/reducer/accepted-line tests, core/core-api/core-plugins/core-skills, app-server/app-server-client/model-provider/feedback, fmt, Bazel lock update/check, stale-reference search, diff check, and OntoIndex `detect-changes --repo codex`.
- Manager stale-reference search found no `codex_analytics` or `codex-analytics` refs in `ontocode-rs`.
- Manager metadata check reports 9 remaining `codex-*` packages, correcting the worker's JSON-grep count of 19.
- Manager `git diff --check` is clean.

## Notes

- Archimedes `019ebd59-fbdb-71a0-a21f-48e0f9649785` completed the scoped patch and verification on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
