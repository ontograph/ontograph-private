---
name: Ontocode Architecture
description: Core architecture map for provider, auth, MCP, hooks, shell, context, external-agent import, and rename work
type: project
---

# Ontocode Architecture

Monorepo root: `/opt/demodb/_workfolder/ontocode`.

The Rust implementation lives in `codex-rs/`. The repository is currently a Codex-derived workspace under active Ontocode identity migration and heterogeneous-provider expansion.

## Repository Layout

| Path | Role |
| --- | --- |
| `codex-rs/` | Rust workspace: core agent runtime, CLI, TUI, app-server, MCP, provider, auth, state, shell, hooks |
| `codex-rs/model-provider/` | Provider descriptors, runtime engine metadata, provider capability/status surfaces |
| `codex-rs/model-provider-info/` | Model/provider metadata and model catalog related support |
| `codex-rs/codex-api/`, `codex-rs/codex-client/` | Model API client behavior, retry/timeout/error telemetry |
| `codex-rs/login/` | ChatGPT/OpenAI login flow, OAuth exchange, auth persistence, login URL sanitization |
| `codex-rs/rmcp-client/` | MCP client, MCP OAuth credential storage, auth status, OAuth login/refresh |
| `codex-rs/codex-mcp/`, `codex-rs/mcp-server/` | MCP command/tool integration and MCP protocol surfaces |
| `codex-rs/hooks/` | Hook matching, hook registry, hook execution and schema |
| `codex-rs/shell-*`, `codex-rs/exec*`, `codex-rs/sandboxing/` | Shell command execution, escalation, sandboxing, policy enforcement |
| `codex-rs/core/` | Agent/session orchestration, turn context construction, rollout/session behavior |
| `codex-rs/context-fragments/`, `codex-rs/response-debug-context/` | Bounded context and response diagnostic fragments |
| `codex-rs/external-agent-migration/`, `codex-rs/external-agent-sessions/` | External-agent import/session migration surfaces |
| `codex-rs/state/` | Persisted state migrations and storage |
| `codex-rs/tui/`, `codex-rs/cli/`, `codex-rs/app-server*` | User-facing surfaces and API boundaries |
| `CLAUDE_CODE_APPROACHES_FOR_CODEBASE*.md` | Current project plan, tracking, and lefties |
| `ADR_*.md`, `ONTOCODE_*.md` | Architecture decisions, implementation trackers, rename inventories |

## Current Architectural Direction

The active direction is to support many heterogeneous providers while preserving existing architecture:

- Extend the existing `model-provider` owner instead of adding a second provider registry, factory, catalog, capability resolver, or runtime stream abstraction.
- Extend existing login/auth-store and RMCP OAuth boundaries instead of adding a second OAuth parser, token store, or credential persistence layer.
- Extend existing MCP client/status/sanitization paths instead of adding parallel MCP result processors.
- Extend existing hook and shell owners instead of adding a second hook registry, permission parser, shell launcher, or sandbox policy evaluator.
- Inject diagnostics into model context only through bounded context-fragment architecture with hard caps.
- Preserve compatibility boundaries for persisted state, CLI commands, package metadata, app-server APIs, config keys, rollout/session data, and external integrations.

## High-Level Flows

### Provider Selection And Runtime

```text
configuration/model selection
  -> provider descriptor / provider kind
  -> create_model_provider
  -> runtime engine/client session
  -> retry/timeout/error telemetry
  -> CLI/TUI/doctor/app-server diagnostics
```

### OAuth And Auth Store

```text
login or MCP OAuth flow
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
| Provider descriptors/capabilities/runtime identity | `codex-rs/model-provider/` |
| Model catalog/provider metadata | `codex-rs/model-provider-info/`, `codex-rs/models-manager/` |
| Provider client retry/error/timeout telemetry | `codex-rs/codex-api/`, provider client crates |
| OAuth login exchange and URL sanitization | `codex-rs/login/` |
| MCP OAuth storage and refresh | `codex-rs/rmcp-client/` |
| MCP auth/status/tool-call behavior | `codex-rs/rmcp-client/`, `codex-rs/codex-mcp/` |
| Hook behavior | `codex-rs/hooks/` |
| Shell execution/escalation/sandboxing | `codex-rs/exec*`, `codex-rs/shell-*`, `codex-rs/sandboxing/` |
| Session/turn context | `codex-rs/core/`, `codex-rs/context-fragments/` |
| External-agent migration | `codex-rs/external-agent-migration/`, `codex-rs/external-agent-sessions/` |
| CLI user-facing behavior | `codex-rs/cli/` |
| TUI user-facing behavior | `codex-rs/tui/` |
| App-server API behavior | `codex-rs/app-server*` |
| Ontocode identity migration | `ONTOCODE_*.md`, compatibility owner crates, affected surface owners |

## Non-Negotiable Architecture Rules

- Use GitNexus context/impact before editing any code symbol.
- Prefer existing owners and test harnesses.
- Keep changes small and reviewable.
- Do not broaden `codex-core` without first checking whether another crate is the right owner.
- Do not create new public API/config/schema surfaces without ADR and compatibility tests.
- Security diagnostics must reuse or extend shared redaction behavior and include no-secret assertions.
- New model-context content must be bounded, capped, and implemented as context fragments.
