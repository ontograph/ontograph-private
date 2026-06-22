# R5W Image Generation Extension Rename Risk Review

Date: 2026-06-11

## Decision

Approve `codex-image-generation-extension` -> `ontocode-image-generation-extension` as the next residual identity-only package rename slice.

## Direct Inventory

- Reverse dependents: 1 direct dependent, `ontocode-app-server`.
- Active refs before rename: 6.
- Ref scope: root workspace metadata, app-server dependency/import wiring, image-generation extension manifest identity, and Bazel crate identity.

## OntoIndex Impact

- Target: `Function:ontocode-rs/ext/image-generation/src/extension.rs:install`
- Risk: LOW.
- Impacted nodes: 0.
- Affected processes: 0.
- Repo path: `/opt/demodb/_workfolder/ontocode`.

## Allowed Changes

- Cargo package name.
- Rust lib crate name.
- Bazel crate identity.
- Root workspace dependency key.
- App-server dependency/import wiring.

## Must Preserve

- Image-generation tool namespace and tool names.
- Image request/response behavior.
- Model/provider selection behavior.
- Auth/provider behavior.
- Markdown description compile data.
- Tool schema behavior.
- Metrics behavior.
- App-server extension registration behavior.
- Env/config/wire/generated names.
- Telemetry/product strings.
- Persisted state.
- Existing `ext/image-generation` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-image-generation-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_image_generation_extension|codex-image-generation-extension`
- Cargo metadata residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`
