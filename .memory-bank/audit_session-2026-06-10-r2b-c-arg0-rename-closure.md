# R2B-C Arg0 Rename Closure

Date: 2026-06-10

## Scope

- Accepted `codex-arg0` -> `ontocode-arg0` as an identity-only package/lib/Bazel/import rename.
- Preserved public binaries, `argv[0]` helper aliases, runtime helper names, package-layout behavior, startup dispatch semantics, dotenv filtering, shell escalation dispatch, apply-patch aliases, Linux sandbox helper dispatch, telemetry, env/config semantics, protocol/schema, and persisted state.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-api` then `--no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-test-binary-support` then `--no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`
- Focused startup/helper alias tests for dispatch, package path preservation, apply-patch alias, shell escalation dispatch, and Linux sandbox alias compatibility.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale package/lib reference search for `codex-arg0` / `codex_arg0`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.

## Notes

- The only active `codex-arg0` source hit is the preserved runtime helper temp-dir prefix.
- `codex_arg0` has no active source references.
- Next R2B candidate must be selected with fresh OntoIndex impact and risk review before dispatch.
