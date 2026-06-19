---
name: OntoIndex Migration Fix
description: Fixed OntoIndex MCP wiring and project instructions after GitNexus replacement
type: audit_session
date: 2026-06-09
status: done
---

# OntoIndex Migration Fix

## Scope

- Repointed Codex MCP config from the OntoIndex tool repository to this `ontocode` repository.
- Indexed `/opt/demodb/_workfolder/ontocode` with OntoIndex.
- Updated project agent rules from GitNexus terminology to OntoIndex terminology.

## Verification

- `ontoindex status` reports `/opt/demodb/_workfolder/ontocode` is indexed and up to date at commit `73ba304`.
- `ontoindex list` reports the ontocode repo under label `codex` with 74267 symbols and 200455 edges.
- `ontoindex context --repo codex command_name_from_arg0` resolves the CLI symbol and caller/callee context.

## Notes

- The active MCP server process must be restarted before `mcp__ontoindex` uses the updated config.
- Reliable repo CLI invocation is from `/opt/demodb/_workfolder/ontocode` with `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js ...`.
- Reliable MCP startup uses cwd `/opt/demodb/_workfolder/OntoIndex/ontoindex`, args `["./dist/cli/index.js", "mcp"]`, and `ONTOINDEX_MCP_REPO=/opt/demodb/_workfolder/ontocode`.
- The legacy `.gitnexus/` directory remains on disk but is no longer the active index.
