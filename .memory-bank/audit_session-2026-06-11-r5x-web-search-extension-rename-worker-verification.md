# R5X Web Search Extension Rename Worker Verification

Date: 2026-06-11

## Result

`codex-web-search-extension` -> `ontocode-web-search-extension` was implemented as an identity-only package/lib/Bazel/import rename in the scoped web-search extension and app-server wiring files.

## Verified

- Cargo package name.
- Rust lib crate name.
- Bazel crate identity.
- Root workspace dependency key.
- `ontocode-app-server` dependency/import wiring.

## Preserved

- Web-search tool namespace and tool names.
- `web_run` behavior.
- Search settings/config behavior.
- Model/provider capability behavior.
- Auth/provider behavior.
- Encrypted output behavior.
- Markdown description compile data.
- Tool schema behavior.
- Metrics behavior.
- App-server extension registration behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `ext/web-search` directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-web-search-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_web_search_extension|codex-web-search-extension`
- `cargo metadata --format-version 1 --no-deps` residual count: 49 remaining `codex-*` packages
- `git diff --check`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

## Notes

- Work completed on fallback `gpt-5.4-mini` after the requested Spark model hit its usage limit.
