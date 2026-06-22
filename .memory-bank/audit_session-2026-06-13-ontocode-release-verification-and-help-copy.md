---
name: Ontocode Release Verification And Help Copy
description: Clean release-profile ontocode binary verification plus user-visible CLI help rename cleanup
type: audit
date: 2026-06-13
status: complete
---

# Ontocode Release Verification And Help Copy

## Scope

- Verify the clean `release`-profile `ontocode` binary after the alias-entrypoint fix.
- Remove remaining user-visible `Codex` rename leaks from the main CLI help surface.
- Record why the binary still lives under `ontocode-rs/` paths.

## Outcome

- `cargo build --release -p ontocode-cli --bin ontocode` completed successfully in `24m 11s`.
- `./target/release/ontocode --version` returned `Ontocode CLI 0.0.0`.
- `./target/release/ontocode --help` rendered successfully.
- Main CLI help text now consistently uses `Ontocode` / `Ontocode Cloud` on the `ontocode` binary surface.
- `ontocode-rs/` remains the Rust workspace directory name and is now explicitly tracked as layout debt rather than a runtime blocker.

## Files Updated

- `ontocode-rs/cli/src/main.rs`
- `ontocode-rs/cloud-tasks/src/cli.rs`
- `.memory-bank/ALPHA_RELEASE_READINESS.md`
- `.memory-bank/project_pending-tasks.md`
- `.memory-bank/project_plan-current.md`

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli`
- `./target/debug/ontocode --help`

## Remaining Alpha Blockers

- Final alpha version choice, with `0.1.0-alpha.1` still the recorded default.
- `Claude OAuth` live validation sample.
- Optional post-alpha cleanup for duplicate-target Cargo warnings and `ontocode-rs/` filesystem layout migration.
