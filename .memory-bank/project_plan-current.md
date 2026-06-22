---
name: Ontocode Current Forward Plan
description: Active project plan snapshot derived from CLAUDE_CODE_APPROACHES_FOR_CODEBASE.md and its tracking file
type: project_plan
date: 2026-06-07
status: active
---

# Ontocode Current Forward Plan

Authoritative source: `CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md`.

This file is a memory-bank summary, not the dispatch source of truth. Update the tracking file before starting or closing work.

## Current Status

Active rename program note: `ONTOCODE_INTERNAL_CRATE_RENAME_TRACKING.md` accepted R5BU. The internal crate rename program now has zero remaining `codex-*` Cargo package identities.

| Order | Epic | IDs | Status |
| --- | --- | --- | --- |
| 1 | Provider provenance/status/capability diagnostics | 2, 3, 4, 8, 9, 12, 13, 14 | done |
| 2 | OAuth/auth-store validation and redacted diagnostics | 5, 6, 7, 11, 160 | done |
| 3 | MCP reliability and auth hardening | 141-146, 149-151, 155-157 | done |
| 4 | Hook and shell permission safety | 47-51, 53-56, 58-63, 161, 166-168, 172, 174, 175 | done |
| 5 | External adapter protocol safety | 16-30 | done |
| 6 | Session/context bounded diagnostics | 1, 87, 89, 100, 104, 109-111, 117, 119-121, 124, 130, 132, 140, 181, 185 | done |
| 7 | External-agent import internals | 213-215, 217, 218, 220 | done |
| 8 | Claude OAuth Import Wiring & Validation | Audit Gap | blocked |
| 9 | Public adapter SDK and schema migrations ADR | Next Phase | done |

## Counts

- Total tracked core-natural point IDs: 86.
- Done: 86.
- In progress: 0.
- Pending: 0.
- Blocked: 1 (Claude OAuth live sample validation).
- Not done: 0.

## Next Phase

All tracked project-plan tasks from the initial core-natural approaches slice are complete.

Upcoming work depends on:
- 2026-06-19 provider-policy reset: native model OAuth is OpenAI/Codex-only.
  Gemini, Claude, Kimi, Antigravity, and future non-OpenAI providers must be
  configured through external OpenAI-compatible API endpoints or user-owned
  sidecars. Stale native multi-provider OAuth project plans were removed.
- Follow-on implementation planning derived from the accepted public adapter SDK/schema ADR.
- Provider-neutral credential routing implementation is complete through S1-S5. Senior review accepted the existing private `ModelProvider` auth seam as the correct S5 contract and rejected creation of a parallel provider-auth trait family.
- `F1-A` is complete: a canonical internal secret-bearing OAuth credential model now exists in `ontocode-provider-auth` without adding a second store or second provider-auth stack.
- `F1-B` is complete: current login/OpenAI, RMCP OAuth, and Claude import owners now project into the canonical type.
- `F1-C` is complete: Copilot now keeps GitHub OAuth/access input canonical and the exchanged Copilot token runtime-only.
- `F1-E` is complete: canonical OAuth credentials now project into bounded redacted routing summaries through a direct internal helper with passing redaction coverage.
- `F1-D` is closed by senior design decision with compatibility coverage: Gemini remains API-key-only until a real OAuth source owner exists, and the runtime now has an explicit test guarding that boundary.
- Native provider model selection is complete through S4 for Claude/Gemini: static Claude/Gemini/Copilot catalogs, provider-aware persistence, grouped `/model` picker snapshots, and Claude/Gemini dynamic discovery with static fallback are verified. Copilot dynamic discovery remains excluded as a separate TODO/blocker.
- First-class native provider support beyond OpenAI/Codex is no longer an
  active plan. External provider configuration remains the supported extension
  path.
- Alpha release cut decisions captured in `ALPHA_RELEASE_READINESS.md`, including version policy, package staging, and remaining evidence gates.
- Alpha publish candidate is `0.1.0-alpha.1`.
- The standalone `ontocode` launcher/runtime blocker is fixed, and both fresh dev-profile and clean release-profile `ontocode` binary verification are complete.
- The clean private alpha release branch is `alpha/0.1.0-alpha.1` in `/tmp/ontocode-alpha-release`; release build plus focused `ontocode-api`, `ontocode-protocol`, and `ontocode-state` tests pass.
- Remaining alpha publish work is now commit/push/tag to `ontograph`, wait for the `rust-release` workflow artifact, stage the native npm package from that artifact, and then create the private alpha release. Claude OAuth live validation remains sample-blocked and is not a publish gate for this private alpha.
- The Rust workspace directory is now `ontocode-rs/`; the shipped binary surface is `ontocode`, and Cargo metadata reports zero `codex` binary targets after the main-checkout layout reconciliation.
- The root npm wrapper directory is now `ontocode-cli/`; local pnpm/release tooling uses the new path while the public npm package identity remains compatibility-preserved.
- `ONTOCODE_FULL_LEGACY_MIGRATION_PROJECT_PLAN.md` remains the staged migration track. Main-checkout F1 layout reconciliation is complete; remaining work is F5 verification/runtime compatibility plus release-gated F6/F7 cleanup.
- Rollout of TUI context visualization.
- Multi-agent goal orchestration refinements.
- `ADR_FIVE_CONCURRENT_CODING_SUBAGENTS.md` is implemented: the default v2 session cap supports five direct coding children, coding sub-agents no longer receive recursive `spawn_agent`, cap refusal explains close-to-free-slots behavior, and focused core tests pass.
- `NATIVE_CONTEXT_TOOLS_CORE_ENGINE_PROJECT_PLAN.md` C0 is complete: shell-output reducers now live in existing `ontocode-core` formatting, focused reducer tests pass, the arg0 test-binary-support alias collision is fixed, and broad `ontocode-core` tests pass when run with a fresh isolated `TMPDIR`.
- `GITNEXUS_CODE_GRAPH_ADOPTION_PRE_JUNIOR_PROJECT_PLAN.md` is complete through S0-S9: the Rust `operational_evidence_records` ledger, bounded state runtime methods, runtime topology evidence, explicit artifact/workflow import, and planned-versus-done gate evaluator are implemented in `ontocode-state`. S10 remains blocked until a separate ADR approves any model-visible context fragment.
- `GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_PRE_JUNIOR_PROJECT_PLAN.md`
  is closed no-dispatch after OntoIndex review found the retained
  context-fidelity slice already covered and no remaining new core-extension
  task.
- `ADR_QWEN_DONOR_BLOCKED_ROWS_UNBLOCK.md` reopens the 12 blocked Qwen donor
  rows only as narrow existing-owner slices. Broad public metadata, persisted
  read-evidence, full transcript, native HTTP-hook, and artifact-classifier
  surfaces remain ADR-blocked.
- `ADR_QWEN_DONOR_REMAINING_BLOCKERS_SOLUTIONS.md` reopens only the ten rows
  still blocked after the first pass. The accepted implementation path remains
  owner-local: internal tool reasons, per-turn in-memory read evidence, bounded
  agent summary if no migration is needed, a bounded context fragment for
  operational evidence, and provider-local context-window classification.
- `CLAUDE_CODE_DONOR_DEFERRED_NARROW_REJECT_PRE_JUNIOR_PROJECT_PLAN.md`
  is closed no-dispatch: all 146 parked DEFER/NARROW/REJECT rows are tracked,
  with zero remaining dispatch rows and no new core-extension implementation
  accepted from the parked set.
- `ADR_JSCPD_DONOR_CORE_EXTENSION_SOLUTIONS.md` is complete. The five retained
  jscpd donor rows closed as owner-local regression coverage or existing
  snapshot coverage in `JSCPD_DONOR_CORE_EXTENSION_TRACKING.md`; duplicate
  detectors, generic report pipelines, and SQLite tracking stores remain
  rejected.
- `ADR_HERMES_DONOR_CORE_EXTENSION_SOLUTIONS.md` is complete. The four accepted
  Hermes donor rows closed in `HERMES_DONOR_CORE_EXTENSION_TRACKING.md` as
  owner-local regression coverage or one minimal existing-owner TUI diagnostic:
  subagent namespace/model/tool exposure, repeated MCP failure hints,
  MCP/plugin connector cache invalidation, and long-running process lifecycle
  cleanup/status. All other Hermes donor rows remain closed unless a concrete
  failing fixture reopens them.
- `ADR_CLAUDE_CODE_DONOR_300_CORE_EXTENSION_SOLUTIONS.md` is complete for
  scoped owner-local regression closure, not full donor feature parity. The
  keep-only Claude Code donor 300 review closed all five bundles in
  `CLAUDE_CODE_DONOR_300_CORE_EXTENSION_TRACKING.md`; remaining broad
  `ontocode-core` failures are unrelated pre-existing suite blockers, and dirty
  snapshots/scratch files are merge hygiene.
- `ADR_DONOR_TOOL_PROPOSALS_CONSOLIDATION.md` is closed-narrowed in
  `DONOR_TOOL_PROPOSALS_CONSOLIDATION_TRACKING.md`.
  `DTP-R1` first-slice final-output schema validation, `DTP-R2` bounded
  evidence ledger, `DTP-R3` narrowed final-answer claim warnings, and
  senior-narrowed `DTP-R4` hosted web-search guarded fetch proof are complete
  with scoped verification. Full structured-output redaction/conformance
  diagnostics and exact file/command/failure/approval verification stay parked
  without a reproduced owner-local failing test. The ADR loop was completed in
  single mode: one active bundle and no overlapping build/test commands, with
  `CARGO_BUILD_JOBS=1`.
- Gemini/Kimi/Antigravity OAuth donor plans are superseded for model runtime.
  Their ADRs remain as historical donor evidence for external sidecars only.
- `ADR_KIMI_OAUTH_CLIPROXY_IMPORT_AND_DEVICE_FLOW.md` is complete to ADR gates: parser/fixture coverage, existing provider OAuth storage projection, and slash auth/status visibility are done. The next allowed Kimi slice is login-only device flow after explicit client-id approval; native runtime remains ADR-blocked and `/model` stays out of scope.

## Completion Criteria For Any Epic

- Tracking file updated before dispatch and after completion.
- GitNexus context/impact recorded.
- HIGH/CRITICAL impact reported before edits and narrowed if possible.
- Existing architecture reused; no duplicate owners introduced.
- Scoped tests pass.
- `just fmt` run after Rust changes.
- GitNexus `detect_changes` run before close-out.
