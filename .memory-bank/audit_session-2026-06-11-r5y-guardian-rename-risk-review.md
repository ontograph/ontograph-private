# R5Y Guardian Rename Risk Review

Date: 2026-06-11

## Decision

Approve `codex-guardian` -> `ontocode-guardian` as the next residual identity-only package rename slice.

## Direct Inventory

- Reverse dependents: 1 direct dependent, `ontocode-app-server`.
- Active refs before rename: 8.
- Ref scope: root workspace metadata, app-server dependency/import wiring, guardian extension manifest identity, Bazel crate identity, and analytics compatibility event strings.

## OntoIndex Impact

- Target: `Function:ontocode-rs/ext/guardian/src/lib.rs:install`
- Risk: LOW.
- Impacted nodes: 0.
- Affected processes: 0.
- Target: `Struct:ontocode-rs/ext/guardian/src/lib.rs:GuardianExtension`
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

- Guardian review behavior.
- Guardian subagent spawn and fork context behavior.
- Thread lifecycle contribution behavior.
- App-server extension registration behavior.
- Extension API and protocol semantics.
- `codex_guardian_review` analytics event strings.
- Env/config/wire/generated names.
- Telemetry/product strings.
- Persisted state.
- Existing `ext/guardian` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-guardian --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference classification for `codex_guardian|codex-guardian`
- Cargo metadata residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`
