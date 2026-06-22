# Claude Parked Row 188 Review

Date: 2026-06-20

## Decision

Row 188 stays parked.

## Source

- ADR row 188: `Partial | Non-core | DEFER | Release notes automation belongs in GitHub workflow.`
- Donor row 188: `Add Bun/Node runtime shim audit. | packaging / scripts | Useful if TypeScript tools stay. | Script tests.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-cli/bin/codex.js` already detects Bun versus npm, sets `CODEX_MANAGED_BY_BUN` or `CODEX_MANAGED_BY_NPM`, and uses package-manager-specific reinstall guidance for missing optional dependencies.
- `ontocode-rs/install-context/src/lib.rs` already models `InstallMethod::Npm` and `InstallMethod::Bun`, reads the managed-install env vars, preserves package layout for npm-managed shims, and tests npm/Bun precedence.
- `scripts/install/install.sh` and `scripts/install/install.ps1` already classify Bun/NPM managed conflicts and choose matching uninstall commands.
- Existing package staging and artifact workflow tests cover package layout behavior; no one concrete script-test gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
