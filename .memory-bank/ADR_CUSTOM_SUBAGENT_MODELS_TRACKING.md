# Custom Sub-Agent Models Tracking

Source ADR: [ADR_CUSTOM_SUBAGENT_MODELS.md](ADR_CUSTOM_SUBAGENT_MODELS.md)

Date opened: 2026-06-15
Status: complete

## Ledger

| ID | Task | Status | Owner | Notes |
| --- | --- | --- | --- | --- |
| CSM-0 | Track contract gaps | done | manager | Reviewed ADR against current code and OntoIndex; no fresh index needed. |
| CSM-1 | Document hidden-metadata gate | done | manager | ADR now states `hide_spawn_agent_metadata` keeps `model`, `reasoning_effort`, and `service_tier` hidden. |
| CSM-2 | Document model-catalog scope | done | manager | ADR now narrows to existing `ModelsManager` catalog and does not require a second registry. |
| CSM-3 | Document fork behavior | done | manager | ADR now covers both v1 `fork_context` and v2 `fork_turns = "all"`. |
| CSM-4 | Verify existing tests | done | manager | Existing tests already cover hidden metadata and full-history rejection; added a focused custom-model acceptance test. |
| CSM-5 | Add/adjust code tests | done | sub-agents/manager | Added `spawn_agent_accepts_custom_model_id_from_catalog` in `core/src/tools/handlers/multi_agents_tests.rs`. |
| CSM-6 | Refresh OntoIndex after slice | done | manager | Refreshed after the accepted code/test slice. |
| CSM-7 | Close ADR decisions | done | manager | ADR accepted; hidden-schema fallback and provider-qualified id ownership are now explicit decisions. |
| CSM-8 | Expose spawn model fields by default | done | manager | Flipped `hide_spawn_agent_metadata` default to `false`; explicit config can still hide metadata. |

## Notes

- Existing sessions that loaded the old schema may still not expose `model`; rebuilt/restarted runtimes use the source default that exposes spawn model fields.
- Code inspection shows `SpawnAgentArgs` already accepts `model`, `reasoning_effort`, and `service_tier`; custom-model acceptance, hidden schema metadata, unknown model rejection, and full-history fork rejection now have focused coverage or existing tests.
