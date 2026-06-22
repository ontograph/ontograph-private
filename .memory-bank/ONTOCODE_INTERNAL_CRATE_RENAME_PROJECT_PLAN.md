# Ontocode Internal Crate Rename Project Plan

Date: 2026-06-09

## Status

Proposed, challenged.

Implementation is blocked until Stage 0 inventory and approval gates are complete.

## Supersession Boundary

This plan supersedes only the deferred internal Rust crate/package rename decision from `ONTOCODE_REMAINING_SURFACES_DISPOSITION.md`.

It does not supersede preservation decisions for:

- package manager identities
- wire identifiers
- telemetry schemas
- generated protocol model names
- persisted state keys
- `CODEX_*` compatibility inputs
- `.codex` storage and integration contracts
- SDK import package names

## Goal

Replace internal Rust Cargo package, crate, Bazel target, helper package, and developer-facing command names from `codex-*` / `codex_*` to `ontocode-*` / `ontocode_*`.

This is Option 4 from the rename discussion: full internal crate rename.

## Non-Goals

- Do not rename persisted state keys, `CODEX_HOME`, `CODEX_*` compatibility inputs, protocol wire identifiers, telemetry schemas, generated protocol model names, package manager identities, or import package names in this program.
- Do not remove compatibility APIs such as `bundled_codex_path()` unless a separate SDK/package migration approves it.
- Do not change external npm/Python package identities such as `@openai/codex`, `openai-codex`, `openai-codex-cli-bin`, or `@openai/codex-sdk`.
- Do not use broad find-and-replace.

## Why This Is Separate From CLI Hard Cutover

The hard cutover made `ontocode` the public/runtime executable frontier. Cargo package names are a different surface:

- `just test -p codex-cli` uses Cargo package identity, not a binary name.
- `codex-cli` can build only an `ontocode` binary.
- Workspace dependency keys, Bazel crate names, generated lockfiles, imports, CI filters, and tests depend on package names.

Renaming Cargo package identity is therefore a build-system migration, not a CLI rename.

## Current Evidence

OntoIndex batch impact on package-name strings was weak because the graph resolved several package names as folders or Markdown sections rather than Cargo package dependency edges.

Direct source inventory is required for every stage:

- `ontocode-rs/Cargo.toml` workspace dependencies.
- Each crate `Cargo.toml` package name and `[lib] name`.
- `BUILD.bazel` crate names, binary targets, and aliases.
- `Cargo.lock` package records.
- `MODULE.bazel.lock` after Bazel lock regeneration.
- Rust `use codex_*` imports.
- scripts and tests that pass `just test -p codex-*`.
- SDK/runtime/package scripts that reference Cargo package names.

Current direct search shows hundreds of Cargo/Bazel/dependency/import references. Treat every crate rename as a build-system migration, not a local text rename.

Mandatory Stage 0 inventory commands:

```sh
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
cargo metadata --format-version 1 > ../.memory-bank/ontocode_internal_crate_rename-cargo-metadata.json
```

```sh
cd /opt/demodb/_workfolder/ontocode
rg '^name = "codex-|crate_name = "codex_|package = "codex-|codex_[a-z0-9_]+::|use codex_' ontocode-rs > .memory-bank/ontocode_internal_crate_rename-direct_refs.txt
```

```sh
cd /opt/demodb/_workfolder/ontocode
rg 'just test -p codex-|cargo .* -p codex-' . ontocode-rs scripts sdk > .memory-bank/ontocode_internal_crate_rename-command_refs.txt
```

## Rename Policy

Use these mappings. Do not collapse these surface classes together:

| Surface | Old | New | Rule |
| --- | --- | --- | --- |
| Cargo package | `codex-foo` | `ontocode-foo` | Rename by crate family in staged commits |
| Rust lib crate | `codex_foo` | `ontocode_foo` | Rename with imports in same family |
| Bazel crate name | `codex_foo` | `ontocode_foo` | Keep Bazel aligned with Cargo lib name |
| Public executable | `codex` | `ontocode` | Already done |
| Runtime helper executable | `codex-*` | `ontocode-*` | Mostly done for selected helper entrypoints; finish by helper family |
| Package identity | `@openai/codex`, `openai-codex` | unchanged | Out of scope |
| Compatibility env/state | `CODEX_*`, `.codex` | unchanged/aliased | Out of scope |
| Protocol/generated identity | `codex_app_server_protocol`, `codex.exec_server.*` | unchanged unless separately approved | Separate protocol gate |

Hard rule:

- One crate family per implementation slice.
- No mixed utility/helper/core batches.
- No broad find-and-replace.
- Every slice must update Cargo, Rust imports, Bazel, scripts, tests, and lockfiles together.
- Every slice must keep package-manager, persisted-state, wire, telemetry, and generated-schema names unchanged unless a separate ADR approves that exact surface.

## Stage 0: Inventory And Freeze

Deliverables:

- Create `ONTOCODE_INTERNAL_CRATE_RENAME_TRACKING.md`.
- Generate an inventory table with every `codex-*` Cargo package and matching `codex_*` lib crate.
- Classify each crate as `leaf`, `tool/helper`, `SDK/package-facing`, `core/shared`, `protocol/wire`, or `deferred`.
- Generate old/new command maps for `just test -p`, `cargo test -p`, Bazel labels, and package scripts.
- Identify all crates whose rename would touch package/runtime layout code, generated artifacts, or protocol schema names.
- Record direct references from:
  - `ontocode-rs/Cargo.toml`
  - `ontocode-rs/**/Cargo.toml`
  - `ontocode-rs/**/BUILD.bazel`
  - `scripts/**`
  - `sdk/**`
  - `.github/**` if present
  - `justfile`

Exit criteria:

- Inventory reviewed.
- Supersession boundary accepted.
- First implementation slice selected by exact crate list.
- No implementation starts until each crate has an owner family and test command.
- Tracking file is created and updated before dispatch.

## Stage 1: Leaf Utility Crates

Scope:

- Low-dependency crates under `ontocode-rs/utils/**`.
- Small support crates such as ANSI/string/path/template helpers.

Candidate examples:

- `codex-utils-absolute-path` -> `ontocode-utils-absolute-path`
- `codex-utils-cli` -> `ontocode-utils-cli`
- `codex-utils-cargo-bin` -> `ontocode-utils-cargo-bin`
- `codex-utils-home-dir` -> `ontocode-utils-home-dir`
- `codex-ansi-escape` -> `ontocode-ansi-escape`

Required edits per crate:

- Package name in `Cargo.toml`.
- `[lib] name`.
- Workspace dependency key in root `Cargo.toml`.
- All dependent `Cargo.toml` entries.
- Rust imports.
- Bazel crate name and deps.
- Tests and scripts using `-p old-name`.

Verification:

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p <new-package>`
- `just bazel-lock-update`
- `just bazel-lock-check`
- OntoIndex scoped `gn_verify_diff`

## Stage 2: Tool And Helper Crates

Scope:

- Low-risk tool/helper crates after leaf utilities are stable.

Candidate examples:

- `codex-linux-sandbox` -> `ontocode-linux-sandbox`
- `ontocode-windows-sandbox` -> `ontocode-windows-sandbox`
- `codex-shell-escalation` -> `ontocode-shell-escalation`

Constraints:

- Keep existing runtime executable names already migrated to `ontocode-*`.
- Do not rename package archive names such as `codex-package.json`.
- Do not rename compatibility field names if they cross app-server or SDK boundaries.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox`
- package layout tests under `scripts/codex_package`

## Stage 2B: Runtime Path And Package Layout Crates

Scope:

- High-risk helper/runtime crates that affect package layout, arg0 dispatch, sandbox runtime paths, managed installs, or SDK binary discovery.

Candidate examples:

- `ontocode-exec` -> `ontocode-exec`
- `ontocode-exec-server` -> `ontocode-exec-server`
- `codex-sandboxing` -> `ontocode-sandboxing`
- `codex-arg0` -> `ontocode-arg0`
- `codex-install-context` -> `ontocode-install-context`

Constraints:

- Do this only after Stage 2 proves helper/package mechanics.
- Preserve `codex-package.json`, `codex-path`, `codex-resources`, and package identity names.
- Preserve compatibility field/API names that cross SDK or app-server boundaries unless a separate compatibility plan exists.
- Include package-layout smoke tests in every slice.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-install-context`
- `python3 scripts/codex_package/test_cargo.py`
- SDK native-binary lookup tests where applicable

## Stage 3: CLI And App Server Crates

Scope:

- User-entry orchestration crates that are high blast-radius.

Candidate examples:

- `codex-cli` -> `ontocode-cli`
- `ontocode-tui` -> `ontocode-tui`
- `ontocode-app-server` -> `ontocode-app-server`
- `ontocode-app-server-daemon` -> `ontocode-app-server-daemon`
- `ontocode-app-server-client` -> `ontocode-app-server-client`
- `ontocode-app-server-transport` -> `ontocode-app-server-transport`
- `ontocode-app-server-test-client` -> `ontocode-app-server-test-client`

Constraints:

- Do not change app-server wire method names.
- Do not change generated protocol model names in this stage.
- Do not change public npm/Python package identities.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-daemon`
- TUI snapshot review/acceptance if user-visible output changes

## Stage 4: Provider/Auth/MCP/Core Support Crates

Scope:

- Shared implementation crates that affect provider, auth, MCP, session, model catalog, and plugin flows.

Candidate examples:

- `codex-model-provider` -> `ontocode-model-provider`
- `codex-rmcp-client` -> `ontocode-rmcp-client`
- `codex-mcp` -> `ontocode-mcp`
- `ontocode-mcp-server` -> `ontocode-mcp-server`
- `codex-login` -> `ontocode-login`
- `codex-config` -> `ontocode-config`
- `codex-plugin` -> `ontocode-plugin`
- `codex-hooks` -> `ontocode-hooks`

Constraints:

- Reuse existing architecture owners.
- Do not create duplicate provider/auth/MCP registries.
- Keep compatibility config keys unless a separate ADR approves schema migration.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-config`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider`

## Stage 5: Core And Protocol Crates

Scope:

- Highest-risk core crates.

Candidate examples:

- `codex-core` -> `ontocode-core`
- `codex-core-api` -> `ontocode-core-api`
- `codex-client` -> `ontocode-client`
- `codex-api` -> `ontocode-api`

Constraints:

- Do this last.
- Expect broad import and Bazel churn.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-core`
- Ask before running complete `CARGO_BUILD_JOBS=8 just test`
- Run `just write-config-schema` if config schema paths/types change
- Run `just write-app-server-schema` if app-server protocol API shapes change

## Stage 5B: Protocol Crate Decision Gate

Scope:

- Protocol and generated-schema crates only after core rename stability is proven.

Candidate examples:

- `codex-protocol` -> `ontocode-protocol`
- `ontocode-app-server-protocol` -> `ontocode-app-server-protocol`
- generated schema bundle names such as `codex_app_server_protocol.v2.schemas.json`
- protobuf package names such as `codex.exec_server.relay.v1`

Default decision:

- Preserve protocol/generated names.

Go criteria:

- A separate protocol-versioning ADR approves the exact rename.
- Dual-reader or alias behavior is specified where clients may already consume old identifiers.
- SDK generated artifacts are regenerated and compatibility-tested.

Verification if approved:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-protocol`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol`
- `just write-app-server-schema`
- Python and TypeScript SDK schema/artifact tests

## Stage 6: Cleanup And Command Alias Retirement

Scope:

- Developer command aliases and docs after crate names are renamed.

Actions:

- Replace `just test -p codex-*` examples with `just test -p ontocode-*`.
- Update `AGENTS.md` only after package rename stages are complete.
- Update memory-bank tracking and prompt templates.
- Remove temporary `just` alias targets if added.
- Keep historical ADR text unchanged where it describes old names.

Verification:

- Search for stale developer-command references:
  - `rg 'just test -p codex-|cargo .* -p codex-|codex_[a-z_]+::|package = "codex-'`
- OntoIndex scoped diff verification.
- Full package inventory shows no accidental public package identity changes.

## Required Tracking File

Create `.memory-bank/ONTOCODE_INTERNAL_CRATE_RENAME_TRACKING.md` before implementation.

Suggested table:

| ID | Crate Family | Old Package | New Package | Status | Owner | Tests | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| R0 | Inventory | all | all | pending | manager | direct inventory | no code edits |
| R1 | Leaf utils | `codex-utils-*` | `ontocode-utils-*` | pending | TBD | scoped package tests | low-risk first |
| R2 | Helpers | helper/runtime crates | `ontocode-*` | pending | TBD | helper/package tests | no package ID rename |
| R2B | Runtime paths | arg0/install-context/exec-server/sandboxing | `ontocode-*` | pending | TBD | runtime/package tests | high risk |
| R3 | CLI/App | CLI/app crates | `ontocode-*` | pending | TBD | CLI/TUI/app tests | high risk |
| R4 | Provider/Auth/MCP | support crates | `ontocode-*` | pending | TBD | provider/auth/MCP tests | reuse owners |
| R5 | Core | core/shared crates | `ontocode-*` | pending | TBD | core/full tests | last before protocol |
| R5B | Protocol gate | protocol/generated crates | preserve or approved rename | pending | TBD | schema/SDK tests | separate ADR required |
| R6 | Cleanup | docs/prompts/commands | new names | pending | TBD | grep + OntoIndex | no historical rewrite |

## Risk Register

| Risk | Severity | Mitigation |
| --- | --- | --- |
| Cargo/Bazel lock drift | High | run `just bazel-lock-update` and `just bazel-lock-check` after each dependency rename stage |
| Massive import churn | High | stage by crate family; keep changes under reviewable size |
| Protocol/schema accidental rename | Critical | freeze wire/schema identifiers unless separate ADR approves |
| Package manager identity breakage | Critical | do not rename npm/Python package names in this program |
| CI/test filters break | High | update CI and local `just` references in same stage |
| Hidden scripts still call old package names | Medium | inventory scripts before each stage; run focused script tests |
| OntoIndex weak package-name modeling | Medium | combine OntoIndex with direct Cargo/Bazel inventory and scoped `gn_verify_diff` |
| Low-level agent over-renames preserved identifiers | Critical | Stage 0 must produce surface taxonomy and forbidden-string classes before implementation |

## Go/No-Go Criteria

Go:

- Inventory complete.
- Supersession boundary accepted.
- First stage limited to leaf crates.
- Old/new command map exists for the selected slice.
- Rollback is simple: restore package/lib names for the current stage only.
- Test commands are converted to new package names and pass.

No-go:

- Stage requires broad find-and-replace.
- Stage renames package manager identities.
- Stage renames wire/protocol/schema names without separate ADR.
- Stage changes more than one high-risk family at a time.
- Stage includes `codex-core`, `codex-cli`, `codex-protocol`, or `ontocode-app-server-protocol` before leaf and runtime-path proof stages are complete.

## Manager Recommendation

Do not start with `codex-core` or `codex-cli`.

Start with leaf utilities and one helper crate to prove the mechanical path:

1. `codex-utils-absolute-path` -> `ontocode-utils-absolute-path`
2. `codex-utils-cargo-bin` -> `ontocode-utils-cargo-bin`
3. `codex-install-context` -> `ontocode-install-context`

Only after Cargo, Bazel, tests, and OntoIndex verification are stable should the plan advance to CLI/app/core crates.
