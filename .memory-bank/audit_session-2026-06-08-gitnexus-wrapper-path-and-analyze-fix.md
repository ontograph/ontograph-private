# GitNexus Wrapper Path And Analyze Fix

Date: 2026-06-08

## Summary

- Added `/home/evrasyuk/.local/bin/gitnexus` as a symlink to `/home/evrasyuk/.local/bin/gitnexus-local`.
- Verified `gitnexus` resolves to the local checkout wrapper and reports GitNexus `1.6.2`.
- Updated stale-index recovery guidance in `AGENTS.md` and `GEMINI.md` to use `gitnexus analyze --skills --skip-agents-md`.
- Fixed local GitNexus LadybugDB relation schema coverage in `../GitNexus/gitnexus/src/core/lbug/schema.ts` for generated Rust pairs:
  - `Enum -> Property`
  - `Enum -> Trait`
  - `Trait -> Function`
  - `Impl -> Function`
- Rebuilt the local GitNexus checkout so the wrapper uses the fixed `dist/` output.

## Verification

- `gitnexus status` reports repo `codex` up to date at commit `ad2012d`.
- `gitnexus mcp` starts and exposes 61 MCP tools.
- `gitnexus analyze --skills --skip-agents-md` completes and generates 20 repo-specific skills.
- `npx vitest run test/integration/lbug-edge-limit.test.ts` passes in `../GitNexus/gitnexus`.

## Notes

- Embeddings remain unavailable; GitNexus reports BM25/graph search only until `gitnexus analyze --embeddings` is run.
- The local `../GitNexus/gitnexus` checkout has intentional changes to `package.json`, `package-lock.json`, and `src/core/lbug/schema.ts`.
