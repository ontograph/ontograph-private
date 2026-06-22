# R5BO Features Rename Closure

Date: 2026-06-12

Result:
- Accepted `codex-features` -> `ontocode-features`.
- Accepted `codex_features` -> `ontocode_features`.
- Identity-only package/lib/Bazel/import rename is complete.
- Residual `codex-*` Cargo package count is 6.

Verification:
- Worker passed `CARGO_BUILD_JOBS=8 just test -p ontocode-features --no-tests=pass`.
- Worker passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`.
- Worker passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`.
- Worker passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`.
- Worker passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`.
- Worker passed `CARGO_BUILD_JOBS=8 just fmt`.
- Worker passed `CARGO_BUILD_JOBS=8 just fix -p ontocode-features`.
- Worker passed `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- Worker passed `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Manager confirmed `git diff --check` is clean.
- Manager confirmed Cargo metadata now lists 6 remaining `codex-*` packages.
- Manager confirmed active old refs are clean outside memory-bank historical tracking.
- OntoIndex `detect-changes --repo codex` reports the known broad high-risk dirty tree.

Preserved:
- Feature keys.
- Default/stage semantics.
- Dependency normalization.
- Legacy alias handling.
- Metrics emission.
- TOML serialization/materialization.
- Unstable warning behavior.
- App-server/client initialization behavior.
- CLI unknown-feature diagnostics.
- Network-proxy feature config mapping.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `features` directory path.

Notes:
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
