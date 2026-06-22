---
name: OpenAI-Only Provider Policy Cleanup
description: ADR and memory-bank cleanup replacing native multi-provider OAuth plans with OpenAI-native plus external API provider policy
type: audit_session
date: 2026-06-19
status: closed
---

# OpenAI-First Provider Policy Cleanup

## Decision

Native browser/device model-provider login remains OpenAI/Codex-only.

Non-OpenAI model providers, including Gemini, Claude, Kimi, Antigravity, and
future providers, must be connected through user-configured external
OpenAI-compatible API endpoints or user-owned sidecars.

External endpoints own their own OAuth, API keys, refresh, account selection,
provider catalogs, and protocol translation.

Rollback recovery: `model-provider` may consume an already persisted
provider-scoped OAuth credential through the existing provider-auth store when a
request explicitly selects that provider/profile. This does not restore native
non-OpenAI login/import UX and does not add another credential store.

## Updated ADRs

- `ADR_MULTI_PROVIDER_OAUTH_CONNECTION_ROUTING.md`
- `ADR_NATIVE_GEMINI_OAUTH_AND_SUBAGENT_PROVIDER_CONCURRENCY.md`
- `ADR_ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB.md`
- `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR.md`
- `ADR_KIMI_OAUTH_CLIPROXY_IMPORT_AND_DEVICE_FLOW.md`
- `ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md`
- `ADR_PROVIDER_AUTH_SLASH_COMMAND_MENUS.md`
- `ADR_ANTIGRAVITY_RUNTIME_CONTRACT.md`
- `ADR_ANTIGRAVITY_NATIVE_RUNTIME_APPROVAL.md`

## Removed Obsolete Plan Artifacts

- `MULTI_PROVIDER_OAUTH_CONNECTION_ROUTING_PROJECT_PLAN.md`
- `MULTI_PROVIDER_OAUTH_CONNECTION_ROUTING_TRACKING.md`
- `MULTI_PROVIDER_OAUTH_FIRST_CLASS_CODEX_PROJECT_PLAN.md`
- `MULTI_PROVIDER_OAUTH_FIRST_CLASS_CODEX_TRACKING.md`
- `FIRST_CLASS_PROVIDER_SUPPORT_PROJECT_PLAN.md`
- `FIRST_CLASS_PROVIDER_SUPPORT_TRACKING.md`
- `GEMINI_OAUTH_DONOR_TRANSFER_PROJECT_PLAN.md`
- `GEMINI_OAUTH_DONOR_TRANSFER_TRACKING.md`
- `NATIVE_GEMINI_OAUTH_AND_SUBAGENT_PROVIDER_CONCURRENCY_PRE_JUNIOR_PROJECT_PLAN.md`
- `NATIVE_GEMINI_OAUTH_AND_SUBAGENT_PROVIDER_CONCURRENCY_TRACKING.md`
- `NATIVE_PROVIDER_MODEL_SELECTION_PROJECT_PLAN.md`
- `NATIVE_PROVIDER_MODEL_SELECTION_TRACKING.md`
- `PROVIDER_AUTH_SLASH_COMMAND_MENUS_PROJECT_PLAN.md`
- `PROVIDER_AUTH_SLASH_COMMAND_MENUS_TRACKING.md`

## Follow-Up

- Keep OpenAI/Codex first in provider/model selection.
- Gate any remaining non-OpenAI native runtime path behind the external-provider
  policy.
- Keep sidecar/API-key diagnostics redacted and independent from OpenAI login.
