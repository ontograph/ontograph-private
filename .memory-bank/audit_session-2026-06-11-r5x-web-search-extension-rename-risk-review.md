# R5X Web Search Extension Rename Risk Review

Date: 2026-06-11

## Decision

Approve `codex-web-search-extension` -> `ontocode-web-search-extension` as the next residual identity-only package rename slice.

## Direct Inventory

- Reverse dependents: 1 direct dependent, `ontocode-app-server`.
- Active refs before rename: 6.
- Ref scope: root workspace metadata, app-server dependency/import wiring, web-search extension manifest identity, and Bazel crate identity.

## OntoIndex Impact

- Target: `Function:ontocode-rs/ext/web-search/src/extension.rs:install`
- Risk: LOW.
- Impacted nodes: 1 same-crate test caller.
- Affected processes: 0.
- Affected modules: `Tests`.
- Repo path: `/opt/demodb/_workfolder/ontocode`.

## Allowed Changes

- Cargo package name.
- Rust lib crate name.
- Bazel crate identity.
- Root workspace dependency key.
- App-server dependency/import wiring.

## Must Preserve

- Web-search tool namespace and tool names.
- `web_run` behavior.
- Search settings/config behavior.
- Model/provider capability behavior.
- Auth/provider behavior.
- Encrypted output behavior.
- Markdown description compile data.
- Tool schema behavior.
- Metrics behavior.
- App-server extension registration behavior.
- Env/config/wire/generated names.
- Telemetry/product strings.
- Persisted state.
- Existing `ext/web-search` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-web-search-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_web_search_extension|codex-web-search-extension`
- Cargo metadata residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`
