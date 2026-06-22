# Ontocode-Only CLI Hard Cutover Removal Matrix

Source ADR: `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`

Worker: HC1

Date: 2026-06-09

## Status

- Scope: planning-only removal matrix for public `codex` command paths and adjacent compatibility surfaces.
- Implementation status: not approved.
- Selected removal mode: unset.
- Recommendation: use `soft-hard` for public command entry points first; preserve package identity, protocol metadata, release assets, and persisted state until separately approved.

## Evidence Used

- Required memory-bank reads:
  - `MEMORY.md`
  - `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`
  - `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER_TRACKING.md`
  - `ONTOCODE_RENAME_TRACKING.md`
  - `ONTOCODE_REMAINING_SURFACES_DISPOSITION.md`
- OntoIndex inspection:
  - `command_name_from_arg0` resolves in `ontocode-rs/cli/src/main.rs`.
  - `arg0_dispatch` resolves in `ontocode-rs/arg0/src/lib.rs`.
- Direct source checks:
  - `ontocode-rs/cli/Cargo.toml`
  - `ontocode-rs/cli/src/main.rs`
  - `ontocode-rs/cli/src/bin/ontocode.rs`
  - `ontocode-rs/arg0/src/lib.rs`
  - `ontocode-rs/shell-escalation/Cargo.toml`
  - `ontocode-rs/linux-sandbox/Cargo.toml`
  - `ontocode-rs/windows-sandbox-rs/Cargo.toml`
  - `codex-cli/package.json`
  - `codex-cli/bin/codex.js`
  - `codex-cli/scripts/build_npm_package.py`
  - `sdk/python/_runtime_setup.py`
  - `sdk/python/scripts/update_sdk_artifacts.py`
  - `sdk/python/pyproject.toml`
  - top-level `README.md`

## Challenge Findings

- The ADR cannot remove package identities under the same approval as command removal. `ONTOCODE_REMAINING_SURFACES_DISPOSITION.md` explicitly preserves package/runtime identities until a versioned package migration is approved.
- The current T13 state installs both `codex` and `ontocode`; this is a compatibility baseline, not unfinished work.
- The current T14 state creates both `codex-*` and `ontocode-*` helper aliases; removal requires helper runtime validation, especially sandbox argv0 behavior.
- `bundled_codex_path()`, `codex_cli_bin`, `openai-codex-cli-bin`, and `codex-package-*` are package/runtime contracts; removing or renaming them conflicts with deferred T15 unless a separate package migration is approved.
- Generated protocol metadata such as `codex_app_server_protocol.v2.schemas.json` and `codex-cli` client/source metadata should be preserved or versioned, not silently renamed.
- `CODEX_HOME` and related persisted-state compatibility are outside this hard cutover unless a state migration and rollback plan is approved.

## Removal Matrix

| Surface | Current `codex` behavior | Proposed `ontocode` behavior | Removal mode | Required tests | Rollback mechanism | Surface class |
| --- | --- | --- | --- | --- | --- | --- |
| Cargo CLI binary `codex` in `ontocode-rs/cli/Cargo.toml` | Builds `codex` from `src/main.rs` as the primary CLI binary. | Make `ontocode` the primary successful CLI binary. If `soft-hard`, keep `codex` only as a migration-error shim. | `soft-hard` recommended; `hard` only with release-owner approval. | `just test -p codex-cli ontocode_alias`; add primary-help and codex-error/absence tests. | Restore `codex` bin target pointing to primary CLI or migration shim. | user-facing command |
| Cargo CLI binary `ontocode` in `ontocode-rs/cli/Cargo.toml` | Builds wrapper binary from `src/bin/ontocode.rs` that relaunches `codex`. | Move real CLI entry behavior to `ontocode`; wrapper should disappear. | `hard` for wrapper removal after `ontocode` is primary. | `just test -p codex-cli`; binary invocation test for help/version identity. | Reintroduce wrapper that sets `ONTOCODE_CLI_COMMAND_NAME`. | user-facing command |
| `ONTOCODE_CLI_COMMAND_NAME` env override | Wrapper sets env so `codex` displays as `ontocode`. | Delete env override once `ontocode` is the real binary. | `hard` after primary binary switch. | CLI help/display tests without env override; regression test that arbitrary env cannot flip command identity. | Restore env override logic and wrapper. | internal field |
| `command_name_from_arg0` default command identity | Defaults to `codex`; switches to `ontocode` when arg0/env indicates alias. | Defaults to `ontocode`; `codex` either errors as shim or is absent. | `soft-hard` or `hard`, matching selected command removal. | Unit or integration tests for `ontocode --help`, plugin subcommand bin names, and `codex` error/absence. | Restore `PRIMARY_COMMAND_NAME = "codex"` and alias behavior. | user-facing command |
| Clap/plugin bin names such as `codex plugin` | Static annotations use `codex`; runtime aliasing can rewrite to `ontocode`. | Help and usage should always advertise `ontocode`. | `hard` for user-facing help text after cutover. | CLI snapshots or help assertions for plugin and marketplace subcommands. | Restore alias rewrite path or static `codex` bin names. | user-facing command |
| npm package `bin.codex` in `codex-cli/package.json` | Installs `codex` and `ontocode`, both pointing at `bin/codex.js`. | Install only `ontocode`, or keep `codex` as migration-error shim for one release. | `soft-hard` recommended first; `hard` follow-up. | Stage npm package and assert exact `bin` map; smoke `ontocode --version`; assert `codex` error/absence. | Restore `codex` bin entry pointing to same launcher or shim. | user-facing command |
| npm launcher file `bin/codex.js` | Single JS launcher used by both npm bin names; finds native `codex` executable. | Either keep filename as internal launcher while `bin` exposes `ontocode`, or rename only with package validation. | preserve filename initially; do not couple to command removal. | npm staged-package file-list check; launcher resolves native payload for `ontocode`. | Keep/restore `bin/codex.js` and bin map. | package identity |
| npm package identity `@openai/codex` | Published package identity and optional dependency root. | Preserve until T15/package identity migration is approved. | preserve. | Package staging must still publish under `@openai/codex`; SDK dependency checks. | No rollback needed; do not change. | package identity |
| npm platform packages `@openai/codex-*` | Platform optional dependencies and native package aliases. | Preserve until dual-publish/package migration exists. | preserve. | Stage platform packages; verify optional dependency names and native payload resolution. | No rollback needed; do not change. | package identity |
| npm package staging selector `--package codex` | Release tooling stages package choice named `codex`. | Preserve as tooling/package identity unless T15 approves rename. | preserve. | Existing npm staging tests plus exact command-bin assertions. | No rollback needed; do not change. | package identity |
| Native npm payload binary `vendor/<target>/bin/codex` | JS launcher resolves native binary named `codex`. | Prefer switching runtime payload to `ontocode` only after package/runtime validation; otherwise keep binary name behind `ontocode` launcher initially. | preserve initially; `hard` only with package runtime decision. | Npm package smoke on each target; exact payload file-list check; absence/error check for public `codex` command. | Restore native payload name or add `codex` shim in payload. | release asset |
| Release archive names `codex-package-*` | Python runtime setup downloads `codex-package-<target>.tar.gz`. | Preserve unless release pipeline can dual-publish and Python can resolve both names. | preserve. | Python runtime setup tests for asset name and download resolution. | No rollback needed; do not change. | release asset |
| Python SDK distribution `openai-codex` | Published SDK identity and metadata. | Preserve until package identity migration is approved. | preserve. | Python package staging test; metadata assertions. | No rollback needed; do not change. | package identity |
| Python runtime distribution `openai-codex-cli-bin` | Runtime wheel dependency and installed binary package identity. | Preserve until package identity migration is approved. | preserve. | `sdk/python` artifact workflow tests; dependency pin checks. | No rollback needed; do not change. | package identity |
| Python import package `codex_cli_bin` | Runtime package import used to locate bundled binary. | Preserve; optionally add `bundled_ontocode_path()` while keeping old API. | preserve for old API. | Runtime wheel import test; both path APIs if a new one is added. | Restore `codex_cli_bin` import and old function behavior. | package identity |
| Python API `bundled_codex_path()` | Required by runtime detection and SDK artifact generation. | Preserve as compatibility API, returning the approved runtime executable path. | preserve. | `_installed_runtime_version` path check; `pinned_runtime_codex_path()` artifact generation test. | Restore function or compatibility shim. | package identity |
| Python runtime bundled executable `bin/codex` | Runtime wheel validation expects `bin/codex` or `codex.exe`. | Do not rename in first CLI cutover unless runtime package has explicit migration plan. | preserve initially. | Runtime package layout validation; SDK schema generation from pinned runtime. | Restore `bin/codex` in runtime wheel. | release asset |
| `codex-linux-sandbox` arg0 alias | Arg0 dispatch accepts `codex-linux-sandbox` and `ontocode-linux-sandbox`; preferred path currently uses `codex-linux-sandbox`. | Prefer `ontocode-linux-sandbox`; remove `codex-linux-sandbox` after sandbox argv0 validation. | `soft-hard` only if hidden compatibility is required; otherwise `hard` after tests. | `just test -p codex-arg0`; `just test -p codex-linux-sandbox`; sandbox argv0 execution test. | Restore alias creation and dispatch for `codex-linux-sandbox`. | user-facing command |
| `ontocode-execve-wrapper` arg0 alias | Arg0 dispatch accepts `ontocode-execve-wrapper` and `ontocode-execve-wrapper`; preferred path currently uses `ontocode-execve-wrapper`. | Prefer `ontocode-execve-wrapper`; remove `ontocode-execve-wrapper` after exec-server/shell-escalation validation. | `soft-hard` or `hard` matching helper strategy. | `just test -p codex-arg0`; `just test -p codex-shell-escalation`; exec-server sandbox path tests. | Restore alias creation and dispatch for `ontocode-execve-wrapper`. | user-facing command |
| Helper binaries in `shell-escalation`, `linux-sandbox`, and `windows-sandbox-rs` manifests | Manifests build both `codex-*` and `ontocode-*` helper binaries for several helpers. | Remove public `codex-*` helper executable outputs only after runtime callers use `ontocode-*`. | `hard` after caller switch and platform tests. | Helper crate tests; Windows helper/service setup tests where applicable; package file-list checks. | Restore `codex-*` manifest bin entries pointing to same sources. | user-facing command |
| `Arg0DispatchPaths` fields such as `codex_self_exe` and `codex_linux_sandbox_exe` | Internal field names carry `codex` but may point to helper paths. | Preserve field names in first slice; values may point at `ontocode-*` paths. | preserve. | Existing app-server/TUI/exec-server consumers compile and smoke tests pass. | No rollback needed if fields are unchanged. | internal field |
| `CODEX_LINUX_SANDBOX_ARG0` constant and related env/argv0 wiring | Linux sandbox still defines and consumes `CODEX_LINUX_SANDBOX_ARG0`. | Preserve or deprecate only after `ONTOCODE_LINUX_SANDBOX_ARG0` is canonical and tested. | preserve initially; later `hard` with sandbox plan. | Linux sandbox argv0 and bubblewrap tests. | Restore constant and dual dispatch. | internal field |
| App-server/protocol schema bundle `codex_app_server_protocol.v2.schemas.json` | Generated schema bundle name uses `codex`. | Preserve unless protocol versioning approves rename. | preserve. | `just write-app-server-schema`; `just test -p ontocode-app-server-protocol` if changed. | No rollback needed; do not change. | protocol metadata |
| Client/source metadata such as `codex-cli` | Integration metadata may identify client/source as `codex-cli`. | Preserve or version; do not rename as part of command removal. | preserve. | App-server/protocol compatibility tests if metadata is touched. | Restore old metadata string. | protocol metadata |
| `CODEX_HOME` and existing `.codex` state | Persisted state and env compatibility remain supported; `ONTOCODE_HOME` aliases exist elsewhere. | Preserve `CODEX_HOME` and legacy state compatibility. | preserve. | Home/env precedence tests; persisted-state migration tests if changed. | Restore dual-read behavior and state lookup. | persisted state |
| `.codex-plugin` and local integration contracts | Plugin and marketplace integration path remains codex-named. | Preserve unless plugin contract migration is separately approved. | preserve. | Plugin install/list/marketplace tests if touched. | Restore old directory contract. | persisted state |
| User-facing README examples `codex ...` | Some docs still instruct running `codex` for compatibility. | Update user-facing command examples to `ontocode`, while preserving package/install identities. | `hard` for docs after removal mode is selected. | Markdown lint/readability check; docs grep for command examples. | Restore migration-era wording or add release-note caveat. | docs-only |
| Release/changelog install examples `npm install -g @openai/codex` | Package install command remains codex package identity. | Preserve package install command; only execution examples should change to `ontocode`. | preserve. | Docs grep distinguishes package identity from command examples. | No rollback needed; do not change. | docs-only |

## Go/No-Go Result

Implementation may not proceed unless all gates below are satisfied:

- Product/release owner accepts `codex` command removal as a breaking change.
- Removal mode is selected for each public command surface.
- Rollback owner and breakage threshold are named.
- Package identity scope is frozen and explicitly excludes T15 unless separately approved.
- HC2 blast-radius report is complete and no HIGH/CRITICAL OntoIndex risk is unresolved.
- HC3 package/runtime validation plan is complete.
- HC4 migration and rollback release plan is complete.
- Package/runtime compatibility decisions exist for `bundled_codex_path()`, native payload binary names, and `codex-package-*` assets.

## Implementation Recommendation

- First approved slice should be `soft-hard`: `ontocode` is the only successful CLI path, and `codex` remains only as a migration-error shim.
- Do not rename package IDs, release assets, protocol metadata, generated schema names, import paths, persisted-state keys, or broad Rust internal fields in the first slice.
- Treat helper alias removal as a separate slice after sandbox, shell-escalation, exec-server, app-server, and TUI path consumers are verified.
