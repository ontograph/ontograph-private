# R5BU Protocol Rename Closure

- Scope: manager-owned final residual crate rename `codex-protocol` -> `ontocode-protocol` and `codex_protocol` -> `ontocode_protocol`.
- Outcome: accepted.
- Evidence:
  - `cargo metadata --no-deps` reports `0` remaining `codex-*` package identities
  - `git diff --check` passed
  - active source/package refs for `codex-protocol` are removed
  - remaining `codex_protocol` hits are generated app-server schema description strings only
- Verification:
  - `CARGO_BUILD_JOBS=8 just test -p ontocode-protocol`
  - `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
  - `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-protocol --tests`
  - `CARGO_BUILD_JOBS=8 just fmt`
- Known warnings:
  - duplicate compatibility bin-target warnings for retained old/new command aliases
  - pre-existing `TotalTokenUsageBreakdown` dead-code warning in `ontocode-core`
