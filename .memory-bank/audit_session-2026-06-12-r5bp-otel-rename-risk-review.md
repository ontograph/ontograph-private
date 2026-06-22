# R5BP OTEL Rename Risk Review

Date: 2026-06-12

Scope:
- `codex-otel` -> `ontocode-otel`.
- `codex_otel` -> `ontocode_otel`.
- Identity-only package/lib/Bazel/import rename; preserve the existing `otel` directory path.

OntoIndex:
- Repo path verified through the OntoIndex CLI as `/opt/demodb/_workfolder/ontocode`.
- `start_global_timer`: CRITICAL, 25 impacted nodes, 3 direct, 7 modules.
- `record_process_start_once`: CRITICAL, 9 impacted nodes, 1 direct, 8 modules, affected exec/app-server/TUI/CLI entrypoints.
- `current_span_w3c_trace_context`: CRITICAL, 48 impacted nodes, 8 direct, 10 modules.
- `bounded_originator_tag_value`: CRITICAL, 9 impacted nodes, 2 direct, 5 modules, affected exec/app-server/TUI entrypoints.
- `OtelSettings`: HIGH, 7 impacted nodes, 3 direct, 3 modules.
- `global_statsig_metrics_settings`: HIGH, 4 impacted nodes, 2 direct, 3 modules, affected windows sandbox permission-profile capture.
- `ToolDecisionSource`, `TelemetryAuthMode`, and `validate_span_attributes`: LOW.
- `SessionTelemetry`, `MetricsClient`, `MetricsConfig`, `OtelProvider`, and `RuntimeMetricsSummary`: UNKNOWN due ambiguous symbol matches.

Guardrails:
- Do not change OTEL config parsing/validation, exporter routing, auth mode mapping, session telemetry events/tags, metrics client behavior, runtime metrics summaries, timers, process-start metrics, originator tag bounding, trace-context propagation, provider filters/layers/shutdown, Statsig settings lookup, env/config/wire/generated names, telemetry/product strings, persisted state, or folder path.

Verification required:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-otel --no-tests=pass`.
- Focused metrics/session telemetry/trace-context/provider/config tests.
- Compile or focused checks for core, CLI, app-server, TUI, exec, model-provider/analytics/features if directly affected.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Stale-reference search for `codex-otel|codex_otel`.
- Cargo metadata residual count, expected 5 remaining `codex-*` packages after success.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`; known broad dirty-tree risk may remain outside this scoped rename.
