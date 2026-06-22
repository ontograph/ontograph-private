---
name: Ontocode Architecture
description: Core architecture map for provider, auth, MCP, hooks, shell, context, external-agent import, and rename work
type: project
---

# Ontocode Architecture

Monorepo root: `/opt/demodb/_workfolder/ontocode`.

The Rust implementation lives in `ontocode-rs/`. The repository is currently a Codex-derived workspace under active Ontocode identity migration. Native model-provider auth is OpenAI/Codex-only; non-OpenAI model providers are external OpenAI-compatible API endpoints or user-owned sidecars.

## Repository Layout

| Path | Role |
| --- | --- |
| `ontocode-cli/` | Root npm wrapper package and release staging entrypoint for the CLI; public npm package names remain compatibility-preserved until an explicit release cutover |
| `ontocode-rs/` | Rust workspace: core agent runtime, CLI, TUI, app-server, MCP, provider, auth, state, shell, hooks |
| `ontocode-rs/model-provider/` | Provider descriptors, runtime engine metadata, provider capability/status surfaces |
| `ontocode-rs/model-provider-info/` | Model/provider metadata and model catalog related support |
| `ontocode-rs/codex-api/`, `ontocode-rs/codex-client/` | Model API client behavior, retry/timeout/error telemetry |
| `ontocode-rs/login/` | ChatGPT/OpenAI login flow, OAuth exchange, auth persistence, login URL sanitization |
| `ontocode-rs/rmcp-client/` | MCP client, MCP OAuth credential storage, auth status, OAuth login/refresh |
| `ontocode-rs/codex-mcp/`, `ontocode-rs/mcp-server/` | MCP command/tool integration and MCP protocol surfaces |
| `ontocode-rs/hooks/` | Hook matching, hook registry, hook execution and schema |
| `ontocode-rs/shell-*`, `ontocode-rs/exec*`, `ontocode-rs/sandboxing/` | Shell command execution, escalation, sandboxing, policy enforcement |
| `ontocode-rs/core/` | Agent/session orchestration, turn context construction, rollout/session behavior |
| `ontocode-rs/context-fragments/`, `ontocode-rs/response-debug-context/` | Bounded context and response diagnostic fragments |
| `ontocode-rs/external-agent-migration/`, `ontocode-rs/external-agent-sessions/` | External-agent import/session migration surfaces |
| `ontocode-rs/state/` | Persisted state migrations and storage |
| `ontocode-rs/tui/`, `ontocode-rs/cli/`, `ontocode-rs/app-server*` | User-facing surfaces and API boundaries |
| `CLAUDE_CODE_APPROACHES_FOR_CODEBASE*.md` | Current project plan, tracking, and lefties |
| `ADR_*.md`, `ONTOCODE_*.md` | Architecture decisions, implementation trackers, rename inventories |

## Current Architectural Direction

The active direction is to keep native GPT/Codex reliable while supporting other providers through external API configuration:

- Extend the existing `model-provider` owner instead of adding a second provider registry, factory, catalog, capability resolver, or runtime stream abstraction.
- Keep existing login/auth-store native model OAuth limited to OpenAI/Codex.
- Do not add native Gemini, Claude, Kimi, Antigravity, or other non-OpenAI model OAuth flows.
- Plug non-OpenAI providers through user-configured OpenAI-compatible API endpoints; any OAuth, refresh, account selection, provider catalog, or protocol translation for those providers belongs to the external endpoint/sidecar.
- Extend RMCP OAuth boundaries only for MCP-domain OAuth; do not reuse MCP OAuth as model-provider OAuth.
- Extend existing MCP client/status/sanitization paths instead of adding parallel MCP result processors.
- Extend existing hook and shell owners instead of adding a second hook registry, permission parser, shell launcher, or sandbox policy evaluator.
- Inject diagnostics into model context only through bounded context-fragment architecture with hard caps.
- Preserve compatibility boundaries for persisted state, CLI commands, package metadata, app-server APIs, config keys, rollout/session data, and external integrations.

## High-Level Flows

### Provider Selection And Runtime

```text
configuration/model selection
  -> OpenAI/Codex native provider or configured external API provider
  -> create_model_provider
  -> runtime engine/client session
  -> retry/timeout/error telemetry
  -> CLI/TUI/doctor/app-server diagnostics
```

### OAuth And Auth Store

```text
OpenAI/Codex login or MCP OAuth flow
  -> OAuth exchange / perform_oauth_login
  -> auth-store mode selection
  -> keyring or fallback file persistence
  -> auth status / refresh / diagnostics
  -> redacted CLI/TUI/doctor output
```

### MCP Tool Execution

```text
MCP config/server definition
  -> RmcpClient / Session::call_tool
  -> tool-call result handling
  -> sanitization for model-visible output
  -> status snapshot / diagnostics
```

### External-Agent Import

```text
external-agent config/session source
  -> import parser / validation
  -> dry-run and provenance report
  -> credential redaction
  -> existing auth/provider persistence boundary
```

## Where To Change What

| Concern | Start in |
| --- | --- |
| Provider descriptors/capabilities/runtime identity | `ontocode-rs/model-provider/` |
| Model catalog/provider metadata | `ontocode-rs/model-provider-info/`, `ontocode-rs/models-manager/` |
| Provider client retry/error/timeout telemetry | `ontocode-rs/codex-api/`, provider client crates |
| OpenAI/Codex OAuth login exchange and URL sanitization | `ontocode-rs/login/` |
| Non-OpenAI model provider auth | external OpenAI-compatible endpoint or user-owned sidecar; not Ontocode core |
| MCP OAuth storage and refresh | `ontocode-rs/rmcp-client/` |
| MCP auth/status/tool-call behavior | `ontocode-rs/rmcp-client/`, `ontocode-rs/codex-mcp/` |
| Hook behavior | `ontocode-rs/hooks/` |
| Shell execution/escalation/sandboxing | `ontocode-rs/exec*`, `ontocode-rs/shell-*`, `ontocode-rs/sandboxing/` |
| Session/turn context | `ontocode-rs/core/`, `ontocode-rs/context-fragments/` |
| External-agent migration | `ontocode-rs/external-agent-migration/`, `ontocode-rs/external-agent-sessions/` |
| CLI user-facing behavior | `ontocode-rs/cli/` |
| TUI user-facing behavior | `ontocode-rs/tui/` |
| App-server API behavior | `ontocode-rs/app-server*` |
| Ontocode identity migration | `ONTOCODE_*.md`, compatibility owner crates, affected surface owners |

## Non-Negotiable Architecture Rules

- Use GitNexus context/impact before editing any code symbol.
- Prefer existing owners and test harnesses.
- Keep changes small and reviewable.
- Do not broaden `codex-core` without first checking whether another crate is the right owner.
- Do not create new public API/config/schema surfaces without ADR and compatibility tests.
- Security diagnostics must reuse or extend shared redaction behavior and include no-secret assertions.
- New model-context content must be bounded, capped, and implemented as context fragments.
