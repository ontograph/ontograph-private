# Provider Lefties

## Summary

No runnable provider-engine or external-adapter ADR task is left.

The native heterogeneous provider queue is complete:

- `N0`-`N7`: done
- `N6`: done via `ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md`

## Blocked

| ID | Task | Blocker |
| --- | --- | --- |
| `E3` | Claude evidence gate | Requires a real redacted Claude MCP OAuth credential sample plus product/security approval |
| `E5` | Claude runtime MCP OAuth wiring | Blocked until `E3` is unblocked and approved |
| `T2` | Stage 1 minimal Claude import spike | Blocked on the same missing real Claude MCP credential evidence |
| `T2A` | Fresh local Claude MCP credential rediscovery | Blocked unless a real local Claude MCP credential source becomes available |
| `P4` | Claude MCP live credential validation | Blocked on a real Claude MCP credential source file |

## Deferred

| ID | Task | Reopen Trigger |
| --- | --- | --- |
| `E6` | Import adapter registry decision | Reopen only when a second foreign credential source appears or Claude import needs reusable importer lifecycle |
| `T8` | Optional internal crate rename decision | Reopen only if internal crate naming becomes in-scope after public-surface rename work |

## Completed Reference

- `ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES_TRACKING.md`: all tasks done.
- `ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md`: created and hardened; proposes stdio-first external adapter protocol only, not implementation.
- `ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION_TRACKING.md`: all unblocked tasks done; only blocked/deferred items remain.
