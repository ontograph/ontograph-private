# Claude Code Approaches For Codebase Tracking

Source: `CLAUDE_CODE_APPROACHES_FOR_CODEBASE.md`

Manager rule: before starting any task, update this file with status, GitNexus context, reuse anchors, owner, test plan, and next action.

## Status Key

- `pending`: not started.
- `in_progress`: assigned or being implemented.
- `review`: agent finished; manager is reviewing/integrating.
- `blocked`: cannot proceed without missing evidence or conflicting changes.
- `done`: implemented and verified.

## Dispatch Queue

| Order | Epic | IDs | Status | Owner | Reuse Anchors | Verification |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Provider provenance/status/capability diagnostics | 2, 3, 4, 8, 9, 12, 13, 14 | done | manager + sub-agents | `create_model_provider`, `ProviderKind::for_provider`, `ProviderKind::create_provider`, provider descriptor/capability tests, doctor/status paths | GitNexus context/impact, scoped Rust tests |
| 2 | OAuth/auth-store validation and redacted diagnostics | 5, 6, 7, 11, 160 | done | manager + sub-agents | auth/login persistence, provider auth boundary, OAuth/MCP auth-store tests, shared redaction | GitNexus context/impact, scoped auth tests |
| 3 | MCP reliability and auth hardening | 141-146, 149-151, 155-157 | done | manager + sub-agents | `execute_mcp_tool_call`, `Session::call_tool`, `sanitize_mcp_tool_result_for_model`, MCP status snapshot, `RmcpClient` | GitNexus context/impact, scoped MCP tests |
| 4 | Hook and shell permission safety | 47-51, 53-56, 58-63, 161, 166-168, 172, 174, 175 | done | manager + sub-agents | permission request hooks, hook registry, `CoreShellActionProvider`, unified exec, shell escalation, sandbox tests | GitNexus context/impact, scoped hooks/shell tests |
| 5 | External adapter protocol safety | 16-30 | done | manager + sub-agents | provider construction/capability/status abstractions, stream event normalization, credential scope metadata, shared redaction | ADR fixture tests, provider runtime tests |
| 6 | Session/context bounded diagnostics | 1, 87, 89, 100, 104, 109-111, 117, 119-121, 124, 130, 132, 140, 181, 185 | done | manager + sub-agents | `Session::make_turn_context`, `new_turn_context_from_configuration`, context fragments, compaction/resume tests, TUI snapshots | context/session/TUI tests |
| 7 | External-agent import internals | 213-215, 217, 218, 220 | done | manager + sub-agents | `ExternalAgentConfigService::import`, request processor import, `claude_oauth_import`, migration tests, startup prompt flow | import tests, no-secret snapshots |
| 8 | Claude OAuth Import Wiring & Live Validation | Audit Gap | in_progress | manager + sub-agents | `ExternalAgentConfigService::import`, `parse_claude_oauth_import_sample`, `save_oauth_tokens` | App-server tests, live sample validation |

## Active Task: Claude OAuth Import Wiring & Live Validation

- Started: 2026-06-08
- Addressing gaps from `audit_session-2026-06-08-claude-oauth-adr-codebase-review.md`.
- GitNexus context required before code edits:
  - `ExternalAgentConfigService`
  - `ExternalAgentConfigMigrationItemType`
  - `save_oauth_tokens`
- Initial split:
  - Agent Generalist: Wire `parse_claude_oauth_import_sample` and `save_oauth_tokens` into `ExternalAgentConfigService::detect_migrations` and `import`. Fix all resulting compiler and protocol processor errors.
  - Agent Generalist (Next): Un-ignore and validate the live sample tests, implement `mcpServerStatus/list` validation.
- Current next action: Dispatch sub-agent to wire the runtime import.

## Log

- 2026-06-06: Created tracking file and marked provider provenance/status/capability diagnostics as `in_progress`.
- 2026-06-07: Verified and marked OAuth/auth-store validation and redacted diagnostics as `done`.
- 2026-06-07: Verified and marked MCP reliability and auth hardening as `done`.
- 2026-06-07: Verified and marked Hook and shell permission safety as `done`.
- 2026-06-07: Verified and marked External adapter protocol safety as `done`.
- 2026-06-07: Verified and marked Session/context bounded diagnostics as `done`.
- 2026-06-07: Verified and marked External-agent import internals as `done`.
