# Memory Index

Keep this file short. Each entry should be one line in the form:
`- [Title](file.md) — one-line hook`

## Projects

- [Architecture](project_architecture.md) — Ontocode/Codex Rust workspace layout, provider/auth/MCP/session ownership, and change homes
- [Current Forward Plan](project_plan-current.md) — Active project plan for provider extensibility, OAuth/auth-store validation, MCP reliability, hooks/shell safety, context diagnostics, and import internals
- [Pending Tasks](project_pending-tasks.md) — Living backlog derived from `CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md`
- [Alpha Release Readiness](ALPHA_RELEASE_READINESS.md) — Release-prep baseline, accepted versioning decisions, remaining blockers, and cut checklist
- [Provider Credential Routing Refactor Project Plan](PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md) — Historical provider credential routing closure; not authority for new native non-OpenAI model OAuth
- [Provider Credential Routing Refactor Tracking](PROVIDER_CREDENTIAL_ROUTING_REFACTOR_TRACKING.md) — Historical dispatch and verification ledger for closed provider credential routing work
- [Multi-Provider OAuth Connection Routing ADR](ADR_MULTI_PROVIDER_OAUTH_CONNECTION_ROUTING.md) — Superseded route-based native OAuth design; current decision is OpenAI/Codex-native-only plus external API providers
- [Multi-Provider OAuth Donor Unblock Review](audit_session-2026-06-18-oauth-donor-unblock.md) — Senior donor review narrowing blocked OAuth gates using examples from `tmp/` without copying donor auth stacks
- [OpenAI-Only Provider Policy Cleanup](audit_session-2026-06-19-openai-only-provider-policy.md) — ADR and memory-bank cleanup replacing native multi-provider OAuth plans with OpenAI-native plus external API provider policy
- [Antigravity OAuth And CLIProxyAPI Hub ADR](ADR_ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB.md) — Superseded native Antigravity OAuth import decision; external sidecar evidence only
- [Antigravity Native Runtime Contract ADR](ADR_ANTIGRAVITY_RUNTIME_CONTRACT.md) — Superseded native runtime contract; external sidecar evidence only
- [Antigravity Native Runtime Approval ADR](ADR_ANTIGRAVITY_NATIVE_RUNTIME_APPROVAL.md) — Superseded native runtime approval gate; external sidecar evidence only
- [Antigravity Endpoint Compatibility Proof Runbook](ANTIGRAVITY_ENDPOINT_COMPATIBILITY_PROOF_RUNBOOK.md) — Minimal redacted fixture evidence required to unblock A6 runtime approval
- [Antigravity Redacted OAuth Record Fixture](ANTIGRAVITY_REDACTED_OAUTH_RECORD_FIXTURE.md) — Redacted shape of a user-supplied Antigravity OAuth credential; tokens intentionally omitted
- [Antigravity OAuth CLIProxyAPI Hub Tracking](ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB_TRACKING.md) — Dispatch and verification ledger for the Antigravity OAuth import slice
- [Antigravity OAuth Import Closure](audit_session-2026-06-17-antigravity-oauth-import-closure.md) — ADR-bounded Antigravity import accepted with focused parser, TUI import, and status tests passing
- [Kimi OAuth Import And Device Flow ADR](ADR_KIMI_OAUTH_CLIPROXY_IMPORT_AND_DEVICE_FLOW.md) — Reviewed import-first Kimi OAuth path from CLIProxyAPI donor evidence, with device flow and runtime gated
- [Kimi OAuth Import And Device Flow Tracking](KIMI_OAUTH_CLIPROXY_IMPORT_AND_DEVICE_FLOW_TRACKING.md) — Dispatch and verification ledger for ADR-bounded Kimi OAuth import/status work
- [Provider Auth Slash Command Menus ADR](ADR_PROVIDER_AUTH_SLASH_COMMAND_MENUS.md) — Superseded for non-OpenAI native OAuth menus; OpenAI/Codex remains native auth owner
- [Native Gemini OAuth And Sub-Agent Provider Concurrency ADR](ADR_NATIVE_GEMINI_OAUTH_AND_SUBAGENT_PROVIDER_CONCURRENCY.md) — Superseded native Gemini OAuth plan; external sidecar evidence only
- [OAuth And Model Functionality Plan Audit](audit_session-2026-06-18-oauth-model-plan-audit.md) — Current-state audit of completed, planned, stale, and blocked OAuth/model/provider work
- [Custom Sub-Agent Models ADR](ADR_CUSTOM_SUBAGENT_MODELS.md) — Accepted contract for exposing configured custom model overrides through `spawn_agent`
- [Custom Sub-Agent Models Tracking](ADR_CUSTOM_SUBAGENT_MODELS_TRACKING.md) — Dispatch and verification ledger for custom sub-agent model override work
- [Slash-Command Sub-Agent Management ADR](ADR_AGENT_SLASH_SUBAGENT_MANAGEMENT.md) — Proposed `/agent` and `/subagents` management surface over existing multi-agent tools
- [MCP Issues Status](MCP_ISSUES_STATUS.md) — Current MCP reliability closure, remaining risks, and owner boundaries
- [Ontocode Full Legacy Migration Project Plan](ONTOCODE_FULL_LEGACY_MIGRATION_PROJECT_PLAN.md) — Reviewed staged migration from `ontocode-rs` and remaining legacy Codex surfaces to Ontocode identities
- [Ontocode Full Legacy Migration Tracking](ONTOCODE_FULL_LEGACY_MIGRATION_TRACKING.md) — Active Stage 0 dispatch ledger for full legacy surface inventory and gated implementation
- [Ontocode Legacy Lefties Todo](ONTOCODE_LEGACY_LEFTIES_TODO.md) — Current actionable queue for remaining `codex` names after the Ontocode rename baseline
- [Ontocode Full Legacy Migration Stage 0 Review](ONTOCODE_FULL_LEGACY_MIGRATION_STAGE0_REVIEW.md) — Manager no-go review accepting Stage 0 matrices and blocking implementation until worktree and release/versioning gates are cleared
- [Lean-ctx Pre-Junior Project Plan](LEAN_CTX_PRE_JUNIOR_PROJECT_PLAN.md) — Junior-safe Stage 0 plan for the approved read-only lean-ctx-inspired repository helper script
- [Lean-ctx Pre-Junior Tracking](LEAN_CTX_PRE_JUNIOR_TRACKING.md) — Dispatch and verification ledger for the repository-only memory helper script
- [GitNexus Code-Graph Adoption Pre-Junior Project Plan](GITNEXUS_CODE_GRAPH_ADOPTION_PRE_JUNIOR_PROJECT_PLAN.md) — Junior-safe staged plan for the Rust operational evidence backbone
- [GitNexus Code-Graph Adoption Tracking](GITNEXUS_CODE_GRAPH_ADOPTION_TRACKING.md) — Dispatch and verification ledger for operational evidence backbone implementation
- [Gemini CLI Donor Pre-Junior Project Plan](GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_PRE_JUNIOR_PROJECT_PLAN.md) — Junior-safe retained Gemini CLI context-fidelity slice after duplicate hardening work moved to the Oh My Pi ADR
- [Oh My Pi Donor 200 Solutions Review](OH_MY_PI_DONOR_200_SOLUTIONS_REVIEW.md) — OntoIndex-backed inventory of useful donor ideas mapped to current owners and defer rules
- [Oh My Pi Donor 200 Solutions Challenge](OH_MY_PI_DONOR_200_SOLUTIONS_CHALLENGE.md) — per-proposal KEEP/NARROW/DEFER/REJECT review mapped to existing Ontocode owners
- [Oh My Pi Donor Non-Keep Proposals](OH_MY_PI_DONOR_200_SOLUTIONS_NON_KEEP.md) — NARROW/DEFER/REJECT donor ideas split out of the accepted Oh My Pi challenge list
- [Oh My Pi Donor Keep Refactor Map ADR](ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md) — accepted Oh My Pi donor rows mapped one-by-one to concrete refactor/test homes
- [Oh My Pi Donor Pre-Junior Project Plan](OH_MY_PI_DONOR_KEEP_REFACTOR_MAP_PRE_JUNIOR_PROJECT_PLAN.md) — junior-safe first-slice implementation plan for the accepted Oh My Pi donor refactor map
- [Claude Code Donor 200 Approaches Review](CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md) — 200 candidate donor approaches from `tmp/claude-code-main` mapped to Ontocode owner areas and first checks
- [Claude Code Donor Parked Rows Pre-Junior Plan](CLAUDE_CODE_DONOR_DEFERRED_NARROW_REJECT_PRE_JUNIOR_PROJECT_PLAN.md) — junior-safe triage-only plan for parked Claude Code donor rows; no direct implementation scope
- [jscpd Donor 40 Proposals Review](JSCPD_DONOR_40_PROPOSALS_REVIEW.md) — 40 duplication-detection, reporting, CI, and refactor-process ideas from `tmp/jscpd-main` mapped to Ontocode owner areas
- [Onto Memory Tools](../scripts/onto_memory_tools.py) — Read-only stdlib helper for memory status, task counts, and local markdown link checks

## References

- [Agent Rules](reference_agent-rules.md) — Binding implementation rules for GitNexus, lean-ctx, Rust tests, architecture reuse, and Ontocode rename work
- [Project Plan Source](CLAUDE_CODE_APPROACHES_FOR_CODEBASE.md) — Core-natural approaches retained after Claude Code repository review
- [Project Plan Tracking](CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md) — Manager dispatch queue and current status
- [Lefties](CLAUDE_CODE_APPROACHES_LEFTIES.md) — Non-core or deferred ideas intentionally removed from the core plan

## ADRs And Tracking

- [Claude OAuth Provider Refactor](ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR.md) — Original Claude OAuth provider integration/refactor decision record
- [Provider Extensibility Remaining Implementation](ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md) — Remaining extensibility slices after provider selector/OAuth work
- [Native Heterogeneous Provider Engines](ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md) — Native Claude/Gemini/Copilot provider engine strategy
- [Claude Code Donor Core Extension Review](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW.md) — ADR gate for the 200 Claude Code donor ideas, classifying newness, core-extension value, and keep/narrow/defer/reject outcomes
- [Claude Code Donor Deferred/Narrow/Rejected](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md) — parked Claude Code donor rows that are not active core-extension work
- [jscpd Donor Core Extension Review](ADR_JSCPD_DONOR_CORE_EXTENSION_REVIEW.md) — ADR gate for jscpd donor ideas, keeping only new core-extension candidates for generated code/text quality
- [Gemini CLI Donor Context/Tools/Agents/Evals](ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS.md) — accepted planning inventory for context/memory, file/search/tools, MCP/hooks/extensions, agents/subagents, and automation/evals donor ideas
- [Gemini CLI Donor Deferred TODO](ADR_GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_DEFERRED_TODO.md) — deferred, narrowed, and rejected donor ideas parked outside the active Gemini CLI donor ADR
- [Native Heterogeneous Provider Engines](ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md) — Superseded for new non-OpenAI native model engines; external API provider sidecars are the current path
- [Public Adapter SDK And Schema Migrations](ADR_PUBLIC_ADAPTER_SDK_SCHEMA_MIGRATIONS.md) — Next-phase compatibility ADR for public adapter config, schema generation, SDK exposure, and migrations
- [Public Adapter SDK And Schema Migrations Tracking](ADR_PUBLIC_ADAPTER_SDK_SCHEMA_MIGRATIONS_TRACKING.md) — Active Stage 0 tracker for adapter public schema proposal, owner map, and compatibility tests
- [External-Agent Interop Detector Consolidation](ADR_EXTERNAL_AGENT_INTEROP_DETECTORS_CONSOLIDATION.md) — Dispatch authority for consolidated Gemini CLI, Hermes Agent, and GBrain redacted Stage 0 detectors
- [Gemini CLI Tool Extensions](ADR_GEMINI_CLI_TOOL_EXTENSIONS.md) — Historical source evidence; Gemini requirements consolidated into external-agent interop detector ADR
- [Hermes Agent Tool Extensions](ADR_HERMES_AGENT_TOOL_EXTENSIONS.md) — Historical source evidence; Hermes requirements consolidated into external-agent interop detector ADR
- [GBrain Tool Extensions](ADR_GBRAIN_TOOL_EXTENSIONS.md) — Historical source evidence; GBrain requirements consolidated into external-agent interop detector ADR
- [Lean-ctx Project Tool Extensions](ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md) — Inlined into GitNexus dependency consolidation; Stage 0 scripts remain bootstrap only
- [Native Context Tools Core Engine ADR](ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md) — Proposed 4+2 path for bounded read/search/shell compression inside existing Ontocode core owners
- [Native Context Tools Core Engine Project Plan](NATIVE_CONTEXT_TOOLS_CORE_ENGINE_PROJECT_PLAN.md) — Pre-junior C0 plan for shell output reducers through existing core formatting
- [Native Context Tools Core Engine Tracking](NATIVE_CONTEXT_TOOLS_CORE_ENGINE_TRACKING.md) — Dispatch and verification ledger for the reviewed C0 shell-output reducer slice
- [Native Context Tools C0 Closure](audit_session-2026-06-15-native-context-tools-c0-closure.md) — C0 shell-output reducers accepted with focused tests passing; broad core tests blocked by existing test-binary-support alias collision
- [Native Context Tools Core Verification Unblock](audit_session-2026-06-15-native-context-tools-core-unblock.md) — arg0 duplicate execve-wrapper alias fixed; broad `ontocode-core` tests pass with a fresh isolated `TMPDIR`
- [OpenCode Tool Extensions](ADR_OPENCODE_TOOL_EXTENSIONS.md) — Challenged OpenCode review retaining only the redacted Stage 0 interop detector
- [CliRelay Tool Extensions](ADR_CLIRELAY_TOOL_EXTENSIONS.md) — Challenged CliRelay review retaining only the redacted Stage 0 interop detector
- [OpenClaw Tool Extensions](ADR_OPENCLAW_TOOL_EXTENSIONS.md) — Challenged OpenClaw review retaining only the redacted Stage 0 interop detector
- [GitNexus Code-Graph Adoption](ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md) — Canonical consolidation ADR for GitNexus/lean-ctx third-party boundaries and operational evidence backbone
- [GitNexus Operational Evidence Closure](audit_session-2026-06-16-gitnexus-operational-evidence-closure.md) — S0-S9 accepted for the Rust operational evidence ledger; S10 context fragment remains ADR-blocked
- [Gemini OAuth Donor Transfer Closure](audit_session-2026-06-16-gemini-oauth-donor-transfer-closure.md) — S2-D/S2-E/S3-A/S5-A accepted; remaining Gemini OAuth work is blocked on external runtime/login/API gates
- [Gemini OAuth S7 Unblock](audit_session-2026-06-16-gemini-oauth-s7-unblock.md) — official Gemini API OAuth docs unblock S7 normal Gemini bearer-auth work; Cloud Code Assist and bundled login stay blocked
- [Gemini OAuth S6/S7 Closure](audit_session-2026-06-16-gemini-oauth-s6-s7-closure.md) — S6-A user-supplied OAuth import and S7-A normal Gemini bearer auth accepted; only external-gated Gemini work remains
- [Ontocode Rename Tracking](ONTOCODE_RENAME_TRACKING.md) — Project identity migration tracker
- [Ontocode Binary Rename Proposal](ONTOCODE_BINARY_RENAME_PROPOSAL.md) — GitNexus-backed staged proposal for making `ontocode` canonical while preserving `codex`
- [R5BP OTEL Rename Worker Verification](audit_session-2026-06-12-r5bp-otel-rename-worker-verification.md) — exact OTEL package/lib/Bazel/import rename verification on `gpt-5.4-mini`; OTEL package tests passed, exact old-name refs were removed, and the restored baseline reports 119 remaining `codex-*` packages.
- [R5BP OTEL Stale Baseline Blocker](audit_session-2026-06-12-r5bp-otel-stale-baseline-blocker.md) — manager rejected R5BP because metadata regressed to 119 `codex-*` packages; R5BP-U1 recovery must restore accepted Ontocode identities before further dispatch.
- [R5BP OTEL Recovery Closure](audit_session-2026-06-12-r5bp-otel-recovery-closure.md) — R5BP accepted after controlled recovery; exactly five `codex-*` Cargo packages remain.
- [R5BQ Extension API Rename Risk Review](audit_session-2026-06-12-r5bq-extension-api-rename-risk-review.md) — dispatch guardrails for the extension-api identity-only rename.
- [R5BQ Extension API Rename Worker Verification](audit_session-2026-06-12-r5bq-extension-api-rename-worker-verification.md) — worker verification for the extension-api identity-only rename; confirms the new `ontocode-extension-api` / `ontocode_extension_api` identity and the residual four `codex-*` package set.
- [R5BQ Extension API Rename Closure](audit_session-2026-06-12-r5bq-extension-api-rename-closure.md) — extension-api identity-only rename accepted; exactly four `codex-*` Cargo packages remain.
- [R5BR State Rename Risk Review](audit_session-2026-06-12-r5br-state-rename-risk-review.md) — dispatch guardrails for the state identity-only rename with CRITICAL SQLite telemetry impact.
- [R5BR State Rename Closure](audit_session-2026-06-12-r5br-state-rename-closure.md) — state identity-only rename accepted; exactly three `codex-*` Cargo packages remain.
- [R5BS Tools Rename Risk Review](audit_session-2026-06-13-r5bs-tools-rename-risk-review.md) — dispatch guardrails for the tools identity-only rename with CRITICAL tool-surface impact.
- [R5BS Tools Rename Closure](audit_session-2026-06-13-r5bs-tools-rename-closure.md) — tools identity-only rename accepted; exactly two protocol-gated `codex-*` Cargo packages remain.
- [R5BT App-Server Protocol Rename Risk Review](audit_session-2026-06-13-r5bt-app-server-protocol-rename-risk-review.md) — dispatch guardrails for the user-approved protocol-sensitive app-server protocol identity rename.
- [R5BT App-Server Protocol Rename Closure](audit_session-2026-06-13-r5bt-app-server-protocol-rename-closure.md) — manager verification accepted the app-server protocol identity rename; only `codex-protocol` remains.
- [R5BU Protocol Rename Closure](audit_session-2026-06-13-r5bu-protocol-rename-closure.md) — manager verification accepted the final protocol identity rename; zero `codex-*` Cargo packages remain.

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
- [OntoIndex SummaryNode Import Fix](audit_session-2026-06-13-ontoindex-summarynode-import-fix.md) — Local OntoIndex import repair after `SummaryNode` schema drift broke `ontoindex analyze` for the ontocode repo
- [Alpha Release Readiness Baseline](audit_session-2026-06-13-alpha-release-readiness-baseline.md) — Release-prep baseline accepted without breaking the `0.0.0` source-build sentinel; native Copilot version headers now derive from crate version
- [Alpha Release Prep Update](audit_session-2026-06-13-alpha-release-prep-update.md) — Confirmed npm/Python version injection owners, set `0.1.0-alpha.1` as the default first-alpha candidate, and narrowed remaining release gates
- [Ontocode Binary Launcher Fix](audit_session-2026-06-13-ontocode-binary-launcher-fix.md) — Replaced the fragile `ontocode` wrapper with the real CLI entrypoint and kept alias coverage without duplicate bin-unit test failures
- [Ontocode Dev Binary Verification](audit_session-2026-06-13-ontocode-dev-binary-verification.md) — Fresh source-built `ontocode` now runs directly and brands itself correctly after the launcher fix
- [Ontocode Release Verification And Help Copy](audit_session-2026-06-13-ontocode-release-verification-and-help-copy.md) — Clean release-profile `ontocode` verification passed; main CLI help text now uses `Ontocode`, and `ontocode-rs/` is recorded as layout debt rather than a runtime blocker
- [Provider Refresh Orchestrator S3-A](audit_session-2026-06-13-provider-refresh-orchestrator-s3a.md) — Shared refresh-orchestrator contract accepted in `ontocode-provider-auth`; existing login and RMCP owners now expose thin adapters without moving refresh authority
- [Provider Scheduler S4 Closure](audit_session-2026-06-13-provider-scheduler-s4-closure.md) — Private scheduler core accepted in `model-provider` with round-robin, priority, failover, and sticky-session behavior over normalized credential and refresh state
- [Provider Auth Contract S5 Closure](audit_session-2026-06-13-provider-auth-contract-s5-closure.md) — Senior review accepted the existing private `ModelProvider` auth seam as the correct S5 contract and rejected adding a parallel provider-auth trait family
- [Provider Canonical OAuth F1-A](audit_session-2026-06-13-provider-canonical-oauth-f1a.md) — Additive canonical secret-bearing OAuth credential type landed in `ontocode-provider-auth` with redacted debug behavior and routing-view projection
- [Provider Canonical OAuth F1-B](audit_session-2026-06-13-provider-canonical-oauth-f1b.md) — Existing OpenAI/login, RMCP OAuth, and Claude import owners now project into the canonical internal OAuth credential type without changing persistence authority
- [Provider Canonical OAuth F1-C F1-E Closure](audit_session-2026-06-13-provider-canonical-oauth-f1c-f1e-closure.md) — Copilot canonical-source/runtime split and canonical-to-routing redaction coverage are verified complete; Gemini OAuth ownership remains the next gap
- [Provider Gemini F1-D Closure](audit_session-2026-06-13-provider-gemini-f1d-closure.md) — Senior design closure keeps Gemini explicitly API-key-only until a real OAuth source owner exists; compatibility coverage now guards that boundary
- [Claude OAuth ADR Codebase Review](audit_session-2026-06-08-claude-oauth-adr-codebase-review.md) — GitNexus-backed review with addendum: runtime wiring exists, live validation remains blocked
- [Third-Party Dependency Consolidation](audit_session-2026-06-08-third-party-dependency-consolidation.md) — GitNexus and lean-ctx dependency boundaries consolidated into one operational evidence backbone
- [External-Agent Interop ADR Consolidation](audit_session-2026-06-08-external-agent-interop-adr-consolidation.md) — Gemini CLI, Hermes Agent, and GBrain interop ADRs consolidated into one detector contract
