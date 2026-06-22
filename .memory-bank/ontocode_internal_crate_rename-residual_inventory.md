# Ontocode Internal Crate Rename Residual Inventory

Date: 2026-06-11

## Purpose

This inventory records remaining `codex-*` Cargo packages after R5 core/shared implementation closure. It corrects the assumption that R6 cleanup could start immediately.

## Summary

- Remaining `codex-*` Cargo packages: 68.
- R6 cleanup is blocked until these packages are renamed or explicitly deferred.
- R5B protocol/generated-schema packages remain blocked by separate ADR.
- Ordering below uses direct reverse-dependency count from `cargo metadata`; this is a prioritization hint, not approval to batch.

## Zero Direct Reverse Dependencies

| Package | Path | Disposition |
| --- | --- | --- |
| `codex-adapter-protocol` | `ontocode-rs/adapter-protocol` | protocol/adapter boundary; needs separate protocol/adapter decision |
| `codex-agent-graph-store` | `ontocode-rs/agent-graph-store` | R5F active slice |
| `codex-bwrap` | `ontocode-rs/bwrap` | helper/runtime candidate; preserve bundled/runtime behavior |
| `ontocode-execpolicy-legacy` | `ontocode-rs/execpolicy-legacy` | policy/docs candidate; verify CLI/docs examples |
| `codex-goal-extension` | `ontocode-rs/ext/goal` | extension candidate; verify extension API/core/state/tool behavior |
| `codex-skills-extension` | `ontocode-rs/ext/skills` | extension candidate; verify skills/core behavior |
| `codex-thread-manager-sample` | `ontocode-rs/thread-manager-sample` | sample candidate; verify README commands and package test |
| `codex-v8-poc` | `ontocode-rs/v8-poc` | experimental candidate; likely defer or isolate |

## One Direct Reverse Dependency

`codex-aws-auth`, `codex-backend-openapi-models`, `codex-cloud-tasks`, `codex-cloud-tasks-mock-client`, `codex-collaboration-mode-templates`, `codex-experimental-api-macros`, `codex-external-agent-migration`, `codex-external-agent-sessions`, `codex-file-watcher`, `codex-guardian`, `codex-image-generation-extension`, `codex-lmstudio`, `codex-memories-extension`, `codex-memories-read`, `codex-message-history`, `codex-ollama`, `codex-prompts`, `codex-realtime-webrtc`, `ontocode-responses-api-proxy`, `codex-secrets`, `codex-skills`, `ontocode-stdio-to-uds`, `codex-web-search-extension`.

## Higher Direct Reverse Dependency Counts

| Count | Packages |
| --- | --- |
| 2 | `codex-agent-identity`, `codex-chatgpt`, `codex-cloud-tasks-client`, `codex-memories-write`, `codex-process-hardening`, `codex-response-debug-context`, `codex-test-binary-support`, `codex-thread-store` |
| 3 | `codex-apply-patch`, `codex-cloud-config`, `codex-code-mode`, `codex-connectors`, `ontocode-file-search`, `codex-file-system`, `codex-keyring-store`, `codex-rollout-trace` |
| 4 | `codex-backend-client`, `codex-context-fragments`, `codex-core-skills`, `codex-network-proxy`, `codex-uds` |
| 5+ | `codex-analytics`, `codex-core-plugins`, `ontocode-execpolicy`, `codex-feedback`, `codex-rollout`, `codex-terminal-detection`, `codex-shell-command`, `codex-tools`, `codex-models-manager`, `codex-state`, `codex-extension-api`, `codex-features`, `codex-git-utils`, `codex-otel`, `ontocode-app-server-protocol`, `codex-protocol` |

## Manager Rule

Proceed one residual slice at a time. Do not batch protocol/generated, telemetry, package-manager, state/env, public command, or persisted compatibility surfaces with ordinary package/lib identity renames.
