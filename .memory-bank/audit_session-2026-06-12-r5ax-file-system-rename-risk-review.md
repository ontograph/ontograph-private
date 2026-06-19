# R5AX File System Rename Risk Review

## Decision

Dispatch R5AX as an identity-only residual package rename:

- `codex-file-system` -> `ontocode-file-system`
- `codex_file_system` -> `ontocode_file_system`

## OntoIndex Impact

- `ExecutorFileSystem`: MEDIUM, 7 impacted implementors, no affected processes.
- `FileMetadata`: CRITICAL, 58 impacted nodes across filesystem/runtime metadata paths.
- `FileSystemSandboxContext`: ambiguous between struct and impl.

## Direct Inventory

- Root workspace metadata and file-system manifest/Bazel identity.
- Dependent manifests/imports in `config`, `exec-server`, and `git-utils`.

## Guardrails

- Preserve file read/write/copy/remove/create-dir/metadata/directory semantics.
- Preserve sandbox context conversion and permission profile/cwd handling.
- Preserve exec-server direct/sandboxed/remote filesystem behavior.
- Preserve config loader filesystem behavior.
- Preserve git-utils filesystem behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `file-system` directory path.

## Verification Required

- `CARGO_BUILD_JOBS=8 just test -p ontocode-file-system --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-config loader`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server file_system`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-git-utils --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_file_system|codex-file-system`
- Cargo metadata residual count, expected 23 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`
