# R1C-R1E Internal Crate Rename Dispatch

Date: 2026-06-09

## Scope

- Dispatched and accepted three exact internal crate rename follow-on slices.
- Kept package manager identities, protocol/wire names, `CODEX_*`, `.codex`, SDK package/import names, and package-layout filenames out of scope.

## Accepted Slices

- R1C: `codex-utils-approval-presets` -> `ontocode-utils-approval-presets`.
- R1C-U1: fixed deterministic TUI test fixture leakage from parent `/tmp/.git`; no snapshots were accepted.
- R1D: `codex-async-utils` -> `ontocode-async-utils`.
- R1E: `codex-utils-template` -> `ontocode-utils-template`.

## Verification Summary

- R1C: package no-test compile check, focused approval/permissions tests, full `ontocode-tui` after R1C-U1, Bazel lock update/check, scoped OntoIndex verification.
- R1D: `ontocode-async-utils`, `codex-protocol`, `codex-mcp`, full `codex-core`, Bazel lock update/check, scoped OntoIndex verification.
- R1E: `ontocode-utils-template`, `codex-goal-extension`, `codex-memories-extension`, `codex-login`, `codex-memories-write`, `codex-models-manager`, `codex-prompts`, Bazel lock update/check, scoped OntoIndex verification.

## Next Gate

- Automatic dispatch paused after R1E.
- Remaining obvious utility candidates need explicit risk selection:
  - `codex-utils-image`: `load_for_prompt_bytes` HIGH.
  - `codex-utils-rustls-provider`: `ensure_rustls_crypto_provider` CRITICAL.
  - `codex-utils-string`: `sanitize_metric_tag_value` CRITICAL.
