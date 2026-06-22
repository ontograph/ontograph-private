# R1J CLI Utility Rename Closure

Date: 2026-06-10

Scope:
- Renamed `codex-utils-cli` to `ontocode-utils-cli`.
- Updated Cargo package identity, Rust lib crate identity, Bazel crate name, workspace dependency names, direct dependent manifests, direct Rust imports, and lockfile identity.
- Preserved CLI option parsing, config override parsing, sandbox/approval CLI argument behavior, resume command/hint behavior, environment display formatting, public command names, runtime package layout, protocol/schema names, telemetry names, `.codex`, persisted state, and `CODEX_*` compatibility.

Risk:
- OntoIndex impact for `CliConfigOverrides`: HIGH.
- OntoIndex impact for `resume_hint`: HIGH in senior review; worker rerun reported LOW in the current graph, but the HIGH gate remains accepted for this closure.
- OntoIndex impact for `SharedCliOptions`: LOW.

Verification:
- `CARGO_BUILD_JOBS=8 cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-cli` — 19 passed
- `CARGO_BUILD_JOBS=8 just test -p codex-cli` — 261 passed
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui` — 2772 passed, 4 skipped
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server` — 810 passed, 1 skipped
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec` — 122 passed
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server` — 14 passed
- `CARGO_BUILD_JOBS=8 just test -p codex-cloud-tasks` — 13 passed, 1 skipped
- `CARGO_BUILD_JOBS=8 just test -p codex-chatgpt` — 8 passed
- `CARGO_BUILD_JOBS=8 PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH just bazel-lock-check`
- Stale-reference search found zero `codex-utils-cli` or `codex_utils_cli` matches under `ontocode-rs`.
- `Cargo.lock` contains `ontocode-utils-cli`; old package name is absent.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff` passed.

Next:
- Remaining R1 candidates are blocked until a fresh exact-slice inventory and senior risk review selects one candidate.
