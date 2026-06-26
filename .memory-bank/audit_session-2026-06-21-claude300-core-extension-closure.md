# Claude Code Donor 300 Core Extension Closure

Date: 2026-06-21
Status: closed

## Scope

Closed all five accepted owner-local regression bundles from
`ADR_CLAUDE_CODE_DONOR_300_CORE_EXTENSION_SOLUTIONS.md` and
`CLAUDE_CODE_DONOR_300_CORE_EXTENSION_TRACKING.md`. This is not a full Claude
Code donor feature-parity claim.

## Outcome

- `CLAUDE300-R1`: multi-agent and agent-job regressions closed.
- `CLAUDE300-R2`: shell and PowerShell policy/parser regressions closed; broad
  `ontocode-core` failures were verified unrelated to the shell bundle.
- `CLAUDE300-R3`: bounded context/file/search/LSP/attachment/compaction
  regressions closed; any model-visible context behavior remains valid only
  while bounded, capped, and inside approved context owners.
- `CLAUDE300-R4`: MCP/resource/auth/tool-discovery/skills/plugins/config
  regressions closed after manager verification; the first worker report was a
  process failure and not accepted as closure evidence.
- `CLAUDE300-R5`: diagnostics/review/web/pacing/plan-mode/model/auth
  regressions closed; release/support-bundle claims still require content
  review for secret-like output.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-tools tool_search`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core-plugins remote_installed_cache_refresh_invalidates_stale_connectors`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core mcp_and_tool_search_follow_direct_and_deferred_tool_exposure deferred_extension_tools_are_discoverable_with_tool_search v1_multi_agent_tools_defer_when_tool_search_available`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-feedback feedback_diagnostics::tests::collect_from_pairs_redacts_oauth_keys_and_cookies`
- Worker-reported scoped passes for R1, R2, R3, and R5 are recorded in the tracking ledger.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `git diff --check`
- OntoIndex freshness check: indexed HEAD matches current HEAD
  `8b61fd0dbfa32aa9e4a00ef15930e5b4fcb9119f`; dirty worktree leaves medium
  scope confidence.
- OntoIndex `gn_verify_diff` was run and failed because no expected file list
  was supplied against the existing broad dirty worktree; use the tracking ledger
  and scoped tests above as closure authority for this bundle.

## Remaining

No remaining dispatch tasks from the Claude Code donor 300 ADR. Reopen only with
a concrete owner-local failing regression. Dirty worktree cleanup and snapshot
review remain merge hygiene, not Claude300 dispatch work.
