# R5AE Responses API Proxy Rename Closure

Date: 2026-06-11

## Scope

- Accepted `ontocode-responses-api-proxy` -> `ontocode-responses-api-proxy`.
- Accepted `codex_responses_api_proxy` -> `ontocode_responses_api_proxy`.
- Scope was identity-only Cargo package/lib/Bazel/import rename.

## Preserved Surfaces

- API-key stdin handling.
- Request forwarding and auth header construction.
- Dump behavior, server-info behavior, HTTP shutdown behavior, and process hardening.
- Public `ontocode-responses-api-proxy` binary name.
- npm package/bin names and README/config examples.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and folder path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-responses-api-proxy --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_responses_api_proxy|ontocode-responses-api-proxy`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5AE.
- Active old refs are clean except intentional public binary/npm/docs/config compatibility surfaces and the dump-test compatibility string.
- Cargo metadata reports 42 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5AE-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
