# R5AP Apply Patch Rename Worker Verification

Date: 2026-06-12

Identity-only `codex-apply-patch` -> `ontocode-apply-patch` and `codex_apply_patch` -> `ontocode_apply_patch` rename verification completed on fallback `gpt-5.4-mini`.

Preserved the public `apply_patch` binary/tool/protocol names, the `--codex-run-as-apply-patch` compatibility argument, patch parsing/application behavior, runtime routing, sandboxing, shell interception, and the existing `ontocode-rs/apply-patch` directory path.

Verified `CARGO_BUILD_JOBS=8 just test -p ontocode-apply-patch --no-tests=pass`, `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`, `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0 --no-tests=pass`, `CARGO_BUILD_JOBS=8 just test -p ontocode-exec --no-tests=pass`, `CARGO_BUILD_JOBS=8 just fmt`, `CARGO_BUILD_JOBS=8 just bazel-lock-update`, `CARGO_BUILD_JOBS=8 just bazel-lock-check`, active-source stale-reference search, `cargo metadata --format-version 1 --no-deps`, `git diff --check`, and OntoIndex `detect-changes --repo codex`.
