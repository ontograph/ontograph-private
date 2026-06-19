# Oh My Pi Donor Review: 200 Useful Solutions

Status: donor inventory only. No implementation is authorized by this file.

Date: 2026-06-16

Donor source: `tmp/oh-my-pi-main`

Current-codebase rule: use these as implementation prompts only after they pass the existing architecture reuse checks. Do not import Oh My Pi code wholesale, and do not create parallel owners for tools, providers, MCP, hooks, context, memory, shell, or agents.

## Donor Evidence

- `README.md`: describes `omp` as an IDE-connected coding agent with read/write/search/LSP/DAP/task/job/browser/web-search tools, provider streaming, subagents, memory, rules, MCP, hooks, skills, review, git helpers, and release/eval automation.
- `docs/tools/read.md`, `docs/tools/edit.md`, `docs/tools/search.md`, `docs/tools/lsp.md`, `docs/tools/task.md`: strongest tool behavior references.
- `docs/session.md`, `docs/memory.md`, `docs/compaction.md`, `docs/handoff-generation-pipeline.md`: strongest context and session references.
- `docs/hooks.md`, `docs/mcp-runtime-lifecycle.md`, `docs/extension-loading.md`, `docs/skills.md`: strongest hooks, MCP, extension, and skill references.
- `docs/bash-tool-runtime.md`, `docs/notebook-tool-runtime.md`, `docs/approval-mode.md`, `docs/secrets.md`: strongest runtime, approval, and security references.
- `docs/provider-streaming-internals.md`, `docs/toolconv/harmony.md`, `docs/slash-command-internals.md`: strongest provider/tool-conversion references.
- `scripts/*release*`, `scripts/*eval*`, `scripts/analyze_small_edits.py`, `scripts/tool_io.py`, `packages/typescript-edit-benchmark/src/tasks.ts`: strongest automation and eval references.

## Current Owners To Reuse

- File/read/search/edit: `ontocode-rs/file-system/`, `ontocode-rs/exec-server/`, `ontocode-rs/core/src/tools/`, `ontocode-rs/apply-patch/`, `ontocode-rs/core/tests/suite/apply_patch_cli.rs`, `ontocode-rs/core/tests/suite/shell_serialization.rs`, `ontocode-rs/core/tests/suite/search_tool.rs`.
- LSP and IDE context: existing TUI IDE context, app-server thread/editor context, and any current LSP-facing helpers. Do not add a second IDE service.
- Context, compaction, and memory: `ontocode-rs/core/src/session/turn.rs`, `ontocode-rs/core/src/compact.rs`, `ontocode-rs/core/src/context_manager/`, `ontocode-rs/core/src/context/`, `ontocode-rs/memories/`, `.memory-bank/`.
- MCP, hooks, skills, plugins: `ontocode-rs/codex-mcp/`, `ontocode-rs/rmcp-client/`, `ontocode-rs/mcp-server/`, `ontocode-rs/hooks/`, `ontocode-rs/core/src/hook_runtime.rs`, `ontocode-rs/core-plugins/`, `ontocode-rs/core-skills/`.
- Agents and subagents: `ontocode-rs/core/src/tools/handlers/multi_agents*`, `ontocode-rs/core/src/agent/`, `ontocode-rs/core/src/thread_manager.rs`, `ontocode-rs/core/tests/suite/subagent_notifications.rs`.
- Providers and streaming: `ontocode-rs/model-provider/`, `ontocode-rs/model-provider-info/`, `ontocode-rs/codex-api/`, `ontocode-rs/codex-client/`, `ontocode-rs/protocol/`.
- Evals and release automation: `ontocode-rs/core/tests/common/responses.rs`, `ontocode-rs/core/tests/common/test_codex.rs`, `ontocode-rs/core/tests/suite/`, `.github/workflows/`, `scripts/`.

## Blocked Or Deferred By Default

- No browser/Electron control, DAP debugger control, A2A/ACP protocol, notebook execution, or persistent language worker without a separate ADR.
- No new public config key, app-server API, SDK behavior, schema, support bundle, persistence path, or export path without an ADR and compatibility tests.
- No second provider registry, tool registry, MCP lifecycle pipeline, hook matcher, shell runtime, memory service, context injection path, or credential store.
- Anything injected into model context must use bounded context fragment architecture with hard caps.

## 200 Useful Solutions

### Read, Search, And File IO

1. Add a unified read behavior matrix for file, directory, archive, and internal-resource reads, mapped onto the existing file-system/tool owners.
2. Add capped directory summaries that show entries, type counts, and selected file snippets without dumping full trees.
3. Add multi-path read tests with stable ordering so model-visible output does not churn.
4. Add multi-path missing-file semantics: skip individual `ENOENT` entries but fail when all requested paths are missing.
5. Add selector-aware read tests for line ranges, symbols, and bounded snippets.
6. Add directory read tests that include tree output plus selected snippets under hard caps.
7. Add archive-member read fixtures for zip/tar-like inputs if the existing file-system layer supports archives.
8. Add SQLite table/row read fixtures only if the existing read owner already exposes database reads.
9. Defer URL/PDF/markdown conversion behind an ADR; keep current local file reads separate until that boundary is approved.
10. Return recoverable read validation failures as structured tool results where the existing tool protocol permits it, reserving hard aborts for unsafe or impossible reads.

### Search

11. Document the existing native search fallback matrix: ripgrep, platform fallback, and any in-process search path.
12. Add stable ranking fields to search results if the current protocol can expose them without a public API break.
13. Add per-file snippet caps so broad searches cannot flood model context.
14. Group search results by file and symbol when OntoIndex or syntax metadata is already available.
15. Add tests proving search respects gitignore defaults and explicit include/exclude options.
16. Add case-sensitive and case-insensitive search fixtures.
17. Add glob include/exclude fixtures for nested paths and generated files.
18. Defer search over URL/internal-resource schemes until the read owner accepts those resources.
19. Add search cost telemetry: files scanned, matches, bytes read, and truncation reason.
20. Add an eval that compares targeted symbol search versus broad text search on known code tasks.

### Edit And Patch Safety

21. Explore content-hash anchors for stale edit detection inside the existing apply-patch/edit owner.
22. Use Oh My Pi hashline as a benchmark donor, not as a replacement patch format.
23. Add explicit stale-anchor recovery messages that tell the model to reread before retrying.
24. Add preview-only edit output for risky transforms where current flows can stage before apply.
25. Add block-level tree-sitter edit fixtures only inside existing syntax-aware owners.
26. Keep line-range edit fallback for unsupported languages and prove it rejects ambiguous spans.
27. Add patch parser tests for foreign sentinels and malformed edit headers.
28. Add a warning channel for auto-normalized edit bodies, bounded and separate from patch content.
29. Include matched span metadata in edit results where current tool schema already supports metadata.
30. Add a wrong-file edit eval to catch patches that match similar text in the wrong module.

### Apply Patch And Shell Serialization

31. Expand custom `apply_patch` tool-call golden tests around create, update, move, and delete cases.
32. Add failure matrix coverage for missing file, duplicate target, invalid hunk, and impossible move.
33. Reuse structured response helpers for patch stdout/stderr assertions instead of string digging.
34. Add move-overwrite rejection tests.
35. Add move-into-new-directory behavior tests using the existing patch CLI harness.
36. Verify patch failure output stays bounded and model-visible.
37. Keep the existing `apply_patch_cli` harness as the single test home for patch CLI behavior.
38. Add parse-failure no-write tests to prove malformed patches leave the tree unchanged.
39. Add patch output truncation tests for very large generated diffs.
40. Verify exact changed files after patch application using the existing diff verification pattern.

### LSP And IDE Context

41. Add LSP rename preflight tests before any future rename automation.
42. Add `workspace/willRenameFiles` compatibility tests only if the existing LSP path already supports file renames.
43. Add references-before-edit eval cases for high-risk symbol edits.
44. Add diagnostics-after-write hook tests that reuse existing IDE or tool result owners.
45. Add active-buffer context parity tests so TUI and app-server agree on selected file state.
46. Add selection omission tests for empty or oversized editor selections.
47. Add hard truncation tests for large selections and opened buffers.
48. Normalize LSP unavailable, timeout, and bad-root errors through the existing tool error path.
49. Keep fallback from LSP to search explicit and model-visible.
50. Add LSP rename blast-radius reporting via OntoIndex before enabling any automated rename workflow.

### Debugger And Runtime Inspection

51. Treat DAP debugger integration as deferred ADR material, not a near-term implementation.
52. If DAP is later approved, require explicit approval for attach and launch.
53. Redact debug frame paths, environment values, and variable values through shared redaction.
54. Add variable count, byte, and nesting caps before exposing debug variables to context.
55. Define timeout and cancel semantics before any debugger command is added.
56. Use donor debugger docs as a crash-triage workflow reference only.
57. Never auto-attach to a process from model output.
58. Build debugger tests from transcript fixtures, not live debugger sessions, if this area is later approved.
59. Define fallback from debugger absence to logs and shell diagnostics.
60. Keep debugger tools disabled by default even after a future implementation.

### Shell And Command Runtime

61. Defer persistent shell sessions unless the existing shell owner needs them for a measured workflow.
62. Add command risk-classifier fixtures for destructive, network, secret-dumping, and harmless commands.
63. Surface critical command approval reasons with the existing approval prompt owner.
64. Add missing-CWD and not-directory shell error tests.
65. Treat output artifact spill as non-fatal and always keep a bounded model-visible summary.
66. Keep full artifact paths out of model-visible output unless already sanitized.
67. Add command repetition evals to detect looped failed commands.
68. Add timeout and cancellation tests around long-running shell commands.
69. Prefer existing native search/file helpers over adding shell wrappers for basic file discovery.
70. Add an environment snapshot test proving sensitive variables are redacted.

### Notebook And Code Execution

71. Add notebook-as-text conversion tests only for editing notebook files, not executing them.
72. Keep notebook execution disabled by default.
73. If notebook editing is needed, use explicit cell markers and round-trip tests.
74. Preserve metadata and cell ordering in notebook edit tests.
75. Defer persistent Python/Bun workers behind a separate execution-runtime ADR.
76. Block tool calls from code execution by default.
77. Require a permission profile before any interactive code execution worker exists.
78. Add code-output artifact spill tests before exposing long results to context.
79. Define kernel cancel semantics before any live notebook runtime.
80. Add snapshot tests for notebook diffs if the UI renders them.

### Context And Compaction

81. Add non-compaction retry policy tests for context overflow.
82. Add context overflow fixtures that exercise the existing `session/turn.rs` path.
83. Add same-model overflow retry guards so a failed retry does not loop.
84. Preserve injected rules across compaction only through existing bounded context fragments.
85. Add compaction request shape snapshots to catch accidental cache-busting churn.
86. Make compaction failure visible as an event with bounded details.
87. Add token-estimate telemetry for before and after compaction.
88. Add hard cap tests for every context item type.
89. Add prompt cache stability tests for repeated similar turns.
90. Add reinjection tests for required context after summary replacement.

### Memory

91. Keep retained memory mapped to `.memory-bank/` and existing memory crates rather than adding a donor memory service.
92. Make project-scoped memory the default for any future donor-inspired memory behavior.
93. Defer automatic memory extraction until source citations and redaction are designed.
94. Add memory redaction tests for tokens, cookies, auth headers, and private paths.
95. Add age and active-session skip policy tests for stale memory.
96. Bound memory synopsis size and fail tests when it can exceed context caps.
97. Require source citation for generated memory updates.
98. Add tests that memory writes are minimal and do not paste raw logs.
99. Route donor memory imports through the external-agent detector contract, not a new importer.
100. Add a memory drift checker that flags stale memory against authoritative code or ADRs.

### Rules, TTSR, And Policy Injection

101. Defer time-traveling stream rules until streaming abort/retry ownership is explicitly approved.
102. Use donor rulebook behavior to add regex rule lint tests for existing hooks or prompt config.
103. Ignore invalid regex rules with a warning instead of aborting startup.
104. Add glob-gated rule tests for included and excluded paths.
105. Defer AST-grep rule matching unless an existing syntax owner needs it.
106. If rules enter context, use the current context fragment owner and hard caps.
107. Surface rule source and rule id in diagnostics so injected text is explainable.
108. Add retry-budget tests before any stream-abort rule implementation.
109. Add infinite-loop guard tests for repeated rule matches.
110. Add redaction tests for rule-triggered diagnostics.

### Hooks

111. Collect per-hook load errors without aborting all hook loading.
112. Add startup tests with one bad hook path and one good hook path.
113. Add schema fixtures for hook inputs and outputs.
114. Add hook timeout result tests.
115. Add hook permission-request event tests if existing hooks can request approval.
116. Add hook secret-leak tests through shared redaction.
117. Add hook order tests for deterministic sequential execution.
118. Add hook stdout/stderr cap tests.
119. Add hook trust prompt tests before enabling untrusted hook sources.
120. Add config-layer merge tests for hook definitions from multiple sources.

### MCP

121. Split MCP discovery hard failures from per-server runtime/connect failures in tests and docs.
122. Add partial MCP server load fixtures where one server fails and others remain usable.
123. Emit a synthetic `.mcp.json`-style error only through the existing MCP status owner.
124. Normalize MCP runtime, schema, auth, and connect errors through one error map.
125. Add fixture MCP servers for resource read/list behavior.
126. Add MCP prompt/tool name conflict tests.
127. Add OAuth and token redaction tests for MCP diagnostics.
128. Add reconnect/backoff status tests for flaky MCP servers.
129. Include MCP tool provenance metadata where the current tool metadata model already supports it.
130. Add schema cycle guard tests for MCP tool schemas.

### Extensions, Skills, And Plugins

131. Add extension manifest detector tests without adding a new plugin loader.
132. Add plugin cache invalidation tests in the existing plugin owner.
133. Collect extension load errors per path, similar to hook load errors.
134. Treat `skill://` as a future internal resource idea, not an immediate URI scheme.
135. Add skill authoring fixture tests for manifest shape and path rules.
136. Add marketplace manifest validation if the existing plugin marketplace owner exposes one.
137. Add extension declared-permission tests before allowing new extension actions.
138. Defer plugin install/reload behavior unless current plugin code already has a lifecycle gap.
139. Add bundled-skill enable/disable tests.
140. Include extension provenance in tool metadata through the existing tool metadata owner.

### Agents And Subagents

141. Add typed subagent result schema tests.
142. Defer isolated subagent worktrees unless existing multi-agent handlers require filesystem isolation.
143. Treat `agent://id/path` output reads as a future internal-resource idea.
144. Add subagent progress snapshot tests.
145. Add async task/job state aggregation tests to the current multi-agent handler.
146. Add subagent cancel semantics tests.
147. Add subagent cost and duration display tests where TUI/app-server already exposes progress.
148. Add strict structured-result parsing tests that reject prose where JSON is required.
149. Defer peer coordination or IRC-style agent communication.
150. Add orphan cleanup tests for canceled or failed subagents.

### Review, Commit, And Git

151. Add review verdict fixtures with P0-P3 severity, confidence, and actionable file references.
152. Defer reviewer subagent dispatch until typed subagent results are mature.
153. Expose git overview, file diff, and hunk reads through existing read/search surfaces where possible.
154. Treat atomic commit splitting as an external workflow until current git owners need it.
155. Add commit-cycle rejection tests for invalid staged state and message failures.
156. Exclude lockfile churn from commit headline heuristics unless explicitly requested.
157. Treat `pr://` as a deferred internal-resource scheme.
158. Treat `issue://` as a deferred internal-resource scheme.
159. Treat `conflict://N` as a deferred conflict-resolution scheme.
160. If conflict writes are later approved, add tests for `ours`, `theirs`, and `base` resolution.

### Providers, Streaming, And Tool Conversion

161. Add provider streaming terminal event mapping tests.
162. Add failed, canceled, and error terminal-path fixtures.
163. Normalize stop reasons across providers through the existing provider owner.
164. Add usage extraction fixtures for providers with missing or partial usage metadata.
165. Add harmony/tool-conversion prompt rendering snapshots only in the existing prompt/protocol owner.
166. Add model-specific prompt adjustment evals before adding more prompt branches.
167. Add tool channel routing tests for commentary, analysis, and final outputs.
168. Add tool schema compaction tests for oversized schemas.
169. Add tool prompt usage telemetry to show expensive tool definitions.
170. Add provider-unavailable fallback message tests.

### TUI, UX, And Session Operations

171. Add session listing and recent-session snapshot tests.
172. Add session fork/resume behavior tests only through existing session owners.
173. Add handoff generation guards for too few messages.
174. Add TUI status rendering for async jobs only if current job state already exists.
175. Use accept/preview patterns for codemods instead of applying large edits silently.
176. Add approval prompt snapshots that show command, reason, and risk class.
177. Use donor theme sync scripts only as a reference for existing theme generation.
178. Add keybinding drift tests if current TUI keymaps are generated from docs or config.
179. Add transcript snapshot tests for user-visible session operations.
180. Add install-id and version diagnostics through existing diagnostics owners.

### Security, Approval, And Secrets

181. Turn donor secrets docs into a central redaction test checklist.
182. Add approval mode matrix tests for read-only, on-request, and dangerous operations.
183. Add critical-command override reason tests.
184. Add remote-fetch-execute risk fixtures.
185. Add denial fixtures for host shutdown, password file writes, and credential exfiltration patterns.
186. Reject or escalate clipboard/copy commands that look like bulk data exfiltration.
187. Prove MCP tokens and OAuth credentials never appear in diagnostics.
188. Keep browser/Electron control blocked by default.
189. Apply quote and citation caps before any future URL read feature.
190. Add artifact path redaction tests.

### Automation, Evals, And Release

191. Add an edit benchmark harness comparing existing patch behavior with hash-anchor alternatives.
192. Add a small-edit analyzer for wrong-file, wrong-range, and stale-context failures.
193. Add tool IO normalization scripts for fixture generation.
194. Add a prompt rewrite style checker for generated system prompt artifacts.
195. Add release notes generator tests rather than manual release-note assembly.
196. Add package version spoof checks for release workflows.
197. Add CI concurrency tests or config checks for duplicate release jobs.
198. Keep native build/signing docs as a release checklist, not runtime code.
199. Add session stats analysis for selector hit rate, read size, and search precision.
200. Add a deflake/retry report script for flaky eval and release jobs.

## Highest-Value First Slice

1. Add MCP partial-failure lifecycle tests: items 121-124 and 127.
2. Add apply-patch stale/wrong-file safety tests: items 21, 23, 30-40.
3. Add context overflow and compaction caps: items 81-90.
4. Add hook partial-load and redaction tests: items 111-120.
5. Add subagent typed-result and cleanup tests: items 141, 145, 146, 148, 150.

These slices are useful because they strengthen existing Ontocode owners without importing a parallel Oh My Pi architecture.
