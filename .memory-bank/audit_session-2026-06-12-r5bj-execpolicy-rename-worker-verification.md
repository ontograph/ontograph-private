# R5BJ Execpolicy Rename Worker Verification

Date: 2026-06-12

## Result

- Renamed `ontocode-execpolicy` to `ontocode-execpolicy` and `codex_execpolicy` to `ontocode_execpolicy`.
- Kept the existing `execpolicy` directory path and preserved policy parsing, prefix/network rule semantics, example validation, host executable lookup, policy merge/check behavior, amendment persistence, JSON output shape, CLI argument behavior, core exec-policy integration, config requirements-policy conversion, prompt permission text, env/config/wire/generated names, telemetry/product strings, and persisted state.
- Updated Cargo metadata, Bazel crate naming, README/doc references, and crate imports to the new identity.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-execpolicy --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --lib`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-config --lib`
- `CARGO_BUILD_JOBS=8 just test -p codex-protocol format_allow_prefixes_limits_output`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-prompts permissions_instructions`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n 'codex_execpolicy|ontocode-execpolicy' .`
- `cargo metadata --format-version 1 --no-deps --quiet | jq -r '.packages[].name' | rg '^codex-' | wc -l`
- `git diff --check`

## Notes

- Fallback model used: `gpt-5.4-mini`.
- Residual `codex-*` Cargo package count: 11.
