# Ontocode-Only CLI Hard Cutover Tracking

Source ADR: `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`

## Status Key

- `pending`
- `in_progress`
- `blocked`
- `done`

## Manager Status

- Overall status: `hard-cutover-implementation-verified`
- Blocker: none for tracked HC0-HC11 scope.
- Dispatch mode: complete for hard-removal implementation scope.
- Selected removal mode: `hard`
- Product/release owner approval: `accepted by user instruction on 2026-06-09`
- Rollback owner and threshold: `manager-owned default: restore codex launcher/bin if any hard-removal package or startup smoke fails`
- Package identity scope: `freeze codex-named package identities; remove installed codex command/helper aliases only`
- OntoIndex verification: `available with caveats`
- Manager OntoIndex evidence:
  - `command_name_from_arg0`, `current_command_name`, and `arg0_dispatch` resolved under repo `codex` with repoPath `/opt/demodb/_workfolder/ontocode`.
  - Individual impact risk for those three symbols: `LOW`; batch union risk: `MEDIUM`.
  - Worker depth/disambiguated checks found HIGH/CRITICAL risk for broader CLI/helper/runtime path flows; use the higher risk until reconciled.
  - Stage 0 targets such as `transform_linux_seccomp_request` may resolve to test helpers and require disambiguation before implementation.
- Hard-mode safeguards:
  - No migration-error shim: `codex` command/helper entrypoints must be absent from installed/package binary surfaces.
  - Do not rename package identities, release asset names, import paths, protocol metadata, telemetry, persisted state, or internal Rust crates.
  - Rollback must be a small launcher/bin restoration patch, not a package/protocol rename.
  - Implement in sequence: HC5 -> HC6 -> HC7 -> HC8 -> HC9 -> HC10 -> HC11.

## Current Dispatch Queue

| ID | Task | Scope | Status | Owner | Notes |
| --- | --- | --- | --- | --- | --- |
| HC0 | Gate audit and approval checklist | ADR gates and current rename tracker | done | manager | Gate audit closed: implementation remains blocked by missing approval, removal mode, rollback owner/threshold, and package/runtime decisions. |
| HC1 | Public command removal matrix | CLI, helper, package, SDK, docs surfaces | done | Aristotle `019eab0a-a78a-7643-8735-4b80be18a6ae` | Report created: `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER_REMOVAL_MATRIX.md`; recommends `soft-hard`. |
| HC2 | OntoIndex blast-radius report | CLI and helper symbols from ADR Stage 0 | done | Boyle `019eab0a-ddf5-71e0-a2b8-c17a6af7f8f7` | Report created: `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER_IMPACT_REPORT.md`; HIGH/CRITICAL risks block implementation. |
| HC3 | Package/runtime validation plan | npm, Python runtime, native package layout | done | Sartre `019eab0b-138b-7af1-8f48-90302453e7c5` | Report created: `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER_PACKAGE_VALIDATION.md`; package identities must stay codex-named unless T15 is approved. |
| HC4 | Migration and rollback release plan | release notes, migration shim, rollback threshold | done | Zeno `019eab0b-3e5c-7bd2-98c6-3c328ea174bc` | Report created: `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER_MIGRATION_ROLLBACK.md`; rollback owner/threshold still missing. |
| HC5 | Stage 1 CLI implementation | Rust CLI binary identity | done | manager | `ontocode` is the only Cargo/Bazel CLI binary target; wrapper/env override removed; focused alias tests passed. |
| HC6 | Stage 2 helper gate removal | arg0/helper dispatch and sandbox paths | done | manager | Public `codex-*` arg0 helper dispatch/alias creation removed; internal field names preserved; `codex-arg0` tests passed. |
| HC7 | Stage 3 helper manifest removal | Cargo manifests for helper executables | done | manager | Public `codex-*` helper executable outputs removed from manifests and test lookup surfaces; `codex-cli` and `ontocode-exec` passed. |
| HC8 | Stage 4 package installed-bin removal | npm/Python/native package bins | done | Sagan `019eabb5-228f-7da2-b4a7-f1e8e362068d` | `codex-cli/package.json` now installs only `ontocode`; package identity remains `@openai/codex`; package staging checks passed. |
| HC9 | Stage 5 CLI text/docs cleanup | user-facing examples, completions, release notes | done | Euler `019eabb5-4a3b-7cb3-b8cb-badbba480725` | README current run/rename guidance now uses `ontocode`; package/release identity references preserved. |
| HC10 | Stage 6 full verification | Rust/package/docs test matrix | done | manager | Package staging, sandbox/escalation crates, full TUI verification, scoped fixes, and OntoIndex scoped diff verification passed. |
| HC11 | Stage 7 rollback validation | rollback patch and release decision | done | manager | No rollback triggered: package bin maps and canonical `ontocode` launcher smoke passed; partial npm staging lacks optional native deps by design, so direct staged `node` launch is not a valid rollback trigger. |
| HC12 | Package/runtime executable stem cleanup | npm launcher, Python runtime path API, source package layout, managed install lookup | done | manager | Runtime payload lookup now expects `ontocode`; package IDs and layout metadata names remain frozen. Focused Rust/package/TS tests passed; Python SDK test remains blocked by local Python 3.10 missing `tomllib`. |

## Dispatch Log

| Step | Tracking Update | Action |
| --- | --- | --- |
| 1 | Created tracking file | Manager stopped implementation dispatch because ADR is not approved and opened HC1-HC4 planning/unblock tasks. |
| 2 | Recorded worker IDs | Dispatched HC1-HC4 to disjoint memory-bank report files; Stage 1-7 implementation remains blocked. |
| 3 | Added manager OntoIndex evidence | Batch impact for `command_name_from_arg0`, `current_command_name`, and `arg0_dispatch` was LOW individually / MEDIUM union. |
| 4 | Closed HC1-HC4 | All planning reports completed; all workers agree implementation may not proceed until ADR gates are satisfied. |
| 5 | Closed HC0 | Manager gate audit keeps HC5-HC11 blocked; no implementation workers dispatched. |
| 6 | Selected hard removal | User selected `hard`; manager accepted this as removal-mode/product approval and unblocked staged implementation with package-identity freeze and rollback guardrails. |
| 7 | Started HC5 | Stage 1 CLI binary identity patch started with OntoIndex HIGH/CRITICAL risk acknowledged. |
| 8 | Closed HC5 and started HC6 | `just test -p codex-cli -E 'binary(ontocode_alias)'` passed; helper dual-target warnings now drive HC6. |
| 9 | Closed HC6 and started HC7 | `just test -p codex-arg0` passed; cargo warnings show remaining `codex-*` helper bin targets. |
| 10 | Patched HC7 implementation | Removed old helper/bin targets and updated Cargo/Bazel/test lookup surfaces to `ontocode-*`; `just test -p codex-cli` passed after accepting the renamed doctor snapshot. |
| 11 | Resumed manager dispatch | Manager is rerunning HC7 verification locally and preparing HC8/HC9 sub-agent slices with disjoint ownership before advancing the queue. |
| 12 | Closed HC7 and started HC8 | `CARGO_BUILD_JOBS=8 just test -p ontocode-exec` passed 122 tests; no bench-smoke process remained. |
| 13 | Dispatched HC10/HC11 prep sidecar | Worker owns rollback/verification checklist updates only; HC10/HC11 remain pending until HC8/HC9 implementation closes. |
| 14 | Closed HC8 and HC9; started HC10 | Package bin removal and docs cleanup workers completed; manager reviewed assigned surfaces and started final verification. |
| 15 | HC10 verifier failure recorded | `CARGO_BUILD_JOBS=8 just test -p codex-linux-sandbox` passed 112/116 and failed 4 landlock tests on missing bwrap lookup; manager is unblocking before advancing. |
| 16 | HC10 Linux sandbox unblocked | Added existing bwrap-prerequisite skip behavior to legacy landlock tests; `CARGO_BUILD_JOBS=8 just test -p codex-linux-sandbox` passed 116/116. |
| 17 | HC10 shell escalation verified | `CARGO_BUILD_JOBS=8 just test -p codex-shell-escalation` passed 20/20. |
| 18 | HC10 Windows sandbox verified | `CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox` passed 10/10 on Linux host coverage. |
| 19 | HC10 TUI triage started | `CARGO_BUILD_JOBS=8 just test -p ontocode-tui` passed 2770/2772; expected thread-goal snapshot and one `ide_context` test require focused triage. |
| 20 | HC10 TUI focused fixes passed | Accepted intentional `ontocode` thread-goal snapshot; hardened IDE IPC test socket dir to `0700`; both focused TUI tests passed. |
| 21 | HC10 TUI full verification passed | `CARGO_BUILD_JOBS=8 just test -p ontocode-tui` passed 2772/2772 with 4 expected skips after fixes. |
| 22 | Closed HC10 and started HC11 | Scoped `gn_verify_diff` passed for HC10 after noting whole-worktree verification is noisy from pre-existing dirty files. |
| 23 | Closed HC11 | Rollback checklist validated: no package/protocol identity rollback needed; restore `codex` launcher/bin only if a future full package/install smoke fails. |
| 24 | Started HC12 | OntoIndex impact for `managed_codex_file_name` and `bundled_codex_path` was LOW; JS/package scripts are not fully indexed and will be covered by package/runtime tests. |
| 25 | Closed HC12 | `ontocode` is now the packaged/runtime executable stem for npm launcher, Python runtime API, TypeScript SDK native lookup, source package layout, native installers, managed app-server install lookup, and current command-path fixtures. |
