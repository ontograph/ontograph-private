# ADR: Claude Code Donor Deferred, Narrow, and Rejected Proposals

- Status: proposed
- Date: 2026-06-16
- Parent ADR: [Claude Code Donor Core Extension Review](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW.md)
- Donor: `/home/evrasyuk/_workfolder/ontocode/tmp/claude-code-main`

## Purpose

This file parks the Claude Code donor proposals classified as `NARROW`, `DEFER`, or `REJECT`. They are not active core-extension work. Reopen one only after it has a concrete generated-code/text behavior, an existing Ontocode owner, bounded context/redaction rules, and a test plan.

## OntoIndex Challenge Review

OntoIndex owner checks do not justify promoting any parked row back to `KEEP`.

- MCP/resource rows stay parked. `ontocode-rs/codex-mcp/src/connection_manager.rs` already owns model-visible tool filtering, approval policy, permission profile, server/resource listing, resource reads, and tool calls. It is already large, so rows 122, 123, 128-130, and 145-147 must not become an MCP browser, source explorer, teaching server, command debugger, or second diagnostics surface.
- Context/cache/speculation rows stay parked. `ontocode-rs/core/src/session/turn.rs` already owns turn execution, prompt building, tool construction, compaction, and context updates. Rows 073, 084, 089, 090, 094, 095, and 187 can only reopen as one failing regression test in the existing context/compaction owners; no new context fragment, speculative cache, or eval asset.
- Hook rows stay parked. `ontocode-rs/hooks/src/engine/discovery.rs` already owns hook discovery and is a hot owner. Rows 097 and 101-104 must not introduce a second hook registry, policy layer, or uncapped hook output path.
- Agent/job/session rows stay parked. `ontocode-rs/state/src/runtime/agent_jobs.rs` already owns job creation, item state, cancellation, results, and progress. Rows 057-059 and 148-150 must not introduce scheduler behavior, parent/child job models, session command behavior, or persisted-state expansion.
- Code-search rows remain rejected. OntoIndex is the code-intelligence owner for symbol/file/process exploration, so rows 121 and 124-127 should stay `REJECT`.

Challenge outcome: keep this file as the parking lot. Reopen a row only by replacing its broad donor idea with a narrower existing-owner test gap and linking the accepted ADR that owns the work.

## Gemini Pre-Junior Consolidation Review

[GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_PRE_JUNIOR_PROJECT_PLAN.md](GEMINI_CLI_DONOR_CONTEXT_TOOLS_AGENTS_EVALS_PRE_JUNIOR_PROJECT_PLAN.md) was challenged against this parked list. None of these parked Claude Code rows should be dispatched from that Gemini pre-junior plan.

Duplicate Gemini scopes removed or blocked there:

- Context/memory/prompt-cache overlap: rows 066-078 and 081-095.
- Hook/MCP/command overlap: rows 097, 101-104, 106-130, and 131-160.
- UI/release/eval/plugin overlap: rows 161-180 and 181-200.

The Gemini plan keeps only its existing narrow context-fidelity regression test. It must not reopen parked Claude rows 073, 084, 089, 090, 094, 095, or 187.

## Oh My Pi Pre-Junior Consolidation Review

[OH_MY_PI_DONOR_KEEP_REFACTOR_MAP_PRE_JUNIOR_PROJECT_PLAN.md](OH_MY_PI_DONOR_KEEP_REFACTOR_MAP_PRE_JUNIOR_PROJECT_PLAN.md) was challenged against this parked list. Its accepted Oh My Pi rows remain dispatchable only as narrow test-first hardening of existing owners; this parked Claude list must not be used to broaden that plan.

Duplicate or broader Claude scopes blocked there:

- MCP/resource/debug overlap: rows 122, 123, 128-130, and 145-147.
- Context/compaction/prompt-cache overlap: rows 073, 084, 089, 090, 094, 095, and 187.
- Hook overlap: rows 097 and 101-104.
- Agent/job/session overlap: rows 057-059 and 148-150.
- UI/release/eval/plugin overlap: rows 161-180 and 181-200.

The Oh My Pi plan must not reopen those parked rows as MCP browser/debugger work, prompt-cache mechanism changes, a second hook registry, scheduler/session behavior, eval framework work, or plugin/release/UI scope.

## Parked Rows

| ID | Newness | Core Extension | Verdict | Refactor Home / Challenge |
| --- | --- | --- | --- | --- |
| 002 | Partial | Conditional | NARROW | Model presets must be config-driven and avoid a second model catalog. |
| 004 | Partial | Conditional | NARROW | Use only for measurable planning misses; avoid prompt churn. |
| 007 | Partial | Conditional | NARROW | Keep as bounded ranking metadata, not a new planner. |
| 008 | Partial | Conditional | NARROW | Only useful if current generated text lacks source attribution. |
| 013 | Existing | Non-core | DEFER | Preference presets are not core unless tied to runtime safety. |
| 015 | New | Conditional | NARROW | Notebook-style output is useful only as protocol/UI surface. |
| 016 | New | Conditional | DEFER | Windows-specific command behavior needs concrete bug evidence. |
| 017 | New | Conditional | DEFER | OS shell adapter polish is not a donor priority without failures. |
| 018 | New | Conditional | DEFER | Synthetic-output tooling risks fake evidence; require strict labeling. |
| 019 | Existing | Yes | NARROW | Extend current tool deny path, not a new policy layer. |
| 020 | Existing | Yes | NARROW | Keep as test coverage around existing tool planner behavior. |
| 022 | Partial | Conditional | NARROW | Add only missing explanation fields; avoid new permission engine. |
| 027 | Existing | Non-core | DEFER | Documentation/examples do not extend core. |
| 030 | New | Conditional | DEFER | Multi-profile permission UX needs product demand first. |
| 031 | Existing | Conditional | NARROW | Review command ideas belong in existing review prompt/skill path. |
| 032 | Partial | Non-core | NARROW | Useful as workflow docs or skill, not runtime core. |
| 038 | Partial | Conditional | NARROW | Keep only as structured review metadata, not new review service. |
| 039 | Existing | Conditional | NARROW | Git commit helpers should stay plugin/skill unless core state changes. |
| 040 | New | Non-core | DEFER | Issue triage workflow is useful but not core. |
| 041 | Partial | Non-core | NARROW | PR-body automation belongs in GitHub skill/plugin. |
| 042 | Partial | Non-core | NARROW | CI diagnostics belong in GitHub skill/plugin first. |
| 043 | Partial | Non-core | NARROW | Release note generation is non-core automation. |
| 044 | New | Non-core | DEFER | Repo workflow assistant should not enter core runtime. |
| 045 | Partial | Non-core | NARROW | Review templates are useful as memory-bank/prompt assets. |
| 046 | Partial | Non-core | DEFER | Changelog workflows are not core. |
| 047 | New | Non-core | DEFER | Marketplace-style triage belongs outside core. |
| 048 | New | Non-core | DEFER | Reviewer assignment is workflow automation, not core. |
| 049 | New | Non-core | DEFER | Merge babysitting is plugin behavior. |
| 050 | New | Non-core | DEFER | Branch hygiene automation is outside generated-code core. |
| 057 | Existing | Conditional | NARROW | Avoid duplicating current job list/status queries. |
| 058 | Partial | Conditional | NARROW | Add only missing parent/child relationships if needed. |
| 059 | New | Conditional | DEFER | Scheduler changes need concrete concurrency requirements. |
| 066 | New | Conditional | NARROW | Autonomous memory writes require explicit policy and caps. |
| 067 | Existing | Conditional | NARROW | Current memory-bank process can absorb this as docs discipline. |
| 068 | Existing | Conditional | NARROW | Keep as verification rule, not new storage. |
| 070 | New | Non-core | DEFER | Personalization memory is risky without product boundary. |
| 071 | New | Non-core | DEFER | Cross-project memory sharing should not be core now. |
| 073 | Existing | Conditional | NARROW | Respect existing context-fragment architecture. |
| 075 | Existing | Non-core | NARROW | Memory docs can be improved outside core. |
| 076 | Existing | Non-core | NARROW | Command guidance belongs in prompt/docs. |
| 077 | Existing | Non-core | NARROW | Project memory status can stay memory-bank tracking. |
| 078 | Existing | Non-core | NARROW | ADR hygiene belongs in docs process. |
| 081 | New | Conditional | DEFER | Speculative generation is risky and cache-hostile by default. |
| 082 | New | Conditional | DEFER | Background speculation needs measured latency win first. |
| 083 | New | Conditional | DEFER | Prediction cache should not be introduced without telemetry. |
| 084 | Partial | Conditional | NARROW | Keep only deterministic prompt caching constraints. |
| 085 | New | Conditional | DEFER | Multi-branch generation is expensive and hard to verify. |
| 086 | New | Conditional | DEFER | Autocomplete speculation is UI/product work. |
| 089 | Partial | Conditional | NARROW | Use as lint/check, not runtime rewrite. |
| 090 | Partial | Conditional | NARROW | Keep as bounded compression rule. |
| 094 | New | Conditional | NARROW | Use only for diagnostics, not context mutation. |
| 095 | Partial | Conditional | NARROW | Avoid frequent prompt mutation; enforce stability. |
| 097 | Partial | Conditional | NARROW | Hook output must be bounded and redacted. |
| 101 | Partial | Conditional | NARROW | Avoid a second hook registry. |
| 102 | Partial | Conditional | NARROW | Extend matcher tests only. |
| 103 | Partial | Conditional | NARROW | Keep hook policy declarative and bounded. |
| 104 | Existing | Conditional | NARROW | Existing hooks should absorb this as tests/docs. |
| 106 | New | Non-core | DEFER | Remote bridge is platform work, not immediate core. |
| 107 | New | Non-core | DEFER | Browser/web bridge should wait for app-server demand. |
| 108 | New | Conditional | DEFER | Bridge auth needs ADR and compatibility tests first. |
| 109 | New | Conditional | DEFER | Bridge transport should not duplicate MCP/app-server. |
| 110 | New | Conditional | DEFER | Bridge protocol must prove app-server cannot serve it. |
| 111 | New | Conditional | DEFER | External agent interop already has ADR context; avoid overlap. |
| 112 | New | Conditional | DEFER | Bridge resource sync risks parallel state. |
| 113 | New | Conditional | DEFER | Remote execution needs security review first. |
| 114 | New | Conditional | DEFER | Bridge event streaming belongs in app-server if needed. |
| 115 | Partial | Conditional | DEFER | Reuse app-server v2 protocol, not donor-specific bridge. |
| 116 | New | Conditional | DEFER | Cross-process bridge tests need concrete protocol. |
| 117 | New | Non-core | DEFER | Browser extension ideas are not core. |
| 118 | New | Non-core | DEFER | IDE extension ideas are not core runtime. |
| 119 | New | Conditional | DEFER | Remote workspace mapping is high-risk. |
| 120 | New | Conditional | DEFER | Bridge status UI is downstream of platform decision. |
| 122 | Partial | Conditional | NARROW | Add metadata to existing MCP resources only if missing. |
| 123 | New | Conditional | DEFER | MCP source browsing must not bypass OntoIndex/security policy. |
| 128 | Existing | Non-core | DEFER | Diagnostic explorer UI can wait. |
| 129 | New | Non-core | DEFER | MCP teaching/demo server is documentation/plugin work. |
| 130 | Partial | Conditional | NARROW | MCP resource caps/redaction are useful if added to current manager. |
| 131 | Partial | Conditional | NARROW | Slash command metadata should extend existing command system. |
| 133 | Partial | Conditional | NARROW | Command discovery must not duplicate tools. |
| 135 | Partial | Non-core | NARROW | Command docs can be generated from metadata. |
| 136 | Partial | Non-core | NARROW | User command examples are docs/skill assets. |
| 137 | New | Conditional | DEFER | Custom command marketplace is speculative. |
| 138 | Partial | Non-core | NARROW | Commit command belongs in GitHub/plugin workflow. |
| 139 | Partial | Non-core | NARROW | Review command belongs in review skill/prompt. |
| 140 | Partial | Non-core | NARROW | Security-review command belongs in review skill/hook surface. |
| 141 | Partial | Conditional | NARROW | Command execution policy should reuse existing approval path. |
| 143 | Partial | Conditional | NARROW | Command telemetry should be compact and opt-in. |
| 144 | Partial | Non-core | DEFER | Release command is automation, not core. |
| 145 | Partial | Non-core | NARROW | Eval command can live in tooling first. |
| 146 | Partial | Conditional | NARROW | Tool-call command debugging should be dev-only. |
| 147 | Partial | Conditional | NARROW | MCP command debugging extends existing manager diagnostics. |
| 148 | Existing | Conditional | NARROW | Config command should not add new config owner. |
| 149 | Existing | Conditional | NARROW | Session command should reuse current session state. |
| 150 | Existing | Conditional | NARROW | Resume command belongs in current rollout/session flow. |
| 151 | Partial | Non-core | NARROW | Skill command polish belongs in skill manager/UI. |
| 152 | Partial | Non-core | NARROW | Plugin command polish belongs in plugin manager/UI. |
| 153 | New | Non-core | DEFER | Command packs are speculative. |
| 154 | Partial | Non-core | DEFER | Command localization is not core now. |
| 155 | Partial | Conditional | NARROW | Command permissions can extend existing approval metadata. |
| 156 | Partial | Non-core | NARROW | Command usage docs should be generated, not hand-maintained. |
| 157 | Existing | Conditional | NARROW | Shell command behavior must reuse existing shell/sandbox owner. |
| 158 | New | Non-core | DEFER | Workflow macros are high-abstraction and speculative. |
| 159 | Partial | Non-core | NARROW | Project command templates belong in memory-bank/skills. |
| 160 | Partial | Non-core | NARROW | Command test fixtures useful, but not core alone. |
| 161 | Partial | Non-core | NARROW | TUI display polish is useful but not core extension. |
| 162 | Partial | Non-core | NARROW | TUI review panes need snapshot-backed UI task. |
| 163 | Partial | Non-core | DEFER | UI-only command palette changes need product demand. |
| 164 | Partial | Non-core | DEFER | Conversation decorations are not core. |
| 165 | New | Non-core | DEFER | Rich media output is not current core need. |
| 166 | Partial | Non-core | NARROW | Diagnostics display can improve supportability. |
| 167 | Partial | Non-core | NARROW | Compact status lines may help but need UI owner. |
| 168 | Partial | Non-core | NARROW | Diff display should reuse existing TUI diff patterns. |
| 169 | Existing | Non-core | DEFER | Rendering abstractions already exist; avoid churn. |
| 170 | New | Non-core | DEFER | Theme changes are cosmetic. |
| 171 | New | Non-core | DEFER | Layout experiments need design task. |
| 172 | Partial | Conditional | NARROW | Error topology display can surface existing structured errors. |
| 173 | New | Non-core | DEFER | Onboarding UI is not generated-code core. |
| 174 | Partial | Non-core | NARROW | Status/debug panes can use existing diagnostics. |
| 178 | Partial | Conditional | NARROW | Hook status display depends on hook diagnostic work. |
| 179 | New | Non-core | DEFER | Session timeline UI is optional. |
| 180 | New | Non-core | DEFER | UI eval viewer belongs in tooling first. |
| 181 | New | Non-core | DEFER | Release automation belongs outside runtime core. |
| 182 | Existing | Non-core | DEFER | Docs generation exists as process, not core runtime. |
| 183 | Partial | Non-core | NARROW | Evals can be a project plan, not core change. |
| 184 | Existing | Non-core | DEFER | CI workflow polish should stay repo automation. |
| 185 | Existing | Non-core | DEFER | Packaging changes require separate release ADR. |
| 186 | Partial | Non-core | DEFER | Benchmarks useful after runtime changes land. |
| 187 | Partial | Non-core | DEFER | Golden prompts/evals are test assets, not core behavior. |
| 188 | Partial | Non-core | DEFER | Release notes automation belongs in GitHub workflow. |
| 189 | Partial | Non-core | DEFER | Version checks are release engineering. |
| 190 | Partial | Non-core | DEFER | Dependency audit workflow is useful but not donor core. |
| 191 | Partial | Non-core | NARROW | Plugin packaging docs can improve extension workflow. |
| 192 | Partial | Conditional | NARROW | Plugin permission checks can extend existing plugin manager. |
| 193 | Partial | Conditional | NARROW | Skill loading diagnostics can extend `core-skills`. |
| 194 | Partial | Conditional | NARROW | Plugin cache validation can extend existing cache path. |
| 195 | Existing | Non-core | DEFER | Test matrix planning should stay project-plan work. |
| 196 | New | Non-core | DEFER | Marketplace publishing is speculative. |
| 197 | Partial | Non-core | NARROW | Extension docs should link to existing plugin/skill owners. |
| 198 | New | Non-core | DEFER | External extension certification is too early. |
| 199 | New | Non-core | DEFER | Extension analytics/privacy surface needs product ADR. |
| 200 | Partial | Non-core | NARROW | Senior-review simplification is useful as review skill/process. |
| 121 | Existing | Non-core | REJECT | OntoIndex already handles code exploration; do not clone it. |
| 124 | Existing | Non-core | REJECT | Raw code exposure duplicates existing tools and risks leakage. |
| 125 | Existing | Non-core | REJECT | Symbol search should use OntoIndex/search path. |
| 126 | Existing | Non-core | REJECT | File search should use existing shell/search/OntoIndex. |
| 127 | Existing | Non-core | REJECT | Grep wrappers are not core functionality. |
