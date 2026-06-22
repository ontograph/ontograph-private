# R5BS Tools Rename Risk Review

Date: 2026-06-13

## Scope

- Rename `codex-tools` to `ontocode-tools`.
- Rename `codex_tools` to `ontocode_tools`.
- Keep the existing `tools` directory path.

## OntoIndex

- `ToolExecutor`: CRITICAL; broad upstream reach across core tool handlers, extension integration, and tests.
- `ResponsesApiTool`: CRITICAL; broad upstream reach across tool spec planning and tool conversion paths.
- `ToolCall`: UNKNOWN/ambiguous across `tools`, `core`, and `rollout-trace`.
- `JsonToolOutput`: UNKNOWN/ambiguous across struct and impl symbols in `tools`.
- `ToolSpec`: UNKNOWN/ambiguous.

## Guardrails

- Do not change tool schema/spec behavior, JSON schema generation, or model-visible tool definitions.
- Do not change tool execution semantics, tool routing, tool search behavior, or multi-agent tool behavior.
- Do not change MCP/dynamic-tool/Responses API conversion behavior.
- Do not change code-mode augmentation, image-detail normalization, request-plugin-install behavior, or tool payload/history truncation behavior.
- Do not change public config, wire, generated-schema, telemetry/product strings, persisted state, or directory paths.
- Preserve the residual protocol-gated package identities `ontocode-app-server-protocol` and `codex-protocol`.

## Verification Required

- Tools package compile/tests.
- Core/app-server/MCP-server compile checks if imports changed.
- Extension-api and extension crate compile checks if imports changed.
- CLI/TUI compile checks if imports changed.
- `just fmt`.
- `just bazel-lock-update` and `just bazel-lock-check`.
- Stale-reference search.
- Cargo metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
