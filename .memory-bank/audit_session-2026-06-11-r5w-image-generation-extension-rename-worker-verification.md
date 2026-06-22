# R5W Image Generation Extension Rename Worker Verification

Date: 2026-06-11

## Summary

- Verified the identity-only rename from `codex-image-generation-extension` / `codex_image_generation_extension` to `ontocode-image-generation-extension` / `ontocode_image_generation_extension`.
- Preserved image-generation tool namespace and tool names, request/response behavior, provider selection behavior, auth/provider behavior, markdown description compile data, tool schema behavior, metrics behavior, and app-server extension registration behavior.
- Ran on fallback `gpt-5.4-mini` after the requested Spark model hit its usage limit.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-image-generation-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_image_generation_extension|codex-image-generation-extension`
- `cargo metadata --format-version 1 --no-deps` residual count: 50 `codex-*` packages
- `git diff --check`
- `detect-changes --repo codex`

## Notes

- OntoIndex `detect-changes` reported the known repository-wide dirty-tree high-risk context; it did not block the scoped rename verification.
