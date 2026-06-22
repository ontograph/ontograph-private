# Claude OAuth Provider Refactor Tracking

Source ADR: `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR.md`

## Status Key

- `pending`
- `in_progress`
- `blocked`
- `done`
- `deferred`

## Task Queue

| ID | Task | Scope | Status | Notes |
| --- | --- | --- | --- | --- |
| T1 | Stage 0 evidence gate | Local Claude artifacts, repo auth paths, feasibility doc updates | done | Completed 2026-06-04. See `CLAUDE_OAUTH_EVIDENCE_GATE.md`; current machine has no importable Claude credential artifact, and live import remains blocked pending a real credential sample |
| T2 | Stage 1 minimal import spike | MCP OAuth storage reuse, one connector import path, targeted tests | blocked | Fresh audit on 2026-06-06 found no local Claude MCP connector OAuth credential source; T2 remains externally blocked on a real redacted sample |
| T2A | Fresh local Claude MCP credential rediscovery | Re-check likely local paths and environment surfaces for a real Claude MCP connector credential source, redact if found, and run the opt-in validator | blocked | Completed 2026-06-06; likely local paths/env/keychain tooling checked, no credential-like Claude MCP OAuth JSON source found |
| T2B | Parser/storage readiness and remaining-task verification | Verify parser/report/status boundary, MCP OAuth token helper fit, and targeted tests without claiming live Claude compatibility | done | Completed 2026-06-06; parser/status and token helper tests pass, no unblocked code tasks remain without a real redacted Claude MCP credential sample |
| T3 | Stage 2 foreign identity bridge | Claude-specific import boundary and non-importable outcome handling | done | Added an isolated parser/report/status boundary with complete, partial, non-importable, and empty outcomes; live schema integration still waits on T2 sample validation |
| T4 | Stage 3 generalization decision | ADR update on whether a broker is necessary | done | `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR.md` now rejects a new credential broker for the current effort and defers generalization until T2 or T3 produce contrary evidence |
| T5 | Stage 4 provider selector ADR | Separate ADR for capability-driven provider selection | done | Added `ADR_MODEL_PROVIDER_SELECTOR_REFACTOR.md`; kept explicitly non-blocking for the Claude MCP import path |

## Dispatch Log

| Step | Tracking Update | Action |
| --- | --- | --- |
| 1 | Created tracker and marked `T1` in progress | Ready for Stage 0 evidence-gate dispatch |
| 2 | Confirmed `T1` active for Stage 0 evidence collection on 2026-06-04 | Inspect local Claude artifacts, compare against `StoredOAuthTokens`, and produce feasibility verdict before any code work |
| 2 | Marked `T5` in progress before new dispatch | Spawning a separate worker to draft the provider-selector ADR without coupling it to the Claude import path |
| 3 | Closed `T1` with a blocked import verdict for the current machine | No Claude credential file, settings file, env token, or inspectable OS key store entry was present locally; only changelog evidence indicates likely storage shape |
| 4 | Marked `T2` and `T3` blocked before next dispatch | Evidence gate proved the current machine cannot support a real import spike or bridge design without a sanitized Claude credential sample |
| 5 | Marked `T5` done and `T4` in progress before next dispatch | Separate provider-selector ADR landed; dispatching the generalization decision based on the T1 blocker evidence |
| 6 | Marked `T4` done after decision synthesis | Main ADR now rejects broker work for the current effort, preserves the MCP OAuth store as the only approved first target, and records the concrete unblockers for reopening T2/T3 |
| 7 | Reopened `T2` for fixture-contract work | Senior unblock split live credential validation from internal prep; T2 can now define sanitized sample shape, parser acceptance rules, and test strategy without real secrets |
| 8 | Marked `T2` blocked on live validation and `T3` in progress | Fixture-driven parser and tests are now available, so the bridge can define explicit complete, partial, and non-importable outcomes without touching storage |
| 9 | Marked `T3` done | The foreign identity bridge now returns normalized importable MCP OAuth credentials plus structured rejection reasons and high-level import statuses |
| 10 | Marked `T2` in progress before manager dispatch | Dispatching remaining Stage 1 work to sub-agents with separate lanes for sample discovery, storage wiring, and verification |
| 11 | Kept `T2` in progress before storage-wiring preparation | Inspecting whether fixture parser output can be mapped toward existing MCP OAuth storage without adding cross-crate dependency risk or claiming live Claude compatibility |
| 12 | Kept `T2` in progress for live-sample discovery lane | Running a read-only search for a real Claude MCP connector OAuth credential artifact; no secrets may be printed or committed |
| 13 | Marked `T2` blocked after live-sample discovery lane | No usable Claude MCP connector OAuth credential sample was found locally; Stage 1 cannot complete live validation without an external redacted sample |
| 14 | Kept `T2` in progress for verification/status lane | Reviewing parser and bridge-status behavior against the ADR contract; code changes are limited to local parser tests and no storage/runtime wiring |
| 15 | Marked `T2` blocked after manager consolidation | Parser tests, storage helper tests, formatter, and scoped lint fixes passed; T2 cannot close until an external redacted Claude MCP connector credential sample proves the real schema and refresh behavior |
| 16 | Marked `T2` in progress for senior unblock | Creating a self-service live-sample collection and validation path so the external sample blocker has a concrete owner workflow and acceptance check |
| 17 | Marked `T2` blocked on sample execution only | Added `CLAUDE_OAUTH_LIVE_SAMPLE_RUNBOOK.md`, redacted debug output for parser reports, and an ignored validator test for `CLAUDE_OAUTH_REDACTED_SAMPLE`; synthetic validator run passed |
| 18 | Marked `T2` in progress for redaction-helper unblock | Adding an offline helper that converts credential-like JSON into a sanitized sample plus structural summary without requiring manual token editing |
| 19 | Marked `T2` blocked on real source file only | Added `scripts/redact_claude_oauth_sample.py`; helper preserves structural `client_id`, redacts token/account/workspace/email values, and its output passed the opt-in live validator |
| 20 | Confirmed `T2` remains blocked before broad-provider dispatch | Claude import cannot progress without a real credential source file; continuing with the separate provider-selector readiness track from `ADR_MODEL_PROVIDER_SELECTOR_REFACTOR.md` |
| 21 | Reopened `T2` for manager-dispatch blocker audit | Dispatching read-only lanes to re-check for a real Claude MCP credential sample, verify current parser/storage readiness, and confirm whether any unblocked implementation work remains |
| 22 | Marked `T2A` in progress before local rediscovery | Re-checking likely Claude local credential paths and environment surfaces; if a JSON credential source is found, redact to `/tmp` and run the ignored live-sample validator |
| 23 | Marked `T2A` and `T2` blocked after local rediscovery | No Claude MCP connector credential source file was found in the runbook paths, bounded home filename pass, environment variable names, or available keychain tooling; no redaction or validator run was possible |
| 24 | Marked `T2B` in progress before parser/storage verification | Reviewing the Claude import parser/status boundary and `StoredOAuthTokens` token-parts fit, then running targeted import/OAuth tests |
| 25 | Marked `T2B` done after parser/storage verification | `codex-external-agent-migration` and `codex-rmcp-client` tests passed; top-level T2 remains blocked on a real redacted Claude MCP credential source |
