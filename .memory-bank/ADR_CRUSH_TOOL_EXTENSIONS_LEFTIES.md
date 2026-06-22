# Lefties: Crush Tool Extensions

## Status

Deferred

## Date

2026-06-07

## Source

Moved from [ADR_CRUSH_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_CRUSH_TOOL_EXTENSIONS.md:1) during GitNexus challenge.

## Reason

These items may be useful product polish or optional integrations, but they do not naturally extend Ontocode core provider, MCP, hooks, shell, context, credential, memory-bank, or diagnostics architecture.

They should not be implemented from the Crush ADR. Reconsider only through a dedicated UI/product-extension ADR with an explicit owner, user-facing value, compatibility impact, and tests.

## Moved Items

| Original points | Items | Reason |
|---|---|---|
| 221-240 | TUI compact/diff/transparent/completion/dialog/status/notification/cache/thinking/version-polish proposals | TUI has existing owners, but this range is mostly Crush UI parity/polish rather than a required core architecture extension. |
| 289-292 | Sourcegraph search/auth/result/context proposals | Optional vendor-specific source search integration; not natural core and not required for existing web-search extension. |
| 336-339 | Project registry/list/root/delete-state proposals | Product workflow/state management, not a Crush interop or core architecture requirement. |

## Reconsideration Criteria

Reconsider a moved item only if a later ADR defines:

- a concrete Ontocode owner
- user-facing behavior that cannot be achieved through existing surfaces
- compatibility and migration impact
- snapshot or integration tests for UI/API behavior
- redaction and policy tests for any diagnostic or network output
