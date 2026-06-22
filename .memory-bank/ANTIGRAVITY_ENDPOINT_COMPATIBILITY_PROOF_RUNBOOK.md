---
name: Antigravity Endpoint Compatibility Proof Runbook
description: Minimal evidence required before A6 native runtime execution can be approved
type: runbook
date: 2026-06-18
status: blocked_on_external_evidence
---

# Antigravity Endpoint Compatibility Proof Runbook

## Goal

Produce the one missing A6 approval artifact: a checked-in endpoint
compatibility proof for the intended Antigravity runtime target.

## Required Evidence

1. A redacted request fixture that includes:
   - endpoint host and path, with secrets removed
   - `loadCodeAssist` / `onboardUser` metadata shape when discovery is required
   - model request shape for the intended runtime adapter
2. A redacted response fixture that proves the endpoint returns the expected
   model response shape.
3. A failure fixture for unsupported tool/schema fields if cleanup is required.
4. A note proving no raw tokens, authorization headers, keychain paths, or
   credential files are included.

## Where The Proof Should Land

- Request/response fixture or test:
  `ontocode-rs/model-provider/src/antigravity_runtime_tests.rs`
- Runtime owner after approval:
  `ontocode-rs/model-provider/src/antigravity_runtime.rs`
- Approval record:
  `ADR_ANTIGRAVITY_NATIVE_RUNTIME_APPROVAL.md`

## Stop Conditions

- Do not call a live endpoint from normal tests.
- Do not store real credentials or raw private endpoint logs.
- Do not enable runtime, catalog rows, config, or public APIs from this proof.

## Current Blocker

No local endpoint compatibility proof exists in this repository. A6 remains
blocked until a redacted fixture or approved test artifact is provided.
