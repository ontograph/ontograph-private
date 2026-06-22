---
name: Ontocode Pending Tasks
description: Living backlog summary derived from the current project tracking file
type: backlog
date: 2026-06-07
status: active
---

# Ontocode Pending Tasks

Authority: `CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md`.

## Active Tasks

### Provider Policy Reset

Status: `done`.

Authority:
- `ADR_MULTI_PROVIDER_OAUTH_CONNECTION_ROUTING.md`

Outcome:
- Native model-provider OAuth is OpenAI/Codex-only.
- Non-OpenAI model providers are external OpenAI-compatible API endpoints or
  user-owned sidecars.
- Obsolete native multi-provider OAuth project plans were removed.

Next actions:
- Keep GPT/Codex as the default native route.
- Remove or gate any remaining runtime path that selects Gemini, Claude, Kimi,
  Antigravity, or similar providers as native OAuth-backed model providers.
- Keep external-provider diagnostics redacted and independent from OpenAI login.

### Kimi OAuth CLIProxyAPI Import And Device Flow

Status: `superseded-for-native-runtime`.

Authority:
- `ADR_KIMI_OAUTH_CLIPROXY_IMPORT_AND_DEVICE_FLOW.md`
- `KIMI_OAUTH_CLIPROXY_IMPORT_AND_DEVICE_FLOW_TRACKING.md`

Outcome:
- Import parser/fixture coverage, existing auth storage projection, and slash auth/status rows are complete.

Next actions:
- Do not redispatch native Kimi OAuth/device-flow/runtime work.
- Kimi model use belongs behind a user-owned external OpenAI-compatible
  endpoint or sidecar.

### Native Provider Model Selection

Status: `superseded-for-native-non-openai-runtime`.

Authority:
- `ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md`
- `audit_session-2026-06-19-openai-only-provider-policy.md`

Outcome so far:
- S0 baseline, S1 static native catalogs, S2 provider-aware persistence, and S3 grouped `/model` picker are complete.
- S3 uses active app-server model snapshots plus configured static native catalogs and persists `model_provider`, `model`, and `model_reasoning_effort` together.
- True live provider switching for an already-running app-server thread is deferred behind app-server thread-settings/API design.
- S4 Claude/Gemini dynamic discovery is implemented behind existing model-manager interfaces with static fallback and mocked redaction coverage.
- Copilot discovery is excluded from this implementation slice and remains an explicit TODO/blocker.

Next actions:
- Do not redispatch native non-OpenAI catalog/model-selection work.
- Non-OpenAI models should be exposed only through configured external
  providers.

### First-Class Provider Support

Status: `superseded`.

Authority:
- `ADR_MULTI_PROVIDER_OAUTH_CONNECTION_ROUTING.md`
- `audit_session-2026-06-19-openai-only-provider-policy.md`

Outcome targeted:
- Make provider identity backend-owned across catalog discovery, model selection, and active-thread switching.
- Remove TUI-local provider aggregation so Codex/OpenAI, Gemini, and Claude behave as first-class providers through the same backend contract.
- Stage Copilot separately behind an explicit discovery/token contract decision.

Next actions:
- Do not redispatch first-class native Gemini/Claude/Copilot provider work.
- Keep OpenAI/Codex first-class natively; route other providers through
  external API endpoints.

### Gemini OAuth Donor Transfer

Status: `superseded-for-native-runtime`.

Authority:
- `ADR_NATIVE_GEMINI_OAUTH_AND_SUBAGENT_PROVIDER_CONCURRENCY.md`
- `audit_session-2026-06-19-openai-only-provider-policy.md`

Outcome targeted:
- Translate only the useful CLIProxyAPI Gemini OAuth ideas into Ontocode.
- Keep `gemini` API-key support unchanged and introduce OAuth as a separate `gemini-cli` account-backed lane first.

Outcome so far:
- S0/S1 remain accepted: API-key Gemini compatibility is locked and donor credential import is narrow/redacted.
- S2-D/S2-E are accepted: provider OAuth credentials persist through existing `AuthDotJson` / `AuthStorageBackend`, and `AuthManager` hands them to the existing model-provider bearer auth path.
- K2 is now complete for Kimi: imported Kimi provider OAuth credentials round-trip through existing auth storage helpers without a new metadata store.
- S3-A is accepted: `gemini-cli` is the canonical provider id and `Gemini CLI` is the display name.
- S5-A is accepted: `/model` shows a separate disabled `gemini-cli` provider group using static local catalog data, with no runtime/network model-list call and no selection actions.
- S7-A is accepted: normal `gemini` can use bearer auth through the existing provider auth path when `GEMINI_API_KEY` is absent.
- S6-A is accepted: user-supplied Google ADC / desktop OAuth JSON can be imported into normal `gemini` provider OAuth credentials.

Next actions:
- Do not redispatch native Gemini OAuth/import/runtime work.
- Treat donor notes as external sidecar evidence only.

### Gemini CLI Donor Context/Tools/Agents/Evals Pre-Junior

Status: `closed-no-dispatch`.

Authority:
- `GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_PRE_JUNIOR_PROJECT_PLAN.md`

Outcome:
- OntoIndex-backed review found no remaining Gemini-specific pre-junior slice
  that is both new and a core functionality extension.
- The retained context-fidelity slice is already covered by existing TUI/core
  tests.

Next actions:
- Do not dispatch tasks from this plan.
- Reopen only with a fresh manager card that identifies a current core owner,
  proves a missing behavior, and starts from one failing core regression test.

### Qwen Donor Blocked Rows Unblock

Status: `done`.

Authority:
- `ADR_QWEN_DONOR_BLOCKED_ROWS_UNBLOCK.md`
- `ADR_QWEN_DONOR_REMAINING_BLOCKERS_SOLUTIONS.md`
- `tmp/qwen-code-donor-dispatch-tracking.md`

Outcome targeted:
- Resolve the remaining ten Qwen donor blocker rows only through existing
  owners.
- Keep SQLite read-evidence persistence, public metadata APIs, transcript
  storage, native HTTP hooks, and raw artifact injection blocked.

Next actions:
- Dispatch only `R1` through `R5` from the remaining-blockers ADR.
- Update the tracker before each slice starts and after each slice closes.
- Use OntoIndex before edits and refresh/check it after each completed slice.

### jscpd Donor Core Extension

Status: `done`.

Authority:
- `ADR_JSCPD_DONOR_CORE_EXTENSION_REVIEW.md`
- `ADR_JSCPD_DONOR_CORE_EXTENSION_SOLUTIONS.md`
- `JSCPD_DONOR_CORE_EXTENSION_TRACKING.md`

Outcome:
- `JSCPD-R1` added apply-patch diagnostic/adversarial parser coverage.
- `JSCPD-R2` added hook spill duplicate-output coverage and a minimal
  owner-local budget fix.
- `JSCPD-R3` added a static base-instructions duplication guard.
- `JSCPD-R4` closed as already covered by existing guardian/model-visible layout
  snapshots.

Next actions:
- No remaining dispatch tasks from the jscpd donor ADR.
- Reopen only with a concrete owner-local failing regression; keep duplicate
  detection, clone scanning, generic report generation, and SQLite tracking out
  of core.

### Hermes Donor Core Extension

Status: `done`.

Authority:
- `tmp/hermes-agent-500-tools-for-ontocode-challenged.md`
- `ADR_HERMES_DONOR_CORE_EXTENSION_SOLUTIONS.md`
- `HERMES_DONOR_CORE_EXTENSION_TRACKING.md`

Outcome targeted:
Outcome:
- Closed the four implementation-queue rows as owner-local regression coverage or one minimal existing-owner TUI diagnostic.
- Kept browser/CDP tools, provider plugins, optional skills, cron, memory stores, process registries, and SQLite tracking rejected.

Next actions:
- No remaining dispatch tasks from the Hermes donor ADR.
- Reopen only with a concrete owner-local failing regression.

### Claude Code Donor Deferred/Narrow/Rejected Pre-Junior

Status: `closed-no-dispatch`.

Authority:
- `CLAUDE_CODE_DONOR_DEFERRED_NARROW_REJECT_PRE_JUNIOR_PROJECT_PLAN.md`

Outcome:
- All 146 parked rows from
  `ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md`
  have 2026-06-20 tracking entries.
- Rows 121 and 124-127 closed as rejected because source exploration/search is
  already owned by OntoIndex, MCP resource/tool discovery, file-search, and
  shell/rg paths; no new MCP source-browsing surface was accepted.

Next actions:
- Do not redispatch parked Claude Code donor rows from this plan.
- Reopen only with a fresh manager card that proves a current owner-local defect
  or failing regression test and does not introduce a parallel source, command,
  MCP, plugin, hook, UI, or context owner.

### Five Concurrent Coding Sub-Agents

Status: `done-narrowed`.

Authority:
- `ADR_FIVE_CONCURRENT_CODING_SUBAGENTS.md`
- `ADR_FIVE_CONCURRENT_CODING_SUBAGENTS_TRACKING.md`

Outcome:
- Allow five simultaneous direct coding sub-agents through the existing multi-agent v2 cap and close path.
- Remove recursive `spawn_agent` access from ordinary coding sub-agents.
- Keep scheduler, SQLite task tables, worker pools, default worktrees, and nested fan-out out of phase one.

Next actions:
- No remaining dispatch tasks from this ADR.
- Reopen only with a concrete failing fixture in the existing multi-agent v2 owners.

### Alpha Release Readiness

Status: `published-with-infra-debt`.

Outcome so far:
- Release-prep baseline documented in `ALPHA_RELEASE_READINESS.md`.
- Existing release staging architecture retained; no broad source-manifest bump forced into dev/main workflow.
- Native Copilot release metadata now derives from crate version instead of hardcoded `0.0.0`.
- Standalone `ontocode` launcher no longer depends on a sibling `codex` wrapper binary; the alias now builds from the real CLI entrypoint.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli` passed after disabling duplicate unit-test execution on the alias bin target.
- Fresh source-built `ontocode --version` and `ontocode --help` execute successfully and brand correctly as `Ontocode CLI`.
- Fresh clean `release`-profile `ontocode --version` and `ontocode --help` also execute successfully after a full `24m 11s` build.
- Main CLI help copy now says `Ontocode` / `Ontocode Cloud` on the verified binary surface.
- Clean private alpha branch `alpha/0.1.0-alpha.1` builds the release binary as `Ontocode CLI 0.1.0-alpha.1`.
- Focused release-branch validation passes for `ontocode-api`, `ontocode-protocol`, and `ontocode-state`; state needed the missing `0037_agent_jobs_final_summary.sql` migration to match the already-present agent-job summary code.
- Python SDK and `codex-sdk` npm staging pass with injected alpha versions.
- Private alpha release is published at `https://github.com/ontograph/ontograph-private/releases/tag/rust-v0.1.0-alpha.1`.
- Included assets are the local Linux release binary, root npm wrapper tarball, SDK npm tarball, and checksums.
- Full native platform npm packaging remains blocked on private release infrastructure: the upstream `rust-release` workflow cannot start in `ontograph/ontograph-private` without the OpenAI runner labels and signing secrets.

Next actions:
- Add private release infrastructure before claiming full native npm coverage: either configure the required runners/signing secrets or add a dedicated unsigned private-alpha workflow that builds only the supported platform set.
- Stage native `codex` npm with `CODEX_RELEASE_GITHUB_REPO=ontograph/ontograph-private` after a suitable artifact workflow exists.
- Treat `ontocode-rs/` as the active Rust workspace directory.
- Cargo metadata now reports zero `codex` binary targets after the dedicated alias-entrypoint cleanup; remaining legacy names are compatibility/runtime/prose surfaces, not duplicate Cargo bins.
- Close `Claude OAuth Live Validation` if a real redacted sample becomes available.

### Native Context Tools Core Engine

Status: `done`.

Authority:
- `ADR_NATIVE_CONTEXT_TOOLS_CORE_ENGINE.md`
- `NATIVE_CONTEXT_TOOLS_CORE_ENGINE_PROJECT_PLAN.md`
- `NATIVE_CONTEXT_TOOLS_CORE_ENGINE_TRACKING.md`

Outcome:
- Added deterministic shell-output reducers inside existing `ontocode-core` formatting owners.
- Kept command execution, shell policy, app-server/config/protocol, persistence, and model-visible tool surfaces unchanged.
- Focused reducer coverage passes.
- The prior `test-binary-support` PATH alias collision is fixed in `ontocode-arg0`.
- Broad `just test -p ontocode-core` passes when run with a fresh isolated `TMPDIR`; a reused temp root containing `.codex` can still pollute root-discovery tests.
- Default shell/tool formatting now applies deterministic reduction before the existing final truncation path.

Next actions:
- Treat the C0 reducer slice as complete.
- For full core verification, run with a fresh `TMPDIR`, for example `env TMPDIR="$(mktemp -d /var/tmp/ontocode-core.XXXXXX)" CARGO_BUILD_JOBS=8 just test -p ontocode-core`.

### GitNexus Code-Graph Adoption

Status: `done-with-blocked-s10`.

Authority:
- `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md`
- `GITNEXUS_CODE_GRAPH_ADOPTION_PRE_JUNIOR_PROJECT_PLAN.md`

Outcome:
- Added one Rust state-backed `operational_evidence_records` ledger.
- Stored compact operational evidence for code graph, workflow, tests, docs, redaction, architecture, and runtime topology.
- Kept GitNexus/OntoIndex-style analyzer output as explicit bounded artifacts, not runtime dependencies.
- Completed S0-S9 with `ontocode-state` tests passing after each accepted implementation stage.

Next actions:
- Keep S10 context fragment blocked until a separate ADR approves model-visible evidence.
- Reuse the state-owned operational evidence ledger and closure evaluator for future manager/subagent planned-versus-done checks.

### Ontocode Full Legacy Migration

Status: `reviewed-proposed`.

Authority:
- `ONTOCODE_FULL_LEGACY_MIGRATION_PROJECT_PLAN.md`
- `ONTOCODE_FULL_LEGACY_MIGRATION_TRACKING.md`
- `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`
- `ONTOCODE_PACKAGE_IDENTITY_MIGRATION.md`
- `ONTOCODE_REMAINING_SURFACES_DISPOSITION.md`

Outcome targeted:
- Keep the Rust workspace path at `ontocode-rs/` after the completed main-checkout layout migration.
- Move public command, helper, package, env/state, protocol, telemetry, and compatibility cleanup through staged gates instead of broad string replacement.

Outcome so far:
- Stage 0 manager dispatch completed layout/build, CLI/helper/runtime, and package/SDK/state/protocol/telemetry inventory matrices.
- Manager review accepted the Stage 0 matrices and recorded a no-go for implementation dispatch while the worktree has broad unrelated changes.
- F1 main-checkout layout reconciliation is complete. F5 verification/runtime compatibility remains in progress, and F6/F7 remain gated by package/state/protocol/telemetry release-versioning decisions.
- F4-H root npm wrapper cleanup is complete: `codex-cli/` moved to `ontocode-cli/`, local workspace/staging paths were updated, and public npm package compatibility was preserved.

Next actions:
- Resume F5-L package-wide verification triage from the known code-mode and compact-remote-parity failures.
- Keep protocol, telemetry, and final compatibility cleanup blocked until their prerequisites and owners are recorded.

### Provider Credential Routing Refactor

Status: `done`.

Outcome so far:
- Refactor sequence accepted conceptually as `3 -> 1 -> 4 -> 2 -> 5`.
- A dedicated staged plan now exists in `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md`.
- Execution tracking now exists in `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_TRACKING.md`.
- The plan preserves current architecture ownership instead of introducing a second provider or auth stack.
- S1 internal alias/prefix routing and diagnostics are complete.
- S2 normalized credential views, source adapters, and bounded redacted summaries are complete.
- S3-A shared refresh adapter/orchestrator contract is complete in `ontocode-provider-auth`.
- Existing `login` and `rmcp-client` refresh owners now expose thin adapters instead of a duplicated refresh stack.
- S4 scheduler internals are complete with deterministic round-robin, priority, failover, and sticky-session behavior in the private `model-provider::schedule` core.

Next actions:
- No remaining implementation slices in `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md`.
- Reuse the landed canonical OAuth and routing/scheduling owners for future provider work; do not introduce a parallel auth stack.

### Ontocode Internal Crate Rename Recovery

Status: `done`.

Authority:
- `ONTOCODE_INTERNAL_CRATE_RENAME_TRACKING.md`

Current status:
- R5BP OTEL is accepted after R5BP-U1 controlled recovery.
- R5BQ `codex-extension-api` identity-only slice is verified complete.
- R5BR `codex-state` identity-only slice is accepted.
- R5BS `codex-tools` identity-only slice is accepted after manager verification.
- The user explicitly lifted the protocol gate for the last two crates.
- R5BT `ontocode-app-server-protocol` is accepted after manager verification.
- R5BU `codex-protocol` is accepted after manager verification.
- Exactly zero `codex-*` Cargo package identities remain.

Next actions:
- No remaining internal crate package identities require rename dispatch.

### Public Adapter SDK And Schema Migrations ADR

Status: `done`.

Outcome:
- Stage 0 schema proposal, owner map, compatibility test plan, conformance expansion plan, and readiness decision are accepted.
- Follow-on implementation work is split into P1 config schema, P2 adapter conformance, P3 experimental app-server visibility, and P4 SDK parity tracks.

Next actions:
- Dispatch P1-P4 only after the protocol-stage rename tree is stable enough for safe implementation work.

## Blocked Tasks

### Claude OAuth Live Validation

Status: `blocked`.

Reason:
- No `CLAUDE_OAUTH_REDACTED_SAMPLE` path is available in the environment.

Needed:
- One real redacted Claude MCP connector credential sample that preserves non-secret schema fields.

### Native Provider Dynamic Discovery

Status: `blocked`.

Reason:
- No accepted native discovery manager/cache contract exists for Claude or Gemini.
- No accepted provider-specific model-list endpoint/auth contracts exist for Claude or Gemini inside the repo.
- Copilot discovery is excluded from this implementation slice and remains separately blocked on whether discovery may perform GitHub-to-Copilot token exchange and how raw account data is redacted.

Needed:
- Claude/Gemini discovery-contract ADR/test spike with mocked HTTP fixtures and redaction assertions before any dynamic model-manager implementation.
- Separate Copilot account-scoped discovery design before any Copilot implementation.

### Claude Code Donor 300 Core Extension Bundles

Status: `done-scoped-regressions`.

Source:
- `ADR_CLAUDE_CODE_DONOR_300_CORE_EXTENSION_SOLUTIONS.md`
- `CLAUDE_CODE_DONOR_300_CORE_EXTENSION_TRACKING.md`

Outcome:
- `CLAUDE300-R1`: multi-agent and agent-job regressions closed.
- `CLAUDE300-R2`: shell and PowerShell policy/parser regressions closed; broad
  `ontocode-core` failures were verified unrelated to the bundle.
- `CLAUDE300-R3`: bounded context/file/search/LSP/attachment/compaction
  regressions closed; model-visible context remains valid only while bounded,
  capped, and inside approved context owners.
- `CLAUDE300-R4`: MCP/resource/auth/tool-discovery/skills/plugins/config
  regressions closed by manager verification after unusable worker output.
- `CLAUDE300-R5`: diagnostics/review/web/pacing/plan-mode/model/auth
  regressions closed; release/support-bundle claims still need secret-output
  content review.

Next actions:
- No remaining dispatch tasks from the Claude Code donor 300 ADR.
- Reopen only with a concrete owner-local failing regression.
- Treat dirty worktree and snapshot artifacts as merge hygiene, not a reason to
  redispatch Claude300.

### Donor Tool Proposals Consolidation

Status: `done-narrowed`.

Authority:
- `ADR_DONOR_TOOL_PROPOSALS_CONSOLIDATION.md`
- `DONOR_TOOL_PROPOSALS_CONSOLIDATION_TRACKING.md`

Outcome targeted:
- Close only the current accepted/narrowed slices from the consolidation ADR:
  structured final-output schema shape validation, bounded evidence ledger,
  bounded final-answer claim warnings, and hosted web-search guarded fetch
  proof.
- Keep verification-only candidates parked until a concrete current-owner
  failing test is reproduced.

Outcome so far:
- `DTP-R1` first slice is complete in single-mode recovery:
  `final_output_json_schema` updates are validated in the existing session turn
  path. Redaction and conformance diagnostics stay parked without a failing
  current-owner test.
- `DTP-R2` is complete in single-mode recovery: bounded evidence buckets and a
  capped operational evidence context fragment stay inside existing
  evidence/context/session owners, with scoped protocol and core tests passing
  under `CARGO_BUILD_JOBS=1`.
- `DTP-R3` narrowed slice is complete in single-mode recovery: final assistant
  answers emit bounded warnings for unsupported test, policy-check, or
  source-change claims. Exact file/command/failure/approval verification stays
  parked without a failing current-owner test.
- `DTP-R4` is closed with no new code by senior-narrowed proof: hosted `web_search` already
  covers guarded open-page/find-in-page fetch-style actions behind provider,
  config, standalone-web, and web-search mode gates; a separate `web_fetch`
  stack remains rejected.

Next actions:
- No remaining dispatch tasks from the Donor Tool Proposals Consolidation ADR.
- Reopen only with a concrete provider/API surface for dedicated `web_fetch` or
  a current owner-local failing regression.

## Done

### External-Agent Import Internals

IDs: `213-215, 217, 218, 220`.

Completed slices:
- User consent gate for foreign credential reads.
- Import dry-run mode for safe preview.
- Provenance metadata for imported configs.
- Locked-keychain and unavailable-store diagnostic statuses.
- Redacted import reports verified with snapshots.

### Session/Context Bounded Diagnostics

IDs: `1, 87, 89, 100, 104, 109-111, 117, 119-121, 124, 130, 132, 140, 181, 185`.

Completed slices:
- Bounded `DiagnosticFragment` with 1000-token hard cap.
- Granular usage attribution (model vs tool tokens) in `history.rs`.
- Multi-agent safety inheritance logic verified via `Session::update_settings` and context creation.
- Improved hook error handling and deterministic action semantics.

### External Adapter Protocol Safety

IDs: `16-30`.

Completed slices:
- v1 Adapter Protocol crate with NDJSON framing.
- Timeout categorization (handshake, list, stream, idle, shutdown).
- Crash semantics and circuit breaker status.
- Credential gate state (handoff after handshake).
- Protocol transcript fixtures and conformance runner.

### Hook And Shell Permission Safety

IDs: `47-51, 53-56, 58-63, 161, 166-168, 172, 174, 175`.

Completed slices:
- Thread-local regex caching for hook matcher.
- HookAction semantics (Warn, Block, SystemMessage).
- Loop protection for Stop hooks (max 10 iterations).
- Robust PowerShell discovery with safety timeouts.
- Cross-platform PowerShell safety identification.

### MCP Reliability And Auth Hardening

IDs: `141-146, 149-151, 155-157`.

Completed slices:
- Internal pagination loops for tool/resource/template discovery.
- Improved config parsing diagnostics in `codex-core`.
- Image sanitization for non-vision models in `execute_mcp_tool_call`.
- Proactive OAuth refresh with 60s safety timeout in `RmcpClient`.

### OAuth/Auth-Store Validation And Redacted Diagnostics

IDs: `5, 6, 7, 11, 160`.

Completed slices:
- Centralized URL/query redaction in `codex-login`.
- OAuth token structural validation in `codex-rmcp-client`.
- Redaction-aware Claude OAuth import in `codex-external-agent-migration`.
- Scope-less retry fallback in `codex-cli`.

### Provider Provenance/Status/Capability Diagnostics

IDs: `2, 3, 4, 8, 9, 12, 13, 14`.

Completed slices:
- Provider descriptor diagnostic snapshot.
- Retry/timeout/5xx/429 telemetry coverage.
- Doctor WebSocket provider diagnostics.
- Scoped verification recorded in tracking.
