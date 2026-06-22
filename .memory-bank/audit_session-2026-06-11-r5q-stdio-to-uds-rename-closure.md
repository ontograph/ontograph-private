# R5Q Stdio To UDS Rename Closure

Date: 2026-06-11

## Scope

- Accepted `ontocode-stdio-to-uds` -> `ontocode-stdio-to-uds`.
- Accepted `codex_stdio_to_uds` -> `ontocode_stdio_to_uds`.
- Identity-only package/lib/Bazel/import rename.
- Preserved stdio/UDS relay behavior, Unix socket transport behavior, CLI MCP proxy dispatch behavior, public `ontocode-stdio-to-uds` executable compatibility, README/MCP command examples, helper usage text, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `stdio-to-uds` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-stdio-to-uds --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active-source stale-reference classification for `codex_stdio_to_uds|ontocode-stdio-to-uds`: no `codex_stdio_to_uds` matches; remaining `ontocode-stdio-to-uds` matches are intentional public helper compatibility refs in `[[bin]]`, README, usage text, and tests.
- `git diff --check`: passed through lean-ctx wrapper.
- Cargo metadata reports 56 remaining `codex-*` workspace packages.
- OntoIndex CLI fallback `detect-changes --repo codex` still reports the known broad dirty-tree high-risk context, not a scoped R5Q-only blocker.

## Notes

- Worker verification completed on fallback model `gpt-5.4-mini` after the `gpt-5.3-codex-spark` usage-limit fallback.
- Known unrelated warnings remain: duplicate Windows sandbox bin targets and the existing `ontocode-core` dead-code warning for token-usage breakdown fields.
- R6 cleanup remains blocked while residual `codex-*` package identities remain.
