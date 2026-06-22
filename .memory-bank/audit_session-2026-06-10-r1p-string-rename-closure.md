# R1P String Utility Rename Closure

Date: 2026-06-10

Status: worker verification complete; awaiting manager acceptance.

Change:
- Renamed `codex-utils-string` to `ontocode-utils-string` as an identity-only Cargo package, Rust crate import surface, and Bazel crate name change.
- Updated root workspace dependency key, direct dependent manifests/imports/usages, `ontocode-rs/Cargo.lock`, `MODULE.bazel.lock`, and active direct refs required for compilation.

Preserved surfaces:
- Telemetry sanitization, UTF-8 boundary handling, markdown hash parsing, UUID parsing, token estimate/truncation behavior, protocol shape, tool-output semantics, public command names, telemetry keys, persisted data, and env/config semantics.

Verification:
- Passed: `cargo metadata --format-version 1 --no-deps`.
- Passed: `CARGO_BUILD_JOBS=8 just fmt`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-string` (18 passed).
- Exact `CARGO_BUILD_JOBS=8 just test -p codex-context-fragments` compiled but exited 4 because the package has zero tests; passed with `--no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just test -p codex-core` completed with 2647 passed, 1 failed, 14 skipped; failed realtime test passed on focused rerun and is classified unrelated to the string rename.
- Passed: `codex-otel` (44), `codex-protocol` (231), `codex-tools` (80), `ontocode-tui` (2772, 4 skipped), `ontocode-windows-sandbox` (10), and `codex-utils-output-truncation` (17).
- Passed focused checks: core telemetry previews, otel service-name tag sanitization, Windows logging UTF-8 boundary availability check, and tool-output log-preview availability check.
- Passed: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- Passed: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Passed: stale-reference search for `codex-utils-string|codex_utils_string` under active `ontocode-rs` sources and lockfiles.
- Passed: `git diff --check`.
- Passed: scoped OntoIndex `gn_verify_diff`.
