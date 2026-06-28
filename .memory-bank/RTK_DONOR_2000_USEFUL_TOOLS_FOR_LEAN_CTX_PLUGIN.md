# RTK Donor 2000 Useful Tools For Lean-ctx Plugin

Status: challenged-native-only, no implementation dispatch
Date: 2026-06-28
Donor source: `tmp/rtk-main`
Target: `plugins/ontocode-lean-ctx` package boundary and
`third_party/lean-ctx-fork` native backend owner

## Senior Challenge

The donor is a command-output compression proxy. The target is a repo-local
plugin for the Ontocode-maintained lean-ctx backend.

This is not a 2000-tool implementation queue. RTK has useful operational
patterns, but most of its surface violates the current lean-ctx plugin contract
and the native-only constraint for this review:

- no `ctx_shell`
- no editing
- no session or knowledge tools
- no plugin-owned backend process spawning
- no required external third-party runtime dependency

Challenge result: the first pass over-kept RTK operational wrapper ideas. Under
the current third-party migration rule and the native-only constraint,
`plugins/ontocode-lean-ctx` is not the owner for RTK-style preflight scripts,
shell rewrites, telemetry, TOML filters, plugin docs automation, or package
validation wrappers. Keep only ideas that can be adopted into the maintained
lean-ctx backend implementation behind the existing `ctx_read`, `ctx_search`,
and `ctx_summary` tools.

## Current Contract

Current `ontocode-lean-ctx` plugin state:

- package root: `plugins/ontocode-lean-ctx`
- backend source home: `third_party/lean-ctx-fork`
- transport: Streamable HTTP MCP
- endpoint: `http://127.0.0.1:7777`
- auth env var: `LEANCTX_TOKEN`
- allowed tools: `ctx_read`, `ctx_search`, `ctx_summary`

Authority:

- `plugins/ontocode-lean-ctx/README.md`
- `plugins/ontocode-lean-ctx/.codex-plugin/plugin.json`
- `plugins/ontocode-lean-ctx/.mcp.json`
- `third_party/lean-ctx-fork/rust/src/tools/ctx_read/`
- `third_party/lean-ctx-fork/rust/src/tools/ctx_search.rs`
- `third_party/lean-ctx-fork/rust/src/tools/ctx_summary.rs`
- `AGENTS.md` third-party migration rule
- `ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL.md`

## Donor Evidence Reviewed

RTK surfaces sampled:

- command proxy and filter architecture:
  - `tmp/rtk-main/src/cmds/README.md`
  - `tmp/rtk-main/src/filters/README.md`
  - `tmp/rtk-main/src/core/toml_filter.rs`
- hook and rewrite architecture:
  - `tmp/rtk-main/src/hooks/README.md`
  - `tmp/rtk-main/src/hooks/verify_cmd.rs`
  - `tmp/rtk-main/src/hooks/integrity.rs`
  - `tmp/rtk-main/src/discover/README.md`
  - `tmp/rtk-main/src/discover/rules.rs`
- install and documentation checks:
  - `tmp/rtk-main/scripts/check-installation.sh`
  - `tmp/rtk-main/scripts/test-install.sh`
  - `tmp/rtk-main/scripts/validate-docs.sh`
- plugin packaging examples:
  - `tmp/rtk-main/openclaw/openclaw.plugin.json`
  - `tmp/rtk-main/openclaw/README.md`
- agent guidance examples:
  - `tmp/rtk-main/hooks/codex/README.md`
  - `tmp/rtk-main/hooks/codex/rtk-awareness.md`

OntoIndex note:

- `gn_ensure_fresh(repo=codex)` reported the index fresh at HEAD
  `5edde24a78efe0f10bc710936dfa228427ab7fd1`.
- The worktree is dirty and embeddings are missing, so this review uses
  OntoIndex as routing evidence and direct source inspection as authority.
- OntoIndex semantic search routed to the existing plugin/MCP owner surface,
  while direct source inspection confirmed that the lean-ctx native backend
  already has concrete `ctx_read`, `ctx_search`, `ctx_summary`, registry, and
  tool-visibility owners. Native proposals must land there, not in plugin
  scripts.

## 2000 Combination Coverage

The requested "2000 useful tools" review is covered as a cross-product review,
not as 2000 dispatch rows:

- 40 donor tool surfaces
- 10 concern areas
- 5 review lenses

`40 x 10 x 5 = 2000` reviewed combinations.

Donor tool surfaces:

1. command filter runner
2. streaming filter runner
3. passthrough runner
4. TOML filter DSL
5. inline filter tests
6. project-local filter trust
7. hook install lifecycle
8. hook integrity hash
9. hook verification command
10. hook permission precedence
11. command rewrite rules
12. command lexer
13. compound command splitting
14. rewrite guard rules
15. discover missed savings
16. token-savings tracking
17. gain analytics
18. install check script
19. install archive safety test
20. docs consistency validator
21. command support inventory
22. source filter caps
23. language detection
24. read filtering modes
25. grep result grouping
26. find result grouping
27. git status filtering
28. git diff filtering
29. cargo output filtering
30. test runner filtering
31. linter output filtering
32. package-manager filtering
33. cloud command filtering
34. system command filtering
35. agent awareness docs
36. OpenClaw plugin metadata
37. plugin config schema
38. measured savings table
39. security disclaimer/docs
40. release/install docs

Concern areas:

1. native `ctx_read`
2. native `ctx_search`
3. native `ctx_summary`
4. backend registry/tool visibility
5. backend health and capabilities
6. backend provenance/version metadata
7. auth/token handling
8. fail-closed behavior
9. plugin package boundary
10. update/provenance workflow

Review lenses:

1. self-contained
2. read-only
3. no hidden external dependency
4. no duplicate core owner
5. testable with current repo tools

## Verdict Counts

- KEEP-NATIVE-CANDIDATE: 9
- REJECTED/NON-NATIVE: 50

`KEEP-NATIVE-CANDIDATE` means useful only for future backend-owner review. It
is not an implementation task without fresh source/test evidence that the
maintained backend lacks the behavior.

## Keep-Native Candidate Tool Families

| ID | Native Candidate | Donor Evidence | Target Owner | Useful Action |
| --- | --- | --- | --- | --- |
| RTK-LCTX-N01 | bounded `ctx_read` compression guard | RTK source filter caps and read filtering modes | `third_party/lean-ctx-fork/rust/src/tools/ctx_read/` | prove whether current read modes ever inflate or overrun bounds, then add backend-local test/fix only if a real gap exists |
| RTK-LCTX-N02 | native read line/window cap proof | RTK source filter caps | `ctx_read` mode/render tests | keep line-window output bounded in backend tests, not via plugin wrapper checks |
| RTK-LCTX-N03 | native `ctx_search` grouped output | RTK grep/find grouping | `third_party/lean-ctx-fork/rust/src/tools/ctx_search.rs` | improve grouping/truncation only inside existing search output if current tests show noisy or unstable results |
| RTK-LCTX-N04 | native search deadline/partial-result proof | RTK streaming filter runner | `ctx_search` tests | verify timeout/partial-result behavior remains bounded without adding command proxy plumbing |
| RTK-LCTX-N05 | native `ctx_summary` bounded recall/list output | RTK compact summaries | `third_party/lean-ctx-fork/rust/src/tools/ctx_summary.rs` | add clamp/truncation proof if summary output can exceed the plugin contract |
| RTK-LCTX-N06 | Ontocode build allowlist profile | RTK rewrite guard rules | backend registry/tool-visibility owner | expose only `ctx_read`, `ctx_search`, and `ctx_summary` for the Ontocode-maintained backend mode, without adding shell/edit/session/knowledge tools |
| RTK-LCTX-N07 | backend capability/health response | RTK install verification | backend server/registry owner | report native allowed tools and backend version from the backend itself; plugin scripts must not become the authority |
| RTK-LCTX-N08 | backend provenance/version metadata | RTK release/install docs | backend version/provenance owner | keep local fork provenance and version visible without update checks, telemetry, or external release dependency |
| RTK-LCTX-N09 | native claim guard for savings numbers | RTK measured savings table | backend read/search tests and docs near owner | avoid numeric savings claims unless backend tests or fixtures prove the exact claim |

## Rejected And Non-Native RTK Surfaces

| Surface | Verdict | Reason |
| --- | --- | --- |
| Plugin preflight script | NON-NATIVE | Operational wrapper; does not improve `ctx_read`, `ctx_search`, or `ctx_summary` backend behavior. |
| Backend source presence check in plugin | NON-NATIVE | Repository packaging concern, not native lean-ctx capability. |
| Redacted token check in plugin | NON-NATIVE | Useful hygiene, but not a native lean-ctx proposal for this artifact. |
| Endpoint health probe in plugin script | NON-NATIVE | Health must be a backend capability if implemented. |
| MCP allowlist verifier script | NON-NATIVE | Allowlist should be enforced by plugin config and backend visibility/profile owner, not a separate script. |
| Required-server verifier script | NON-NATIVE | Plugin packaging proof already belongs to core plugin tests, not lean-ctx native backend work. |
| README contract smoke | NON-NATIVE | Documentation automation, not native backend behavior. |
| Docs consistency check | NON-NATIVE | Documentation automation, not native backend behavior. |
| Default prompt cap check | NON-NATIVE | Plugin metadata hygiene; no backend capability. |
| Agent awareness skill | NON-NATIVE | Guidance surface, not backend adoption. |
| Fail-closed runbook | NON-NATIVE | Documentation, not a native backend tool. |
| Backend startup runbook | NON-NATIVE | Documentation, not a native backend tool. |
| Backend update checklist | NON-NATIVE | Process hygiene; keep outside this native-only catalog. |
| Provenance note in memory-bank | NON-NATIVE | Process/historical record; not backend functionality. |
| License inventory check | NON-NATIVE | Packaging/legal hygiene, not lean-ctx native feature work. |
| No hidden download check script | NON-NATIVE | Valid third-party rule enforcement, but not native backend capability. |
| Archive/path traversal package lesson | NON-NATIVE | Future package installer concern, not current backend owner. |
| Plugin package validator wrapper | NON-NATIVE | Existing plugin validator owns this generally. |
| Backend build smoke as plugin task | NON-NATIVE | Build validation can be used when editing backend, but is not a donor tool proposal. |
| Live backend smoke as plugin task | NON-NATIVE | Runtime validation only; no native feature. |
| Allowlist diff report script | NON-NATIVE | Wrapper/reporting task; enforce in backend/plugin owners instead. |
| Unsupported tool explanation | NON-NATIVE | Documentation only. |
| Bounded usage examples | NON-NATIVE | Documentation only. |
| Fallback examples | NON-NATIVE | Documentation only. |
| Health output redaction in plugin script | NON-NATIVE | Redaction belongs in backend diagnostics if implemented. |
| CI-safe preflight mode | NON-NATIVE | Wrapper validation. |
| Live-only preflight mode | NON-NATIVE | Wrapper validation. |
| JSON preflight output | NON-NATIVE | Wrapper API. |
| Human preflight output | NON-NATIVE | Wrapper UI. |
| Stale backend warning script | NON-NATIVE | Backend version/capability endpoint is the native owner. |
| Config schema note | NON-NATIVE | Documentation only. |
| Local-only policy check script | NON-NATIVE | Third-party rule belongs in review/CI policy, not this backend catalog. |
| Backend ownership check in memory-bank | NON-NATIVE | Process state, not backend implementation. |
| No telemetry default in plugin docs | NON-NATIVE | Documentation; native keep item is backend-side no-telemetry/version behavior only. |
| No analytics DB note | NON-NATIVE | Documentation; analytics DB remains rejected below. |
| No hook install note | NON-NATIVE | Documentation; hook runtime remains rejected below. |
| No permission parser clone note | NON-NATIVE | Documentation; permission parser clone remains rejected below. |
| No TOML DSL clone note | NON-NATIVE | Documentation; TOML runtime remains rejected below. |
| Plugin-bundle integrity future | NON-NATIVE | Core plugin-manager concern, not lean-ctx native backend. |
| Install/load proof extension | NON-NATIVE | Core plugin-manager test surface, not lean-ctx native backend. |
| Shell command rewrite hook | REJECTED | Violates no `ctx_shell` and no plugin-owned shell behavior. |
| `rtk rewrite` clone | REJECTED | Creates a second command parser/rewrite owner. |
| TOML command filter DSL | REJECTED | Useful for RTK, but a parallel plugin filtering runtime here. |
| SQLite token tracking | REJECTED | Adds analytics/persistence not needed for plugin contract. |
| Token savings dashboard | REJECTED | Requires telemetry/claims beyond current proof. |
| Auto-patching agent settings | REJECTED | Plugin install should not mutate global agent config. |
| External install script/curl path | REJECTED | Conflicts with self-contained third-party migration rule. |
| Broad command filter catalog | REJECTED | Belongs to shell/output owner, not read-only lean-ctx plugin. |
| Permission parser clone | REJECTED | Existing hook/shell owners already own permission semantics. |
| OpenClaw-style exec plugin | REJECTED | Different product surface and shell execution model. |

## Recommended Next Slice

No implementation dispatch from this artifact yet.

Exact reopen gate:

Reopen one bounded backend-native task only after direct source/test evidence
proves a concrete gap in `third_party/lean-ctx-fork/rust/src/tools/ctx_read/`,
`third_party/lean-ctx-fork/rust/src/tools/ctx_search.rs`,
`third_party/lean-ctx-fork/rust/src/tools/ctx_summary.rs`, or the backend
registry/tool-visibility owner for the Ontocode allowlist.

First eligible task after that evidence:

`RTK-LCTX-N1`: inspect the backend owner for the proven gap and add one focused
backend-native test/fix for either bounded `ctx_read`, grouped/bounded
`ctx_search`, bounded `ctx_summary`, or backend-native allowlist/capability
reporting. Do not add plugin scripts, docs automation, shell hooks, telemetry,
or external install checks.

Expected touch files depend on the proven gap and must stay in native backend
owners, for example:

- `third_party/lean-ctx-fork/rust/src/tools/ctx_read/*`
- `third_party/lean-ctx-fork/rust/src/tools/ctx_search.rs`
- `third_party/lean-ctx-fork/rust/src/tools/ctx_summary.rs`
- `third_party/lean-ctx-fork/rust/src/server/tool_visibility.rs`
- backend-local tests beside those owners

Validation:

- OntoIndex impact/context for the specific backend symbol before edits.
- Backend-local Rust test command for the changed lean-ctx crate/package.
- file-scoped `git diff --check`

## Stop Rule

Do not dispatch any RTK-derived shell, hook, command-rewrite, telemetry,
analytics, TOML-filter runtime, plugin preflight script, docs validator, or
manifest wrapper into `plugins/ontocode-lean-ctx`. Those are valid RTK features
or hygiene ideas, but they are not native lean-ctx backend proposals.
