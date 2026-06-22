# R1F Image Internal Crate Rename Closure

Date: 2026-06-10

## Scope

Approved high-risk identity-only rename:

- `codex-utils-image` -> `ontocode-utils-image`
- Root workspace dependency key
- `utils/image` package/lib/Bazel crate name
- Direct imports in `codex-core`, `codex-protocol`, and the image benchmark
- Cargo/Bazel lockfiles

## Impact

- OntoIndex `load_for_prompt_bytes`: HIGH, 19 impacted nodes, 9 direct callers/tests.
- `PromptImageMode`: LOW.
- `ImageProcessingError`: LOW after UID disambiguation.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-image`
- `CARGO_BUILD_JOBS=8 just test -p codex-protocol`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`

## Notes

- `cargo generate-lockfile` initially advanced `allocative` to `0.3.6`, which broke `starlark_map 0.13.0`.
- `allocative 0.3.5` was compatible but yanked, so the final lockfile uses non-yanked `allocative 0.3.4`.
- `codex-core` passed with 2648 tests, 14 skipped, and one flaky retry that passed on try 2.
