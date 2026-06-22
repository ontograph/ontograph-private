# Provider Extensibility Remaining Implementation Tracking

Source ADR: `ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md`

## Status Key

- `pending`
- `in_progress`
- `blocked`
- `done`
- `deferred`

## Task Queue

| ID | Task | Scope | Status | Notes |
| --- | --- | --- | --- | --- |
| E1 | Tracking bootstrap | Create task tracker and classify blocked, runnable, and deferred ADR stages | done | Initial manager pass created this tracker and separated runnable Stage B/Stage D work from blocked Stage A/C and deferred Stage E/F work |
| E2 | Claude persistence acceptance spec | Convert Stage B decisions into a concrete acceptance/test contract without persisting credentials | done | Added `CLAUDE_OAUTH_PERSISTENCE_ACCEPTANCE_CRITERIA.md`; diff checks passed; no runtime credential persistence was implemented |
| E3 | Claude evidence gate | Validate a real redacted Claude MCP OAuth credential sample and import approval | blocked | Requires external real redacted Claude MCP OAuth sample and product/security approval before runtime import wiring can begin |
| E4 | Private descriptor hardening verification | Verify Stage D: no public provider-engine config/schema, descriptor remains internal, and built-in engine tests still pass | done | Stage D verification passed; no code edits needed; `just test -p codex-model-provider` passed 41/41; `descriptor.rs` remains untracked and must be included before staging/commit |
| E5 | Claude runtime MCP OAuth wiring | Convert validated Claude credentials into existing MCP OAuth storage | blocked | Blocked until E2 is done and E3 is unblocked; no runtime persistence should be implemented before both gates pass |
| E6 | Import adapter registry decision | Decide whether a reusable foreign credential importer registry is needed | deferred | Reopen only when a second foreign credential source appears or Claude import needs reusable importer lifecycle handling |
| E7 | External runtime adapter ADR | Separate ADR for process-based heterogeneous provider adapters | done | Created `ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md`; this approves stdio-first ADR direction only, not runtime implementation |
| E8 | Final manager verification | Consolidate sub-agent results, update tracking, run diff checks and GitNexus detect | done | Final checks completed; diff-check passed; GitNexus detect remains CRITICAL due broad dirty workspace; all unblocked ADR tasks are done |

## Dispatch Log

| Step | Tracking Update | Action |
| --- | --- | --- |
| 1 | Created tracker; marked `E2` and `E4` in progress, `E3`/`E5` blocked, `E6`/`E7` deferred | Dispatching acceptance-spec and descriptor-verification sub-agent lanes |
| 2 | Marked `E4` done | Descriptor hardening verification passed with GitNexus scope review and model-provider targeted tests |
| 3 | Marked `E2` done and `E8` in progress | Acceptance contract landed; starting final manager verification |
| 4 | Marked `E8` done | Consolidated worker results, closed sub-agents, ran diff checks, and recorded GitNexus dirty-workspace risk |
| 5 | Marked `E7` done | External provider adapter runtime ADR created as the separate security/protocol review artifact required before implementation |
