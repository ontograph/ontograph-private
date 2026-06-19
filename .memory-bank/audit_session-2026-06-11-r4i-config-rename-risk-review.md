# R4I Config Rename Risk Review

Date: 2026-06-11

Candidate:
- `codex-config` -> `ontocode-config`
- `codex_config` -> `ontocode_config`

OntoIndex evidence:
- Tool: OntoIndex CLI against repo `codex`.
- Indexed repo path: `/opt/demodb/_workfolder/ontocode`.
- `Struct:ontocode-rs/config/src/config_toml.rs:ConfigToml`: CRITICAL impact.
- Impact count: 82 nodes, 71 direct.
- Affected modules: `Config`, `Bottom_pane`, `Loader`, `Unified_exec`.

Direct inventory:
- `codex-config` package refs under `ontocode-rs`: 51.
- `codex_config` crate refs under `ontocode-rs`: 1073.
- This is the broadest R4 support crate and must remain identity-only.

Approved scope:
- Identity-only Cargo package/lib/Bazel/import rename.
- Preserve existing `config` folder path.
- Preserve config parsing, merging, defaults, and precedence.
- Preserve `ConfigToml` and nested config semantics.
- Preserve managed/project/user config behavior.
- Preserve feature flags, auth-store config, provider config, sandbox/permissions config, hooks/plugins/MCP config.
- Preserve env/config key names, generated schema/wire names, persisted state, telemetry/product strings.

Rejected scope:
- No config behavior changes.
- No `ConfigToml` or nested config shape changes.
- No config schema regeneration unless an existing generated artifact changes only because crate identity is referenced.
- No config key, env var, persisted-state, public command, or generated-wire rename.
- No broad find-and-replace outside active package/lib/Bazel/import references.

Required verification:
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider-info --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for active `codex_config` / `codex-config` refs under `ontocode-rs`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff` or CLI `detect-changes`.
