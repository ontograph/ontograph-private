# Ontocode-Only CLI Hard Cutover Impact Report

Date: 2026-06-09

Worker: HC2

Source ADR: `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`

Scope: Stage 0 OntoIndex blast-radius report only. No implementation code was edited.

## Gate Status

- Overall verdict: implementation may not proceed.
- Reason: the ADR remains proposed and the tracking file records missing product/release owner approval, unset removal mode, missing rollback owner/threshold, and incomplete package/runtime gate decisions.
- Required before implementation: close HC0-HC4 planning gates, select `soft-hard` or `hard`, approve package identity scope, approve rollback threshold, and re-run verification on the implementation diff.

## OntoIndex Evidence Status

- Repo confirmation: OntoIndex `codex` maps to `/opt/demodb/_workfolder/ontocode`.
- Evidence quality: partial.
- Limitations: `gn_diagnose` reports index staleness against current HEAD and target-context ambiguity despite explicit `repo=codex`; enrichment/sidecar store is unavailable.
- Graph snapshot evidence from impact calls reports `repoPath: /opt/demodb/_workfolder/ontocode` and storage path `.ontoindex`.
- Direct source search/read evidence was used for unresolved/disambiguated symbols and packaging/runtime surfaces.

## Symbol Impact Summary

| Target | Resolution | Risk | Impact Summary |
| --- | --- | --- | --- |
| `command_name_from_arg0` | `ontocode-rs/cli/src/main.rs:910` | CRITICAL | 26 impacted nodes, 1 affected process, 6 affected modules; direct callers include `current_command_name` and `try_parse_multitool_cli_from`; affected `cli_main`, completion/help/parser tests. |
| `current_command_name` | `ontocode-rs/cli/src/main.rs:927` | HIGH | 3 impacted nodes, 1 affected process, 3 modules; direct callers are `cli_main` and `print_completion`. |
| `arg0_dispatch` | `ontocode-rs/arg0/src/lib.rs:59` | HIGH | 10 impacted nodes, 4 modules; direct callers include `configure_test_binary_dispatch` and `arg0_dispatch_or_else`; downstream binary entrypoints include CLI, TUI, exec, app-server, mcp-server, thread-manager sample, core/exec-server tests. |
| `prepare_path_entry_for_codex_aliases` | `ontocode-rs/arg0/src/lib.rs:323` | LOW | OntoIndex upstream impact returned 0 impacted nodes, but direct inspect shows helper alias path construction, `Arg0DispatchPaths`, cleanup, and PATH mutation dependencies. Treat as partial due stale index. |
| `transform_linux_seccomp_request` | `ontocode-rs/sandboxing/src/manager_tests.rs:289` | LOW | Test helper only; 2 impacted Linux sandbox tests: `transform_linux_seccomp_preserves_helper_path_in_arg0_when_available` and `transform_linux_seccomp_uses_helper_alias_when_launcher_is_not_helper_path`. |
| `run_main_with_arg0_guard` | `ontocode-rs/arg0/src/lib.rs:235` | HIGH | 7 impacted nodes, 3 modules; direct caller `arg0_dispatch_or_else`; affected main entrypoints include thread-manager sample, mcp-server, CLI, TUI, exec, app-server. |
| `ExecServerRuntimePaths::new` | Qualified target unresolved; disambiguated as `new` in `ontocode-rs/exec-server/src/runtime_paths.rs:28` | CRITICAL | 17 impacted nodes, 5 affected processes, 9 modules; affected processes include app-server `run_main_with_transport_options`, exec `run_main`, thread-manager sample `run_main`, TUI `run_main`, and CLI `cli_main`. |

## High-Risk Findings

- CRITICAL: CLI command identity is not isolated to help text; `command_name_from_arg0` feeds parsing, completion, resume/fork/archive/profile/help paths, and plugin marketplace help tests.
- HIGH: `arg0_dispatch` is shared startup plumbing for CLI, TUI, exec, app-server, mcp-server, thread-manager sample, and test binary support; removing helper names can break process startup before command-specific tests run.
- HIGH: `run_main_with_arg0_guard` owns lifetime and propagation of `Arg0DispatchPaths`; helper aliases must remain alive for async main execution and child re-exec paths.
- CRITICAL: `ExecServerRuntimePaths::new` is transitively used by app-server, exec, TUI, CLI, core connector/prompt paths, filesystem sandbox tests, and runtime path fixtures; internal field renames must remain quarantined.
- Direct source evidence expands risk beyond graph results: npm, Python runtime setup, SDK artifact generation, app-server daemon/test-client, Linux sandbox argv0 handling, and release archive naming still expose or depend on `codex` names.

## Direct Source Evidence

- `ontocode-rs/cli/src/main.rs` defines `PRIMARY_COMMAND_NAME = "codex"`, `ALIAS_COMMAND_NAME = "ontocode"`, `ONTOCODE_CLI_COMMAND_NAME`, and command-name selection used by parser/help/completion.
- `ontocode-rs/arg0/src/lib.rs` dispatches both `ontocode-execve-wrapper`/`ontocode-execve-wrapper` and `codex-linux-sandbox`/`ontocode-linux-sandbox`, creates both helper aliases, and still stores paths in `codex_*` fields.
- `ontocode-rs/sandboxing/src/manager.rs` still reports `missing codex-linux-sandbox executable path` and computes Linux seccomp argv0 override from the helper executable path.
- `ontocode-rs/exec-server/src/runtime_paths.rs` exposes `ExecServerRuntimePaths { codex_self_exe, codex_linux_sandbox_exe }` and validates absolute runtime paths.
- `codex-cli/package.json` publishes both `codex` and `ontocode` bin entries pointing at `bin/codex.js`.
- `codex-cli/scripts/build_npm_package.py` stages package `codex`, copies `bin/codex.js`, uses `codex-package` native components, and defines platform packages such as `@openai/codex-linux-x64`.
- `sdk/python/_runtime_setup.py` uses package names `openai-codex-cli-bin` and `openai-codex`, release assets named `codex-package-*`, and validates `codex_cli_bin.bundled_codex_path()`.
- `sdk/python/scripts/update_sdk_artifacts.py` expects runtime binary name `codex`/`codex.exe`, imports `bundled_codex_path`, and emits/loads `codex_app_server_protocol.v2.schemas.json`.
- `sdk/python/pyproject.toml` keeps distribution identity `openai-codex` and dependency pin `openai-codex-cli-bin==0.137.0a4`.

## Impacted Files, Modules, And Processes

- CLI command identity: `ontocode-rs/cli/src/main.rs`; process `cli_main`; parser/help/completion/plugin marketplace paths.
- Arg0 helper dispatch: `ontocode-rs/arg0/src/lib.rs`; startup paths for CLI, TUI, exec, app-server, mcp-server, thread-manager sample, test-binary support.
- Runtime path propagation: `ontocode-rs/exec-server/src/runtime_paths.rs`, `ontocode-rs/exec-server/src/environment.rs`, `ontocode-rs/exec-server/src/fs_sandbox.rs`, `ontocode-rs/app-server/src/lib.rs`, `ontocode-rs/exec/src/lib.rs`, `ontocode-rs/tui/src/lib.rs`, `ontocode-rs/core/src/connectors.rs`, `ontocode-rs/core/src/prompt_debug.rs`.
- Linux sandbox argv0 behavior: `ontocode-rs/sandboxing/src/manager.rs`, `ontocode-rs/sandboxing/src/manager_tests.rs`, `ontocode-rs/linux-sandbox/src/linux_run_main.rs`.
- Package/runtime install surfaces: `codex-cli/package.json`, `codex-cli/scripts/build_npm_package.py`, `sdk/python/_runtime_setup.py`, `sdk/python/scripts/update_sdk_artifacts.py`, `sdk/python/pyproject.toml`, native package archive layout.
- App-server/client surfaces found by source search: `ontocode-rs/app-server-daemon/src/lib.rs`, `ontocode-rs/app-server-test-client/src/lib.rs`, app-server tests and schema fixtures.

## Unresolved Or Disambiguated Symbols

- Unresolved exact target: `ExecServerRuntimePaths::new`.
- Attempted disambiguation: OntoIndex `inspect` and `impact` with `name=new`, `file_path=ontocode-rs/exec-server/src/runtime_paths.rs`, `kind=Function`.
- Resolved fallback: `Function:ontocode-rs/exec-server/src/runtime_paths.rs:ExecServerRuntimePaths.new`.
- Risk after disambiguation: CRITICAL.
- Source search evidence: `ExecServerRuntimePaths` appears in app-server-client, app-server, CLI, core-api, core connectors/environment/prompt debug/thread-manager tests, core test harness, exec-server environment/fs_sandbox/local filesystem/remote, and exec-server README.

## Required Focused Tests Before Any Implementation Merge

- CLI identity: `CARGO_BUILD_JOBS=8 just test -p codex-cli ontocode_alias`.
- CLI identity additions/updates: `ontocode_binary_is_primary_help_name` and `codex_binary_is_absent_or_errors_with_migration_message`.
- Arg0/helper dispatch: `CARGO_BUILD_JOBS=8 just test -p codex-arg0`.
- Shell helper: `CARGO_BUILD_JOBS=8 just test -p codex-shell-escalation`.
- Linux sandbox argv0: `CARGO_BUILD_JOBS=8 just test -p codex-linux-sandbox` where applicable, plus sandboxing manager tests covering helper alias fallback.
- Windows helper manifests: `CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox` where applicable.
- Exec-server runtime paths: focused exec-server filesystem sandbox/runtime path tests.
- App-server/TUI smoke: app-server startup tests and TUI paths that consume `Arg0DispatchPaths`/`ExecServerRuntimePaths`.
- Packaging: npm staged package bin-map validation, platform optional dependency validation, Python runtime wheel path API validation, `sdk/python/scripts/update_sdk_artifacts.py` against the pinned runtime, and release asset lookup validation.

## Proceed / Stop Decision

- Implementation may not proceed unless gates are satisfied.
- Required gates: explicit breaking-change approval, selected removal mode, package identity scope freeze, rollback owner/threshold, package/runtime compatibility decision for `bundled_codex_path()` and `codex-package-*`, and fresh/accepted OntoIndex verification or an approved fallback verification plan.
- If implementation is later approved, use `soft-hard` first unless external automation breakage is explicitly accepted.
