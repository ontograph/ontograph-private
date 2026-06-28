# OpenHuman Donor 2000 Useful Approaches Review

Status: challenged and narrowed, OntoIndex-grounded, no implementation dispatch  
Date: 2026-06-28  
Donor source: `tmp/OpenHuman`  
Target repo: Ontocode Rust workspace (`codex`)

## Review Standard

Keep a donor idea only if both are true:

1. it is still new relative to current Ontocode source, and
2. it extends an existing core owner instead of creating a parallel product/runtime stack.

Under that bar, the previous 72-row catalog was too broad. Most of it mixed landed Ontocode behavior, donor product surface, or ideas that would introduce a second owner.

## Challenge Findings

- The prior artifact overstated novelty. Current Ontocode already owns substantial goal state, idle continuation, connector discovery/filtering, plugin-local MCP loading, skill rendering, and provider compatibility behavior.
- OpenHuman's strongest reusable value is not its product shell. The useful material is a small set of runtime patterns around bounded context preparation, bounded helper behavior, and recoverable compaction.
- Voice, mascot, meeting-agent, dashboard, theme, and broad personal-assistant ingestion work are out of scope here. They do not extend current core functionality.
- Several donor rows are only worth keeping as owner-local invariants or prompt-shaping changes, not as new public APIs or new runtime subsystems.

## Current-Source Coverage That Invalidates Prior Rows

OntoIndex evidence against current Ontocode source invalidates most of the earlier keep list:

- Goal/accounting/status coverage already exists in `ontocode-rs/ext/goal/src/accounting.rs` (`mark_idle_goal_active`, `mark_idle_progress_accounted_for_status`, `reset_idle_progress_baseline_and_clear_active_goal`), `ontocode-rs/ext/goal/src/runtime.rs` (`continue_if_idle`), `ontocode-rs/state/src/runtime/goals.rs`, and `ontocode-rs/tui/src/chatwidget/goal_status.rs`.
- Pre-sampling setup already exists in `ontocode-rs/core/src/session/turn.rs` (`run_pre_sampling_compact`) plus related compact tests in `ontocode-rs/core/tests/suite/compact.rs`. Any retained context-prep row must therefore be narrower than "do work before sampling".
- Tool/schema compaction and bounded-output handling already exist in `ontocode-rs/tools/src/json_schema.rs` (`parse_tool_input_schema`, `compact_large_tool_schema`), `ontocode-rs/core/tests/suite/compact.rs`, `ontocode-rs/hooks/src/output_spill_tests.rs`, and `ontocode-rs/core/src/tools/context_exec_output.rs`.
- Connector discovery/filtering and schema exposure already exist in `ontocode-rs/core/src/connectors.rs`, `ontocode-rs/codex-mcp/src/rmcp_client.rs`, `ontocode-rs/core/src/mcp_tool_call.rs`, and `ontocode-rs/core/src/mcp_tool_exposure_test.rs`.
- Plugin-local MCP loading, installed-skill metadata, and plugin rendering already exist in `ontocode-rs/core-plugins/src/loader.rs` (`load_plugin_mcp_servers`), `ontocode-rs/core/src/mcp_skill_dependencies.rs`, `ontocode-rs/core/src/plugins/render.rs`, and related app-server/plugin tests.
- Helper-session skill suppression already exists in current owners: guardian review sessions disable skill instructions in `ontocode-rs/core/src/guardian/review_session.rs`, and subagent coverage already asserts skipped skill instructions for parent and spawned child in `ontocode-rs/core/tests/suite/subagent_notifications.rs`.

Those existing owners mean the earlier rows for basic goal state, connector listing/filtering, plugin-local MCP loading, installed-skills rendering, service-tier compatibility, and similar provider hygiene are not new donor opportunities.

## Retained Core-Extension Candidates

Only the following rows still survive current-source challenge.

| ID | Candidate | Existing owner to extend | Why it still looks new |
|---:|---|---|---|
| OH-01 | Pre-sampling context-enrichment pass distinct from existing pre-sampling compaction | `ontocode-rs/core/src/session/turn.rs`; `ontocode-rs/core/src/context_manager/*` | Current source already has `run_pre_sampling_compact`; the only still-plausible donor delta is a separate bounded context-enrichment pass rather than more compaction. |
| OH-02 | Salvage bounded structured context bundles even when wrapped in extra prose | `ontocode-rs/core/src/session/turn.rs`; context fragment parsing owners | Current core is strong on caps and trimming, but this specific parser-hardening behavior was not proven covered by the OntoIndex pass. |
| OH-03 | Read-only scout helper for context prep with hard caps and no mutation/tool-write rights | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`; worker prompt/tool-planning owners | Ontocode already has worker/tool gating, but not a clearly scoped scout-only helper mode dedicated to context preparation. |
| OH-04 | Recoverable lossy tool-output compaction with an explicit retrieval handle back to the original payload | `ontocode-rs/core/src/tools/*`; `ontocode-rs/core/src/session/*`; output/context owners | Current source compacts schemas and bounds output, but the donor's stronger idea is "lossy only if recoverable", which was not shown as covered. |
| OH-05 | Content-kind-aware output reduction that preserves signatures and risk markers for code-like payloads | `ontocode-rs/core/src/tools/*`; read/search/review/context owners | Current compaction is present, but kind-specific reductions for code/search/diff payloads appear materially beyond today's generic truncation/compaction surface. |
| OH-06 | Unattended continuation inherits stricter approval rules for irreversible actions than an interactive turn | goal runtime + approval/permission owners | Idle continuation already exists, but the OntoIndex pass did not show an explicit fail-closed approval clamp for irreversible unattended work. |

## Dropped Families

Dropped completely:

- `VOICE`
- `UI-PRODUCT`
- donor backend/platform expansion
- personal-data autofetch and broad subconscious ingestion
- memory-platform replacement ideas

Dropped as already covered in current core:

- most `GOAL` rows
- most `CONNECT` rows
- most `SKILL` rows
- most `PROVIDER` rows
- broad "compress large things" rows without a stronger recoverability or content-kind delta

## Bottom Line

OpenHuman is not a good donor for broad product adoption in Ontocode. It is only useful here as a source of a few bounded runtime patterns.

After challenge, the practical keep set is six rows:

1. prepared-context pre-pass
2. wrapped-bundle salvage
3. read-only scout helper
4. recoverable lossy compaction
5. content-kind-aware reduction
6. unattended approval clamp

Everything else should stay dropped as already covered, non-core, or product-only.
