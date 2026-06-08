# Memory Index

Keep this file short. Each entry should be one line in the form:
`- [Title](file.md) — one-line hook`

## Projects

- [Architecture](project_architecture.md) — Ontocode/Codex Rust workspace layout, provider/auth/MCP/session ownership, and change homes
- [Current Forward Plan](project_plan-current.md) — Active project plan for provider extensibility, OAuth/auth-store validation, MCP reliability, hooks/shell safety, context diagnostics, and import internals
- [Pending Tasks](project_pending-tasks.md) — Living backlog derived from `CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md`

## References

- [Agent Rules](reference_agent-rules.md) — Binding implementation rules for GitNexus, lean-ctx, Rust tests, architecture reuse, and Ontocode rename work
- [Project Plan Source](CLAUDE_CODE_APPROACHES_FOR_CODEBASE.md) — Core-natural approaches retained after Claude Code repository review
- [Project Plan Tracking](CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md) — Manager dispatch queue and current status
- [Lefties](CLAUDE_CODE_APPROACHES_LEFTIES.md) — Non-core or deferred ideas intentionally removed from the core plan

## ADRs And Tracking

- [Claude OAuth Provider Refactor](ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR.md) — Original Claude OAuth provider integration/refactor decision record
- [Provider Extensibility Remaining Implementation](ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md) — Remaining extensibility slices after provider selector/OAuth work
- [Native Heterogeneous Provider Engines](ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md) — Native Claude/Gemini/Copilot provider engine strategy
- [Public Adapter SDK And Schema Migrations](ADR_PUBLIC_ADAPTER_SDK_SCHEMA_MIGRATIONS.md) — Next-phase compatibility ADR for public adapter config, schema generation, SDK exposure, and migrations
- [Public Adapter SDK And Schema Migrations Tracking](ADR_PUBLIC_ADAPTER_SDK_SCHEMA_MIGRATIONS_TRACKING.md) — Active Stage 0 tracker for adapter public schema proposal, owner map, and compatibility tests
- [External-Agent Interop Detector Consolidation](ADR_EXTERNAL_AGENT_INTEROP_DETECTORS_CONSOLIDATION.md) — Dispatch authority for consolidated Gemini CLI, Hermes Agent, and GBrain redacted Stage 0 detectors
- [Gemini CLI Tool Extensions](ADR_GEMINI_CLI_TOOL_EXTENSIONS.md) — Historical source evidence; Gemini requirements consolidated into external-agent interop detector ADR
- [Hermes Agent Tool Extensions](ADR_HERMES_AGENT_TOOL_EXTENSIONS.md) — Historical source evidence; Hermes requirements consolidated into external-agent interop detector ADR
- [GBrain Tool Extensions](ADR_GBRAIN_TOOL_EXTENSIONS.md) — Historical source evidence; GBrain requirements consolidated into external-agent interop detector ADR
- [Lean-ctx Project Tool Extensions](ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md) — Inlined into GitNexus dependency consolidation; Stage 0 scripts remain bootstrap only
- [OpenCode Tool Extensions](ADR_OPENCODE_TOOL_EXTENSIONS.md) — Challenged OpenCode review retaining only the redacted Stage 0 interop detector
- [CliRelay Tool Extensions](ADR_CLIRELAY_TOOL_EXTENSIONS.md) — Challenged CliRelay review retaining only the redacted Stage 0 interop detector
- [OpenClaw Tool Extensions](ADR_OPENCLAW_TOOL_EXTENSIONS.md) — Challenged OpenClaw review retaining only the redacted Stage 0 interop detector
- [GitNexus Code-Graph Adoption](ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md) — Canonical consolidation ADR for GitNexus/lean-ctx third-party boundaries and operational evidence backbone
- [Ontocode Rename Tracking](ONTOCODE_RENAME_TRACKING.md) — Project identity migration tracker

## Audits

- [Memory Bank Initialization](audit_session-2026-06-07-memory-bank-initialization.md) — Initial Ontocode memory-bank bootstrap from GitNexus definition
- [OAuth/Auth-Store Validation Completion](audit_session-2026-06-07-oauth-validation-completion.md) — Verification and closure of the OAuth/auth-store validation epic
- [MCP Reliability Epic Completion](audit_session-2026-06-07-mcp-reliability-completion.md) — Verification and closure of the MCP reliability epic
- [Hook and Shell Safety Epic Completion](audit_session-2026-06-07-hook-shell-safety-completion.md) — Verification and closure of the hook/shell safety epic
- [External Adapter Protocol Safety Epic Completion](audit_session-2026-06-07-adapter-protocol-safety-completion.md) — Verification and closure of the external adapter protocol safety epic
- [Session and Context Bounded Diagnostics Epic Completion](audit_session-2026-06-07-session-diagnostics-completion.md) — Verification and closure of the session/context diagnostics epic
- [External-Agent Import Internals Epic Completion](audit_session-2026-06-07-external-agent-import-completion.md) — Verification and closure of the external-agent import internals epic
- [GitNexus Deinstall](audit_session-2026-06-08-gitnexus-deinstall.md) — Removal of active GitNexus repo integration, local index, and agent enforcement rules
- [GitNexus Reinstall](audit_session-2026-06-08-gitnexus-reinstall.md) — Restoration of GitNexus CLI, MCP config, skills, and codebase index
- [Lean-ctx Core Backbone Challenge](audit_session-2026-06-08-lean-ctx-core-backbone-challenge.md) — ADR challenge accepting only a narrow operational backbone contract for core
- [GitNexus Wrapper Path And Analyze Fix](audit_session-2026-06-08-gitnexus-wrapper-path-and-analyze-fix.md) — Local wrapper restored as `gitnexus`; analyzer schema gap fixed and repo-specific skills generated
- [Claude OAuth ADR Codebase Review](audit_session-2026-06-08-claude-oauth-adr-codebase-review.md) — GitNexus-backed review with addendum: runtime wiring exists, live validation remains blocked
- [Third-Party Dependency Consolidation](audit_session-2026-06-08-third-party-dependency-consolidation.md) — GitNexus and lean-ctx dependency boundaries consolidated into one operational evidence backbone
- [External-Agent Interop ADR Consolidation](audit_session-2026-06-08-external-agent-interop-adr-consolidation.md) — Gemini CLI, Hermes Agent, and GBrain interop ADRs consolidated into one detector contract
