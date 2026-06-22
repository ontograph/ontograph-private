# ADR: Ontocode-Only CLI Hard Cutover

## Status

Approved for hard-removal implementation with staged guardrails.

Review update, 2026-06-09:

- Current rename tracker has T13 and T14 done: npm/Python/native packaging now install both `codex` and `ontocode`, and internal helpers now have `ontocode-*` aliases.
- The remaining double gates are therefore deliberate compatibility gates, not accidental unfinished work.
- User selected `hard` removal on 2026-06-09; implementation may proceed in staged slices with package-identity freeze, rollback guardrails, and focused verification.

## Context

The current rename program keeps `ontocode` as a public CLI alias while preserving `codex` compatibility. That creates double gates in the CLI, helper dispatch, and packaging surfaces:

- `ontocode` is a wrapper binary that re-launches `codex` and sets `ONTOCODE_CLI_COMMAND_NAME`.
- Main CLI still treats `codex` as primary and conditionally switches display/help to `ontocode`.
- Helper arg0 dispatch accepts both `codex-*` and `ontocode-*` helper names.
- Internal helper path exports still prefer `codex-*` paths even after creating `ontocode-*` aliases.

This ADR proposes the hard-cutover option: keep only `ontocode` as the CLI frontier and remove public `codex` command compatibility.

## Decision

Implement this as a hard-removal program under the dedicated hard-cutover tracker.

Make `ontocode` the only supported CLI command, remove public `codex` binary compatibility, and collapse helper dispatch to `ontocode-*` names.

Hard mode means no migration-error shim is retained in successful install surfaces. Rollback remains a separate restoration patch if package/startup validation fails.

## Challenge Summary

This ADR is intentionally high risk. The hard cutover conflicts with the current rename disposition, which preserves package identities and defers internal/package renames until a separate breaking-change program exists.

The largest risk is not the Rust CLI parser. It is the package/runtime ecosystem around it:

- `codex-cli/package.json` currently publishes both `codex` and `ontocode` bin entries pointing at `bin/codex.js`.
- `codex-cli/scripts/build_npm_package.py` still models the npm package and platform packages as `codex-*`.
- Python SDK tooling expects `openai-codex-cli-bin`, `codex_cli_bin`, `bundled_codex_path()`, and a bundled binary named `codex`.
- Runtime package archives and package metadata still use `codex-package*` names.
- Existing SDK tests assert protocol/server metadata such as `codex-cli`.

Therefore, "only `ontocode` as CLI frontier" must be split from "rename packages/import paths/protocol metadata." The first can be implemented as a CLI hard cutover; the second remains T15/package identity migration and must not be hidden inside this ADR.

OntoIndex review note, 2026-06-09:

- The active OntoIndex repo label is `codex` and resolves to `/opt/demodb/_workfolder/ontocode`.
- `inspect` resolves `command_name_from_arg0` in `ontocode-rs/cli/src/main.rs` and shows callers `current_command_name` and `try_parse_multitool_cli_from`.
- `search` finds the current alias process around `multitool_command`, `command_name_override_from_env`, and `ontocode_alias` tests.
- `impact` on `command_name_from_arg0` reports LOW risk at depth 1, but that is not enough to approve the hard cutover because package/runtime surfaces are outside the local CLI parser blast radius.
- `gn_pre_commit_audit` is currently unavailable due to a missing OntoIndex build artifact; use `gn_verify_diff`, `gn_review_diff`, direct source evidence, and focused tests until the OntoIndex defect is fixed.
- Docs sidecar and embeddings are missing, so docs/enrichment evidence is advisory only.

Direct source evidence expands the blast radius beyond CLI:

- `Arg0DispatchPaths` fields such as `codex_self_exe`, `codex_linux_sandbox_exe`, and `main_execve_wrapper_exe` are consumed by exec-server, app-server, TUI, sandboxing, tools, and tests.
- `ontocode-rs/sandboxing/src/manager.rs` still special-cases `CODEX_LINUX_SANDBOX_ARG0`.
- `ontocode-rs/linux-sandbox/src/linux_run_main.rs` still passes `CODEX_LINUX_SANDBOX_ARG0` to bubblewrap `--argv0`.
- `sdk/python/_runtime_setup.py` downloads `codex-package-*` assets and validates `bundled_codex_path()`.
- `sdk/python/scripts/update_sdk_artifacts.py` expects the bundled runtime binary to be named `codex`.

This means a safe implementation must quarantine internal field/API renames until after executable path behavior is switched and verified.

## Goals

- Make `ontocode` the only supported CLI frontier.
- Remove public `codex` command compatibility.
- Remove duplicate `codex`/`ontocode` helper gates.
- Make `ontocode-*` the only helper naming convention.
- Keep implementation staged and rollbackable.

## Non-Goals

- Do not rename internal Rust crates only for cosmetics.
- Do not rename wire protocols, telemetry schemas, generated SDK protocol models, persisted state keys, or package import paths unless a separate compatibility plan approves them.
- Do not remove `CODEX_HOME` or legacy state compatibility in the same cutover unless there is a migration and rollback plan.
- Do not rename npm or Python package identities in the first hard-cutover slice.
- Do not rename `openai_codex`, `codex_cli_bin`, generated protocol modules, or app-server metadata in this ADR.
- Do not rename internal runtime path fields in the first slice; field names like `codex_self_exe` can temporarily point at the `ontocode` executable.
- Do not rename release asset names such as `codex-package-*` unless package identity migration is separately approved.

## Go/No-Go Gates

Implementation may start only when all gates below are met:

- Product/release owner explicitly accepts that `codex` command removal is a breaking change.
- Removal strategy is `hard`: remove `codex` immediately from successful command/helper/install surfaces.
- Package identity scope is frozen:
  - CLI binary name may change.
  - npm/Python package names and import paths remain unchanged unless T15 is separately approved.
- OntoIndex impact is available for target symbols, and any OntoIndex limitation is documented with direct source evidence.
- Release tooling can build and validate packages with no public `codex` bin entry.
- Rollback owner and threshold are named before code edits: manager-owned default, restore `codex` launcher/bin if any hard-removal package or startup smoke fails.
- Direct-source blast radius includes exec-server, app-server, TUI, sandboxing, npm packaging, and Python runtime setup.
- A compatibility decision exists for `bundled_codex_path()` and `codex-package-*` release assets.
- The current dual-bin package state from T13 is explicitly replaced by a package-level removal or migration-error strategy.
- The current helper-alias state from T14 is explicitly replaced by an `ontocode-*`-only helper strategy with sandbox and exec-server tests.
- OntoIndex MCP/runtime defects that block required verification are either fixed or recorded with an approved fallback verification plan.

If any gate is missing, stop at documentation/planning.

## Stage 0: Approval And Blast Radius

- Create explicit breaking-change approval for this ADR.
- Reopen the deferred rename gates only for this program.
- Decide whether this is `soft-hard` or `hard` removal:
  - `soft-hard` is recommended.
  - `hard` is allowed only when external automation breakage is explicitly accepted.
- Run OntoIndex impact on:
  - `command_name_from_arg0`
  - `current_command_name`
  - `arg0_dispatch`
  - `prepare_path_entry_for_codex_aliases`
  - `transform_linux_seccomp_request`
  - `run_main_with_arg0_guard`
  - `ExecServerRuntimePaths::new`
  - CLI packaging scripts
  - npm/Python binary package install surfaces
- Produce a removal matrix for every public `codex` command path.
- Warn before proceeding if OntoIndex reports HIGH or CRITICAL risk.
- Inventory external-facing package/runtime surfaces:
  - `codex-cli/package.json`
  - `codex-cli/scripts/build_npm_package.py`
  - `sdk/python/_runtime_setup.py`
  - `sdk/python/scripts/update_sdk_artifacts.py`
  - `sdk/python/pyproject.toml`
  - platform package archives and bundled runtime binary names

Challenge update:

- Stage 0 must treat T13/T14 as compatibility baselines to remove, not as incomplete implementation tasks.
- A hard-removal worker must not start from broad `codex` string removal; it must start from a removal matrix with one approved behavior per external surface.
- If package identity migration T15 remains deferred, this ADR can only remove installed command aliases and helper aliases; it must not rename package IDs, release assets, generated protocol bundles, import paths, or state keys.

Required removal matrix columns:

- Surface.
- Current `codex` behavior.
- Proposed `ontocode` behavior.
- Removal mode: `soft-hard`, `hard`, or preserve.
- Required tests.
- Rollback mechanism.
- Whether the string is user-facing command, internal field, package identity, protocol metadata, release asset, or persisted state.

## Stage 1: Canonical CLI Binary

- Make `ontocode` the primary binary path.
- Remove `ontocode-rs/cli/src/bin/ontocode.rs` as a forwarding wrapper.
- Move current `ontocode-rs/cli/src/main.rs` behavior to the `ontocode` binary.
- Remove the `codex` binary or replace it temporarily with a hard migration error:

```text
codex has been replaced by ontocode. Run: ontocode ...
```

- Delete `ONTOCODE_CLI_COMMAND_NAME`.
- Delete env-based command-name override logic.
- Make help and usage always show `ontocode`.
- Update tests so `codex` no longer has separate success semantics:
  - `soft-hard`: `codex --help` exits non-zero or prints a migration-only message.
  - `hard`: `codex` binary is not built or packaged.

Implementation note:

- Prefer introducing a small command identity owner before deleting compatibility branches if it reduces churn.
- Do not use broad find-and-replace for `codex`; most `codex` strings are package, crate, protocol, state, or compatibility names.
- Keep Rust crate/package names and internal path field names unchanged in the first slice unless the field is public user-facing behavior.

## Stage 2: Remove Double Helper Gates

- In `ontocode-rs/arg0/src/lib.rs`, keep only `ontocode-*` helper names.
- Remove `ontocode-execve-wrapper` dispatch.
- Remove `codex-linux-sandbox` dispatch.
- Prefer exported helper paths:
  - `ontocode_linux_sandbox_exe`
  - `ontocode_execve_wrapper_exe`
- Rename internal struct fields only if contained and low-risk.
- If field renames would create broad churn, keep field names temporarily but point values to `ontocode-*` paths.
- Keep hidden one-shot compatibility only if selected by `soft-hard`; remove it in the follow-up hard-removal slice.

Challenge:

- Helper names may be referenced by sandbox/process launch code that does not surface in CLI tests.
- The `Arg0DispatchPaths` struct is widely consumed; renaming its fields is a separate refactor and should not be required for CLI cutover.
- Linux sandbox behavior depends on argv0 rewriting, so replacing `CODEX_LINUX_SANDBOX_ARG0` must be validated through sandbox execution tests, not just manifest checks.
- This stage must include sandbox and exec-server focused tests before merge.

Field-name quarantine rule:

- First implementation slice may change values to point at `ontocode-*`.
- First implementation slice must not rename `codex_self_exe`, `codex_linux_sandbox_exe`, or similar internal fields.
- A later cleanup slice may rename fields only after OntoIndex impact resolves and all call sites are staged.

## Stage 3: Helper Binary Manifests

- Remove `codex-*` helper binaries from Cargo manifests where they are public executable outputs:
  - `ontocode-rs/shell-escalation/Cargo.toml`
  - `ontocode-rs/linux-sandbox/Cargo.toml`
  - `ontocode-rs/windows-sandbox-rs/Cargo.toml`
  - `ontocode-rs/exec/Cargo.toml` if `ontocode-exec` remains public
- Keep crate names as `codex-*` unless separately approved.
- Treat crate rename as T8 and do not bundle it into this CLI cutover.
- Do not remove helper binaries used only as internal implementation details unless runtime callers have already been switched to `ontocode-*` paths.
- On Windows, validate service/setup helper behavior separately because manifest names and command runner behavior are platform-sensitive.

## Stage 4: Package Install Surfaces

- npm CLI package installs only `ontocode`.
- Python runtime package exposes only `ontocode`.
- Native packaging puts only `ontocode` on `PATH`.
- Remove tests that require both `codex` and `ontocode` binaries to be installed.
- Add package-level absence or migration-error tests for `codex`, depending on the approved removal mode.

Required package decisions:

- Keep npm package identity `@openai/codex` unless T15 approves `@openai/ontocode`.
- Keep Python distribution identity `openai-codex` and runtime dependency `openai-codex-cli-bin` unless T15 approves package rename.
- Rename only the installed command in this ADR.
- If package APIs expose `bundled_codex_path()`, either keep it as a compatibility API returning the `ontocode` binary path or add `bundled_ontocode_path()` while preserving the old function until the package-rename program.
- Keep release asset names `codex-package-*` unless the release pipeline can dual-publish assets and Python runtime setup can resolve both names.
- Keep generated schema bundle names such as `codex_app_server_protocol.v2.schemas.json` unless app-server/protocol versioning separately approves a rename.

Challenge:

- Removing `codex` from package `bin` maps can break installers, shell completions, CI scripts, and SDK runtime discovery.
- Package validation must prove the installed package has exactly the intended bin entries for each removal mode.
- Python runtime setup currently treats failure to import or call `bundled_codex_path()` as absence of the runtime package, so changing this API without compatibility will break SDK artifact generation.

## Stage 5: CLI Text And Docs

- Replace user-facing CLI examples:
  - `codex ...` -> `ontocode ...`
- Update update messages, doctor output, completions, resume hints, plugin help, and marketplace help.
- Keep historical/internal docs unchanged where they describe legacy compatibility or old persisted data.
- Keep package, import, protocol, telemetry, and persisted-state wording where `codex` is still the stable identifier.
- Add a migration page or release-note section that lists old command, new command, and removal schedule.

## Stage 6: Tests

Required focused tests:

- `just test -p codex-cli ontocode_alias`
- new or updated test: `ontocode_binary_is_primary_help_name`
- new or updated test: `codex_binary_is_absent_or_errors_with_migration_message`
- `just test -p codex-arg0`
- `just test -p codex-shell-escalation`
- `just test -p codex-linux-sandbox` where applicable
- `just test -p ontocode-windows-sandbox` where applicable
- package build tests for npm and Python binary layouts

If Rust code changes are made, run `just fmt` after edits.

Additional required package checks:

- npm staged package contains only the approved `bin` map for the selected mode.
- npm platform packages still resolve the native payload used by `ontocode`.
- Python runtime wheel exposes the approved binary path API.
- SDK artifact generation still works against the bundled runtime.
- Shell completions are generated for `ontocode` and not advertised for `codex` after hard removal.
- Linux sandbox tests prove argv0 dispatch still enters sandbox mode through the selected helper name.
- Exec-server file-system sandbox tests prove runtime paths still resolve under the selected helper name.
- App-server and TUI smoke tests prove downstream consumers of `Arg0DispatchPaths` still receive executable paths.

## Stage 7: Migration And Rollback

- Add release notes with exact command migration.
- Provide a shell alias workaround:

```sh
alias codex=ontocode
```

- Define rollback:
  - restore `codex` shim package or binary if install breakage exceeds threshold
  - no data migration rollback is needed if state/env compatibility remains intact

Rollback must be executable without renaming packages:

- npm: restore `codex` bin entry pointing to the same executable or migration shim.
- Python: restore `bundled_codex_path()` semantics if it changed.
- Native artifacts: restore `codex` launcher/shim in package PATH.
- Rust: keep rollback patch small by avoiding crate/package identity renames in this ADR.

## Recommended Implementation Order

1. ADR approval and OntoIndex impact report.
2. Package/runtime inventory with explicit `soft-hard` or `hard` mode.
3. CLI command identity hard cutover.
4. Arg0/helper alias cleanup.
5. Packaging binary removal.
6. Docs/tests cleanup.
7. Release-note and rollback validation.

## Main Risk

The riskiest part is external automation that shells out to `codex`.

If hard cutover is required, prefer one release where `codex` exists only to print a migration error before full removal.

## Manager Recommendation

Use `soft-hard` as the first implementation slice:

- `ontocode` becomes the only successful CLI path.
- `codex` remains for one release only as a migration-error shim.
- Packages keep their existing identities.
- Runtime package APIs keep compatibility names where needed.
- A follow-up hard-removal slice deletes the `codex` shim after telemetry/support evidence confirms acceptable breakage.

Reject immediate `hard` removal unless external automation breakage is an accepted release objective.
