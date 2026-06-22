# R2B-E Sandboxing Rename Closure

Date: 2026-06-10

## Scope

- Accepted `codex-sandboxing` -> `ontocode-sandboxing` as an identity-only package/lib/Bazel/import rename.
- Preserved sandbox policy semantics, permission-profile transforms, platform sandbox selection, Seatbelt argument generation, Landlock/Linux sandbox argument generation, bubblewrap lookup/warnings, helper argv0 compatibility, network policy, managed MITM CA handling, sandbox env vars, public command names, protocol/schema surfaces, telemetry, env/config semantics, runtime layout, persisted state, helper binary names, and compatibility aliases.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-sandboxing`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli debug_sandbox`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox`
- Focused sandbox manager, policy transforms, Landlock/Linux sandbox, bwrap warning/lookup, app-server command exec, core shell/unified exec, apply-patch sandbox safety, CLI debug sandbox, and exec-server sandboxed file-system coverage.
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-sandboxing`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale package/lib reference search for `codex-sandboxing` / `codex_sandboxing`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.

## Notes

- Seatbelt runtime tests are unavailable on this Linux host; local sandboxing test discovery listed no Seatbelt tests.
- Active source stale references for `codex-sandboxing` / `codex_sandboxing` are clean.
- R2B runtime path/package-layout stage is complete after R2B-E.
