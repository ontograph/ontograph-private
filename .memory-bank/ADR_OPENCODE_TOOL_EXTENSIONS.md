# ADR: OpenCode Inspired Tool Extensions Review

## Status

Challenged - OpenCode Interop Stage 0 Only

## Date

2026-06-07

## Context

OpenCode is an open-source AI coding agent with a TypeScript/Bun core, headless server, TUI, desktop/web apps, plugins, MCP support, provider adapters, LSP integration, command automation, and durable session runtime concepts.

This ADR originally stored 400 OpenCode-inspired extension candidates. After GitNexus review, the broad catalog is not accepted as an implementation backlog. Most candidates duplicate current Ontocode owners or belong in prior ADRs. The only approved local direction is an inert OpenCode interop detector/report that helps Ontocode understand whether a workspace already uses `.opencode` artifacts, without importing content, executing plugin code, loading auth plugins, fetching remote config, changing app-server APIs, or changing runtime behavior.

Reviewed upstream at commit `e82542b8023a8374f29c23b70ec019c8f256354e`.

## OpenCode Source Evidence

- Repository and README: <https://github.com/anomalyco/opencode/tree/e82542b8023a8374f29c23b70ec019c8f256354e>
- Agent modes in README: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/README.md>
- Session runtime concepts: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/CONTEXT.md>
- Tool definition and truncation wrapper: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/tool/tool.ts>
- Managed tool-output truncation: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/tool/truncate.ts>
- Agent definitions and per-agent permissions: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/agent/agent.ts>
- Permission evaluator and pending permission flow: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/permission/index.ts>
- Config loading, remote config, variables, and plugin origins: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/config/config.ts>
- Plugin spec resolution and provenance: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/config/plugin.ts>
- Plugin loader pipeline: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/plugin/loader.ts>
- MCP client, status, OAuth, and tolerant schema fallback: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/mcp/index.ts>
- Provider catalog and bundled provider loaders: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/provider/provider.ts>
- Session HTTP API shape: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/server/routes/instance/httpapi/groups/session.ts>
- Interactive prompt queue: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/cli/cmd/run/runtime.queue.ts>
- LSP service: <https://github.com/anomalyco/opencode/blob/e82542b8023a8374f29c23b70ec019c8f256354e/packages/opencode/src/lsp/lsp.ts>
- Project-local OpenCode commands/tools examples: <https://github.com/anomalyco/opencode/tree/e82542b8023a8374f29c23b70ec019c8f256354e/.opencode>

## GitNexus Challenge Evidence

GitNexus found existing Ontocode owners for the major OpenCode surfaces:

- Multi-agent roles and model/provider selection already have owners: [multi_agents_spec.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs:79), [multi_agents_tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/multi_agents_tests.rs:243), [turn_context.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/session/turn_context.rs:456)
- Model-visible tool planning and tool output truncation already have owners: [spec_plan.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:160), [truncation.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/tests/suite/truncation.rs:121), [code_mode.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/tools/src/code_mode.rs:7)
- Shell execution, sandboxing, and approval policy already have owners: [shell.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/shell.rs:59), [sandboxing.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/sandboxing.rs:245), [exec_policy_tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/exec_policy_tests.rs:2138)
- MCP status, resources, config, OAuth, and CLI behavior already have owners: [mcp/mod.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/codex-mcp/src/mcp/mod.rs:317), [connection_manager.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/codex-mcp/src/connection_manager.rs:616), [mcp_cmd.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/cli/src/mcp_cmd.rs:272)
- Provider descriptors, capabilities, rate-limit/status output, and provider tests already have owners: [descriptor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/model-provider/src/descriptor.rs:33), [provider.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/model-provider/src/provider.rs:543), [card.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/tui/src/status/card.rs:161)
- External-agent detection/import already has owners: [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/config/external_agent_config.rs:260), [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/request_processors/external_agent_config_processor.rs:391), [external_agent_config_tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/config/external_agent_config_tests.rs:38)
- App-server thread/session APIs and fork/resume behavior already have owners: [thread_fork.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/tests/suite/v2/thread_fork.rs:252), [app_server_session.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/tui/src/app_server_session.rs:1685), [bespoke_event_handling.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/bespoke_event_handling.rs:134)
- Diagnostics, doctor output, feedback reports, and redaction tests already have owners: [output.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/cli/src/doctor/output.rs:1298), [feedback_doctor_report.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/request_processors/feedback_doctor_report.rs:36), [thread_resume_redaction.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/request_processors/thread_resume_redaction.rs:66)

## Decision

Accept only one Stage 0 task from this ADR:

| Label | Decision | Owner | Implementation boundary |
|---|---|---|---|
| `onto_opencode_interop_detector` | Keep | external-agent config migration / diagnostics | Detect `.opencode` workspace artifacts and produce a redacted dry-run report. No plugin execution, command import, auth import, MCP mutation, provider mutation, remote config fetch, app-server API exposure, model-context injection, or shell execution. |
| `onto_opencode_interop_detector_tests` | Keep | external-agent config migration tests | Add fixture tests proving redaction, bounded output, no executable import, no credential leakage, no remote fetch, and stable classification of detected artifacts. |

Everything else from the original 400 candidates is removed from this ADR and handled by one of three outcomes:

- Delegated to an existing core owner and prior ADR.
- Blocked until a separate ADR proves runtime need and compatibility.
- Moved to [ADR_OPENCODE_TOOL_EXTENSIONS_LEFTIES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_OPENCODE_TOOL_EXTENSIONS_LEFTIES.md:1) because it is not a natural core extension.

## Approved Stage 0 Detector Scope

The detector may inspect only local file names, manifest metadata, config keys, env var names, counts, and redacted endpoint identities.

Allowed findings:

- `.opencode` presence and file inventory: config, commands, tools, skills, themes, plans, glossary, and TUI config counts.
- Config metadata: file path, valid/invalid JSONC shape, top-level key names, deprecated-key presence, and remote-config references as disabled metadata only.
- Agent metadata: names, modes, hidden/native flags, model/provider ID names, and permission rule counts. No prompt body import.
- Permission metadata: permission names, pattern counts, env-file rule presence, and external-directory rule presence. No runtime policy changes.
- MCP metadata: server names, transport kind, endpoint host only, OAuth-required marker, and tool/prompt/resource count if statically declared. No MCP login or config mutation.
- Provider metadata: provider IDs, model IDs, env var names, option key names, and auth source names. No credential import or provider registry mutation.
- Plugin metadata: plugin specs, source kind, declared options key names, server/TUI entrypoint presence if statically detectable. No package install or dynamic import.
- Project command/tool/skill/theme metadata: names, paths, counts, and compatibility classification. No command/tool execution.
- LSP metadata: server names, command names, extension lists, and disabled/enabled state. No LSP process spawn.
- Diagnostic matrix: compatible, delegated, blocked, lefties, and unknown counts.

Rejected Stage 0 behavior:

- No plugin loading, package installation, command execution, shell execution, LSP spawn, auth plugin execution, remote config fetch, or browser OAuth flow.
- No import of OpenCode agents, commands, tools, skills, providers, plugins, MCP servers, themes, credentials, sessions, or permissions.
- No new provider registry, MCP manager, tool registry, context epoch runtime, permission engine, plugin runtime, app-server API, LSP runtime, session store, package manager, or desktop/web UI.
- No model-visible tool exposure.
- No app-server API exposure without a later compatibility ADR.
- No credential persistence.
- No unbounded context injection.

## Architecture Decision For New Work

If implemented, the detector should extend the existing external-agent detection/import boundary:

- Add OpenCode as another inert external-agent detection source beside existing external-agent config detection.
- Keep report structs redacted and serializable for diagnostics, but do not expose public config or app-server APIs in the first slice.
- Reuse existing redaction helpers and import-report conventions.
- Keep implementation outside `codex-core` unless a later ADR proves it must affect runtime behavior.
- Add tests under the existing external-agent migration/config processor test patterns.
- Run GitNexus impact before editing any detection symbol, especially `ExternalAgentConfigService::detect_migrations` or app-server request processors.

## Original Point Disposition

Every original point is covered below. Contiguous ranges are used because the challenged candidates share the same owner and decision.

| Original points | Disposition | Similar solution or owner | Action |
|---|---|---|---|
| `001-020` | Stage 0 may detect agent metadata only; runtime agent/profile work is delegated or blocked. | Multi-agent roles, turn context, and tool specs already exist. | No OpenCode agent runtime in this ADR. |
| `021-040` | Delegated or blocked. | Bounded context fragments and session turn context own context behavior. | New context epoch/source semantics require a separate context ADR. |
| `041-060` | Delegated. | Tool specs, tool routing, truncation tests, and code-mode tools already exist. | No second tool registry or output-retention system. |
| `061-080` | Stage 0 may detect permission metadata only; runtime policy changes are delegated or blocked. | Shell/sandbox/exec policy already owns approvals and filesystem authority. | No OpenCode wildcard permission engine. |
| `081-100` | Stage 0 may detect config shape only; remote config and schema changes are blocked. | Config loader, external-agent detection, and app-server config APIs already exist. | No remote config fetch or public schema change here. |
| `101-120` | Stage 0 may detect plugin specs only; execution is blocked or delegated to adapter ADRs. | Extension/plugin and external adapter ADRs own executable plugins. | No OpenCode plugin runtime. |
| `121-140` | Stage 0 may detect MCP metadata only; runtime MCP behavior is delegated. | `codex-mcp`, `rmcp-client`, MCP CLI, and connection manager already exist. | No second MCP manager. |
| `141-160` | Stage 0 may detect provider metadata only; runtime provider work is delegated. | Provider descriptors/native/external adapter ADRs own providers. | Move provider proposals to provider ADRs. |
| `161-180` | Stage 0 may detect auth metadata/env names only; auth import is blocked. | Login, RMCP OAuth, external-agent migration, and provider auth ADRs own credentials. | No auth plugin import or credential persistence. |
| `181-200` | Stage 0 may detect LSP config only; runtime LSP tools are blocked until a separate ADR proves they complement GitNexus. | GitNexus already owns code intelligence; no LSP runtime owner is approved. | Move most LSP runtime proposals to lefties. |
| `201-220` | Delegated or blocked. | TUI/session orchestration already owns prompt queue and cancellation behavior. | No prompt queue rewrite here. |
| `221-240` | Delegated or blocked. | App-server v2 thread/session APIs and compatibility rules already exist. | Public API changes require separate ADR and tests. |
| `241-260` | Delegated or moved to lefties. | Git utilities, turn diff, patch, GitNexus, and project tooling already exist. | No duplicate git/code-intelligence layer. |
| `261-280` | Delegated. | Patch/apply_patch, filesystem, attachments, redaction, and tests already have owners. | No new file mutation architecture. |
| `281-300` | Stage 0 may detect command/tool/skill/theme artifacts only; execution/import is blocked. | Skills, commands, external-agent migration, and TUI theme owners would need explicit acceptance. | No executable project automation import. |
| `301-320` | Delegated to test/diagnostic/tooling ADRs. | Existing core suite, app-server suite, provider tests, MCP tests, insta snapshots, and lean-ctx project tooling exist. | No new test framework. |
| `321-340` | Moved to lefties or release tooling. | Packaging/release/install surfaces are not natural core runtime extensions. | No packaging work from this ADR. |
| `341-360` | Moved to lefties or app-owner backlog. | Desktop/web UI product surfaces are outside current core-extension scope. | No desktop/web UI work from this ADR. |
| `361-380` | Delegated to diagnostics/security/tooling owners. | Doctor, feedback report, status, redaction, and lean-ctx project tooling exist. | No duplicate diagnostics framework. |
| `381-382` | Kept. | External-agent detection/report boundary exists. | Implement only redacted OpenCode interop detector/report. |
| `383-390` | Delegated to owning ADRs as compatibility matrices. | Provider, MCP, context, policy, app-server, and GitNexus owners exist. | Record in addendums; no implementation in this ADR. |
| `391-400` | Delegated to lean-ctx project tooling or moved to lefties. | Memory-bank challenge/readiness tools belong in project tooling. | Remove from this ADR; readiness gate is satisfied by this challenge. |

## Cross-ADR Delegations

The following prior ADRs now own relevant surviving proposals:

- [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1) owns OpenCode-derived ADR challenge checklists, source-link verification, duplicate-proposal detection, implementation-label generation, test-command planning, redaction acceptance templates, and affordability splitting.
- [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1) owns any future OpenCode credential/import evidence, OAuth/env classification, auth plugin quarantine, and redacted import-report behavior.
- [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:1) owns OpenCode-inspired provider capability, bundled provider evidence, model catalog, model status, and rate-limit diagnostics when they extend native provider descriptors.
- [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md:1) owns OpenCode-style external plugin/provider adapter ideas only if they become explicit external provider adapter capabilities under the approved adapter transport.

## Implementation Task Card

Use this card if a low-capability agent implements the accepted slice:

- Task: implement `onto_opencode_interop_detector`.
- Scope: redacted dry-run detection/report only.
- Before editing: run GitNexus context and impact for the target detection/import symbol.
- Files to inspect first: external-agent config detection, external-agent config processor tests, OAuth import redaction tests, MCP config tests, and doctor diagnostic redaction tests.
- Acceptance tests: fixture with `.opencode/opencode.jsonc`, fixture with command markdown, fixture with custom tool TypeScript, fixture with skill `SKILL.md`, fixture with plugin specs, fixture with MCP/provider/auth/LSP config, fixture with token-like values, fixture with remote config URL.
- Expected result: report contains names/counts/kinds/key names only; report never contains raw tokens, authorization headers, cookies, database URLs, credential file contents, prompt bodies, command bodies, plugin code, shell output, provider secrets, or remote config response content.
- Out of scope: importing OpenCode config, executing commands/tools/plugins, installing packages, fetching remote config, logging in to MCP/OAuth, spawning LSPs, changing providers, adding model-visible tools, adding app-server APIs, or adding desktop/web UI behavior.

## Challenge Outcome

The current codebase is not missing an OpenCode-like runtime layer. Ontocode already has distinct owners for agents, tools, context, permissions, MCP, providers, auth, app-server APIs, diagnostics, external-agent migration, and GitNexus code intelligence. The useful OpenCode extension is therefore interop awareness: detect and report existing `.opencode` workspace configuration safely so a future ADR can decide whether any deeper import is justified by real user evidence.
