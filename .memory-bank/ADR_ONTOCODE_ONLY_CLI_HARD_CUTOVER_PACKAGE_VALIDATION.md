# Ontocode-Only CLI Hard Cutover Package Validation

Source ADR: `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`

Status: planning only; implementation may not proceed unless the gates in this document and the ADR are satisfied.

## Scope

This plan covers Stage 4 package install surfaces and Stage 6 package/runtime verification for the Ontocode-only CLI hard cutover.

Write ownership for this pass was limited to this file. Implementation code and the manager-owned tracking file were not edited.

## Source Evidence

- `codex-cli/package.json` currently preserves package identity `@openai/codex` and exposes both `codex` and `ontocode` bin entries pointing at `bin/codex.js`.
- `codex-cli/scripts/build_npm_package.py` preserves `@openai/codex`, `@openai/codex-*` platform package names, `codex-package` native component naming, and staged files under `bin/codex.js`.
- `sdk/python/_runtime_setup.py` preserves Python identities `openai-codex`, `openai-codex-cli-bin`, `openai/codex`, and `codex-package-*` release asset names.
- `sdk/python/scripts/update_sdk_artifacts.py` preserves `openai-codex`, `openai-codex-cli-bin`, `codex_cli_bin`, `codex-package.json`, and currently validates a bundled runtime binary named `codex` or `codex.exe`.
- `sdk/python/pyproject.toml` preserves distribution identity `openai-codex` and runtime dependency `openai-codex-cli-bin==0.137.0a4`.
- `sdk/python-runtime/src/codex_cli_bin/__init__.py` already exposes both `bundled_codex_path()` and `bundled_ontocode_path()`, but `bundled_codex_path()` still requires `bin/codex`.
- `sdk/python-runtime/pyproject.toml` preserves package namespace `codex_cli_bin` and includes `codex-package.json`, `bin/**`, `codex-resources/**`, and `codex-path/**`.
- `scripts/codex_package/targets.py` currently models native `codex` as entrypoint `codex` with alias `ontocode`.
- `scripts/codex_package/layout.py` copies the entrypoint and aliases into `bin/`, writes `codex-package.json`, and validates all entrypoint/alias binaries.
- `scripts/install/install.sh` and `scripts/install/install.ps1` currently verify and expose `codex` paths and `codex-package-*` release assets.
- Shell completions were not found as a distinct package artifact in the scoped searches; Stage 6 still requires a completion-generation check if implementation discovers or adds completion packaging.

## Package Identity Rule

Package identities must be preserved unless T15/package identity migration is separately approved.

- Preserve npm package identity `@openai/codex`.
- Preserve npm platform package identities `@openai/codex-linux-x64`, `@openai/codex-linux-arm64`, `@openai/codex-darwin-x64`, `@openai/codex-darwin-arm64`, `@openai/codex-win32-x64`, and `@openai/codex-win32-arm64`.
- Preserve Python distribution identity `openai-codex`.
- Preserve Python runtime carrier identity `openai-codex-cli-bin`.
- Preserve Python import/runtime namespace `openai_codex` and `codex_cli_bin`.
- Preserve release asset names `codex-package-*` and checksum asset `codex-package_SHA256SUMS`.
- Preserve native layout directory/file identity `codex-package.json`, `codex-resources`, and `codex-path`.
- Preserve generated protocol/schema bundle names such as `codex_app_server_protocol.v2.schemas.json`.

Any implementation proposal that renames these identities is a blocker unless T15 approval, dual-publish/metapackage strategy, and release automation validation are attached.

## Stage 4 Validation Matrix

| Surface | Current state | `soft-hard` expected state | `hard` expected state | Required validation |
| --- | --- | --- | --- | --- |
| npm package identity | `@openai/codex` | preserved | preserved | staged `package.json` name equals `@openai/codex` |
| npm bin map | `codex` and `ontocode` -> `bin/codex.js` | `ontocode` succeeds; `codex` may remain only as a migration-error shim | only `ontocode` is installed | inspect source and staged tarball `package.json` `bin` exactly matches selected mode |
| npm launcher file | `bin/codex.js` | may remain as internal launcher file if bin map behavior is correct | may remain as internal launcher file if not exposed as `codex` | tarball file list and executable smoke test through installed bin name |
| npm platform packages | `@openai/codex-*` optional deps carrying native payload | preserved | preserved | staged package optional deps still resolve native payload used by `ontocode` |
| Python SDK distribution | `openai-codex` | preserved | preserved | staged SDK package metadata name and dependency pin unchanged |
| Python runtime distribution | `openai-codex-cli-bin` | preserved | preserved | staged runtime package metadata name unchanged |
| Python runtime API | `bundled_codex_path()` and `bundled_ontocode_path()` | `bundled_codex_path()` must remain compatible, either returning the migration shim or the `ontocode` binary path; `bundled_ontocode_path()` must return the successful runtime | `bundled_codex_path()` must remain compatible unless T15/API break is approved; `bundled_ontocode_path()` must return the successful runtime | import smoke test for both APIs, plus missing-file error assertions for the selected mode |
| Python runtime binary layout | validates `bin/codex` or `bin/codex.exe` | `bin/ontocode` must exist and succeed; `bin/codex` may exist only as migration-error shim | `bin/ontocode` must exist and succeed; `bin/codex` absent unless retained only as non-PATH compatibility API target approved by release owner | archive extraction and wheel contents inspection |
| SDK artifact generation | invokes `bundled_codex_path()` for schema generation | generation must still work and must use the successful `ontocode` runtime if `codex` shim exits with migration error | generation must still work without a `codex` executable | `python sdk/python/scripts/update_sdk_artifacts.py generate-types` or a focused equivalent against a staged runtime |
| native package entrypoint | variant `codex`, entrypoint `codex`, alias `ontocode` | entrypoint must be `ontocode`; `codex` may be alias only if it is a migration-error shim | entrypoint must be `ontocode`; no `codex` alias | `scripts/codex_package` layout validation for linux, macOS, and Windows targets |
| native installers | visible command and verification use `codex` | visible command must be `ontocode`; optional `codex` shim validated as migration error | visible command must be `ontocode`; `codex` absent | installer dry-run or fixture tests for `install.sh` and `install.ps1` path/link verification |
| shell completions | not found as distinct package artifact in scoped searches | completions generated/advertised for `ontocode`; `codex` completion only if migration shim explicitly documents no normal CLI use | completions generated/advertised only for `ontocode` | completion generation/package artifact inspection if implementation touches completion surfaces |

## `soft-hard` Mode Checks

`soft-hard` means `ontocode` is the only successful CLI path, while `codex` may exist for one release only to print a migration error.

Required checks:

- Source package manifest check: `codex-cli/package.json` has package name `@openai/codex`; `bin.ontocode` is present; `bin.codex` is present only if mapped to the approved migration-error shim.
- Staged npm check: run `python codex-cli/scripts/build_npm_package.py --package codex --version <version> --staging-dir <tmp>` and assert staged `package.json` preserves `@openai/codex`, exposes the selected `bin` map, and keeps optional dependencies on `@openai/codex-*`.
- npm install smoke: install the packed tarball into an isolated prefix; `ontocode --help` succeeds and shows `ontocode`; `codex --help` exits non-zero and prints only the approved migration message.
- npm native payload smoke: install/resolve a staged platform package and prove `ontocode --version` reaches the native payload.
- Python SDK metadata check: staged SDK package remains `openai-codex` and depends on `openai-codex-cli-bin`.
- Python runtime wheel check: staged runtime wheel remains `openai-codex-cli-bin`, includes `bin/ontocode`, includes `bin/codex` only as the migration shim, and preserves `codex-package.json`, `codex-resources`, and `codex-path`.
- Python runtime API check: `bundled_ontocode_path()` returns an executable that succeeds; `bundled_codex_path()` returns the approved compatibility target and either produces the migration error or delegates according to the owner-approved shim behavior.
- SDK artifact check: schema/type generation must not depend on executing a `codex` migration-error shim; it must use the successful `ontocode` runtime path or a compatibility API that returns the successful runtime.
- native package check: generated package metadata entrypoint is `bin/ontocode`; `bin/codex` exists only if it is the migration-error shim; `codex-package-*` asset names remain unchanged.
- installer check: standalone install exposes `ontocode`; if `codex` is installed, it fails with the migration message and does not launch the app.
- completion check: generated completions advertise `ontocode`; `codex` completions are absent unless the migration shim explicitly has a minimal completion behavior approved by release owner.
- rollback check: restoring a `codex` bin/shim in npm, Python runtime, and native package layouts restores `codex --version` or the previous compatibility behavior without renaming any package.

## `hard` Mode Checks

`hard` means `codex` is not installed as a public command.

Required checks:

- Source package manifest check: `codex-cli/package.json` package name remains `@openai/codex`; `bin` contains `ontocode` only.
- Staged npm check: staged and packed `package.json` contains exactly one public bin entry, `ontocode`; no packed `package/.bin/codex` is created after isolated install.
- npm install smoke: `ontocode --help` succeeds and shows `ontocode`; `command -v codex` or platform equivalent fails in the isolated prefix.
- npm native payload smoke: `ontocode --version` resolves native payload from preserved `@openai/codex-*` optional dependency identities.
- Python SDK metadata check: `openai-codex` and `openai-codex-cli-bin` identities remain unchanged.
- Python runtime wheel check: wheel includes `bin/ontocode` or `bin/ontocode.exe`; wheel does not include `bin/codex` or `bin/codex.exe` unless a non-public compatibility API exception is explicitly approved.
- Python runtime API check: `bundled_ontocode_path()` returns the runtime binary; `bundled_codex_path()` remains import-compatible and either returns the `ontocode` path or raises a documented compatibility exception only if T15/API break is approved.
- SDK artifact check: `generate-types` works without any `codex` executable in the runtime package.
- native package check: package metadata entrypoint is `bin/ontocode`; alias list does not require `codex`; validators fail packages that still install `codex`.
- installer check: standalone install exposes only `ontocode`; `codex` is absent from the visible bin directory and current release link.
- completion check: completion artifacts and help output advertise only `ontocode`.
- rollback check: a rollback patch can restore `codex` bin/shim in all install surfaces while keeping package names and release asset names unchanged.

## Required Test Commands

Rust/package commands must be adjusted to the final implementation scope, but these are the minimum Stage 6 gates for package/runtime work:

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-cli ontocode_alias`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-arg0`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-shell-escalation`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-linux-sandbox` on Linux-capable hosts
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox` on Windows-capable hosts
- `python codex-cli/scripts/build_npm_package.py --package codex --version <version> --staging-dir <tmp>/npm-codex`
- `npm pack --json --pack-destination <tmp>`, run from the staged npm package directory
- isolated npm install smoke for the packed tarball, asserting the selected `ontocode`/`codex` behavior
- platform package staging for every `CODEX_PLATFORM_PACKAGES` entry, using vendor payload fixtures or release artifacts
- `python sdk/python/scripts/update_sdk_artifacts.py stage-runtime <tmp>/runtime-stage <codex-package-archive> --codex-version <version>`
- wheel build for staged Python runtime and metadata/file-list inspection
- Python import smoke for `codex_cli_bin.bundled_ontocode_path()` and `codex_cli_bin.bundled_codex_path()`
- `python sdk/python/scripts/update_sdk_artifacts.py generate-types` or a focused schema-generation equivalent against the staged runtime
- `python -m pytest sdk/python/tests/test_artifact_workflow_and_binaries.py`
- native package layout validation through `scripts/codex_package/cli.py` for representative Linux, macOS, and Windows targets
- installer fixture or dry-run tests for `scripts/install/install.sh` and `scripts/install/install.ps1`
- completion generation/package inspection if completion artifacts are present after implementation
- `ontoindex detect-changes --repo codex` or `mcp__ontoindex.gn_verify_diff` after implementation diffs exist

If Rust code changes are made, run `cd ontocode-rs && just fmt` after edits. If common/core/protocol crates are touched, the complete `just test` gate remains a manager/release-owner decision because this planning task did not approve implementation.

## Package Blockers

- Removal mode is unset in the tracking file.
- Product/release owner approval for the breaking command removal is missing.
- Rollback owner and rollback threshold are missing.
- Current npm bin map still exposes both `codex` and `ontocode`.
- Current native package variant still uses `codex` as the entrypoint and `ontocode` as an alias.
- Current Python runtime validation still requires `bin/codex` for `bundled_codex_path()` and SDK artifact generation.
- Current native installers verify and expose `codex`.
- `soft-hard` requires an approved migration-error shim design before package validation can pass.
- `hard` requires approval that external automation breakage from absent `codex` is accepted.
- Any package identity rename remains blocked unless T15/package identity migration is approved.

## Rollback Validation

Rollback must restore `codex` command compatibility without renaming packages.

Required rollback proof:

- npm rollback patch restores `bin.codex` in `@openai/codex` to either the previous successful launcher or an approved shim, while keeping `bin.ontocode`.
- Python runtime rollback patch restores `bundled_codex_path()` to a successful executable path and restores `bin/codex` or `bin/codex.exe` in `openai-codex-cli-bin`.
- native package rollback patch restores `codex` alias or entrypoint in `codex-package-*` archives and preserves `ontocode`.
- installer rollback patch restores visible `codex` verification and symlink behavior without changing `codex-package-*` asset names.
- shell completion rollback restores any previously shipped `codex` completion only if completions were removed in the cutover.
- rollback smoke installs the rolled-back npm, Python runtime, and native package artifacts in isolated prefixes and proves both `codex --version` and `ontocode --version` satisfy the rollback policy.

## Validation Performed For This Planning Pass

- Read required memory-bank files: `MEMORY.md`, hard-cutover ADR, tracking file, package identity migration, and remaining surfaces disposition.
- Used OntoIndex semantic search for npm/Python/native package validation surfaces. Result repo was `codex` at `/opt/demodb/_workfolder/ontocode`; freshness was degraded only by dirty-worktree overlay and embeddings were unavailable, so direct source reads were used as authority.
- Read direct source files for npm package manifest/build script, Python runtime setup/artifact script/pyproject, runtime carrier API/pyproject, native package layout/targets, installers, and Python artifact tests.
- Ran scoped lean-ctx searches for package assets, bin maps, runtime APIs, native layout aliases, installers, and discoverable shell completion artifacts.

## Go/No-Go

Implementation may not proceed unless all package blockers are resolved and the selected removal mode has passing package/runtime validation.

The recommended first implementation mode remains `soft-hard` because it preserves package identities, limits the external break to command success semantics, and keeps rollback to a small bin/shim restoration.
