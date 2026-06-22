# R5AB Realtime WebRTC Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-realtime-webrtc` -> `ontocode-realtime-webrtc`.
- Accepted `codex_realtime_webrtc` -> `ontocode_realtime_webrtc`.
- Scope was identity-only package/lib/Bazel/import/internal diagnostic thread-name rename.

## Preserved Surfaces

- Realtime WebRTC start/apply-answer/close/local-audio-level behavior.
- Unsupported-platform behavior.
- macOS `libwebrtc` feature/dependency behavior.
- Peer connection offer/answer behavior.
- TUI realtime event handling.
- Telemetry, env/config/wire/generated names, persisted state, and folder path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-realtime-webrtc --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_realtime_webrtc|codex-realtime-webrtc`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5AB.
- Active old refs are clean in `ontocode-rs`.
- Cargo metadata reports 45 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5AB-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
