# Model Provider Selector Predicate Audit

## Status

Done

## Date

2026-06-05

## Scope

Audit provider/auth coupling outside `codex-rs/model-provider` for broad-provider readiness.

Inputs:

- `ADR_MODEL_PROVIDER_SELECTOR_REFACTOR.md`
- GitNexus query/context/cypher for `requires_openai_auth`, `is_amazon_bedrock`, `create_model_provider`, and `auth_manager_for_provider`
- targeted code search for the same symbols

No Rust, TypeScript, or Python source changes were made.

## Summary

The selector implementation slice has already localized the primary runtime branch inside `codex-rs/model-provider` through `ProviderKind`, but external layers still consume provider predicates for UX, diagnostics, protocol, and config serialization.

No external coupling currently prevents adding another non-default provider if that provider can be represented by existing `ModelProviderInfo` fields. Another provider with custom runtime/auth/catalog behavior should not be added by copying Bedrock-style predicates outward; it should extend the selector boundary first.

## Classification

### Leave As-Is

These surfaces are API/user-visible or durable configuration surfaces. Rename/removal would create compatibility churn without improving selector readiness.

| Surface | Reference | Reason |
| --- | --- | --- |
| `ModelProviderInfo.requires_openai_auth` config field | `codex-rs/model-provider-info/src/lib.rs:128` | Existing config contract for OpenAI-auth gating and custom providers. |
| v1 auth status `requires_openai_auth` field | `codex-rs/app-server-protocol/src/protocol/v1.rs:196` | Wire compatibility surface. |
| v2 account `requires_openai_auth` field | `codex-rs/app-server-protocol/src/protocol/v2/account.rs:309` | Wire compatibility surface. |
| thread config proto `requires_openai_auth` | `codex-rs/config/src/thread_config/proto/codex.thread_config.v1.proto:49` | Persisted/remote thread config compatibility surface. |
| Test/config helpers that write `requires_openai_auth` TOML | `codex-rs/app-server/tests/common/config.rs:33`, `codex-rs/app-server/tests/suite/auth.rs:36` | They exercise public config behavior rather than provider selection internals. |

### Migrate Later

These call sites should depend on selector/capability output after the selector boundary exposes the needed concepts, but they do not block the current additive selector slice.

| Surface | Reference | Reason |
| --- | --- | --- |
| App-server auth status directly reads `config.model_provider.requires_openai_auth` | `codex-rs/app-server/src/request_processors/account_processor.rs:751` | Should eventually ask runtime provider/account state for auth requirement instead of inspecting provider metadata. |
| CLI doctor auth check reads `requires_openai_auth` | `codex-rs/cli/src/doctor.rs:1175` | Diagnostic UX should eventually consume provider auth classification instead of a raw OpenAI-specific flag. |
| CLI reachability planner reads `requires_openai_auth` and `is_amazon_bedrock` | `codex-rs/cli/src/doctor.rs:2498`, `codex-rs/cli/src/doctor.rs:2508` | Reachability should become provider-kind/capability driven once the selector exposes diagnostic planning inputs. |
| CLI WebSocket check constructs runtime provider | `codex-rs/cli/src/doctor.rs:2263` | Acceptable now, but diagnostics should avoid duplicating provider setup policy as more runtime provider classes appear. |
| App-server capability read constructs runtime provider | `codex-rs/app-server/src/request_processors/config_processor.rs:175` | Acceptable but could be replaced by selector-owned capability resolution after that API exists. |
| Core/session/thread manager `create_model_provider` call sites | `codex-rs/core/src/client.rs:332`, `codex-rs/core/src/session/turn_context.rs:488`, `codex-rs/core/src/thread_manager.rs:224` | These are runtime factory consumers, not predicate leaks; they should remain callers while factory internals evolve. |

### Must Migrate Now

None identified for the current readiness target.

The only external Bedrock predicate call found by GitNexus outside `codex-rs/model-provider` is CLI doctor reachability planning. That is a diagnostic behavior branch, not a runtime construction blocker. It should move later, but it does not prevent adding another provider if the new provider does not need distinct diagnostic probes immediately.

## GitNexus Findings

- GitNexus cypher found `is_amazon_bedrock` callers only in `create_model_provider` and `provider_reachability_plan`.
- GitNexus context for `auth_manager_for_provider` found no incoming callers outside `codex-rs/model-provider`; it is already localized.
- GitNexus context for `create_model_provider` found broad runtime/test callers in `core`, `cli`, and tests; this supports keeping the factory API stable while refactoring internals.
- GitNexus context for `provider_reachability_plan` confirmed outgoing dependency on both `requires_openai_auth` and `is_amazon_bedrock`.

## Readiness Answer

The codebase is partially ready for multiple providers:

- Ready for additional OpenAI-compatible configured providers that fit existing `ModelProviderInfo` fields.
- Ready for additional provider classes only after adding selector-owned classification and avoiding new external predicates.
- Not ready for many heterogeneous providers with distinct auth, catalog, account-state, or diagnostic behavior unless those capabilities are modeled behind the selector boundary.

## Follow-Up

- Complete P1 verification for `ProviderKind` routing and behavior preservation.
- Add selector-owned diagnostic/capability outputs before introducing another Bedrock-like provider.
- Do not add new external provider-name predicates outside `codex-rs/model-provider`.
