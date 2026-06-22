---
name: Ontocode Binary Launcher Fix
description: Replace the fragile ontocode wrapper with the real CLI entrypoint and keep alias coverage without duplicate binary-unit test failures
type: audit
date: 2026-06-13
status: active
---

# Ontocode Binary Launcher Fix

## Problem

- A standalone built `target/release/ontocode` failed immediately because `cli/src/bin/ontocode.rs` was only a thin launcher that expected a sibling `target/release/codex` binary.
- This blocked alpha-release verification for the canonical `ontocode` binary.

## Change

- `ontocode-rs/cli/Cargo.toml`
  - `[[bin]] name = "ontocode"` now points to `src/main.rs`.
  - `test = false` is set on the alias bin target so the shared binary-local test module only runs once.
- Removed `ontocode-rs/cli/src/bin/ontocode.rs`.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli`

## Remaining Follow-Up

- Complete fresh artifact verification for the clean `ontocode` binary build.
- Optionally reduce the remaining Cargo duplicate-target warnings across renamed helper binaries before the alpha cut.
