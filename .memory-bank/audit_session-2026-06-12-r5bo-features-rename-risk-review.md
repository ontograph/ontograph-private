# R5BO Features Rename Risk Review

Date: 2026-06-12

Scope:
- `codex-features` -> `ontocode-features`.
- `codex_features` -> `ontocode_features`.
- Identity-only package/lib/Bazel/import rename; preserve the existing `features` directory path.

OntoIndex:
- Repo path verified through the OntoIndex CLI as `/opt/demodb/_workfolder/ontocode`.
- `FeatureConfigSource`: CRITICAL, 146 impacted nodes, 10 direct, 11 modules.
- `is_known_feature_key`: CRITICAL, 14 impacted nodes, 2 direct, 6 modules, affected `cli_main`.
- `FeatureConfig`, `FeatureSpec`, `canonical_feature_for_key`, `unstable_features_warning_event`, `legacy_feature_keys`, `MultiAgentV2ConfigToml`, and `NetworkProxyConfigToml`: LOW.
- `Feature`, `Features`, `FeatureOverrides`, `FeaturesToml`, `FeatureToml`, and `feature_for_key`: UNKNOWN due ambiguous symbol matches.

Guardrails:
- Do not change feature keys, default/stage semantics, dependency normalization, legacy alias handling, metrics emission, TOML serialization/materialization, unstable warning behavior, app-server/client initialization behavior, CLI unknown-feature diagnostics, network-proxy feature config mapping, env/config/wire/generated names, telemetry/product strings, persisted state, or folder path.

Verification required:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-features --no-tests=pass`.
- Focused feature config/legacy/warning tests.
- Focused core config/managed-feature checks or `cargo check -p ontocode-core --tests`.
- Focused CLI feature diagnostics checks or compile.
- App-server initialization/config feature checks or compile.
- TUI feature UI/gating compile if directly affected.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Stale-reference search for `codex-features|codex_features`.
- Cargo metadata residual count, expected 6 remaining `codex-*` packages after success.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`; known broad dirty-tree risk may remain outside this scoped rename.
