# R5O Bwrap Rename Closure

Date: 2026-06-11

## Result

- Accepted `codex-bwrap` -> `ontocode-bwrap`.
- Accepted `codex_bwrap` -> `ontocode_bwrap`.
- Preserved the `bwrap` binary name, vendored bubblewrap/libcap FFI behavior, `bwrap_main` symbol mapping, build-script availability detection, Bazel `bwrap-ffi` target wiring, sandbox/helper behavior, env/config/wire/generated names, telemetry/product strings, persisted state, public commands, runtime package layout, and the existing `bwrap` directory path.
- Preserved linux-sandbox synthetic mount-target compatibility strings `codex-bwrap-synthetic-mount-targets-{effective_uid}`.

## Environment Unblock

- Worker hit an environment blocker because `pkg-config` could not find `libcap`.
- Manager installed `libcap-dev`; `pkg-config --modversion libcap` now reports `2.44`.
- The blocked `ontocode-bwrap` package build then passed.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-bwrap --no-tests=pass`: passed, 0 tests plus bench smoke.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox --no-tests=pass`: passed, 116 tests.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active-source stale-reference search for `codex_bwrap|codex-bwrap` in `ontocode-rs`: only the two intentionally preserved synthetic mount-target compatibility strings remain.
- `git diff --check`: passed.
- Cargo metadata reports 58 remaining `codex-*` workspace packages.
- OntoIndex CLI fallback `detect-changes --repo codex`: completed with the known broad dirty-tree high-risk context, not an R5O-specific blocker.

## Model Fallback

- R5O worker ran on `gpt-5.4` because the prior residual slice established `gpt-5.3-codex-spark` usage limit and `gpt-5.4-mini` capacity constraints.
