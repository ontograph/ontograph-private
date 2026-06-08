# Claude Code Repository Review: Core-Natural Approaches

Source reviewed: `tmp/claude-code-main`

This file keeps only approaches that naturally extend existing Codex core functionality. Product packaging, marketplace growth, broad enterprise deployment, workflow automation, and non-core UX ideas are moved to `CLAUDE_CODE_APPROACHES_LEFTIES.md`.

## GitNexus Anchors

GitNexus query used:

`provider runtime auth MCP hooks session turn context sandbox shell execution external agent import app-server diagnostics`

Retained work maps to indexed core surfaces:

- `codex-rs/model-provider/src/descriptor.rs`: provider descriptors and native engine selection
- `codex-rs/model-provider/src/provider.rs`: provider construction, runtime engine, capabilities, account state
- `codex-rs/core/src/client.rs`: native runtime stream dispatch and normalized response events
- `codex-rs/login/src/auth/manager.rs`: auth persistence and refresh behavior
- `codex-rs/app-server/src/request_processors/external_agent_config_processor.rs`: external-agent config import boundary
- `codex-rs/app-server/src/config/external_agent_config.rs`: external-agent import service and hooks/MCP migration
- `codex-rs/core/src/mcp_tool_call.rs`: MCP tool execution and connector auth refresh
- `codex-rs/codex-mcp/src/mcp/mod.rs`: MCP status/resource surfaces
- `codex-rs/hooks/src/engine`: hook governance, validation, and managed-hook behavior
- `codex-rs/core/src/session/turn_context.rs`: turn configuration, context, and runtime inheritance
- `codex-rs/core/src/tools/runtimes/shell`: shell execution, permission, and sandbox behavior
- `codex-rs/core/src/tools/handlers/multi_agents_tests.rs`: multi-agent sandbox/approval/service-tier inheritance
- `codex-rs/tui/src/app/test_support.rs`: UI-facing diagnostics and regression support

## Challenge Verdict

- Keep: provider/runtime correctness, auth safety, MCP reliability, hook governance, sandbox permissions, shell execution, session/context safety, external-agent import internals, diagnostics, and test harnesses.
- Reject from this backlog: marketplace/product features, plugin distribution, PR/review workflows, MDM examples, broad enterprise policy UX, background daemon product work, SDK/headless compatibility promises, app-server public API expansion, and TUI polish that is not diagnostic or correctness-driven.
- Bias implementation toward tests, private structs, bounded diagnostics, and internal validators before public schemas, config keys, app-server APIs, or SDK behavior.
- Treat `L` options as ADR-only unless they stay internal; product dashboards, support bundles, public app-server methods, SDK modes, and marketplace/enterprise controls are not dispatchable from this backlog.
- Run GitNexus impact before editing provider construction, auth manager, session turn context, MCP tool calls, app-server protocol processors, hook execution, or shell runtime symbols.

Implementation option key:

- `S`: smallest useful slice, usually tests, diagnostics, or private helper.
- `M`: internal implementation or existing UI/app-server integration.
- `L`: larger architecture, schema, or cross-surface change that needs ADR/review.

## Dispatchable Epics

Use these epics for sub-agent dispatch. Individual retained rows below are source material, not independent tasks.

| Epic | Included IDs | Dispatch Rule |
| --- | --- | --- |
| Provider provenance/status/capability diagnostics | 2, 3, 4, 8, 9, 12, 13, 14 | Start with tests and internal status structs; no dashboard or public config. |
| OAuth/auth-store validation and redacted diagnostics | 5, 6, 7, 11, 160 | Must include malformed fixtures and no-secret assertions before any repair/migration work. |
| MCP reliability and auth hardening | 141-146, 149-151, 155-157 | Start in MCP client/status/test surfaces; avoid catalog UX and managed connector controls. |
| Hook and shell permission safety | 47-51, 53-56, 58-63, 161, 166-168, 172, 174, 175 | Prefer regression tests before engine changes; security bypass tests are the acceptance gate. |
| External adapter protocol safety | 16-30 | Fixtures, frame parsing, redaction, bounds, and handshake gates first; no adapter SDK until ADR. |
| Session/context bounded diagnostics | 1, 87, 89, 100, 104, 109-111, 117, 119-121, 124, 130, 132, 140, 181, 185 | Every model-context diagnostic needs hard caps and structured fragments. |
| External-agent import internals | 213-215, 217, 218, 220 | Consent, provenance, dry-run, redaction, and security review are mandatory; no public detect API yet. |

## Reuse Before Build

Every dispatched task must prove reuse before implementation:

- Run GitNexus `context` on the target symbol/module and record the existing caller/callee surface in the tracking update.
- Extend an existing function, struct, trait, enum, processor, test harness, or crate boundary when one already owns the behavior.
- Add a new module only when the existing module would become too large or would mix unrelated concepts; the new module must still plug into the existing owner.
- Prefer existing test harnesses and fixtures over new bespoke test utilities.
- If reuse is impossible, document the gap and the reason before writing code.

## Required Existing Anchors

| Epic | Must Reuse These Existing Surfaces |
| --- | --- |
| Provider provenance/status/capability diagnostics | `create_model_provider`, `ProviderKind::for_provider`, `ProviderKind::create_provider`, provider descriptors, provider capability tests, existing doctor/status display paths. |
| OAuth/auth-store validation and redacted diagnostics | Existing auth/login persistence types, provider auth boundary, existing OAuth/MCP auth-store parsing tests, shared redaction/sanitization helpers. |
| MCP reliability and auth hardening | `execute_mcp_tool_call`, `Session::call_tool`, `sanitize_mcp_tool_result_for_model`, `collect_mcp_server_status_snapshot_with_detail`, `list_mcp_server_status_response`, `RmcpClient` list/read/auth methods. |
| Hook and shell permission safety | `hooks/src/events/permission_request.rs`, `hooks/src/registry.rs`, hook engine tests, `CoreShellActionProvider`, unified exec runtime, shell escalation tests, sandboxing tests. |
| External adapter protocol safety | Existing provider construction/capability/status abstractions, runtime stream event normalization, existing credential scope metadata, existing redaction/sanitization helpers. |
| Session/context bounded diagnostics | `Session::make_turn_context`, `new_turn_context_from_configuration`, existing context fragment patterns, compaction/resume/session tests, TUI snapshot helpers. |
| External-agent import internals | `ExternalAgentConfigService::import`, `ExternalAgentConfigRequestProcessor::import`, `claude_oauth_import` parser/report code, existing external-agent migration tests and startup prompt flow. |

## Forbidden Duplicates

- No second provider factory, provider registry, model catalog, runtime stream abstraction, or capability resolver.
- No second OAuth token parser or credential persistence layer.
- No provider-specific, adapter-specific, MCP-specific, or import-specific redactor if an existing sanitizer/redaction utility can be extended.
- No second MCP status pipeline; extend existing MCP status snapshot and app-server response processors.
- No second hook matcher, hook registry, policy evaluator, or shell permission parser.
- No second shell command launcher or sandbox decision path.
- No second context injection path; diagnostics entering model context must use existing bounded context fragment architecture.
- No second external-agent import service; add Claude/OAuth behavior to the existing import and migration boundaries.
- No public API, config schema, SDK behavior, dashboard, wizard, support bundle, or export path as an implementation shortcut.

## Patch Shape

- Tests first for every retained row that touches provider/auth/MCP/hooks/shell/session/import behavior.
- Implementation second, and only through the required anchor unless the task documents why the anchor cannot own the behavior.
- Public surfaces last, only after ADR approval and compatibility tests.
- For Rust changes, keep modules small; introduce a private sibling module rather than growing a large orchestration file.
- For redaction/security work, the acceptance test must fail if a token, cookie, authorization header, keychain path, or raw credential value appears in output.
- For context work, the acceptance test must prove a hard byte/token/item cap.

## Implementation Homes

- Provider construction, runtime engine, and capability work belongs in `codex-rs/model-provider`; avoid adding provider mechanics to `codex-core`.
- OAuth persistence and token validation belongs in auth/login crates or the provider auth boundary; reuse existing auth-store parsing and do not duplicate token parsing across call sites.
- MCP transport/auth/listing/status work belongs in `codex-rs/rmcp-client`, `codex-rs/codex-mcp`, and existing MCP app-server processors.
- Hook validation and execution belongs in `codex-rs/hooks`; shell permission integration belongs beside the shell runtime tests and existing permission-request event path.
- Shell runtime, sandbox, and command parsing work belongs under `codex-rs/core/src/tools/runtimes` and sandbox modules, with focused integration tests.
- Session/context diagnostics belong in session/context modules with bounded `ContextualUserFragment` structs when content enters model context.
- External-agent import work belongs in `codex-rs/external-agent-migration` and existing external-agent config processors; public app-server API additions require a separate API ADR.

## Non-Negotiable Gates

- No raw credential read without prior user consent, source provenance, redaction tests, and security review.
- No public config key, app-server API, SDK behavior, or schema change from this backlog without an ADR and compatibility tests.
- No support bundle, dashboard, wizard, or export feature from this backlog; those remain product lefties until separately approved.
- No unbounded diagnostic/context fragment; all injected context must have a hard cap and structured type.
- No provider fallback/retry change without proving assistant-visible output cannot be duplicated.
- No new implementation path when an existing architecture anchor can be extended.

## Retained Core Backlog

### Provider, Auth, And Runtime

| Original ID | Approach | Why Core | Implementation Options |
| --- | --- | --- | --- |
| 1 | Add per-category usage attribution for model tokens, MCP, skills, plugins, subagents, and provider overhead. | Context and token accounting are core runtime concerns. | `S` internal usage buckets; `M` token-limit diagnostics; `L` bounded persisted usage analytics. |
| 2 | Add provider-specific fallback diagnostics for native providers. | Provider fallback affects runtime correctness. | `S` log fallback reason; `M` surface in status/errors; `L` configurable fallback policy. |
| 3 | Add provider provenance to runtime errors. | Users and support need to know which provider path failed. | `S` provenance enum; `M` thread through TUI/app-server errors; `L` provider error taxonomy. |
| 4 | Split provider status by auth, runtime engine, model catalog, and stream health. | Existing provider surfaces need sharper health reporting. | `S` status struct; `M` expose in existing status/doctor surfaces; `L` ADR for any dashboard. |
| 5 | Validate OAuth token shape during load. | Auth-store safety is core. | `S` malformed-token fixtures; `M` validate auth load; `L` auth-store repair/migration. |
| 6 | Enforce credential/provider scope. | Prevents credential bleed across heterogeneous providers. | `S` metadata tests; `M` enforce in provider auth; `L` policy integration. |
| 7 | Add redacted provider-auth debug report. | Debuggability must not expose secrets. | `S` extend existing sanitizer/redactor fixtures; `M` doctor section; `L` ADR for any export/support bundle. |
| 8 | Add provider first-event timeout. | Native stream hangs are core runtime failures. | `S` timeout tests; `M` wire into streams; `L` provider timeout policy. |
| 9 | Retry streams only before assistant-visible output. | Avoids duplicated assistant output. | `S` regression test; `M` central retry gate; `L` provider retry matrix. |
| 11 | Add malformed OAuth record tests. | Protects auth persistence compatibility. | `S` fixture set; `M` deserialization validation; `L` repair tool. |
| 12 | Add provider helper diagnostics in `/doctor`. | Provider helper failure blocks core startup/auth flows. | `S` capture helper failure; `M` display helper health; `L` managed helper policy. |
| 13 | Add provider-specific error messages naming configured provider/gateway. | Runtime errors need actionable attribution. | `S` include provider name; `M` normalize display; `L` provider support routing. |
| 14 | Add native-provider capability deltas in model/status surfaces. | Capability gaps determine safe tool/media/model behavior. | `S` capability tests; `M` model/status display; `L` capability policy. |

### External Adapter Runtime

| Original ID | Approach | Why Core | Implementation Options |
| --- | --- | --- | --- |
| 16 | Keep stdio adapter constraints from `ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md`. | External provider engines need a bounded runtime contract. | `S` protocol fixtures; `M` frame parser plugged into provider runtime; `L` adapter supervisor crate ADR. |
| 17 | Separate adapter startup, model-list, first-event, idle, and shutdown timeouts. | Timeout phase matters for failure handling. | `S` timeout fields; `M` supervisor enforcement; `L` managed timeout policy. |
| 18 | Categorize adapter crashes. | Crash semantics decide retry, fallback, and diagnostics. | `S` crash enum; `M` map process exits; `L` recovery UI. |
| 19 | Add adapter circuit breaker. | Prevents repeated failing provider launches. | `S` in-memory counter; `M` per-session breaker; `L` persisted reliability policy. |
| 20 | Add adapter transcript fixtures. | Protocol behavior needs reproducible tests. | `S` fixture format; `M` fixture runner; `L` conformance CLI. |
| 21 | Enforce credential handoff only after handshake/provider match. | Prevents secret exposure to wrong adapters. | `S` handshake fixture; `M` supervisor gate; `L` credential broker integration. |
| 22 | Canonicalize adapter command launch. | Command launch is a security boundary. | `S` path validator; `M` safe launch wrapper; `L` allowlist management. |
| 23 | Add adapter stderr redaction tests. | Adapter logs can leak secrets. | `S` token/cookie/header fixtures; `M` extend shared redactor; `L` ADR for support-bundle redaction. |
| 24 | Reject free-form adapter stdout. | Prevents protocol desync and prompt injection via stdout. | `S` malformed-frame tests; `M` strict reader; `L` compatibility suite. |
| 25 | Cap adapter event counts and bytes. | Core context and stream handling require bounds. | `S` constants/tests; `M` stream enforcement; `L` managed cap policy. |
| 26 | Cap adapter model metadata. | Model catalogs must not inject unbounded context. | `S` model-list fixtures; `M` bounded parser; `L` validation suite. |
| 27 | Add adapter protocol deprecation warnings. | Version drift should fail safely. | `S` handshake warning; `M` display warning; `L` multi-version manager. |
| 28 | Use adapter capability allowlists. | Provider-specific capability claims need containment. | `S` static allowlist; `M` negotiated extensions; `L` capability registry. |
| 29 | Distinguish pre-output cancellation from partial-turn cancellation. | Retry/resume correctness depends on output state. | `S` cancel-state tests; `M` partial-turn state; `L` retry/resume policy. |
| 30 | Add adapter conformance runner. | Gives external engines a testable contract without product UX. | `S` transcript parser; `M` internal standalone binary; `L` ADR for adapter SDK. |

### Hooks And Policy Enforcement

| Original ID | Approach | Why Core | Implementation Options |
| --- | --- | --- | --- |
| 47 | Add regex-cached hook rule evaluation. | Hook execution is a hot policy path. | `S` cached regex helper inside existing hook matcher; `M` extend hook engine; `L` managed rules service ADR. |
| 48 | Add hook action semantics for warn/block/system-message. | Core needs deterministic policy outcomes. | `S` action enum tests; `M` hook dispatch; `L` policy UI only after ADR. |
| 49 | Add hook condition fields for commands, file edits, prompts, and transcripts. | Conditions are the hook engine contract. | `S` extractors; `M` evaluator; `L` policy DSL ADR. |
| 50 | Add hook test runner. | Policy behavior needs isolated testability. | `S` reuse hook engine fixtures; `M` internal runner command; `L` hook CI validation ADR. |
| 51 | Add managed-hook-only enforcement. | Existing managed-hook behavior needs hard guarantees. | `S` source tests; `M` hook engine enforcement; `L` enterprise governance ADR. |
| 53 | Add bounded redacted hook debug logging. | Debug logs must be safe and bounded. | `S` bounded helper; `M` redacted logs; `L` doctor hook diagnostics. |
| 54 | Add stop-hook loop caps. | Prevents unbounded stop-hook loops. | `S` cap tests; `M` stop-hook enforcement; `L` policy override. |
| 55 | Add risky-edit hook safety regression tests. | Security-sensitive edit detection belongs in policy tests, not a packaged hook product. | `S` risky-path fixtures; `M` hook evaluator coverage; `L` managed hook pack only after policy ADR. |
| 56 | Add `/doctor` checks for malformed hook command entries. | Bad hook config breaks core command flow. | `S` missing-field detector; `M` doctor section; `L` repair suggestions. |
| 58 | Validate hook wildcard/prefix matchers. | Matcher ambiguity creates policy bypasses. | `S` matcher fixtures; `M` validator; `L` matcher library. |
| 59 | Add hook/permission bypass regression tests. | Hook and permission integration is a security boundary. | `S` env/cwd fixtures; `M` parser fixes; `L` shell-state analyzer. |

### Permissions, Sandbox, And Shell Execution

| Original ID | Approach | Why Core | Implementation Options |
| --- | --- | --- | --- |
| 61 | Add PowerShell permission parsing tests. | Cross-shell permission correctness is core. | `S` built-in/alias fixtures; `M` extend existing shell permission parser; `L` Windows shell policy engine ADR. |
| 62 | Invalidate cwd-related shell variables on directory changes. | Stale cwd state can bypass permissions. | `S` stale variable tests; `M` invalidation logic; `L` shell state graph. |
| 63 | Add git-worktree sandbox allowlist tests. | Sandbox rules protect repository writes. | `S` `.git` allow/deny fixtures; `M` sandbox rule fix; `L` VCS-aware sandbox policy. |
| 68 | Add settings validation for invalid fields. | Invalid settings should fail safely. | `S` invalid-field tests; `M` validator; `L` schema-driven config engine. |
| 70 | Add settings source reporting. | Source provenance helps diagnose managed/local conflicts. | `S` provenance model; `M` status/doctor view; `L` app-server policy report. |
| 161 | Add shell snapshot tests. | Shell environment capture affects command execution. | `S` functions/aliases fixtures; `M` snapshot collector hardening; `L` shell environment service. |
| 164 | Add long-running command progress summaries. | Core command execution needs bounded feedback. | `S` progress extractor; `M` last-N-lines display; `L` command monitor. |
| 165 | Add background command lifecycle tests. | Background processes affect turn completion and cancellation. | `S` running/exited/canceled fixtures; `M` tracking; `L` process manager. |
| 166 | Harden shell syntax, heredoc, and redirection parsing. | Permission parsing depends on shell syntax correctness. | `S` parser fixtures; `M` parser fixes; `L` shell AST permission model. |
| 167 | Classify search no-match exit codes as expected misses. | Tool execution should distinguish errors from valid misses. | `S` no-match tests; `M` result classifier; `L` command semantics layer. |
| 168 | Add safe command-launch policy for wrappers. | Wrappers can obscure executed commands. | `S` wrapper fixtures; `M` shell prefix handling; `L` launch policy engine. |
| 172 | Add robust PowerShell executable discovery. | Windows runtime startup is core shell support. | `S` discovery fixtures; `M` resolver; `L` Windows runtime detector. |
| 174 | Preserve native PowerShell output formatting. | Output normalization must not corrupt results. | `S` formatter tests; `M` output handling fix; `L` PowerShell adapter layer. |
| 175 | Normalize Windows paths for permissions. | Permission checks must be path-equivalent across platforms. | `S` path fixtures; `M` normalization; `L` cross-platform path permission library. |

### MCP

| Original ID | Approach | Why Core | Implementation Options |
| --- | --- | --- | --- |
| 141 | Add MCP pagination tests for tools/resources/templates/prompts. | MCP list handling must be complete and bounded. | `S` pagination fixtures; `M` extend existing `RmcpClient` list methods; `L` MCP catalog pager ADR. |
| 142 | Improve MCP prompt validation errors. | Bad prompt calls need actionable failures. | `S` missing-arg fixture; `M` named error message; `L` prompt schema UX. |
| 143 | Improve MCP config parse diagnostics. | Config errors block startup and tool discovery. | `S` bad-config fixture; `M` parse diagnostic; `L` migration hints. |
| 144 | Add MCP OAuth cancellation. | Auth flow cancellation is a core runtime behavior. | `S` cancel fixture; `M` cancel browser flow; `L` OAuth runtime manager. |
| 145 | Add proactive MCP OAuth refresh. | Expired MCP auth breaks tool calls mid-session. | `S` expiry fixture; `M` refresh scheduler; `L` token lifecycle service. |
| 146 | Add MCP Authorization Server discovery tests. | OAuth discovery is needed for heterogeneous MCP auth. | `S` discovery fixture; `M` discovery implementation; `L` multi-provider discovery. |
| 149 | Separate MCP startup and tool-call timeouts. | Startup and invocation failures need different handling. | `S` timeout fields; `M` phase enforcement; `L` managed timeout policy. |
| 150 | Add MCP server health detail. | Tool failures need server-level diagnostics. | `S` extend status snapshot struct; `M` reuse existing app-server/TUI status paths; `L` health dashboard ADR. |
| 151 | Add MCP resource-link handling tests. | MCP resources must not break response rendering. | `S` resource-link fixture; `M` result handler; `L` resource renderer integration. |
| 155 | Add MCP image sanitization for text-only models. | Media/tool compatibility is core context safety. | `S` media fixture; `M` text-only guard; `L` capability-aware media pipeline. |
| 156 | Add MCP stdio lingering-process tests. | MCP child processes must shut down cleanly. | `S` shutdown fixture; `M` cleanup enforcement; `L` MCP process supervisor. |
| 157 | Prevent local env leakage into remote MCP configs. | Env provenance is a security boundary. | `S` env-source fixture; `M` provenance enforcement; `L` env provenance model. |
| 160 | Add MCP auth store corruption tests. | Auth persistence must tolerate malformed state safely. | `S` corrupt field fixtures; `M` tolerant parser; `L` OAuth store repair tool. |

### Session, Context, Diagnostics, And TUI

| Original ID | Approach | Why Core | Implementation Options |
| --- | --- | --- | --- |
| 87 | Preserve multi-agent sandbox, approval policy, service tier, and runtime config inheritance. | Child agents must inherit core safety settings. | `S` focused inheritance tests; `M` turn-context fixes; `L` runtime profile model. |
| 89 | Add parent/child agent telemetry IDs. | Multi-agent diagnostics need stable trace linkage. | `S` telemetry attrs; `M` propagate parent IDs; `L` trace tree UI. |
| 100 | Add central `/doctor` diagnostics for startup, provider, MCP, hooks, helpers, and managed settings. | Doctor is the natural diagnostics surface. | `S` checklist; `M` command sections; `L` ADR for exportable bundle. |
| 104 | Add provider/MCP/skill/plugin/subagent usage attribution in token-limit messages. | Token-limit errors need bounded attribution. | `S` attribution metadata; `M` visible message; `L` usage budget planner. |
| 109 | Add startup dialog and helper-path regression tests. | Startup/helper failure blocks core app use. | `S` fixtures; `M` helper fixes; `L` modal/helper framework hardening. |
| 110 | Add transcript live-tail tests. | Transcript streaming is a core session diagnostic path. | `S` tail fixture; `M` live update; `L` transcript viewer refactor. |
| 111 | Add config-summary and cached-metadata robustness tests. | Session metadata must survive missing/old fields. | `S` optional-field fixtures; `M` tolerant parser; `L` versioned metadata. |
| 117 | Add terminal resize and wide-character rendering tests. | Rendering corruption can hide permissions/output. | `S` layout fixtures; `M` renderer fixes; `L` Unicode compatibility suite. |
| 119 | Add permission-dialog small-terminal snapshots. | Permission dialogs are a safety-critical UI. | `S` snapshots; `M` layout fix; `L` responsive permission layout. |
| 120 | Add session rename propagation tests. | Session identity must stay consistent. | `S` rename fixture; `M` metadata update; `L` remote/local session sync. |
| 121 | Add MIME/media fallback tests. | Unsupported media must degrade safely. | `S` MIME fixtures; `M` fallback path; `L` media compatibility service. |
| 124 | Add compaction warning threshold tests. | Context pressure should be visible before failure. | `S` threshold fixture; `M` warning; `L` adaptive compaction policy. |
| 130 | Preserve model and prior items on session resume. | Resume compatibility is core session behavior. | `S` resume fixtures; `M` persistence/replay; `L` rollout compatibility layer. |
| 132 | Add context compaction and overflow tests. | Model context must stay bounded. | `S` overflow fixtures; `M` retry/compaction fixes; `L` adaptive context manager. |
| 140 | Add bounded context fragments for diagnostics/provider output. | Injected context must remain capped and structured. | `S` fragment structs; `M` hard caps; `L` context budget allocator. |
| 181 | Validate memory/session metadata before injection. | Prevents untrusted or oversized metadata in model context. | `S` metadata tests; `M` validation; `L` provenance system. |
| 185 | Add remote-session config mismatch reporting. | Remote execution needs explicit config drift diagnostics. | `S` mismatch struct; `M` app-server reporting; `L` migration assistant. |

### External-Agent Credential Import

| Original ID | Approach | Why Core | Implementation Options |
| --- | --- | --- | --- |
| 213 | Require user consent before reading foreign credential stores. | External credential reads are a security boundary. | `S` consent copy; `M` prompt before read; `L` consent/audit subsystem. |
| 214 | Add provenance metadata for imported external-agent configs. | Imported state needs source traceability. | `S` provenance fields; `M` import report; `L` credential provenance store. |
| 215 | Add locked-keychain and unavailable-store statuses. | Import diagnostics must distinguish expected failures. | `S` status enum; `M` report statuses; `L` recovery UX. |
| 217 | Add import dry-run mode. | Users need review before persistence. | `S` dry-run report through existing import service; `M` internal preview response; `L` public endpoint or wizard ADR. |
| 218 | Add no-secret snapshots for import reports and responses. | Import responses must prove redaction. | `S` snapshots; `M` reuse shared redaction assertions; `L` redaction framework ADR. |
| 219 | Add deletion/revocation parity for imported credentials. | Imported credentials need lifecycle parity. | `S` deletion tests; `M` revoke path; `L` unified credential lifecycle. |
| 220 | Gate raw credential access with security review. | Raw secret access requires explicit control. | `S` checklist; `M` codepath gate; `L` formal signoff process. |

## First Five Recommended Slices

1. Provider provenance and status diagnostics: low schema risk, useful for every heterogeneous provider.
2. MCP pagination/status/auth-store corruption tests: high reliability value, naturally core.
3. Adapter transcript fixture runner: proves the external adapter contract without adding public config first.
4. Hook validation and `/doctor` diagnostics: strengthens existing hook behavior without plugin marketplace work.
5. External-agent import dry-run/no-secret snapshots: unblocks Claude credential work safely when evidence exists.

## Advice

- Do not start with public config keys unless tests prove a core runtime gap.
- Do not implement marketplace, enterprise, or workflow ideas as shortcuts inside `codex-core`.
- Prefer existing test harnesses plus doctor/status diagnostics before adding TUI or app-server API surface.
- Treat every provider/auth/MCP/import diagnostic as secret-bearing until redaction tests prove otherwise.
- Before implementing any item, identify the existing architecture anchor and state why the patch extends it rather than duplicating it.
