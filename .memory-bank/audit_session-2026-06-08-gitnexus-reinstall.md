---
name: Audit Session - GitNexus Reinstall
date: 2026-06-08
type: audit
status: completed
---

# Audit Session - GitNexus Reinstall

## Summary

Restored GitNexus CLI, MCP configuration, standard skills, and codebase index for Ontocode.

## Results

- Installed global `gitnexus` CLI from the local `../GitNexus/gitnexus` checkout.
- Verified `gitnexus mcp` starts from the existing Codex MCP config.
- Restored standard GitNexus skills under `.claude/skills/gitnexus`.
- Verified the `codex` repository index is up to date.
- Restored active GitNexus agent guidance in `AGENTS.md`, `GEMINI.md`, and memory-bank rule files.

## Notes

- Local checkout `../GitNexus/gitnexus` initially failed because `@ladybugdb/core@0.15.3` required `GLIBCXX_3.4.32` on this host.
- Updated the local checkout to `@ladybugdb/core@^0.16.1`, which removes the GLIBCXX loader failure and allows `gitnexus mcp` to start from the local checkout.
- `gitnexus analyze --skills --skip-agents-md` no longer crashes on GLIBCXX, but still fails during graph load with a bulk edge COPY error before generating `.claude/skills/generated`.
