# R5AM Response Debug Context Rename Risk Review

Date: 2026-06-12
Status: approved for identity-only dispatch with CRITICAL guardrails
Model fallback: `gpt-5.4-mini` because the required Spark model is unavailable or usage-limited.

## Scope

- Rename Cargo package `codex-response-debug-context` to `ontocode-response-debug-context`.
- Rename Rust crate import `codex_response_debug_context` to `ontocode_response_debug_context`.
- Update workspace metadata, dependent imports, and Bazel crate identity.
- Preserve the existing `response-debug-context` folder path.

## Direct Inventory

- Direct reverse dependencies: `ontocode-core`, `ontocode-model-provider`.
- Active refs are in workspace metadata, the response-debug-context manifest/Bazel identity, core client imports/usages, and model-provider model-endpoint imports/usages.

## OntoIndex Impact

- `extract_response_debug_context`: CRITICAL, 14 impacted symbols, 4 direct, 9 modules, no affected processes.
- `extract_response_debug_context_from_api_error`: CRITICAL, 13 impacted symbols, 3 direct, 8 modules, no affected processes.
- `telemetry_transport_error_message`: LOW, 1 impacted symbol, no affected processes.
- `telemetry_api_error_message`: LOW, 0 impacted symbols, no affected processes.

## Guardrails

- Identity-only rename: do not change extraction, telemetry message, base64 decode, header precedence, or error-body omission behavior.
- Preserve request-id, OpenAI request-id, Cloudflare ray, authorization error, and encoded error-code extraction.
- Preserve tests that prove HTTP bodies are omitted from telemetry.
- Do not print or store authorization headers, cookies, tokens, raw private bodies, keychain paths, or other credentials.
- Do not rename public provider behavior, wire names, telemetry fields, persisted data, or config keys.
- Run package tests, dependent core/model-provider checks, fmt, Bazel lock update/check, stale-reference classification, `git diff --check`, and OntoIndex diff detection before closure.
