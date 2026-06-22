---
name: Antigravity Native Runtime Contract
description: Minimum runtime acceptance contract for Antigravity before any native execution is enabled
type: adr
date: 2026-06-18
status: superseded
---

# ADR: Antigravity Native Runtime Contract

## Status

Superseded by the 2026-06-19 OpenAI-only native provider policy.

Native Antigravity runtime is no longer an accepted future direction for
Ontocode core. Antigravity may be used only through a user-managed external
OpenAI-compatible API endpoint or sidecar.

Do not use this ADR to dispatch implementation work. The contract below remains
historical evidence for evaluating an external sidecar.

## Context

Current repo evidence already proves the non-runtime path:

- `ontocode-rs/tui/src/app.rs` and `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs`
  keep `antigravity` visibly blocked with a runtime-unavailable reason.
- `ontocode-rs/tui/src/app/tests/model_catalog.rs` and
  `ontocode-rs/tui/src/chatwidget/tests/slash_commands.rs` already guard the
  blocked catalog and slash-command behavior.
- `ontocode-rs/model-provider/src/auth_status.rs` keeps provider status logic in
  the existing auth/status owner.
- `ontocode-rs/models-manager/src/native_provider_catalogs.rs` keeps catalog
  data bounded and static.

Donor evidence from `tmp/CLIProxyAPI-main` and `tmp/oh-my-pi-main` is useful
only as contract reference:

- CLIProxyAPI shows refresh deduplication for shared Antigravity tokens and
  provider-identifier separation.
- CLIProxyAPI schema cleaning shows the compatibility pressure around
  Antigravity/Gemini tool-call payloads.
- Oh My Pi shows the Cloud Code Assist discovery shape used after OAuth:
  `loadCodeAssist`, `onboardUser`, and `ideType = ANTIGRAVITY`.

The runtime owner for any later implementation remains the existing
`model-provider` / `models-manager` / `tui` surfaces. This ADR explicitly
rejects a parallel runtime stack.

## Superseded Original Decision

Do not enable native Antigravity runtime yet.

Accept the following contract as the minimum gate for any later runtime
implementation:

1. Cloud Code Assist request construction must preserve the donor metadata
   contract, including the discovery path and `ANTIGRAVITY` identity.
2. Tool/request schema handling must remain provider-owned and compatibility
   driven, not registry-driven.
3. Refresh and retry behavior must stay bounded, deduplicated for concurrent
   refresh, and redacted on failure.
4. Model-provider mismatch behavior must fail closed and keep the provider
   visibly disabled rather than auto-selecting or silently falling back.

## Native Runtime-Auth Owner Seam

The smallest native seam needed before A9 and A10 can land already exists in
`ontocode-rs/model-provider`:

- `ModelProvider::api_auth()` is the single provider-owned request-auth boundary.
- `ModelProvider::runtime_base_url()` is the single provider-owned runtime
  endpoint boundary.
- `auth_manager_for_provider()` and `resolve_provider_auth_with_manager()` stay
  private inside `model-provider/src/auth.rs` and remain the only auth
  resolution helpers for runtime requests.
- No second auth store, broker, registry, or runtime stack is introduced for
  Antigravity.
- A dedicated private contract module/test is only justified once an actual
  private request/refresh owner exists. Before that owner lands, a standalone
  contract module would be dead scaffolding because it would only restate the
  existing `api_auth()` / `runtime_base_url()` seam.

That seam is enough for future Antigravity runtime work to hang tests and
request assembly off the existing owner without widening ownership outside
`model-provider`.

## Minimal Implementation Slices

These are the smallest slices A6 may later implement against this contract:

1. Runtime request assembly in `ontocode-rs/model-provider`, using the existing
   provider auth and runtime owner boundaries.
2. Refresh/retry handling in the existing auth/runtime path, with concurrent
   refresh dedupe and redacted error propagation.
3. Provider catalog exposure in `ontocode-rs/models-manager` and
   `ontocode-rs/tui`, keeping Antigravity disabled until the runtime adapter is
   proven.
4. Status and slash-command messaging in `ontocode-rs/tui` that continues to
   show `runtime_unavailable` / blocked rows instead of selectable models.

## Required Tests And Fixtures

No new runtime code ships in this ADR. A later runtime implementation must add
tests that cover all of the following, and they should live under the existing
`model-provider` owner seam unless a later runtime owner is explicitly approved:

1. A donor-shaped Cloud Code Assist request fixture that preserves the
   `loadCodeAssist`/`onboardUser` metadata contract without introducing a new
   provider registry.
2. A refresh dedupe test equivalent in outcome to the CLIProxyAPI donor case:
   concurrent refreshes for the same Antigravity token must share one upstream
   refresh call and preserve bounded project metadata.
3. A schema-compatibility test proving the Antigravity tool/request payload is
   still accepted after provider-specific cleanup.
4. A mismatch guard test proving the runtime stays disabled when the selected
   provider/model does not match the Antigravity runtime owner.
5. The existing TUI blocked-catalog and slash-command tests remain in place as
   the non-runtime regression guards.

Current state: the narrow non-executing contract owner now exists under
`model-provider` test scope only:

- `provider_api_auth_uses_antigravity_provider_oauth_credential_from_auth_manager`
  proves the existing `AuthManager` provider OAuth path feeds `api_auth()` and
  `runtime_base_url()` without a second auth store.
- `antigravity_runtime` contract tests preserve the donor-shaped
  `loadCodeAssist`/`onboardUser` request metadata, exclude token material from
  request JSON/debug output, and partition refresh dedupe identity by provider,
  credential, and project while ignoring token rotation.

That closes the former A9/A10 test-seam blockers without enabling runtime
execution. A6 remains blocked only on live native runtime approval: an accepted
runtime ADR, endpoint compatibility proof, and a real model execution adapter.

## A6 Approval Packet

A6 is approval-ready only as a packet, not as runtime permission. Treat the
following as the concrete manager gate for the next dispatch, and do not treat
this ADR as approval until the first two proof items exist locally:

1. An accepted native runtime ADR exists in the repo history or memory-bank,
   not just this proposed contract.
2. A local endpoint compatibility proof exists for the intended Antigravity
   runtime target.
3. The implementation stays on the existing `model-provider` owner path and
   keeps `api_auth()` / `runtime_base_url()` as the runtime boundary.
4. Runtime execution still fails closed until the adapter is real and tests
   pass.

Implementation slices for the next runtime owner:

1. Private Antigravity runtime request assembly in
   `ontocode-rs/model-provider/src/antigravity_runtime.rs`, using the existing
   donor-shaped request contract and refresh-key partitioning only.
2. Refresh and retry handling in the same `model-provider` owner, with
   redacted failures and one upstream refresh per concurrent token.
3. Provider catalog and status wiring in `ontocode-rs/models-manager` and
   `ontocode-rs/tui`, but only after runtime approval exists and only while the
   provider remains visibly blocked when the runtime adapter is absent.
4. Keep the blocked-path guards in
   `ontocode-rs/model-provider/src/route_tests.rs`,
   `ontocode-rs/tui/src/app/tests/model_catalog.rs`, and
   `ontocode-rs/tui/src/chatwidget/tests/slash_commands.rs` so the disabled
   state remains the default until execution is explicitly approved.

Required tests for that later owner:

1. A donor-shaped Cloud Code Assist request fixture that preserves
   `loadCodeAssist` / `onboardUser` metadata without broadening provider
   registration.
2. A refresh dedupe test proving concurrent refreshes for the same Antigravity
   token share one upstream refresh call.
3. An endpoint compatibility proof test or fixture showing the runtime adapter
   matches the accepted Antigravity endpoint contract.
4. A mismatch guard test proving the runtime stays disabled when the selected
   provider/model does not match the Antigravity runtime owner.
5. The existing blocked-catalog and slash-command tests remain as non-runtime
   regression guards.

Stop conditions for A6:

- Do not enable runtime execution, model selection, or a native Antigravity
  catalog row.
- Do not add a second auth broker, auth store, provider registry, or proxy
  runtime stack.
- Do not add public API, config, schema, or endpoint-probing surface area for
  Antigravity.
- Do not hardcode client secrets or copy donor runtime stack internals.
- Do not turn Cloud Code Assist discovery into live runtime probing outside the
  accepted contract tests.

Exact next owner files for the runtime packet:

- `ontocode-rs/model-provider/src/antigravity_runtime.rs`
- `ontocode-rs/model-provider/src/auth.rs`
- `ontocode-rs/model-provider/src/route_tests.rs`
- `ontocode-rs/tui/src/app.rs`
- `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs`
- `ontocode-rs/tui/src/app/tests/model_catalog.rs`
- `ontocode-rs/tui/src/chatwidget/tests/slash_commands.rs`

## Stop Conditions

Do not cross any of these lines while this ADR is only proposed:

- Do not enable runtime execution, model selection, or a native Antigravity
  catalog row.
- Do not add a second auth broker, auth store, provider registry, or proxy
  runtime stack.
- Do not add public API, config, or schema surface area for Antigravity.
- Do not hardcode client secrets or copy donor runtime stack internals.
- Do not turn Cloud Code Assist discovery into live runtime probing outside the
  accepted contract tests.

## Consequences

This ADR gives A6 a narrow acceptance target without broadening the runtime
surface:

- Antigravity stays visibly disabled today.
- Later runtime work can be written directly against the existing owner modules.
- The donor examples stay as contract reference only, not as a copied runtime
  architecture.
