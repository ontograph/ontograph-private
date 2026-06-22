# Claude Parked Row 187 Review

Date: 2026-06-20

## Decision

Row 187 stays parked for broad implementation, with one senior-approved package-builder fix applied.

## Source

- ADR row 187: `Partial | Non-core | DEFER | Golden prompts/evals are test assets, not core behavior.`
- Donor row 187: `Add build-bundle script with shims. | packaging | Helps binary/package builds. | Package smoke.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `scripts/build_codex_package.py` and `scripts/codex_package/` already own the package builder.
- `.github/scripts/build-codex-package-archive.sh` already wraps package archive creation for release workflows.
- CI already runs `python3 -m unittest discover -s scripts/codex_package -p 'test_*.py'`.
- OntoIndex impact for `source_binaries_for_target` was MEDIUM and contained to package-builder code, tests, and CLI entrypoint.
- The concrete defect was duplicate Windows helper binary entries in `source_binaries_for_target`, while the existing smoke test expects each helper once.

## Outcome

Senior removed the duplicate `ontocode-command-runner` and `ontocode-windows-sandbox-setup` entries from `scripts/codex_package/cargo.py`.

Validation:

- `python3 -m unittest discover -s scripts/codex_package -p 'test_*.py'`
- `CARGO_BUILD_JOBS=8 just fmt` from `ontocode-rs`
