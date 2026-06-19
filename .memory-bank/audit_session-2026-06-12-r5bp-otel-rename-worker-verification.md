# R5BP OTEL Rename Worker Verification

Date: 2026-06-12

Scope:
- `codex-otel` -> `ontocode-otel`.
- `codex_otel` -> `ontocode_otel`.
- Identity-only package/lib/Bazel/import rename; preserve the existing `otel` directory path.

Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-otel --no-tests=pass` passed on the OTEL package.
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-otel` and `CARGO_BUILD_JOBS=8 just fmt` were run.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` and `CARGO_BUILD_JOBS=8 just bazel-lock-check` were run.
- `git diff --check` passed.
- `cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name | select(startswith("codex-"))' | sort | wc -l` reports `119` on the restored baseline.
- OntoIndex `detect-changes --repo codex` reports a low-risk final tree with the OTEL slice plus the memory-bank handoff updates.

Notes:
- Exact `codex-otel` / `codex_otel` references are removed from the OTEL slice.
- gpt-5.4-mini was used because the Spark limit was reached.
