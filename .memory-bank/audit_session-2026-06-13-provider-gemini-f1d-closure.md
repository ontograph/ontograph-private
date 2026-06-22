---
name: Provider Gemini F1-D Closure
description: Senior design closure for Gemini OAuth source-adapter gap with compatibility coverage
type: audit
date: 2026-06-13
status: accepted
---

# Provider Gemini F1-D Closure

## Decision

- `F1-D` is closed by design decision rather than by adding a speculative OAuth adapter.
- Current Gemini support remains explicitly API-key-only.
- No canonical Gemini OAuth credential source will be fabricated until a real source owner exists in one of:
  - `ontocode-login`
  - `ontocode-rmcp-client`
  - a new narrow provider-auth source adapter with explicit ownership approval

## Why

- Existing Gemini support is runtime/routing-shaped, not OAuth-owner-shaped.
- Adding a Gemini OAuth adapter today would violate the architecture rule against creating a parallel auth stack or fake persistence authority.

## Compatibility Coverage

- `CARGO_BUILD_JOBS=8 just test -p ontocode-core native_runtime_requires_provider_env_key_api_key_auth`

This test locks the current boundary:

- Gemini native runtime requires provider `env_key` API-key auth.
- Gemini native runtime must not silently synthesize canonical OAuth state.

## Residual Note

- `just fix -p ontocode-core` still fails on an unrelated pre-existing Clippy warning in `core/src/mcp_tool_call_tests.rs` (`clippy::unnecessary_literal_unwrap`).
- That warning is not caused by the Gemini compatibility slice.
