---
name: Kimi OAuth CLIProxyAPI Import And Device Flow Tracking
description: Dispatch and verification ledger for the Kimi OAuth import and gated device-flow slice
type: tracking
date: 2026-06-17
status: completed_to_adr_gates
---

# Kimi OAuth CLIProxyAPI Import And Device Flow Tracking

Authority: [ADR_KIMI_OAUTH_CLIPROXY_IMPORT_AND_DEVICE_FLOW.md](ADR_KIMI_OAUTH_CLIPROXY_IMPORT_AND_DEVICE_FLOW.md)

## Manager Rules

- Scope is import, redacted status/readiness, and disabled runtime state only.
- Do not add device flow, Kimi runtime execution, model catalogs, `/model` entries, app-server OAuth APIs, CLIProxyAPI process management, or a new credential store.
- Reuse existing provider OAuth storage and slash auth owners.
- Run OntoIndex before edits and refresh the index after each accepted task.
- No raw credential values, token files, keychain paths, or private import paths in logs, tests, status, or this memory file.

## Dispatch Queue

| Task | Status | Owner | Scope | Verification |
| --- | --- | --- | --- | --- |
| K0 | completed | senior-manager | Create tracking ledger and link it from memory index. | Memory link present; OntoIndex is commit-fresh with dirty-tree warning. |
| K1 | completed | worker-gpt-5.4-mini | Add redacted Kimi OAuth import fixture/parser coverage, including missing refresh rejection and malformed expiry rejection. | `just test -p ontocode-external-agent-migration` passed. |
| K2 | completed | worker-gpt-5.4-mini | Project imported Kimi OAuth into existing provider OAuth credential storage without a new metadata store. | `ontocode-login` storage compatibility test added; Kimi-imported provider OAuth credential round-trips through existing auth storage. |
| K3 | completed | worker-gpt-5.4-mini | Add `/login kimi --import <path>` plus `/auth`, `/logout`, and `/status` provider rows through existing slash auth flow. | `just test -p ontocode-tui slash_login_kimi_shows_import_options slash_login_kimi_import_stores_kimi_oauth_credential slash_auth_kimi_remove_removes_imported_kimi_credential slash_status_auth_shows_kimi_runtime_blocked slash_login_gemini_import_rejects_malformed_quotes` passed. |
| K4 | blocked_by_adr | future | Kimi device flow. | Blocked until donor-observed client id is explicitly approved. |
| K5 | blocked_by_adr | future | Native Kimi runtime and model visibility. | Requires separate runtime ADR and execution fixtures. |

## Next Approved Slice

If client-id approval is recorded, the next allowed code slice is device-flow
login only under the existing provider-auth/login/status owner.

Required tests before that slice can land:

- device authorization request fixture uses the approved client id plus donor
  headers
- polling coverage for pending, slow-down, denied, expired, and timeout
- redaction coverage for verification URLs, tokens, and raw import paths
- status coverage keeps Kimi runtime-blocked after import and after device-flow login

Stop conditions:

- do not add app-server OAuth callbacks, background management sessions, a new
  store/registry, `/model` entries, or runtime execution
- do not leak raw client ids, tokens, headers, keychain paths, or import paths
  into status/log/error output

## Current Notes

- Provider id is `kimi`.
- K1 is complete with focused parser and redaction coverage in `ontocode-external-agent-migration`.
- K2 is complete with a focused storage compatibility test in `ontocode-login`.
- K3 is complete with focused slash-command coverage for Kimi import, removal, and blocked status rows in `ontocode-tui`.
- The next code slice stays blocked by the client-id approval gate; when it opens, it must stay login-only and reuse the existing auth/login/status owner.
- OntoIndex refresh after K1: commit-fresh; dirty-tree degraded due existing uncommitted work.
- OntoIndex refresh after K2: commit-fresh; dirty-tree degraded due existing uncommitted work.
- OntoIndex impact for K3 target functions was LOW.
- OntoIndex refresh after K3: commit-fresh; dirty-tree degraded due existing uncommitted work.
- Allowed import/status work is complete; remaining rows are intentionally blocked by ADR.
- Runtime must remain unavailable after this slice.
- Device id may be parsed only if it fits the existing bounded credential shape; otherwise runtime stays blocked.
