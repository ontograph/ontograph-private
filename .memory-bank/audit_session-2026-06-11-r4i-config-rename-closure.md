# R4I Config Rename Closure

Date: 2026-06-11

Scope:
- Accepted identity-only rename `codex-config` -> `ontocode-config`.
- Accepted crate/lib import rename `codex_config` -> `ontocode_config`.
- Preserved existing `config` folder path, config parsing/merge/default behavior, `ConfigToml` and nested config semantics, managed/project/user config precedence, feature flags, auth-store config, provider config, sandbox/permissions config, hooks/plugins/MCP config, env/config key names, generated schema/wire names, persisted state, and telemetry/product strings.

Manager verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n "\\bcodex_config\\b|\\bcodex-config\\b" ontocode-rs --glob "!target" || true`
- `git diff --check`
- OntoIndex CLI `detect-changes --repo codex` completed with broad dirty-tree medium-risk output; worker scoped MCP `gn_verify_diff` passed for the R4I file set.

Notes:
- Worker verification already covered fmt, CLI config, TUI config, provider-info, provider runtime, login, MCP config, Bazel lock update/check, stale-reference classification, and scoped OntoIndex verification.
- No active `codex_config` refs remain.
- Remaining `codex-config` refs are intentional test tempdir stems.
- R4 provider/auth/MCP support-crate stage is complete; R5 core/shared crates are unblocked but require fresh senior risk review before dispatch.
