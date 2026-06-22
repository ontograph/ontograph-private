# Ontocode Full Legacy Migration Project Plan

Date: 2026-06-14
Status: reviewed-proposed

## Goal

Fully migrate the repository from legacy `codex` identity surfaces toward `ontocode`, including the Rust workspace path `ontocode-rs/` -> `ontocode-rs/`, while preserving compatibility only where an explicit package, protocol, telemetry, or persisted-state contract requires it.

This is a breaking-change program. It must not be implemented with broad search-and-replace.

## Current Baseline

- The public binary `ontocode` builds and reports `Ontocode CLI`.
- The Rust workspace directory remains `ontocode-rs/`.
- The repo has roughly 249 direct `ontocode-rs` path references outside build outputs.
- The repo has thousands of remaining `codex`, `Codex`, `CODEX_*`, `codex-*`, package, protocol, telemetry, test, and historical references.
- Existing policy documents classify many remaining legacy surfaces as intentionally preserved or deferred:
  - `ONTOCODE_REMAINING_SURFACES_DISPOSITION.md`
  - `ONTOCODE_PACKAGE_IDENTITY_MIGRATION.md`
  - `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`

## OntoIndex Review Notes

Review date: 2026-06-14.

OntoIndex semantic review was run against repo `codex` for `ontocode-rs ontocode-rs migration package identity CLI hard cutover CODEX_HOME protocol telemetry generated schemas`.

Capability notes:

- The index is usable but degraded by a dirty-worktree overlay.
- Embeddings are unavailable, so the review used graph/BM25 evidence rather than vector evidence.
- This is enough for planning, but implementation slices still require fresh OntoIndex impact on exact symbols before edits.

Risk anchors found by OntoIndex:

- SDK generation and runtime staging:
  - `sdk/python/scripts/update_sdk_artifacts.py::generate_v2_all`
  - `sdk/python/scripts/update_sdk_artifacts.py::generate_types`
  - `sdk/python/scripts/update_sdk_artifacts.py::default_cli_ops`
- State and persisted data:
  - `ontocode-rs/state/src/runtime.rs::StateRuntime.init_inner`
  - `ontocode-rs/state/src/runtime.rs::open_state_sqlite_tolerates_newer_applied_migrations`
  - `ontocode-rs/state/src/migrations.rs::runtime_migrator`
- Package/runtime validation:
  - `sdk/python/tests/test_artifact_workflow_and_binaries.py::test_runtime_package_layout_is_included_by_wheel_config`
  - `sdk/python/tests/test_artifact_workflow_and_binaries.py::test_stage_runtime_stages_package_without_type_generation`
  - `codex-cli/scripts/build_npm_package.py::stage_sources`
- App-server packaged test paths and schema fixtures:
  - `ontocode-rs/app-server/tests/suite/v2/turn_start_zsh_fork.rs::create_test_package_app_server`
  - `ontocode-rs/app-server-protocol/tests/schema_fixtures.rs::assert_schema_fixtures_match_generated`
  - `ontocode-rs/app-server-protocol/tests/schema_fixtures.rs::schema_root`
- CLI/config/env compatibility:
  - `ontocode-rs/core/src/config/mod.rs::Config.load_default_with_cli_overrides_for_codex_home`
  - `ontocode-rs/cli/src/doctor.rs::stored_auth_mode`
  - `ontocode-rs/cli/src/doctor.rs::auth_mode_name`

Conclusion:

- The plan is directionally correct, but Stage 1 cannot be treated as a simple path move.
- The first implementation stage must be preceded by an evidence-backed surface matrix and a dedicated tracking file.
- Package/runtime, state/env, and generated protocol names must remain separate stages with release/versioning approval.

## Non-Negotiable Rules

- Do not use broad find-and-replace.
- Run OntoIndex context and impact before editing any code symbol.
- Warn before proceeding if impact is HIGH or CRITICAL.
- Keep each implementation slice reviewable and independently buildable.
- Update tracking before starting each slice.
- Refresh OntoIndex after each completed code slice.
- Keep package, protocol, telemetry, and persisted-state compatibility unless the stage explicitly approves a versioned migration.
- Provide exact build commands, working directories, and artifact paths for every compilation or binary task.
- Treat filesystem paths, Bazel labels, Cargo package names, Rust crate names, binary names, package names, env vars, state paths, protocol names, and telemetry names as separate surfaces.
- Do not combine Stage 1 layout movement with CLI hard cutover, package rename, env/state migration, protocol/schema generation, or telemetry renames.

## Surface Classes

| Class | Examples | Default Treatment |
| --- | --- | --- |
| Workspace path | `ontocode-rs/` | Rename in Stage 1 |
| Public CLI command | `codex`, `ontocode` | Move to `ontocode` primary; remove or hard-error `codex` by approved mode |
| Helper binaries | `ontocode-exec`, `codex-linux-sandbox`, `ontocode-execve-wrapper` | Migrate to `ontocode-*` after runtime path impact review |
| Cargo crate/package names | `codex-*`, `codex_*` | Already mostly migrated; verify zero active Cargo package identities remain |
| npm/Python package identities | `@openai/codex`, `openai-codex`, `openai-codex-cli-bin` | Dual-publish or preserve until release owner approves cutover |
| SDK import paths | `openai_codex`, `@openai/codex-sdk` | Preserve until major-version or dual-publish plan |
| Env/state compatibility | `CODEX_HOME`, `CODEX_*` | Add/read `ONTOCODE_*`; remove old only after migration window |
| Protocol/generated names | `codex_app_server_protocol`, generated `Codex*` models | Version or preserve; never local-only rename generated output |
| Telemetry schemas | `CodexTurnEventRequest`, analytics event names | Preserve unless telemetry schema versioning is approved |
| Historical docs/audits | old audit entries, migration notes | Preserve as history |

## Stage 0: Approval, Inventory, And Tracking

Status: pending.

Deliverables:

- Create `ONTOCODE_FULL_LEGACY_MIGRATION_TRACKING.md`.
- Produce a complete surface matrix with one row per legacy surface class.
- For each row record:
  - current behavior
  - target behavior
  - compatibility mode: preserve, alias, soft-hard, hard, dual-publish, metapackage, or version
  - owner files
  - OntoIndex targets
  - required tests
  - rollback plan
- Confirm release owner approval for any hard removal of public `codex` commands or package names.
- Confirm whether the first release mode is:
  - layout-only
  - CLI/helper hard cutover
  - dual-publish migration
  - major-version hard rename
- Inventory all generated outputs and decide whether each one is regenerated, path-rewritten, preserved, or excluded:
  - app-server protocol JSON and TypeScript schemas
  - Python generated SDK models
  - TypeScript generated SDK models
  - snapshot tests
  - Bazel generated lock/update outputs
- Inventory all external package/runtime owner files before any implementation dispatch:
  - `codex-cli/package.json`
  - `codex-cli/scripts/build_npm_package.py`
  - `scripts/codex_package/*.py`
  - `sdk/python/pyproject.toml`
  - `sdk/python/scripts/update_sdk_artifacts.py`
  - `sdk/python/tests/test_artifact_workflow_and_binaries.py`
- Record current exact inventory counts for:
  - direct `ontocode-rs` references
  - active `codex-*` Cargo package identities
  - `CODEX_*` env vars
  - public `codex` command/package surfaces
  - generated `Codex*` protocol model names

Go gate:

- No implementation starts until the surface matrix is reviewed and accepted.
- No implementation starts while the worktree has unclassified unrelated rename/package changes that would make verification ambiguous.
- No package, env/state, protocol, or telemetry migration starts without a named release/versioning owner.

## Stage 1: Workspace Layout Rename

Status: pending.

Goal:

- Rename the Rust workspace directory from `ontocode-rs/` to `ontocode-rs/`.

Scope:

- Filesystem directory move.
- Root `justfile` working directory and manifest paths.
- `MODULE.bazel` and Bazel package labels such as `//ontocode-rs/...`.
- Package scripts that set `CODEX_RS_ROOT`.
- Docs, memory-bank references, CI/build scripts, schema generation commands, test fixtures, snapshots, and helper scripts that reference the workspace path.

Out of scope:

- Public binary behavior.
- Package identities.
- Protocol/generated schema names.
- Telemetry and persisted-state names.
- Bazel target names such as `:codex` unless Stage 2 or Stage 3 explicitly owns that rename.
- npm/Python package identities such as `@openai/codex`, `openai-codex`, and `openai-codex-cli-bin`.
- Python import paths such as `openai_codex`.

Challenge:

- This stage is still high blast radius because it touches build graph paths, test fixtures, schema roots, package staging code, docs, and memory-bank references.
- Keep it purely path/layout focused. If a patch changes binary behavior, package names, protocol schema names, or env/state semantics, reject and split it.
- A temporary compatibility symlink `ontocode-rs -> ontocode-rs` may be useful only as a rollback or transition aid, but it must not become a permanent second source root.
- The repo label in OntoIndex may remain `codex` initially; repo label migration is a separate tooling/indexing operation from the filesystem move.

Required verification:

```bash
cd /opt/demodb/_workfolder/ontocode
CARGO_BUILD_JOBS=8 cargo build --manifest-path ontocode-rs/Cargo.toml -p ontocode-cli --bin ontocode
```

Expected debug artifact:

```text
/opt/demodb/_workfolder/ontocode/ontocode-rs/target/debug/ontocode
```

Additional verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p ontocode-cli
```

If Bazel labels change:

```bash
cd /opt/demodb/_workfolder/ontocode
bazel build --jobs=8 //ontocode-rs/cli:codex
```

Additional path checks:

```bash
cd /opt/demodb/_workfolder/ontocode
rg -n --glob '!ontocode-rs/target/**' --glob '!node_modules/**' --glob '!bazel-*' 'ontocode-rs'
```

The remaining matches must be classified as historical docs, compatibility notes, or follow-up work before closing Stage 1.

Rollback:

- Move `ontocode-rs/` back to `ontocode-rs/`.
- Revert path-only references in the same slice.

## Stage 2: CLI Command Cutover

Status: pending.

Authority:

- `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`

Goal:

- Make `ontocode` the only primary public CLI command.

Scope:

- Remove successful `codex` command behavior or replace it with a short migration-only failure, depending on the approved mode.
- Remove command-name override logic that only exists for alias compatibility.
- Ensure help, version, completions, doctor output, update messages, docs, and examples use `ontocode`.
- Update CLI tests so `ontocode` is primary and `codex` behavior matches the approved removal mode.

Required OntoIndex targets before edits:

- `command_name_from_arg0`
- `current_command_name`
- `try_parse_multitool_cli_from`
- `command_name_override_from_env`
- `multitool_command`
- `ontocode_alias` tests

Required verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-cli ontocode_alias
CARGO_BUILD_JOBS=8 cargo build -p ontocode-cli --bin ontocode
./target/debug/ontocode --version
./target/debug/ontocode --help
```

Challenge:

- Stage 2 must not assume package managers have removed `codex`; package install surfaces are Stage 4.
- Removing the Cargo `codex` binary target may remove duplicate-target warnings, but it also breaks callers that use `cargo_bin("codex")`, package tests, and compatibility docs. Require explicit absence/migration-error tests.
- If the approved mode is hard removal, tests must assert that `codex` is absent from successful install outputs. If the approved mode is soft-hard, tests must assert the exact migration error and exit status.

Rollback:

- Restore `codex` binary target or package shim.

## Stage 3: Helper Binary And Runtime Path Cutover

Status: pending.

Goal:

- Replace public/helper runtime paths from `codex-*` to `ontocode-*` without breaking sandbox, exec-server, TUI, app-server, or packaged runtime behavior.

Scope:

- `arg0` helper dispatch.
- shell escalation wrapper names.
- Linux sandbox helper names and argv0 handling.
- Windows sandbox/setup helper names.
- exec-server helper paths.
- app-server and TUI runtime path consumers.

Required OntoIndex targets before edits:

- `arg0_dispatch`
- `Arg0DispatchPaths`
- `prepare_path_entry_for_codex_aliases`
- `transform_linux_seccomp_request`
- `run_main_with_arg0_guard`
- `ExecServerRuntimePaths::new`

Challenge:

- `Arg0DispatchPaths` is consumed by exec-server, app-server, TUI, sandboxing, tools, and tests. Do not rename its fields in the same slice that changes executable values unless OntoIndex impact shows a bounded blast radius.
- Linux sandbox behavior depends on argv0 rewriting. Tests must exercise sandbox entry behavior, not only Cargo manifest names.
- Windows helper names and setup binaries need a separate Windows verification path; do not mark Stage 3 complete from Linux-only evidence.

Required verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-arg0
CARGO_BUILD_JOBS=8 just test -p ontocode-shell-escalation
CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox
CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server
```

Windows-only verification, when available:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox
```

Rollback:

- Restore helper aliases and dispatch entries.

## Stage 4: Package Identity Migration

Status: pending.

Authority:

- `ONTOCODE_PACKAGE_IDENTITY_MIGRATION.md`

Goal:

- Move published package surfaces toward Ontocode without breaking existing installs.

Scope:

- npm CLI package.
- Python SDK distribution.
- Python runtime carrier.
- TypeScript SDK package.
- native runtime/platform packages.
- release asset names.

Decision options:

- `preserve`: keep legacy package identity but install/run `ontocode`.
- `dual-publish`: publish old and new package identities from the same source.
- `metapackage`: publish a compatibility package that depends on the new one.
- `version`: carry rename in a major or protocol version.
- `hard`: remove old identity only after explicit release approval.

Required package checks:

- npm package bin map contains only approved entries.
- Python runtime exposes approved binary path APIs.
- SDK artifact generation still works.
- Native release archive names and package metadata match the approved migration mode.
- Existing package identities either dual-publish, metapackage, or preserve according to the release-owner decision.
- Runtime package tests cover both the bundled binary path and the schema bundle path.

Challenge:

- `@openai/codex`, `openai-codex`, `openai-codex-cli-bin`, `codex_cli_bin`, and `codex-package-*` are release contracts, not cosmetic strings.
- Do not hard-rename any package identity until dual-publish/metapackage automation is proven locally and approved for release.
- If the Python runtime API keeps `bundled_codex_path()`, document whether it returns an `ontocode` binary path during the compatibility window.

Rollback:

- Continue or restore old package identity as a compatibility package.

## Stage 5: Env, State, Config, And File Layout Migration

Status: pending.

Goal:

- Move user state/config/env naming toward `ONTOCODE_*` and Ontocode paths without data loss.

Scope:

- `CODEX_HOME` / `ONTOCODE_HOME`.
- persisted state directory names.
- logs, cache, rollout/session paths.
- config aliases and diagnostics.

Rules:

- Add `ONTOCODE_*` first.
- Read old `CODEX_*` during a deprecation window.
- Never remove old persisted-state readers without a migration and rollback test.
- Diagnostics must redact secrets and paths where required.

Required verification:

- config/home-dir tests
- state migration tests
- resume-session compatibility tests
- no-secret diagnostic assertions
- doctor/auth diagnostics tests that prove old and new names are reported without leaking secrets or private paths

Rollback:

- Continue reading old state and env names.

## Stage 6: Protocol, Generated SDK, And Telemetry Versioning

Status: pending.

Goal:

- Remove or version remaining `codex` names that are wire-visible, generated, or telemetry-visible.

Scope:

- app-server protocol schemas.
- generated Python and TypeScript protocol models.
- telemetry event shapes.
- MCP/resource identifiers.

Rules:

- Do not hand-edit generated output as a rename shortcut.
- Add versioned schema or compatibility aliases first.
- Coordinate downstream consumers before removing legacy names.
- Keep historical audit docs unchanged.
- Schema bundle filenames, generated class names, and SDK import paths require a versioned compatibility plan before rename.
- Telemetry event names require an analytics schema migration plan before rename.

Required verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol
```

If schema output changes:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just write-app-server-schema
```

Rollback:

- Keep old protocol version and generated aliases available.

## Stage 7: Final Legacy Removal

Status: pending.

Goal:

- Remove remaining compatibility-only `codex` names after release telemetry, support load, and downstream integrations confirm the migration is safe.

Removal candidates:

- `codex` command shims.
- `CODEX_*` env aliases.
- compatibility package names.
- generated compatibility aliases.
- legacy helper dispatch paths.
- docs that describe current commands as `codex`.

Preserve permanently:

- historical audit files and migration notes.
- references to upstream OpenAI Codex where historically accurate.
- legacy data readers if removal risks user data loss.

Go gate:

- No removal before at least one release train validates the new Ontocode surfaces.

## Tracking Template

Create `ONTOCODE_FULL_LEGACY_MIGRATION_TRACKING.md` with this table:

| ID | Stage | Scope | Status | Owner | OntoIndex Evidence | Verification | Rollback |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F0 | Approval and inventory | surface matrix | pending | manager | pending | doc review | no code changes |
| F1 | Workspace layout rename | `ontocode-rs/` -> `ontocode-rs/` | pending | TBD | pending | Cargo/Bazel/package smoke | move path back |
| F2 | CLI cutover | public command | pending | TBD | pending | CLI tests/build | restore `codex` shim |
| F3 | Helper cutover | runtime helpers | pending | TBD | pending | arg0/sandbox/exec tests | restore aliases |
| F4 | Package migration | npm/Python/SDK/native packages | pending | TBD | pending | package staging tests | restore old package |
| F5 | Env/state migration | config/state/log/cache | pending | TBD | pending | state/config/resume tests | keep old readers |
| F6 | Protocol/telemetry versioning | wire/generated/analytics | pending | TBD | pending | schema/protocol tests | keep old version |
| F7 | Final legacy removal | compatibility cleanup | pending | TBD | pending | full release smoke | restore aliases |

## Senior Challenge

- A complete legacy migration is not a single engineering task. It is a release program.
- `ontocode-rs/` can be renamed earlier than package/protocol surfaces, but only if build, Bazel, package scripts, and memory-bank paths are moved together.
- The `codex` command can be removed only after package/runtime install behavior is explicitly approved.
- Package identities such as `@openai/codex`, `openai-codex`, and `openai-codex-cli-bin` should not be hard-renamed without dual-publish or metapackage support.
- `CODEX_*` env and persisted-state compatibility should be retained longer than visible command aliases.
- Generated protocol and telemetry names require versioning, not cosmetic local edits.
- Stage 1 is valuable, but it is not enough to claim the project is "fully Ontocode"; it only removes a source-tree layout debt.
- Stage 4 and Stage 6 are release/versioning decisions and must not be delegated as ordinary refactor tasks.
- The project should prefer deletion of compatibility only after observability shows successful adoption of the new Ontocode surfaces.

## Recommended First Dispatch

1. F0: create and review the surface matrix.
2. F1: layout-only rename from `ontocode-rs/` to `ontocode-rs/`.
3. F2: CLI command cutover using `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`.
4. F3: helper binary cutover after CLI behavior is stable.
5. F4-F6: package/env/protocol migrations only after release-owner approval.
