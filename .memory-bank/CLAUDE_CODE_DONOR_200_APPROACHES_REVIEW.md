# Claude Code Donor 200 Approaches Review

status: proposed
donor: `/home/evrasyuk/_workfolder/ontocode/tmp/claude-code-main`
target: Ontocode Rust workspace
date: 2026-06-16

## Scope

This is an idea inventory, not an implementation plan. Each item is useful only if a later ADR/impact pass proves it extends the existing Ontocode owner instead of creating a parallel stack.

Donor evidence sampled:
- `README.md`, `docs/architecture.md`, `docs/subsystems.md`, `docs/tools.md`, `docs/commands.md`, `docs/bridge.md`
- `src/tools.ts`, `src/Tool.ts`, `src/Task.ts`
- `src/commands/review.ts`, `src/commands/security-review.ts`, `src/commands/commit.ts`
- `src/services/SessionMemory/sessionMemory.ts`, `src/services/SessionMemory/prompts.ts`
- `src/services/PromptSuggestion/speculation.ts`, `src/query/tokenBudget.ts`, `src/query/stopHooks.ts`

OntoIndex evidence sampled:
- `ontocode-rs/core/src/tools/spec_plan.rs` is the current tool planning owner.
- `ontocode-rs/core/src/session/turn_context.rs` is a current turn config/context owner.
- `ontocode-rs/codex-mcp/src/connection_manager.rs` is a current MCP lifecycle/tool/resource owner.
- `ontocode-rs/core-skills/src/manager.rs` is a current skill loading/cache owner.

## 200 Candidate Approaches

| ID | Donor approach | Ontocode home | Why useful | First validation |
|---:|---|---|---|---|
| 001 | Keep a single exhaustive tool registry with feature-gated entries. | `core/src/tools/spec_plan.rs` | Reduces scattered tool exposure decisions. | `just test -p ontocode-core spec_plan` |
| 002 | Add a tool preset abstraction even if only `default` exists. | `core/src/tools/spec_plan.rs` | Gives future read-only/review presets a stable home. | Verify no new model-visible tool names. |
| 003 | Separate all possible tools from enabled tools. | `core/src/tools/spec_plan.rs` | Makes diagnostics explain disabled capabilities. | Snapshot planned tools for default config. |
| 004 | Add explicit disabled-reason metadata per tool. | `core/src/tools/spec_plan.rs` | Improves `/status` and tool troubleshooting. | Test disabled feature reports reason. |
| 005 | Keep tool visibility metadata distinct from runtime availability. | `codex-mcp/src/connection_manager.rs` | Prevents hidden MCP tools leaking to model context. | Test model-visible filtering. |
| 006 | Preserve stable ordering for tool list generation. | `core/src/tools/spec_plan.rs` | Improves prompt cache stability. | Compare rendered tool plan ordering. |
| 007 | Add tool alias resolution at registry boundary. | `protocol/src/tool_name.rs` | Centralizes old/new tool name compatibility. | Round-trip alias tests. |
| 008 | Add deny-rule filtering as a registry step. | `core/src/tools/spec_plan.rs` | Avoids per-tool policy checks for unavailable tools. | Config deny hides matching tool. |
| 009 | Make tool search optimistic but bounded. | `core/src/tools/handlers/tool_search.rs` | Keeps deferred tools discoverable without prompt bloat. | Tool-search result cap test. |
| 010 | Expose tool source/category metadata for diagnostics. | `core/src/tools/registry.rs` | Helps users understand built-in vs MCP vs plugin tools. | Protocol event shape test. |
| 011 | Track concurrency-safety per tool. | `core/src/tools/registry.rs` | Enables safe parallel tool-call execution policy. | Unit test blocks unsafe parallel pair. |
| 012 | Add tool progress message filtering. | `protocol/src/protocol.rs` | Reduces noisy progress events. | Event mapping tests. |
| 013 | Keep tool UI rendering separate from execution result. | `tui/src/*` | Prevents protocol/result semantics from depending on TUI formatting. | Snapshot one tool result. |
| 014 | Make read-only tool set explicit for review mode. | `core/src/tools/spec_plan.rs` | Makes reviews safer and cheaper. | Review-mode tool list test. |
| 015 | Add embedded-search capability detection. | `exec-server` / `core/src/tools/spec_plan.rs` | Avoids duplicate grep/glob tools when native search exists. | Feature-gated tool list test. |
| 016 | Add PowerShell tool exposure behind shell capability detection. | `exec-server` / `core/src/shell.rs` | Better Windows support without default complexity. | Windows-gated config test. |
| 017 | Add notebook-edit capability as a separately gated tool. | `core/src/tools` | Keeps notebook support optional and visible. | Tool disabled by default test. |
| 018 | Add synthetic-output/read-only structured result tool. | `protocol` / `core/src/tools` | Useful for agents returning machine-readable outputs. | JSON schema fixture. |
| 019 | Add ask-user tool with typed choices. | `protocol/src/request_user_input.rs` | Makes model-to-user clarifications structured. | Existing request-user-input tests. |
| 020 | Add explicit task output tool separate from task control. | `state/src/runtime/agent_jobs.rs` | Enables safe background result reads. | Agent job output pagination test. |
| 021 | Add typed tool permission context with allow/deny/ask sources. | `protocol/src/permissions.rs` | Makes permission behavior explainable. | Permission merge tests. |
| 022 | Preserve pre-plan permission mode and restore on exit. | `core/src/session/turn_context.rs` | Prevents plan mode from weakening permissions. | Plan enter/exit test. |
| 023 | Add auto-deny mode for background agents. | `core/src/tools/handlers/multi_agents*` | Prevents invisible permission prompts. | Background agent denied prompt test. |
| 024 | Await automated checks before permission dialog. | `core/src/guardian` | Reduces prompt churn and respects policy gates. | Guardian-before-dialog test. |
| 025 | Split permission prompt rendering from permission decision. | `tui` / `protocol` | Keeps app-server and TUI consistent. | Protocol approval fixture. |
| 026 | Add permission logging with redaction. | `otel` / `analytics` / `core/src/guardian` | Helps diagnose approvals without leaking secrets. | Redaction test. |
| 027 | Add wildcard permission rule docs in generated help. | `execpolicy` / `config` | Makes rule behavior discoverable. | Config schema/help test. |
| 028 | Add read-only shell validation before command execution. | `execpolicy` / `exec-server` | Prevents accidental writes in review/speculation paths. | Read-only command denial tests. |
| 029 | Track denial reasons for repeated tool failures. | `core/src/tools/sandboxing.rs` | Better model recovery after permission denial. | Denial metadata test. |
| 030 | Add batch approval plan mode for grouped changes. | `core/src/session` | Useful for multi-file edits with explicit scope. | Plan approval flow test. |
| 031 | Add review command as first-class prompt command. | `core/src/tasks/review.rs` | Existing review path can be more discoverable. | Review prompt snapshot. |
| 032 | Add security-review command as plugin-backed command. | `core-skills` / `core-plugins` | Security review can live outside core. | Plugin command load test. |
| 033 | Add false-positive filter phase to security review. | `core/src/tasks/review.rs` | Raises finding precision. | Prompt snapshot asserts filter rules. |
| 034 | Add confidence threshold to security findings. | `core/src/tasks/review.rs` | Avoids noisy low-confidence reports. | Review output parser test. |
| 035 | Add hard exclusions for known noisy security categories. | `prompts/templates/review` | Reduces security-review false positives. | Snapshot prompt. |
| 036 | Add concrete exploit-path requirement in security review. | `prompts/templates/review` | Keeps findings actionable. | Snapshot prompt. |
| 037 | Add review toolset restriction to read/diff/search tools. | `core/src/tasks/review.rs` | Avoids mutation during review. | Tool plan test. |
| 038 | Add local-only review mode separate from remote review. | `core/src/tasks/review.rs` | Keeps hosted review optional. | Config flag test. |
| 039 | Add branch diff preamble to review prompts. | `core/src/tasks/review.rs` | Gives model consistent evidence. | Prompt fixture. |
| 040 | Add PR selection fallback when PR number missing. | `app-server` / `tui` review command | Improves review command UX. | Command arg test. |
| 041 | Add commit command with scoped git allowlist. | `tui` command layer / `core` task | Safer commit automation. | Tool allowlist fixture. |
| 042 | Forbid commit amend unless user explicitly asks. | Git command prompt rules | Prevents destructive history edits. | Prompt snapshot. |
| 043 | Warn before committing likely secrets. | Git command prompt rules | Prevents accidental credential commits. | Fixture with `.env`. |
| 044 | Use HEREDOC commit messages. | Git command prompt rules | Avoids shell quoting bugs. | Prompt snapshot. |
| 045 | Include recent commit style in commit prompt. | Git command prompt rules | Produces repo-local commit messages. | Prompt fixture. |
| 046 | Add PR creation command as plugin/skill. | `core-skills` / GitHub plugin | Avoids adding GitHub workflow to core. | Skill invocation test. |
| 047 | Add duplicate issue triage workflow as external automation. | `.github/workflows` / scripts | Useful repo ops without runtime bloat. | Workflow lint. |
| 048 | Add issue lifecycle comment automation. | `.github/workflows` | Keeps issue state transparent. | Workflow dry-run. |
| 049 | Add non-write-user CI check. | `.github/workflows` | Prevents unsafe external write actions. | Workflow permissions review. |
| 050 | Add auto-close duplicate issue flow with audit log. | `.github/workflows` / scripts | Lowers maintainer load. | Script test. |
| 051 | Add background task IDs with type prefixes. | `state/src/runtime/agent_jobs.rs` | Makes job IDs readable and collision-resistant. | ID format tests. |
| 052 | Use case-insensitive-safe task ID alphabet. | `state/src/runtime/agent_jobs.rs` | Avoids platform path ambiguity. | ID charset test. |
| 053 | Store task output in per-task files with offsets. | `state` / `thread-store` | Enables paged output reads. | Output offset test. |
| 054 | Add terminal-state helper for jobs. | `state/src/runtime/agent_jobs.rs` | Prevents updates to completed jobs. | State transition tests. |
| 055 | Track notified flag for task completion. | `state/src/runtime/agent_jobs.rs` | Avoids duplicate completion notifications. | Notification idempotence test. |
| 056 | Add task cleanup callback hook. | `state/src/runtime/agent_jobs.rs` | Prevents orphaned resources. | Cancellation cleanup test. |
| 057 | Separate task kill interface from spawn/render. | `state` | Reduces trait surface. | Compile and state tests. |
| 058 | Add task type enum for local, remote, workflow, monitor. | `protocol` / `state` | Makes job origin clear. | Serialization test. |
| 059 | Add background monitor task type. | `state` / MCP monitor | Useful for long-running MCP health checks. | Monitor lifecycle test. |
| 060 | Add task output read-only API. | `app-server-protocol` | Lets clients inspect background jobs safely. | API schema fixture. |
| 061 | Add session memory file with fixed section template. | `.memory-bank` / `core/src/context` | Improves continuity after compaction. | Template preservation test. |
| 062 | Preserve section headers and descriptions during memory updates. | `.memory-bank` tooling | Prevents model from corrupting memory schema. | Markdown structure test. |
| 063 | Gate memory extraction by token threshold. | `core/src/context_manager` | Avoids over-frequent memory writes. | Threshold tests. |
| 064 | Gate memory extraction by tool-call threshold. | `core/src/context_manager` | Extract only after meaningful activity. | Tool-call count tests. |
| 065 | Extract memory at natural no-tool conversation breaks. | `core/src/context_manager` | Captures decisions before compaction. | Turn history fixture. |
| 066 | Use forked subagent for memory extraction. | `core/src/context_manager` | Keeps main turn responsive. | Background extraction test. |
| 067 | Use strict file permissions for memory files. | `thread-store` / memory layer | Protects private notes. | Permission mode test. |
| 068 | Read current memory through existing file-read path. | `core/src/context` | Reuses cache/redaction behavior. | File-read integration test. |
| 069 | Drop stale file-read cache before memory read. | `core/src/context` | Avoids updating from stale memory content. | Cache invalidation test. |
| 070 | Allow custom memory template path. | Config / memory layer | Lets teams tailor continuity. | Config path test. |
| 071 | Allow custom memory update prompt. | Config / memory layer | Enables domain-specific memory rules. | Missing-file fallback test. |
| 072 | Add per-section memory token limits. | `core/src/context` | Prevents one section dominating context. | Section truncation test. |
| 073 | Add total session memory token limit. | `core/src/context` | Hard cap for prompt injection. | Token budget test. |
| 074 | Generate reminders for oversized memory sections. | memory update prompt | Nudges model to condense safely. | Prompt fixture. |
| 075 | Keep exact user-requested outputs in memory key-results. | `.memory-bank` practice | Prevents losing deliverables. | Memory fixture. |
| 076 | Track failed approaches in memory. | `.memory-bank` practice | Avoids repeated dead ends. | Section presence check. |
| 077 | Add current-state memory section as mandatory update target. | `.memory-bank` | Makes resumed work accurate. | Template test. |
| 078 | Add workflow commands section in memory. | `.memory-bank` | Speeds repeated validation. | Memory content lint. |
| 079 | Add memory extraction telemetry without content. | `otel` / `analytics` | Diagnoses extraction frequency safely. | Redaction test. |
| 080 | Drain pending memory extraction before graceful shutdown. | `core/src/session` | Avoids lost continuity. | Shutdown test. |
| 081 | Add prompt suggestion/speculation in read-only overlay. | `core/src/context_manager` / TUI | Can suggest next prompts without mutating workspace. | Overlay cleanup test. |
| 082 | Limit speculation to max turns. | `core/src/context_manager` | Bounds background work. | Max-turn test. |
| 083 | Limit speculation to max messages. | `core/src/context_manager` | Prevents runaway context. | Message cap test. |
| 084 | Only allow safe read-only tools in speculation. | `core/src/tools/spec_plan.rs` | Prevents hidden writes. | Tool list test. |
| 085 | Copy accepted speculative writes from overlay to main workspace only. | `exec-server` | Makes speculative write adoption explicit. | Overlay copy test. |
| 086 | Remove speculation overlay with retry. | `exec-server` | Prevents temp dir leaks. | Cleanup test. |
| 087 | Strip thinking/redacted thinking before injecting speculation context. | `core/src/context_manager` | Avoids leaking non-user content. | Context fixture. |
| 088 | Strip pending or interrupted tool calls before speculation reuse. | `core/src/context_manager` | Prevents invalid tool-result pairs. | Message normalization test. |
| 089 | Track speculation boundary type/tool/detail. | `otel` | Helps tune prompt suggestions. | Telemetry redaction test. |
| 090 | Suppress speculation when command changes directories. | `execpolicy` / shell parser | Avoids overlay path confusion. | Command parser test. |
| 091 | Add token-budget continuation tracker. | `core/src/context_manager` | Gives bounded autonomous continuation. | Budget decision tests. |
| 092 | Stop continuation on diminishing token returns. | `core/src/context_manager` | Avoids wasting budget. | Diminishing-return test. |
| 093 | Emit completion event for budget stop. | `protocol` / `otel` | Makes auto-continuation auditable. | Event fixture. |
| 094 | Keep token budget disabled for subagents by default. | `core/src/agent` | Prevents nested runaway loops. | Subagent budget test. |
| 095 | Add model-visible nudge message for budget continuation. | prompts / context fragments | Makes continuation explicit. | Prompt fixture. |
| 096 | Save cache-safe turn snapshot for stop hooks. | `core/src/session` | Allows post-turn jobs to inspect stable context. | Snapshot test. |
| 097 | Classify dispatched job state after each turn. | `state` / agent jobs | Better job status accuracy. | Job classifier fixture. |
| 098 | Time-bound job classifier writes. | `state` | Prevents hung post-turn hooks. | Timeout test. |
| 099 | Skip background bookkeeping in bare/script mode. | CLI / session | Keeps non-interactive mode predictable. | Print-mode test. |
| 100 | Run memory extraction as fire-and-forget but drainable. | session shutdown | Balances latency and durability. | Drain test. |
| 101 | Add teammate idle hook. | `hooks` / agent jobs | Enables agent coordination events. | Hook event test. |
| 102 | Add task completed hook. | `hooks` | Lets automation react to finished jobs. | Hook payload test. |
| 103 | Add stop hook summary message. | `core/src/session` | Gives model concise post-turn hook feedback. | Context fixture. |
| 104 | Store hook progress as structured events. | `protocol` / `hooks` | Better TUI/app-server rendering. | Protocol fixture. |
| 105 | Isolate subagent hooks from main-session snapshots. | `core/src/agent` | Prevents child agents overwriting parent context. | Nested agent test. |
| 106 | Add bridge feature gate with import-safe stubs. | `app-server` / bridge owners | Allows IDE bridge prep without runtime activation. | Feature-off compile test. |
| 107 | Add no-op bridge handle. | `app-server` / TUI | Simplifies callers when bridge disabled. | Disabled bridge test. |
| 108 | Add bridge state enum. | `app-server-protocol` | Makes clients render connection state consistently. | Serialization test. |
| 109 | Add bridge session spawn modes. | `app-server` / exec-server | Supports single-session/worktree/same-dir later. | Config enum test. |
| 110 | Use JWT expiry to schedule proactive bridge token refresh. | auth / app-server | Prevents mid-session disconnects. | Token refresh scheduler test. |
| 111 | Add trusted-device token boundary. | login/auth | Supports stronger remote session auth. | Redacted token test. |
| 112 | Add workspace-scoped work secret type. | app-server / auth | Avoids loose env payload parsing. | Decode/validate test. |
| 113 | Deduplicate inbound/outbound bridge message UUIDs. | app-server | Prevents echo loops. | Bounded set test. |
| 114 | Batch bridge outbound writes. | app-server | Reduces network chatter. | Flush gate test. |
| 115 | Add bridge permission delegation path. | app-server / core approvals | Lets IDE decide approvals. | Permission response fixture. |
| 116 | Add bridge attachments ingestion. | app-server / protocol | Supports IDE file/context attachments. | Attachment parsing test. |
| 117 | Add bridge status TUI indicator. | `tui` | Users need connection health. | Snapshot test. |
| 118 | Add bridge diagnostics command. | `tui` / app-server | Helps debug remote-control setup. | Command output fixture. |
| 119 | Add bridge env-less transport abstraction. | app-server | Future-proofs direct worker connections. | Transport trait test. |
| 120 | Add bridge teardown flush before close. | app-server | Avoids lost final messages. | Teardown test. |
| 121 | Add MCP explorer server for source browsing. | `codex-mcp` / dev tooling | Useful for donor/codebase exploration. | MCP server smoke test. |
| 122 | Add list-tools MCP endpoint with source metadata. | `codex-mcp` | Helps external agents inspect capabilities. | List tools test. |
| 123 | Add list-commands MCP endpoint. | app-server / commands | Makes slash commands discoverable. | Command registry test. |
| 124 | Add get-tool-source MCP endpoint gated to dev mode. | `codex-mcp` | Improves local debugging. | Dev-only permission test. |
| 125 | Add get-command-source MCP endpoint gated to dev mode. | `codex-mcp` | Helps command review. | Dev-only permission test. |
| 126 | Add read-source-file MCP endpoint with path allowlist. | `codex-mcp` | Safe code exploration. | Path traversal test. |
| 127 | Add search-source MCP endpoint using `rg`. | `codex-mcp` / search | Fast donor/current repo search. | Result cap test. |
| 128 | Add architecture overview MCP prompt. | docs/MCP | Helps external clients orient. | Prompt fixture. |
| 129 | Add compare-tools MCP prompt. | docs/MCP | Useful for tool migration reviews. | Prompt fixture. |
| 130 | Add MCP server custom source root env. | `codex-mcp` | Lets users point at alternate repo roots. | Env validation test. |
| 131 | Add command types: prompt/local/local-UI. | TUI command layer | Clarifies command execution surface. | Command registry enum test. |
| 132 | Add command-level allowed tool list. | TUI command layer | Keeps slash commands least-privilege. | Allowed tools test. |
| 133 | Add command progress message field. | TUI command layer | Better UX during long commands. | Snapshot test. |
| 134 | Add command source metadata. | TUI command layer | Distinguishes builtin/plugin/user commands. | Command listing test. |
| 135 | Add moved-to-plugin command wrapper. | core-plugins | Shrinks core while preserving UX. | Legacy command test. |
| 136 | Add frontmatter parser for markdown commands. | core-skills / plugins | Lets plugins ship prompt commands as markdown. | Frontmatter tests. |
| 137 | Execute shell substitutions in prompt commands with allowlist. | command layer | Gives commands current evidence safely. | Substitution allowlist test. |
| 138 | Add `/doctor` style diagnostics as structured screen. | TUI / app-server | Existing diagnostics can be unified. | Snapshot + API test. |
| 139 | Add `/status` session/system summary. | TUI | Gives compact operational state. | Snapshot test. |
| 140 | Add `/cost` or usage summary command. | token usage / TUI | Makes spend visible. | Usage fixture. |
| 141 | Add `/files` context listing. | context manager / TUI | Helps users inspect active context. | Context snapshot. |
| 142 | Add `/ctx` visualization. | context manager / TUI | Debugs prompt/context bloat. | Snapshot test. |
| 143 | Add `/add-dir` with scoped workspace roots. | config / permissions | Safer multi-root context. | Permission root test. |
| 144 | Add `/output-style` via prompt fragments. | prompts / config | Customizes response style without core branches. | Fragment test. |
| 145 | Add `/fast` mode as config overlay. | model-provider / config | Easier low-latency mode. | Config overlay test. |
| 146 | Add `/effort` command mapped to reasoning effort. | config / model-provider | User can tune reasoning. | Turn config test. |
| 147 | Add `/model` picker command. | models-manager / TUI | Makes provider model changes discoverable. | Picker snapshot. |
| 148 | Add `/permissions` command. | config / TUI | Users can inspect approval rules. | Config edit test. |
| 149 | Add `/hooks` command. | hooks / TUI | Makes hook state manageable. | Hook list test. |
| 150 | Add `/skills` command. | core-skills / TUI | Makes skill availability visible. | Skill list test. |
| 151 | Add plugin marketplace command. | core-plugins / app-server | Manages plugins without manual files. | Plugin command test. |
| 152 | Add reload-plugins command. | core-plugins | Faster plugin dev loop. | Cache clear test. |
| 153 | Add terminal setup command. | TUI / install context | Improves shell integration. | Platform-gated snapshot. |
| 154 | Add remote-env command. | app-server / config | Makes remote execution setup explicit. | Config fixture. |
| 155 | Add sandbox-toggle command. | permissions / TUI | Easier controlled permission mode changes. | Permission profile test. |
| 156 | Add session rename/tag commands. | thread-store / TUI | Better thread organization. | Thread metadata test. |
| 157 | Add export session command. | thread-store / rollout-trace | Useful audit artifact. | Export fixture. |
| 158 | Add share command behind explicit consent. | app-server | Optional collaboration. | Consent gate test. |
| 159 | Add history search dialog. | TUI / thread-store | Faster resume. | Snapshot + search test. |
| 160 | Add global search dialog for commands/files. | TUI | Better discoverability. | Snapshot test. |
| 161 | Add plugin install errors as typed UI states. | core-plugins / TUI | Improves failure diagnosis. | Snapshot test. |
| 162 | Add MCP server approval dialog copy tests. | TUI / MCP | Prevents confusing trust prompts. | Snapshot test. |
| 163 | Add MCP multiselect dialog for import. | TUI / MCP | Safer bulk server selection. | Snapshot test. |
| 164 | Add desktop MCP import dialog. | app-server / MCP | Helps migrate desktop configs. | Import fixture. |
| 165 | Add IDE auto-connect onboarding. | TUI / app-server | Smooth bridge setup. | Feature-gated snapshot. |
| 166 | Add IDE selection context indicator. | TUI / context | Makes active selection visible. | Snapshot test. |
| 167 | Add clickable file refs in TUI. | TUI markdown/file links | Improves review navigation. | Markdown snapshot. |
| 168 | Add structured diff component. | TUI | Better edit review. | Snapshot test. |
| 169 | Add virtual message list. | TUI | Handles long sessions efficiently. | Render regression test. |
| 170 | Add offscreen freeze for expensive components. | TUI | Reduces render cost. | Performance smoke. |
| 171 | Add FPS metrics context for TUI. | TUI | Diagnoses terminal rendering. | Metrics test. |
| 172 | Add token warning component. | TUI / context manager | Warns before compaction pressure. | Snapshot test. |
| 173 | Add cost threshold dialog. | TUI / token usage | Lets users stop expensive sessions. | Snapshot test. |
| 174 | Add rate-limit option command/UI. | TUI / model-provider | Makes retry/limit state clear. | Rate-limit fixture. |
| 175 | Add mock rate-limit scenarios for tests. | model-provider tests | Improves limit handling coverage. | Scenario tests. |
| 176 | Add API error utility with typed cases. | model-provider / response-debug-context | Better provider diagnostics. | Error mapping test. |
| 177 | Add diagnostic tracking service. | diagnostics / otel | Centralizes file/summary diagnostics. | Redaction test. |
| 178 | Add internal logging container ID. | otel | Helps reproduce environment issues. | No-secret test. |
| 179 | Add package manager auto-update wrapper. | install / TUI | Better update UX. | Platform-gated test. |
| 180 | Add native auto-updater abstraction. | install context | Separates install channels. | Config test. |
| 181 | Add devcontainer firewall init script ideas. | devcontainer | Safer network defaults. | Shellcheck/static review. |
| 182 | Add strict/lax settings examples. | docs/examples | Helps users understand permission profiles. | Link check. |
| 183 | Add managed settings example. | config / docs | Enterprise policy clarity. | Schema validation. |
| 184 | Add issue templates for model behavior vs bugs. | `.github/ISSUE_TEMPLATE` | Better triage. | Template lint. |
| 185 | Add funding/security docs separation. | docs | Cleaner repo governance. | Link check. |
| 186 | Add changelog/feed generation. | release automation | Better release consumption. | Generated feed test. |
| 187 | Add build-bundle script with shims. | packaging | Helps binary/package builds. | Package smoke. |
| 188 | Add Bun/Node runtime shim audit. | packaging / scripts | Useful if TypeScript tools stay. | Script tests. |
| 189 | Add package-npm script with artifact checks. | npm packaging | Prevents broken packages. | Dry-run package test. |
| 190 | Add CI build script wrapper. | CI | Keeps local/CI commands aligned. | CI dry-run. |
| 191 | Add command test scripts for slash commands. | scripts/tests | Fast command regression checks. | `test-commands` equivalent. |
| 192 | Add MCP test script. | scripts/tests | Quick MCP smoke. | MCP script smoke. |
| 193 | Add auth test script. | login/auth | Faster auth flow verification. | Auth script smoke. |
| 194 | Add services-layer test script. | app-server/core services | Catches service wiring drift. | Service smoke. |
| 195 | Add plugin development guide. | plugins/README.md | Lowers extension authoring cost. | Link check. |
| 196 | Add plugin-specific README template. | core-plugins | Standardizes plugin docs. | Template lint. |
| 197 | Add code-review plugin as extension. | core-plugins / skills | Keeps review logic outside core. | Plugin fixture. |
| 198 | Add feature-dev plugin workflow. | core-plugins / skills | Encodes common feature implementation steps. | Skill fixture. |
| 199 | Add frontend-design plugin workflow. | core-plugins / skills | Keeps UI guidance opt-in. | Skill fixture. |
| 200 | Add simplification/senior-review plugin workflow. | core-skills / plugins | Captures overengineering review without bloating base prompt. | Skill fixture. |

## Immediate Next Step

Challenge this inventory before implementation. The fastest useful pass is to classify each row as `KEEP`, `NARROW`, `DEFER`, or `REJECT`, then move non-KEEP rows to a separate deferred file.
