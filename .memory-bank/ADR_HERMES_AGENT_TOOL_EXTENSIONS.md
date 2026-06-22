# ADR: Hermes Agent Interop And Tool Extension Review

## Status

Consolidated - historical source evidence; dispatch via `ADR_EXTERNAL_AGENT_INTEROP_DETECTORS_CONSOLIDATION.md`

## Date

2026-06-07

## Context

The first version of this ADR stored 400 Hermes Agent-inspired extension candidates. After GitNexus review, most candidates are not Hermes-specific. They either extend existing Ontocode owners, duplicate prior ADR proposals, or are product integrations that should not be implemented as core architecture.

This ADR now owns only Hermes external-agent interop detection: finding Hermes configuration/state/plugin surfaces and producing a redacted dry-run report. Provider runtime, credential import, MCP, tools, hooks, shell, context, skills, diagnostics, and manager-loop tooling are delegated to existing owners or moved to lefties.

## Consolidation Override

This ADR is no longer an independent dispatch plan. Its retained Hermes Stage 0 requirements are consolidated into [External-Agent Interop Detector Consolidation](ADR_EXTERNAL_AGENT_INTEROP_DETECTORS_CONSOLIDATION.md).

If this ADR conflicts with the consolidation ADR, the consolidation ADR wins.

Dispatch rule:

- Do not implement `onto_hermes_interop_detector` as a separate detector stack unless it is a compatibility wrapper around the shared external-agent interop detector contract.
- Use the shared report envelope, redaction rules, source-specific Hermes requirements, and blocked-scope rules from the consolidation ADR.
- Keep this file as upstream/source evidence and historical disposition for the original Hermes Agent review.

## Hermes Source Evidence

Reviewed upstream at commit `cb3e41e2fd8253456b4a2958567b539a9a8ca322`.

- README feature overview: <https://github.com/NousResearch/hermes-agent/blob/cb3e41e2fd8253456b4a2958567b539a9a8ca322/README.md>
- Architecture map: <https://github.com/NousResearch/hermes-agent/blob/cb3e41e2fd8253456b4a2958567b539a9a8ca322/website/docs/developer-guide/architecture.md>
- Tools runtime: <https://github.com/NousResearch/hermes-agent/blob/cb3e41e2fd8253456b4a2958567b539a9a8ca322/website/docs/developer-guide/tools-runtime.md>
- Provider runtime resolution: <https://github.com/NousResearch/hermes-agent/blob/cb3e41e2fd8253456b4a2958567b539a9a8ca322/website/docs/developer-guide/provider-runtime.md>
- Model provider plugin guide: <https://github.com/NousResearch/hermes-agent/blob/cb3e41e2fd8253456b4a2958567b539a9a8ca322/website/docs/developer-guide/model-provider-plugin.md>
- Gateway internals: <https://github.com/NousResearch/hermes-agent/blob/cb3e41e2fd8253456b4a2958567b539a9a8ca322/website/docs/developer-guide/gateway-internals.md>
- Skills guide: <https://github.com/NousResearch/hermes-agent/blob/cb3e41e2fd8253456b4a2958567b539a9a8ca322/website/docs/developer-guide/creating-skills.md>
- Context compression and caching: <https://github.com/NousResearch/hermes-agent/blob/cb3e41e2fd8253456b4a2958567b539a9a8ca322/website/docs/developer-guide/context-compression-and-caching.md>
- Session storage: <https://github.com/NousResearch/hermes-agent/blob/cb3e41e2fd8253456b4a2958567b539a9a8ca322/website/docs/developer-guide/session-storage.md>
- Trajectory format: <https://github.com/NousResearch/hermes-agent/blob/cb3e41e2fd8253456b4a2958567b539a9a8ca322/website/docs/developer-guide/trajectory-format.md>

## GitNexus Challenge Evidence

GitNexus source-owner review found existing Ontocode owners for the major Hermes-inspired surfaces:

- External-agent detection/import already belongs to [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/config/external_agent_config.rs:260), [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/request_processors/external_agent_config_processor.rs:391), and [lib.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/external-agent-migration/src/lib.rs:227).
- Provider runtime work already belongs to [descriptor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/model-provider/src/descriptor.rs:7) and [provider.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/model-provider/src/provider.rs:108).
- Model-visible tools already flow through [spec_plan.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:160), [spec_plan.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:190), and extension dispatch tests in [router_tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/router_tests.rs:322).
- MCP status, OAuth, resources, and app-server exposure already belong to [connection_manager.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/codex-mcp/src/connection_manager.rs:551), [mcp/mod.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/codex-mcp/src/mcp/mod.rs:317), [session/mcp.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/session/mcp.rs:238), and [mcp_processor.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/request_processors/mcp_processor.rs:256).
- Skills and context injection already have extension/context owners in [extension.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/skills/src/extension.rs:68), [fragment.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/context-fragments/src/fragment.rs:46), and [contextual_user_message_tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/context/contextual_user_message_tests.rs:73).
- Shell, sandbox, and permissions must extend [shell_command.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/shell/shell_command.rs:143), [exec_policy.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/exec_policy.rs:631), and existing shell runtime tests.
- Diagnostics, rate limits, and status surfaces already exist in [card.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/tui/src/status/card.rs:125), [card.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/tui/src/status/card.rs:161), and [lib.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/response-debug-context/src/lib.rs:72).

Challenge result:

- Do not implement the original 400 items as independent tools.
- Do not add another provider registry, credential store, MCP manager, tool registry, hook runtime, shell backend, session DB, context compressor, gateway, skill loader, plugin manager, diagnostics bus, or trajectory exporter from this ADR.
- Keep this ADR limited to a redacted Hermes interop detector and dry-run report.
- Delegate generic architecture work to prior ADRs and existing owners.
- Move broad messaging gateways, desktop/web dashboard polish, media generation, voice, computer-use, serverless runtime, analytics, marketplace, and product packaging ideas to lefties.

## Prior ADR Delegation

- Provider engines, provider descriptors, provider capability metadata, model catalogs, native Claude/Gemini/Copilot behavior, and provider runtime quirks go to [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:1).
- Credential import, external auth adapters, provenance, redaction, and persistence decisions go to [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1).
- External provider adapter/runtime plugin concepts go to [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md:1).
- Memory-bank, GitNexus gates, manager-loop prompts, tracking-file automation, and implementation-card generators go to [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1).
- Non-natural product/media/gateway/UI/runtime items move to [ADR_HERMES_AGENT_TOOL_EXTENSIONS_LEFTIES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_HERMES_AGENT_TOOL_EXTENSIONS_LEFTIES.md:1).

## Retained Scope

The only retained implementation label is:

| Label | Scope | Owner | Acceptance |
|---|---|---|---|
| `onto_hermes_interop_detector` | Detect Hermes home/profile/repo surfaces and output a redacted dry-run report. | external-agent migration / app-server external-agent config | No mutation, no secret values, no plugin execution, fixture coverage for config, env names, providers, MCP, skills, plugins, sessions, cron, gateway config, and trajectory file presence. |

Supporting test label:

| Label | Scope | Acceptance |
|---|---|---|
| `onto_hermes_interop_detector_tests` | Fixture-driven tests for redaction and classification. | Fails if token, cookie, authorization header, API key value, OAuth refresh token, local secret path, raw message content, or media artifact content appears in the report. |

Stage 0 report fields:

- `home_detected`: whether a Hermes home directory was found.
- `profiles`: profile names and config-file presence only.
- `providers`: provider IDs, auth type names, base-url hostnames, model IDs, plugin manifest presence, and secret env var names only.
- `mcp`: server IDs, transport kinds, command names, URL hostnames, and OAuth metadata presence only.
- `skills`: skill names, declared tool/env requirements, size class, and helper-script presence only.
- `plugins`: plugin names, kinds, manifest paths, and quarantine recommendation.
- `sessions`: state DB presence, schema version if safely readable, and row counts only.
- `cron`: job count, schedule strings, and target platform type only.
- `gateway`: platform kinds and allowlist/pairing config presence only.
- `trajectories`: JSONL file count, approximate size, and schema-shape status only.
- `unsupported`: entries that require runtime/plugin/product decisions outside Stage 0.

## Original Point Disposition

This table covers every original point from `001` through `400`.

| Original points | Disposition | Similar solution in core / ADR | Architecture decision |
|---|---|---|---|
| 001-020 | Removed from current; delegated | `ProviderDescriptor`, `ProviderKind`, native provider ADR | Provider plugin/profile/catalog/quirk ideas must extend model-provider descriptors or native engine work. Hermes detector may only report provider IDs and manifests. |
| 021-040 | Removed from current; delegated | auth/login, RMCP OAuth, provider extensibility ADR | Auth and credential import ideas need credential evidence, provenance, overwrite/delete semantics, and redaction tests before persistence. Hermes detector may only report env names and auth type names. |
| 041-060 | Removed from current; delegated | core tool spec planning, extension tool dispatch, hooks | Tool/plugin schema ideas must extend existing tool/extension/hook owners. Hermes detector may only report manifest metadata and quarantine status. |
| 061-080 | Removed from current; delegated | `McpConnectionManager`, RMCP OAuth, app-server MCP processor | MCP ideas belong to codex-mcp/rmcp-client. Hermes detector may only report MCP server config presence and normalized non-secret fields. |
| 081-100 | Removed from current; delegated | `SkillsExtension`, bounded context fragments, lean-ctx ADR | Skill import/context ideas must use existing skill/context owners and hard caps. Hermes detector may only report skill metadata and size class. |
| 101-120 | Removed from current; delegated | external-agent sessions, rollout/search/session owners | Session DB and history import must not happen in this ADR. Hermes detector may only report schema/count metadata and never message content. |
| 121-140 | Removed from current; delegated | context fragments, compaction/session tests | Context engine/compression/prompt caching ideas require context owner review and hard caps. Hermes detector may only report config presence. |
| 141-149 | Removed from current; delegated | external-agent config, app-server/status surfaces | Generic gateway detection may be a Stage 0 report field only; runtime gateway behavior is not retained. |
| 150-160 | Moved to lefties | Hermes lefties | Telegram, Discord, Slack, WhatsApp, Signal, Matrix, SMS, Home Assistant, and similar chat gateways are product integrations, not core runtime. |
| 161-180 | Removed from current; delegated | scheduler would need separate owner/ADR | Cron config detection may be Stage 0 report data only. Scheduler/runtime behavior is not retained. |
| 181-200 | Removed from current; delegated | multi-agent tooling and lean-ctx ADR | Delegation/manager-loop ideas belong to existing multi-agent workflows or memory-bank operational tooling, not Hermes interop. |
| 201-220 | Removed from current; partly lefties | shell handler, sandbox, exec policy, Hermes lefties | Shell policy tests belong to shell owners. Modal/Daytona/Termux/packaging product runtime ideas move to lefties. |
| 221-240 | Removed from current; partly delegated | web-search extension, browser/security owners | Browser/web provider ideas must extend existing tool owners. Managed browser gateway/product fallback goes to lefties. |
| 241-260 | Moved to lefties except existing hosted tool checks | hosted image/web specs, Hermes lefties | Image/video/TTS/transcription/voice/computer-use are non-core unless separately approved by product/runtime ADR. |
| 261-280 | Removed from current; partly lefties | memories extension, memory-bank | Memory provider metadata may be reported; third-party memory sync/user modeling belongs to memory ADRs or lefties. |
| 281-300 | Removed from current; partly lefties | rollout/search/eval would need separate ADR | Trajectory files may be detected by name/size/schema only. Training-data export and batch generation go to lefties. |
| 301-320 | Removed from current; delegated | TUI status, response-debug-context, lean-ctx ADR | Diagnostics must extend existing status/debug owners. Operational checklists go to lean-ctx ADR. |
| 321-340 | Removed from current; delegated | plugin namespace/utils, external adapter ADR | Plugin manifest ideas belong to extension/adapter security ADRs. Hermes detector may only classify manifests without executing code. |
| 341-360 | Moved to lefties except app-server comparison notes | app-server protocol, TUI tests, Hermes lefties | ACP/TUI/dashboard/desktop/web UI parity ideas are product or protocol work requiring separate ADRs. |
| 361-380 | Kept only as Hermes Stage 0 detection inputs | external-agent config/migration owners | These points collapse into `onto_hermes_interop_detector`; no import, mutation, runtime enablement, or content migration. |
| 381-400 | Removed from current; delegated | lean-ctx project-tool ADR | Memory-bank validators, GitNexus templates, task cards, and readiness gates belong to the project tooling ADR. |

## Required Architecture Decisions For New Work

If future challenge reactivates any removed item, it must use one of these architecture decisions:

| Surface | Decision |
|---|---|
| Provider runtime | Extend model-provider descriptors and native/external provider adapter ADRs; no new provider registry. |
| Credential import | Target existing auth/login/RMCP stores explicitly; no broker and no persistence before redacted sample evidence. |
| MCP | Extend codex-mcp/rmcp-client/app-server MCP processors; no parallel manager. |
| Tools | Use existing tool spec planning, handlers, and extension registry; no Hermes-style self-registering runtime. |
| Skills/context | Use existing skills extension and bounded context fragments with hard caps. |
| Shell/policy | Extend shell runtime and exec policy with impact analysis and security tests. |
| Sessions | Use rollout/session/external-agent-session owners; no raw message import in Stage 0. |
| Plugins | Treat plugin manifests as inert/quarantined metadata until a trusted extension runtime ADR approves execution. |
| Product gateways/media/UI | Move to lefties unless a product ADR explicitly accepts scope. |
| Operations tooling | Keep in memory-bank/lean-ctx ADR as repo-only validators or prompts unless runtime exposure is separately approved. |

## Manager Guidance

Do not dispatch the original 400 rows. Dispatch only `onto_hermes_interop_detector` after creating a tracking file entry and running GitNexus context/impact on the external-agent detection owner. If sub-agents propose runtime provider, MCP, tool, skill, shell, media, gateway, or UI implementation from this ADR, reject and route to the delegated ADR or lefties file.
