# R5BQ Extension API Rename Risk Review

Date: 2026-06-12

## Scope

- Rename `codex-extension-api` to `ontocode-extension-api`.
- Rename `codex_extension_api` to `ontocode_extension_api`.
- Keep the existing `ext/extension-api` directory path.

## OntoIndex

- `ContextContributor`: LOW, 3 direct implementers.
- `ToolContributor`: LOW, 7 impacted nodes.
- `ExtensionRegistry` and `ExtensionRegistryBuilder`: ambiguous between struct and impl symbols.
- `Extension`, `ExtensionProvider`, and `ExtensionOutput`: UNKNOWN/not found.

## Guardrails

- Do not change extension trait signatures.
- Do not change registry builder behavior.
- Do not change contributor lifecycle behavior.
- Do not change prompt/context/tool/event/response item injection behavior.
- Do not change public config, wire, generated/schema names, persisted state, telemetry/product strings, or directory paths.

## Verification Required

- Extension API compile or package test/no-tests pass.
- Built-in extension compile checks where imports changed.
- Core extension registry compile or focused checks.
- App-server/CLI/TUI compile if directly affected.
- `just fmt`.
- `just bazel-lock-update` and `just bazel-lock-check`.
- Stale-reference search.
- Cargo metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
