# R5AU Code Mode Rename Worker Verification

- Date: 2026-06-12
- Model: `gpt-5.4-mini` fallback after Spark usage-limit fallback.
- Scope: `codex-code-mode` -> `ontocode-code-mode`, `codex_code_mode` -> `ontocode_code_mode`.
- Result: package/lib/Bazel/import identity renamed; the `code-mode` directory path and runtime sentinel `__codex_code_mode_exit__` were preserved.
- Verification:
  - `CARGO_BUILD_JOBS=8 just fmt`
  - `CARGO_BUILD_JOBS=8 just test -p ontocode-code-mode --no-tests=pass`
  - `CARGO_BUILD_JOBS=8 just test -p ontocode-core code_mode`
  - `CARGO_BUILD_JOBS=8 just test -p ontocode-core spec_plan`
  - `CARGO_BUILD_JOBS=8 just test -p codex-tools code_mode`
  - `CARGO_BUILD_JOBS=8 just test -p codex-rollout-trace code`
  - `CARGO_BUILD_JOBS=8 just bazel-lock-update`
  - `CARGO_BUILD_JOBS=8 just bazel-lock-check`
  - `cargo metadata --format-version 1 --no-deps | jq "[.packages[].name | select(startswith(\"codex-\"))] | length"` => `26`
  - `git diff --check`
  - `OntoIndex detect-changes --repo codex` => high-risk due unrelated dirty-tree changes
- Notes: the old-name search is clean except for the intentional runtime sentinel; the requested `ontocode-tools` package name is not present in this workspace, so the equivalent current-package check ran against `codex-tools`.
