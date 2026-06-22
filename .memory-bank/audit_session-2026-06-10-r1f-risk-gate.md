# R1F Internal Crate Rename Risk Gate

Date: 2026-06-10

## Scope

Senior unblock review for the next internal crate/package rename slice after accepted R1C-R1E.

## Findings

- `codex-utils-oss` is not an automatic slice: OntoIndex reports CRITICAL impact for `get_default_model_for_oss_provider` and `ensure_oss_provider_ready` through CLI/TUI/exec startup flows.
- `codex-utils-sandbox-summary` is not an automatic slice: OntoIndex reports CRITICAL/HIGH impact for sandbox summary functions through TUI status surfaces and related MCP/status display paths.
- `codex-utils-plugins` is not an automatic slice: `find_plugin_manifest_path` reports CRITICAL impact through plugin discovery, install, marketplace, app-server, and CLI surfaces.
- The remaining R1 work is not blocked by missing information; it is blocked by explicit high-risk approval policy.

## Decision

- Keep Rust code unchanged in this unblock pass.
- Move R1F to an explicit high-risk slice approval gate.
- Recommend `codex-utils-image` as the next first high-risk identity-only slice because its direct textual scope is bounded compared with runtime, plugin, path, rustls, and string utility crates.

## Required Next-Slice Contract

- Update tracking before dispatch.
- Rename only package/lib/Bazel/import references; do not change behavior.
- Run expanded tests for the renamed package and all direct dependents.
- Run OntoIndex verification before accepting the slice.
