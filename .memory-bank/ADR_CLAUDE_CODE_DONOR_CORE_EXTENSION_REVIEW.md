# ADR: Claude Code Donor Core Extension Review

- Status: proposed
- Date: 2026-06-16
- Donor: `/home/evrasyuk/_workfolder/ontocode/tmp/claude-code-main`
- Source inventory: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md`

## Context

The Claude Code donor review lists 200 potentially useful approaches for Ontocode. This ADR challenges those ideas against the current Ontocode architecture with two questions:

1. Is the idea new to this codebase, or does Ontocode already have an owner/implementation pattern?
2. Does the idea extend core functionality, or is it only workflow, documentation, plugin, or release-process surface?

OntoIndex evidence used for this review:

- `ontocode-rs/core/src/tools/spec_plan.rs` already owns tool routing/search/suggest policy. It is large and should be extended carefully, not bypassed with a parallel tool registry.
- `ontocode-rs/codex-mcp/src/connection_manager.rs` already owns MCP connection behavior, tool visibility, resource reads, tool calls, approval policy, permission profile, and OAuth-related surfaces.
- `ontocode-rs/state/src/runtime/agent_jobs.rs` already owns agent job lifecycle, progress, items, cancellation, and status persistence.
- `ontocode-rs/core-skills/src/manager.rs` already owns skill roots, skill config cache, and skill discovery.

## Decision

Do not implement the 200 donor approaches as-is.

Keep only proposals that add real new behavior, safety, compatibility, bounded context value, or operational value, and that plug into an existing Ontocode owner. Narrow or reject anything that creates a second provider factory, permission engine, tool registry, MCP gateway, session memory service, task runtime, hook registry, or bridge protocol.

Useful donor ideas should be implemented as small extensions to existing owners:

- Tool behavior extends `core/src/tools` and existing tool-spec routing.
- MCP behavior extends `codex-mcp` or `rmcp-client`.
- Job/task behavior extends `state/src/runtime/agent_jobs.rs`.
- Context and memory behavior uses bounded context fragments and the memory-bank/session owners.
- Skill/plugin behavior extends `core-skills`, `core-plugins`, or the existing plugin cache/manifest path.
- Review/security workflows belong in existing prompt, skill, hook, or GitHub integration surfaces unless they must affect runtime behavior.

## Classification Rules

- `New`: no clear current equivalent found from OntoIndex evidence or project memory.
- `Partial`: an owner exists, but the donor idea adds missing behavior or sharper policy.
- `Existing`: Ontocode already materially covers it.
- `Core`: changes runtime behavior, context construction, tool routing, MCP execution, permissions, session/job state, or protocol behavior.
- `Non-core`: docs, workflow, command convenience, release automation, UI presentation, or plugin-only behavior.
- `Conditional`: useful only after narrowing to an existing owner and proving a concrete user-facing gap.

Verdicts:

- `KEEP`: worth converting into a scoped implementation proposal.
- `NARROW`: useful direction, but too broad or partly duplicate.
- `DEFER`: not core now, needs real demand, or depends on later platform work.
- `REJECT`: duplicate or would create a parallel stack.

## High-Value Core Extensions

The strongest donor ideas are not the flashy commands. They are the small runtime constraints that make generated code/text safer and more predictable:

- Deterministic tool exposure and tool-search caps in existing tool planning.
- Permission denial explanations, dry-run previews, and compact approval traces in existing MCP/guardian paths.
- Agent-job progress and cancellation polish in `state/src/runtime/agent_jobs.rs`.
- Bounded session memory and context-fragment dedupe that respects the model-context rules.
- Security-review and sensitive-output checks wired through existing review/hook surfaces.
- Prompt/context budget accounting that avoids cache churn and oversized fragments.

## Row-Level Review

`NARROW`, `DEFER`, and `REJECT` rows were moved to [Claude Code Donor Deferred, Narrow, and Rejected Proposals](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md). This ADR keeps only active `KEEP` rows.

| ID | Newness | Core Extension | Verdict | Refactor Home / Challenge |
| --- | --- | --- | --- | --- |
| 001 | Partial | Yes | KEEP | Extend existing tool-spec metadata; no new registry. |
| 003 | Partial | Yes | KEEP | Add stricter tool visibility policy in existing router. |
| 005 | Partial | Yes | KEEP | Add deterministic tool ordering/caps where tool lists are built. |
| 006 | Partial | Yes | KEEP | Improve tool search/suggest behavior in `core/src/tools`. |
| 009 | Partial | Yes | KEEP | Add safety metadata to current tool specs. |
| 010 | Partial | Yes | KEEP | Add explicit disabled-tool reasons in current tool exposure path. |
| 011 | New | Yes | KEEP | Add compact tool-call audit facts through existing telemetry/state. |
| 012 | Partial | Yes | KEEP | Strengthen tool result normalization before model context injection. |
| 014 | Partial | Yes | KEEP | Add bounded tool-output caps to existing context handling. |
| 021 | Partial | Yes | KEEP | Permission prompts should reuse current MCP/guardian approval path. |
| 023 | Partial | Yes | KEEP | Dry-run command previews are core if wired into existing approvals. |
| 024 | Partial | Yes | KEEP | Denial reason propagation improves runtime safety. |
| 025 | Partial | Yes | KEEP | Compact approval history can improve context and auditability. |
| 026 | Partial | Yes | KEEP | Allow/deny pattern tests should target existing matcher. |
| 028 | Partial | Yes | KEEP | Add redaction checks to approval/log output. |
| 029 | New | Yes | KEEP | Permission fallback behavior can close safety gaps. |
| 033 | New | Yes | KEEP | Security-review false-positive filters can extend review hooks. |
| 034 | New | Yes | KEEP | Secret-detection review evidence is core if enforced in diagnostics. |
| 035 | New | Yes | KEEP | Sensitive-output checks extend existing redaction policy. |
| 036 | New | Yes | KEEP | Bounded review findings can improve generated text quality. |
| 037 | Partial | Yes | KEEP | Existing review behavior can gain stronger test-gap checks. |
| 051 | Partial | Yes | KEEP | Extend `state/src/runtime/agent_jobs.rs`; no parallel Task runtime. |
| 052 | New | Yes | KEEP | Add richer job progress events to existing job owner. |
| 053 | Partial | Yes | KEEP | Cancellation semantics should strengthen current job lifecycle. |
| 054 | Partial | Yes | KEEP | Job item status history belongs in state runtime. |
| 055 | New | Yes | KEEP | Add compact job timeline for agent-generated work. |
| 056 | Partial | Yes | KEEP | Persist task errors with redacted structured details. |
| 060 | Partial | Yes | KEEP | Extend job progress API with bounded summaries. |
| 061 | Partial | Yes | KEEP | Memory exists, but runtime session memory needs bounded owners. |
| 062 | Partial | Yes | KEEP | Add hard caps and provenance for injected memory fragments. |
| 063 | Partial | Yes | KEEP | Add dedupe before context injection. |
| 064 | New | Yes | KEEP | Add stale-memory detection tied to source evidence. |
| 065 | New | Yes | KEEP | Add memory write audit records without raw private data. |
| 069 | New | Yes | KEEP | Add bounded memory fragment type if runtime context needs it. |
| 072 | Partial | Yes | KEEP | Improve context-source attribution in existing context path. |
| 074 | New | Yes | KEEP | Add context item size warnings before model calls. |
| 079 | Partial | Yes | KEEP | Add memory/context validation tests. |
| 080 | Partial | Yes | KEEP | Add context budget assertions for generated text/code paths. |
| 087 | Partial | Yes | KEEP | Add prompt-budget accounting to existing model-call construction. |
| 088 | Partial | Yes | KEEP | Add cache-miss diagnostics for context changes. |
| 091 | Partial | Yes | KEEP | Add stable fragment ordering tests. |
| 092 | New | Yes | KEEP | Add oversize-fragment hard failure where context is built. |
| 093 | New | Yes | KEEP | Add per-fragment provenance for generated text context. |
| 096 | Partial | Yes | KEEP | Stop-hook summaries can extend existing hook/event path. |
| 098 | New | Yes | KEEP | Add hook failure classification if current path lacks it. |
| 099 | Partial | Yes | KEEP | Add hook result context caps. |
| 100 | Partial | Yes | KEEP | Add hook execution audit summary. |
| 105 | Partial | Yes | KEEP | Hook diagnostics can extend runtime safety. |
| 132 | Partial | Yes | KEEP | Command allow-list behavior affects runtime safety. |
| 134 | Partial | Yes | KEEP | Command arguments should get stricter validation. |
| 142 | Partial | Yes | KEEP | Command output redaction is core safety. |
| 175 | Partial | Yes | KEEP | User-visible redaction failures are core safety. |
| 176 | Partial | Yes | KEEP | Tool failure explanations improve generated workflow behavior. |
| 177 | Partial | Yes | KEEP | Approval status rendering can prevent unsafe confusion. |

## Implementation Guidance

The next implementation plan should not start with 200 items. Start with one thin vertical slice:

1. Tool and MCP safety: rows 001, 003, 005, 010, 014, 021, 023, 024, 028, 029, 130, 132, 142.
2. Job/task state: rows 051, 052, 053, 054, 055, 056, 060.
3. Bounded context and memory: rows 061, 062, 063, 064, 065, 069, 072, 074, 079, 080, 087, 088, 091, 092, 093.
4. Hook/review safety: rows 033, 034, 035, 036, 096, 098, 099, 100, 105, 175, 176, 177.

Everything else should remain deferred unless it is converted into a narrower proposal with:

- existing owner module,
- concrete generated-code/text behavior improved,
- bounded context and redaction plan,
- compatibility/test plan,
- proof that it does not duplicate OntoIndex, MCP, plugin, hook, command, or job infrastructure.

## Consequences

This ADR reduces the donor review from an idea inventory to an architecture gate. It preserves useful Claude Code ideas, but only when they strengthen Ontocode's existing core owners. The main expected benefit is safer generated code/text behavior without creating parallel runtimes or broad product churn.

The cost is that many donor ideas remain deferred even if they are attractive. That is intentional: non-core automation, bridge, marketplace, and UI ideas should wait until a concrete Ontocode owner and user-facing need are proven.
