---
name: Ontocode Dev Binary Verification
description: Fresh source-built ontocode binary now runs directly with correct Ontocode branding after the launcher fix
type: audit
date: 2026-06-13
status: active
---

# Ontocode Dev Binary Verification

## Outcome

- Built `ontocode` from source in the `dev` profile after the launcher fix.
- Verified:
  - `./target/debug/ontocode --version`
  - `./target/debug/ontocode --help`
- The binary now runs directly instead of attempting to exec a missing sibling `codex` binary.

## Observed Output

- Version line: `Ontocode CLI 0.0.0`
- Help banner and usage use `ontocode` consistently.

## Remaining Follow-Up

- Complete the same verification for a clean `--release` artifact.
- Keep duplicate-target Cargo warnings and unrelated dead-code warnings documented as non-blocking alpha debt unless a dedicated cleanup slice is approved.
