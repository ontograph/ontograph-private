# Ontocode Legacy Lefties Todo

Purpose: track remaining `codex` leftovers after the folder/package/binary rename work. This is a cleanup queue, not permission to broad find-and-replace.

Last scan: 2026-06-15 from `/opt/demodb/_workfolder/ontocode`.
OntoIndex: fresh at `73ba3040e201390b3b6b0bc05f7d8d33e9c215b6`; dirty worktree count is high because the large rename is still unstaged.

## Rules

- Use OntoIndex impact before editing any symbol.
- Preserve compatibility where public APIs, persisted state, config keys, protocols, package names, or external scripts still depend on old names.
- Do not touch `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` or `CODEX_SANDBOX_ENV_VAR`.
- Do not raw replace `codex` across the repo.
- Refresh OntoIndex after each accepted implementation slice.

## P0: Stabilize The Rename Baseline

- [ ] Stage or otherwise reconcile the large `codex-rs/` -> `ontocode-rs/` and `codex-cli/` -> `ontocode-cli/` moves so `git status` can distinguish real leftovers from move noise.
- [ ] Remove generated temp leftovers under `tmp/` after confirming no active process needs them.
- [ ] Re-run a focused old-name scan after the worktree is readable:
  `rg -n --glob '!target/**' --glob '!.git/**' --glob '!.ontoindex/**' --glob '!node_modules/**' 'codex|Codex|CODEX|\\.codex'`.

## P1: Active Build And Packaging Names

- [ ] Rename `scripts/codex_package/` to an Ontocode package module, keeping import shims only if release tooling still imports the old module.
- [ ] Rename `scripts/build_codex_package.py` or replace it with an Ontocode-named wrapper; keep the old entrypoint only as a documented compatibility shim.
- [ ] Finish package helper variable cleanup such as `codex_command_runner_bin` and `codex_windows_sandbox_setup_bin` once the artifact names are final.
- [ ] Review `.github/`, `.devcontainer/`, and install packaging paths that still contain `codex` in file or directory names.

## P1: Runtime Home And State Compatibility

- [ ] Keep `ONTOCODE_HOME` as the active home and `CODEX_HOME` / `.codex` as fallback while migration is still open.
- [ ] Convert install scripts `scripts/install/install.sh` and `scripts/install/install.ps1` to Ontocode-first wording and paths without breaking existing users.
- [ ] Review runtime users of `find_codex_home()` in app-server daemon, network proxy, Windows sandbox setup, login/auth storage, state runtime, and tests.
- [ ] Add explicit resume/state compatibility verification before removing any `.codex` fallback.

## P1: Public CLI And NPM Compatibility

- [ ] Decide whether `ontocode-cli/bin/codex.js` remains a compatibility alias or moves behind an install-time compatibility option.
- [ ] Decide when `@openai/codex` package metadata can be renamed or whether it must stay as the upstream-compatible npm package name.
- [ ] Keep the final user command path focused on `ontocode`; old `codex` invocations should be compatibility-only and documented as such.

## P2: Internal Crate And Source Directory Names

- [ ] Rename remaining source directories with old names, including `ontocode-rs/codex-api`, `ontocode-rs/codex-client`, `ontocode-rs/codex-mcp`, `ontocode-rs/codex-backend-openapi-models`, and `ontocode-rs/codex-experimental-api-macros`.
- [ ] Rename Rust module/file names such as `codex_thread.rs`, `codex_tool.rs`, `codex_tool_runner.rs`, and `codex_tool_config.rs` only with impact checks.
- [ ] Rename test helpers like `test_codex` after production owner names settle.

## P2: Protocol And Wire Names

- [ ] Review `codex_error_info` before renaming; this is protocol-visible and needs compatibility or versioning.
- [ ] Review `ontocode-rs/exec-server/src/proto/codex.exec_server.relay.v1.proto`; proto package and generated names need a versioned migration plan.
- [ ] Rename docs such as `ontocode-rs/docs/codex_mcp_interface.md` only after matching protocol/API names are settled.

## P2: User-Visible Copy And Prompts

- [ ] Update install script messages that still say `Codex CLI`.
- [ ] Review model prompt files and `ontocode-rs/models-manager/models.json`; keep upstream model-family names like `gpt-5-codex` if they are actual model IDs.
- [ ] Update generated snapshots only after intentional UI/text changes, using the repo snapshot workflow.

## P3: Tests, Fixtures, And Historical References

- [ ] Rename test temp prefixes such as `codex-core-tests*` after the relevant crate/test helper rename lands.
- [ ] Clean fixture-only `.codex` references only when the tested compatibility behavior has an Ontocode equivalent.
- [ ] Leave historical memory-bank/audit references alone unless they mislead current dispatch; they are not runtime blockers.

## Done Criteria

- [ ] `find . -maxdepth 3 \( -name '*codex*' -o -name '.codex*' \)` returns only approved compatibility or historical paths.
- [ ] Active build/test/package commands use `ontocode` and `ontocode-*` names.
- [ ] `cargo metadata --manifest-path ontocode-rs/Cargo.toml --no-deps` shows no unintended `codex-*` package or binary targets.
- [ ] Compatibility shims are documented with owner, removal condition, and tests.
- [ ] `just fmt`, focused `just test -p <changed-crate>`, `git diff --check`, and `ontoindex analyze --skills --skip-agents-md` pass for each implementation slice.
