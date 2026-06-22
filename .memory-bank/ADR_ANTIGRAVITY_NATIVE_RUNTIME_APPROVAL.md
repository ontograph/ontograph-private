---
name: Antigravity Native Runtime Approval
description: Approval gate for enabling native Antigravity runtime execution
type: adr
date: 2026-06-18
status: superseded
---

# ADR: Antigravity Native Runtime Approval

## Status

Superseded by the 2026-06-19 OpenAI-only native provider policy.

Native Antigravity runtime is rejected for Ontocode core. Antigravity may be
used only through a user-managed external OpenAI-compatible API endpoint or
sidecar. This approval gate is retained as historical evidence only.

## Superseded Original Decision Needed

Approve A6 only after a local endpoint compatibility proof exists for the
intended Antigravity runtime target.

Until then, keep Antigravity visibly blocked and keep `/status auth antigravity`
as the user-facing readiness surface.

## Existing Owners

- Request auth boundary: `ontocode-rs/model-provider/src/provider.rs`
  `ModelProvider::api_auth()`.
- Runtime endpoint boundary: `ontocode-rs/model-provider/src/provider.rs`
  `ModelProvider::runtime_base_url()`.
- Private contract owner:
  `ontocode-rs/model-provider/src/antigravity_runtime.rs`.
- Route fail-closed tests:
  `ontocode-rs/model-provider/src/route_tests.rs`.
- Blocked UI/status owners: `ontocode-rs/tui/src/app.rs` and
  `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs`.

## Approval Gates

1. Endpoint compatibility proof: checked-in fixture or test proves the selected
   Antigravity endpoint accepts the request shape and returns the expected model
   response shape.
2. Runtime adapter proof: model execution uses the existing `model-provider`
   owner path; no proxy runtime, provider registry, or second auth broker.
3. Refresh proof: concurrent refreshes for the same provider credential/project
   share one upstream refresh and redact failures.
4. Disabled-by-default proof: provider/model mismatch and missing endpoint proof
   keep runtime blocked.

## First Implementation Slice After Approval

1. Add private request execution behind `antigravity_runtime.rs`.
2. Reuse `api_auth()` and `runtime_base_url()`; do not add config or public API.
3. Add model-provider tests for request shape, refresh dedupe, endpoint fixture,
   and mismatch fail-closed behavior.
4. Only then wire catalog/status visibility, preserving blocked state when the
   adapter is absent.

## Stop Conditions

- No runtime execution before endpoint proof.
- No public config, schema, app-server API, or endpoint probing.
- No copied donor runtime stack.
- No raw tokens, keychain paths, credential files, or authorization headers in
  logs, tests, status, or docs.
