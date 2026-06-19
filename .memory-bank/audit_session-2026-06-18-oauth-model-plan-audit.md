---
name: OAuth And Model Functionality Plan Audit
description: Review of completed and planned OAuth/model/provider work, with stale authority and remaining gates
type: audit_session
date: 2026-06-18
status: completed
---

# OAuth And Model Functionality Plan Audit

## Scope

Reviewed ADRs, project plans, tracking files, and local code evidence for the new OAuth, provider-auth, provider-model, and sub-agent model functionality.

Primary documents reviewed:

- `GEMINI_OAUTH_DONOR_TRANSFER_PROJECT_PLAN.md`
- `GEMINI_OAUTH_DONOR_TRANSFER_TRACKING.md`
- `NATIVE_GEMINI_OAUTH_AND_SUBAGENT_PROVIDER_CONCURRENCY_PRE_JUNIOR_PROJECT_PLAN.md`
- `NATIVE_GEMINI_OAUTH_AND_SUBAGENT_PROVIDER_CONCURRENCY_TRACKING.md`
- `ADR_PROVIDER_AUTH_SLASH_COMMAND_MENUS.md`
- `PROVIDER_AUTH_SLASH_COMMAND_MENUS_PROJECT_PLAN.md`
- `PROVIDER_AUTH_SLASH_COMMAND_MENUS_TRACKING.md`
- `ADR_ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB.md`
- `ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB_TRACKING.md`
- `ADR_KIMI_OAUTH_CLIPROXY_IMPORT_AND_DEVICE_FLOW.md`
- `KIMI_OAUTH_CLIPROXY_IMPORT_AND_DEVICE_FLOW_TRACKING.md`
- `ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md`
- `ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES_TRACKING.md`
- `ADR_MODEL_PROVIDER_SELECTOR_REFACTOR.md`
- `ADR_MODEL_PROVIDER_SELECTOR_REFACTOR_TRACKING.md`
- `ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md`
- `ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION_TRACKING.md`
- `ADR_CUSTOM_SUBAGENT_MODELS.md`
- `ADR_CUSTOM_SUBAGENT_MODELS_TRACKING.md`
- `FIRST_CLASS_PROVIDER_SUPPORT_PROJECT_PLAN.md`
- `NATIVE_PROVIDER_MODEL_SELECTION_PROJECT_PLAN.md`
- `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md`

## Current Implementation State

Done:

- Provider OAuth persistence exists in the existing auth storage path via `provider_oauth_credentials`.
- Provider OAuth helpers exist for upsert, load, and removal.
- Runtime provider OAuth handoff exists through `AuthManager` and `model-provider` bearer auth resolution.
- Gemini has two explicit lanes:
  - `gemini` for normal Gemini API support and OAuth bearer fallback when `GEMINI_API_KEY` is absent.
  - `gemini-cli` for the Cloud Code Assist / donor OAuth lane, still runtime-gated.
- Gemini API-key precedence is preserved: `GEMINI_API_KEY` wins over OAuth for the normal `gemini` provider.
- User-supplied Gemini ADC / desktop OAuth JSON import exists and projects to provider id `gemini`.
- Donor Gemini CLI import remains separate and projects to `gemini-cli`.
- Kimi import exists through the existing provider OAuth storage path.
- Antigravity import exists through the existing provider OAuth storage path.
- `/login <provider> --import <path>` import flow exists for Gemini, Kimi, and Antigravity.
- `/auth <provider> remove` provider-scoped OAuth removal exists.
- `/status auth` provider rows exist and include redacted provider OAuth metadata.
- `/logout <provider>` remains read-only guidance and does not call global logout.
- Native provider engine scaffolding and implementations have landed for Anthropic, Gemini, and GitHub Copilot through `ProviderEngine`.
- Provider auth status rows exist in `model-provider`.
- Custom sub-agent model override support is accepted and covered by catalog validation, hidden-schema behavior, and full-history fork rejection.

Code evidence checked:

- `ontocode-rs/login/src/auth/storage.rs`
- `ontocode-rs/login/src/auth/manager.rs`
- `ontocode-rs/login/src/gemini_oauth.rs`
- `ontocode-rs/login/src/server.rs`
- `ontocode-rs/external-agent-migration/src/gemini_oauth_import.rs`
- `ontocode-rs/external-agent-migration/src/kimi_oauth_import.rs`
- `ontocode-rs/model-provider/src/auth.rs`
- `ontocode-rs/model-provider/src/auth_status.rs`
- `ontocode-rs/model-provider/src/descriptor.rs`
- `ontocode-rs/model-provider/src/provider.rs`
- `ontocode-rs/model-provider-info/src/lib.rs`
- `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs`
- `ontocode-rs/tui/src/model_catalog.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`

## Planned Or Blocked Work

Still blocked by explicit gates:

- Bundled Gemini browser OAuth login: blocked until approved Google OAuth client metadata exists.
- `/login gemini --code`: blocked until there is a safe no-history secret input seam in the TUI.
- Gemini device flow: blocked unless a real supported device endpoint is verified.
- `gemini-cli` Cloud Code Assist runtime: blocked until product/API endpoint approval.
- Kimi device flow: blocked until the donor-observed client id is explicitly approved.
- Kimi native runtime and model visibility: blocked pending a runtime ADR and execution fixtures.
- Antigravity native runtime, refresh adapter, schema cleanup, and model execution: blocked pending a runtime ADR and endpoint/API proof.
- Claude live OAuth import/runtime wiring: blocked pending a sanitized real Claude credential sample plus product/security approval.
- External provider adapter runtime implementation: blocked pending later schema/config/security ADRs; current ADR is protocol direction only.
- Public app-server/provider catalog APIs and schema-backed adapter config: gated by compatibility ADRs and schema/test updates.

Still planned but not immediate:

- First-class backend-owned provider catalog and live provider switching.
- TUI conversion from local provider grouping to backend-owned grouped provider catalogs.
- Unified dynamic discovery lifecycle across first-class providers.
- Copilot dynamic discovery contract decision.

## Stale Or Confusing Authority

High priority cleanup:

- `GEMINI_OAUTH_DONOR_TRANSFER_PROJECT_PLAN.md` still reads like `gemini-cli` is the primary canonical OAuth answer in several sections. The current state is split: normal Gemini OAuth belongs to `gemini`; donor Cloud Code Assist belongs to `gemini-cli` and remains runtime-gated.
- `ADR_PROVIDER_AUTH_SLASH_COMMAND_MENUS.md` and `PROVIDER_AUTH_SLASH_COMMAND_MENUS_PROJECT_PLAN.md` contain future-tense warnings about provider-scoped removal even though `/auth <provider> remove` now exists. Keep the warning only for `/logout <provider>`.
- `ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md` still reads as proposed in places even though provider engines and native runtimes have landed. It should be marked accepted/implemented with remaining work limited to external adapters and first-class catalog polish.
- `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR.md` should remain as a blocked evidence gate, not as an active broad broker proposal.
- `FIRST_CLASS_PROVIDER_SUPPORT_PROJECT_PLAN.md` and `NATIVE_PROVIDER_MODEL_SELECTION_PROJECT_PLAN.md` overlap with already-landed native engines and local TUI model grouping. They should be treated as future backend/catalog ownership plans only.

Low priority cleanup:

- Some tracking files mention old crate names in historical verification commands. Do not churn those unless editing the file for other reasons.
- Several ADRs repeat the same "no second provider registry / no second auth store" rule. Keep it in the current authority doc and avoid adding more duplicate warnings.

## Recommended Next Action

Do not start a new architecture branch.

The smallest useful next step is documentation consolidation:

1. Add a short "Current Authority" section to the Gemini OAuth plan:
   - `gemini` = official Gemini API, API key first, OAuth bearer fallback.
   - `gemini-cli` = Cloud Code Assist donor lane, disabled until runtime approval.
2. Mark provider-auth menu stages for import, status, and `/auth <provider> remove` as implemented.
3. Mark native heterogeneous provider engines as implemented for Anthropic, Gemini, and Copilot; leave external adapters as future.
4. Leave blocked runtime/login gates blocked.

Update, 2026-06-18:

- [Multi-Provider OAuth With First-Class Codex Project Plan](MULTI_PROVIDER_OAUTH_FIRST_CLASS_CODEX_PROJECT_PLAN.md)
  is now the current authority for provider OAuth sequencing.
- Codex/OpenAI uses provider id `openai` and remains the first-class default and
  fallback.
- Provider OAuth credentials are additive and provider-scoped. They must not
  create an exclusive app-wide auth mode or break Codex/OpenAI when a
  non-default provider fails.

After that, the next implementation work should be chosen from one of only two lanes:

- backend-owned provider catalog/live switching, if product needs smoother model/provider selection now
- safe secret-input TUI seam, if `/login gemini --code` is still desired

Everything else is gated or already done.

## Verification Notes

- This audit did not edit Rust code.
- No tests were run because this was a document/code-state audit.
- OntoIndex MCP was unavailable for this checkout during review: the MCP server is scoped to `/opt/demodb/_workfolder/OntoIndex`, not `/opt/demodb/_workfolder/ontocode`.
- Local evidence came from memory-bank tracking files and targeted code search.
