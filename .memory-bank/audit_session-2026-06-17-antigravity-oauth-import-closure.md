---
name: Antigravity OAuth Import Closure
description: Verification closure for the ADR-bounded Antigravity OAuth import slice
type: audit_session
date: 2026-06-17
status: accepted
---

# Antigravity OAuth Import Closure

Authority:

- [ADR_ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB.md](ADR_ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB.md)
- [ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB_TRACKING.md](ANTIGRAVITY_OAUTH_CLIPROXY_PROVIDER_HUB_TRACKING.md)

## Accepted Scope

- Added `antigravity` OAuth import parsing through the existing Gemini/Google-style import owner.
- Required `refresh_token` for Antigravity imports while leaving existing Gemini non-refreshable behavior unchanged.
- Stored imported Antigravity credentials through existing provider OAuth auth storage.
- Added `/login antigravity --import <path>` and provider auth/status visibility.
- Kept runtime and `/model` execution disabled; `/status auth antigravity` reports the runtime block.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-migration` passed: 46 passed, 1 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui antigravity` passed: 4 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui gemini_import` passed: 7 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui slash_status_auth` passed: 3 passed.
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-external-agent-migration` completed.
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-tui` completed and applied two local fixes in slash dispatch.
- `CARGO_BUILD_JOBS=8 just fmt` completed after fixes.

## Residuals

- Broad `CARGO_BUILD_JOBS=8 just test -p ontocode-tui slash_` compiled and ran but failed six unrelated existing insta snapshot tests that emitted `.snap.new` files with snapshot-name drift; generated `.snap.new` files were removed.
- Native Antigravity runtime, refresh adapter, schema cleanup, and selectable models remain blocked by a future runtime ADR.
