# R5AB Realtime WebRTC Rename Risk Review

Date: 2026-06-11

## Decision

- Dispatch `codex-realtime-webrtc` -> `ontocode-realtime-webrtc`.
- Dispatch `codex_realtime_webrtc` -> `ontocode_realtime_webrtc`.
- Scope is identity-only package/lib/Bazel/import/internal diagnostic thread-name rename.

## Direct Inventory

- Cargo metadata direct reverse dependency: `ontocode-tui`.
- Active direct refs before dispatch: 12.
- Refs are confined to root workspace metadata, realtime-webrtc manifest/Bazel identity, native worker/thread diagnostic names, and TUI dependency/import usage.

## OntoIndex Impact

- MCP impact is miswired to repo `OntoIndex`; CLI fallback was used.
- `Struct:ontocode-rs/realtime-webrtc/src/lib.rs:RealtimeWebrtcSession`: LOW, 0 impacted nodes, 0 direct, 0 affected processes.
- `Struct:ontocode-rs/realtime-webrtc/src/lib.rs:RealtimeWebrtcSessionHandle`: LOW, 1 impacted node, 1 direct, 0 affected processes.
- `Enum:ontocode-rs/realtime-webrtc/src/lib.rs:RealtimeWebrtcEvent`: LOW, 0 impacted nodes, 0 direct, 0 affected processes.
- `Function:ontocode-rs/realtime-webrtc/src/native.rs:start`: LOW, 0 impacted nodes, 0 direct, 0 affected processes.
- `Function:ontocode-rs/realtime-webrtc/src/native.rs:worker_main`: LOW, 1 impacted node, 1 direct, 0 affected processes.

## Guardrails

- Preserve realtime WebRTC start/apply-answer/close/local-audio-level behavior.
- Preserve unsupported-platform behavior.
- Preserve macOS `libwebrtc` feature/dependency behavior.
- Preserve peer connection offer/answer behavior.
- Preserve TUI realtime event handling.
- Preserve telemetry, env/config/wire/generated names, persisted state, and the existing `realtime-webrtc` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-realtime-webrtc --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_realtime_webrtc|codex-realtime-webrtc`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Model

- Use `gpt-5.4-mini` high because `gpt-5.3-codex-spark` reached usage limit.
