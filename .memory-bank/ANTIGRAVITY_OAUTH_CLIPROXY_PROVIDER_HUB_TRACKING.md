---
name: Antigravity OAuth CLIProxyAPI Provider Hub Tracking
description: Dispatch and verification ledger for the Antigravity OAuth import slice
type: tracking
date: 2026-06-17
status: completed
---

# Antigravity OAuth CLIProxyAPI Provider Hub Tracking

Authority: [ADR_ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB.md](ADR_ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB.md)

## Manager Rules

- Scope is import, redacted status/readiness, and disabled runtime state only.
- Do not add browser OAuth or native runtime execution.
- Do not add CLIProxyAPI process management, model translation, or app-server APIs.
- Reuse existing provider OAuth storage and slash auth owners.
- Run OntoIndex before edits and refresh the index after each accepted task.
- No raw credential values, token files, keychain paths, or private import paths in logs, tests, status, or this memory file.
- Sub-agent tooling is available for follow-up manager packets; use only `gpt-5.3-codex-spark` or `gpt-5.4-mini`.

## Dispatch Queue

| Task | Status | Owner | Scope | Verification |
| --- | --- | --- | --- | --- |
| A0 | completed | senior-manager | Create tracking ledger and link it from memory index. | Memory link present; OntoIndex refreshed; index is commit-fresh with dirty-tree warning. |
| A1 | completed | worker-gpt-5.4-mini | Add redacted Antigravity OAuth parser fixture and missing-refresh rejection coverage. | `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-migration` passed: 46 passed, 1 skipped. |
| A2 | completed | worker-gpt-5.4-mini | Project imported Antigravity OAuth into existing provider OAuth credential storage. | `CARGO_BUILD_JOBS=8 just test -p ontocode-tui antigravity` passed: 4 passed. |
| A3 | completed | worker-gpt-5.4-mini | Add `/login antigravity --import <path>` plus `/auth`, `/logout`, and `/status` provider rows through existing slash auth flow. | `gemini_import` filter passed 7/7; `slash_status_auth` filter passed 3/3. |
| A4 | completed_skipped | senior-manager | Decide whether disabled `/model` visibility is useful before native runtime exists. | Skipped by design: `/status auth antigravity` gives blocked runtime visibility without exposing fake models. |
| A5 | completed | senior-manager | Convert Antigravity blocker notes into a narrow loopback-OAuth + Cloud Code Assist discovery slice and explicit stop conditions. | Docs-only completion; `/memory-bank` ADR/tracking files updated with exact pre-runtime gates and required tests. |
| A6 | blocked_by_runtime_approval | future | Native Antigravity runtime, refresh adapter, schema cleanup, and model execution. | Local non-executing contract tests are complete through A17; live runtime still requires an accepted runtime ADR, endpoint compatibility proof, and a real model execution adapter. |
| A7 | completed | worker-a7 | Draft the minimal native Antigravity runtime ADR/test contract from donor evidence and current model-provider owners. | Runtime ADR drafted in `ADR_ANTIGRAVITY_RUNTIME_CONTRACT.md`; runtime still disabled until a later implementation satisfies the contract and tests. |
| A8 | completed | worker-a8 | Add donor-shaped Cloud Code Assist request contract fixture/test without live network calls. | Added a redacted external-agent-migration fixture assertion covering `loadCodeAssist` / `onboardUser` metadata and `ANTIGRAVITY` identity; no runtime calls. |
| A9 | completed_superseded | worker-a9 | Add or document bounded Antigravity refresh dedupe test seam using existing auth/runtime owners. | Superseded by A16/A17: private `model-provider` contract owner now covers redacted refresh dedupe identity and partitioning without enabling runtime. |
| A10 | completed_superseded | worker-a10 | Add or document Antigravity tool/request schema compatibility fixture. | Superseded by A16/A17: private `model-provider` contract owner now covers the donor-shaped Cloud Code Assist request contract and token exclusion without enabling runtime. |
| A11 | completed | worker-a11 | Add mismatch guard coverage proving Antigravity runtime remains disabled on provider/model mismatch. | Added `antigravity_provider_hint_mismatch_keeps_route_blocked`; `just test -p ontocode-model-provider` passed. |
| A12 | completed | worker-a12 | Define the minimal native runtime-auth owner seam needed before A9/A10 can land. | Documented the seam in `ADR_ANTIGRAVITY_RUNTIME_CONTRACT.md`: `ModelProvider::api_auth()` + `runtime_base_url()` remain the native runtime-auth/request owner boundary inside `model-provider`, with private `auth_manager_for_provider()` / `resolve_provider_auth_with_manager()` as the only auth helpers. No runtime code or second auth stack was added. |
| A13 | completed | worker-a13 | Add the smallest `model-provider` auth regression around `api_auth()` sourcing provider OAuth credentials. | Added Antigravity `api_auth()` regression proving existing `AuthManager` provider OAuth path supplies bearer/account headers and keeps debug redacted; `just test -p ontocode-model-provider` passed. |
| A14 | completed | worker-a14 | Add the smallest `model-provider` request-contract test around `api_auth()` + `runtime_base_url()` as the future Antigravity request boundary. | Extended the Antigravity provider OAuth test with `runtime_base_url()` assertion; `just test -p ontocode-model-provider` passed. |
| A15 | completed_superseded | worker-a15 | Decide whether a private, non-executing `model-provider` Antigravity runtime contract module/test can land now. | Superseded by A16: the private non-executing `model-provider` request/refresh owner now exists under test scope only. |
| A16 | completed | worker-a16 | Implement the smallest private `model-provider` Antigravity request/refresh owner used by tests only. | Added `ontocode-rs/model-provider/src/antigravity_runtime.rs` plus private test coverage for the donor-shaped Cloud Code Assist request contract and the redacted refresh dedupe key; `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider` passed. |
| A17 | completed | senior-manager | Harden the private request/refresh owner contract with token-exclusion and refresh-key partition coverage. | Added request token-exclusion coverage plus refresh key partition assertions; `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider` passed: 81/81. |
| A18 | completed | worker-a18 | Convert A6 into an approval-ready runtime implementation packet without enabling runtime. | Added the A6 approval packet with concrete gates, implementation slices, required tests, stop conditions, and exact next-owner files; A6 stays blocked until an accepted runtime ADR and local endpoint compatibility proof exist. |
| A19 | completed | senior-manager | Create the smallest reviewable A6 runtime approval ADR draft. | Added `ADR_ANTIGRAVITY_NATIVE_RUNTIME_APPROVAL.md` and linked it from `MEMORY.md`; runtime remains blocked until endpoint proof exists. |
| A20 | blocked_on_external_evidence | senior-manager | Define the exact endpoint compatibility proof needed to unblock A6. | Added `ANTIGRAVITY_ENDPOINT_COMPATIBILITY_PROOF_RUNBOOK.md`; local search found donor references and contract tests, but no checked-in endpoint compatibility proof. |
| A21 | blocked_external_evidence | worker-a21 | Inspect local donor material for a redacted Antigravity endpoint compatibility fixture that can satisfy A20. | Local search found donor request-contract tests and docs, but no checked-in redacted endpoint request/response fixture for the intended Antigravity runtime target. |
| A22 | completed | senior-manager | Capture the user-supplied Antigravity OAuth record shape without storing secrets. | Added `ANTIGRAVITY_REDACTED_OAUTH_RECORD_FIXTURE.md`; credential shape is documented, but endpoint request/response proof is still missing. |

## Current Notes

- CLIProxyAPI remains a donor format and optional manually configured custom OpenAI-compatible endpoint, not a built-in provider hub.
- Provider id is `antigravity`.
- Runtime must remain unavailable after this slice.
- Discovery contract is now explicit: `Cloud Code Assist project discovery` and `onboardUser` metadata behavior are documented-only until runtime ADR approval.
- The runtime contract has now been drafted in
  [ADR_ANTIGRAVITY_RUNTIME_CONTRACT.md](ADR_ANTIGRAVITY_RUNTIME_CONTRACT.md),
  but it still only authorizes a later implementation path; no runtime code is
  enabled here.
- A7 was dispatched after OntoIndex identified the runtime touchpoints:
  `model-provider`, `models-manager`, TUI provider catalog disable logic, and
  existing Antigravity import/status tests. This is an ADR/test-contract task,
  not runtime enablement.
- A8-A11 dispatched from `ADR_ANTIGRAVITY_RUNTIME_CONTRACT.md` after OntoIndex
  found existing Antigravity import fixtures, TUI blocked-catalog tests, and
  model-provider alias mismatch tests. These tasks must add tests/fixtures or
  explicit blocked documentation only; runtime execution stays disabled.
- A8 is now complete: `ontocode-rs/external-agent-migration/src/gemini_oauth_import_tests.rs`
  carries the donor-shaped Cloud Code Assist request contract fixture assertion
  for `loadCodeAssist` / `onboardUser` metadata and `ANTIGRAVITY` identity.
- A11 is complete: model-provider route coverage now proves an Antigravity
  provider hint mismatch fails closed instead of enabling fallback routing.
- A12 dispatched because A9/A10 both found the same root blocker: there is no
  native Antigravity runtime-auth/request owner seam yet. This is an ADR/design
  seam task first; do not add a second auth stack or enable runtime.
- A12 is now complete: the minimal native seam is the existing
  `model-provider` provider-auth/request boundary (`api_auth()` and
  `runtime_base_url()`) plus the private auth helper pair in
  `model-provider/src/auth.rs`. Future Antigravity runtime work must stay on
  that owner path.
- A13/A14 dispatched as the concrete retry tasks for A9/A10 after A12 clarified
  the owner seam. They must stay private to `model-provider` and must not enable
  Antigravity runtime.
- A13/A14 completed as model-provider tests only. They prove the existing
  provider OAuth auth path and runtime base-url seam are usable for a future
  Antigravity runtime without enabling model selection or network execution.
- A16 dispatched as the actual owner trigger. It may add only private,
  non-executing model-provider helpers used by tests; no model selection,
  network execution, public API, or provider registry.
- A16 is now complete: the private `model-provider` helper module exists only
  under `cfg(test)`, carries the donor-shaped Cloud Code Assist request
  contract, and keeps the refresh dedupe key limited to provider/credential/
  project metadata while redacting token material in debug output.
- A17 is complete: the private request/refresh owner now proves Cloud Code
  Assist request JSON excludes token material and refresh dedupe keys partition
  by provider, credential, and project while ignoring token rotation.
- A19 is complete: A6 now has a reviewable approval ADR draft. The only
  remaining blocker is evidence, not planning: a checked-in endpoint
  compatibility proof for the intended Antigravity runtime target.
- A20 is blocked on external evidence: the repo has request-contract tests and
  donor references, but no checked-in redacted endpoint request/response
  fixture that can approve live runtime execution.
- A22 is complete: the Antigravity OAuth record shape is now captured in a
  redacted fixture. This narrows the remaining A6 blocker to endpoint
  compatibility evidence only.
- Closure recorded in [audit_session-2026-06-17-antigravity-oauth-import-closure.md](audit_session-2026-06-17-antigravity-oauth-import-closure.md).
