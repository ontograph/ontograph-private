# Ontocode Hard Cutover Closure

Date: 2026-06-09

Scope: `.memory-bank/ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md` tracked HC0-HC11.

Outcome:

- HC0-HC11 are closed in `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER_TRACKING.md`.
- Hard mode remains selected: public installed `codex` command/helper aliases are removed; package identities and internal crate/package names remain frozen.
- HC10 package bin checks, package staging, sandbox/escalation focused tests, TUI full suite, scoped fixes, and OntoIndex scoped diff verification passed.
- HC11 rollback validation did not trigger rollback. Restore `codex` launcher/bin only if a future full package/install smoke fails.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p codex-linux-sandbox`
- `CARGO_BUILD_JOBS=8 just test -p codex-shell-escalation`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just fix -p codex-linux-sandbox`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-tui`
- npm package bin map/staging checks for `@openai/codex` with only `ontocode` installed bin.
- OntoIndex scoped `gn_verify_diff` passed; whole-worktree verification remains noisy because unrelated dirty files pre-existed.

Notes:

- Linux sandbox landlock tests now use the existing bwrap-prerequisite skip guard consistently when bwrap is unavailable.
- TUI thread-goal snapshot was updated for `ontocode`.
- IDE IPC test fixture now sets socket directory permissions to `0700` to satisfy the production socket safety contract.
