# R2B-D Exec Server Rename Worker Verification

Date: 2026-06-10

## Scope

- Renamed Cargo package `ontocode-exec-server` to `ontocode-exec-server`.
- Renamed Rust library crate `codex_exec_server` to `ontocode_exec_server`.
- Updated workspace and dependent manifests, Rust imports/usages, Bazel crate name, Cargo lock entries, and Bazel lock data.
- Updated active README text only where it described the internal package/library identity.
- Preserved public binaries, remote/local exec protocol names, protobuf package/message names, `codex.exec_server.*` wire identifiers, environment URL semantics, runtime path behavior, local process behavior, sandboxed file-system behavior, remote relay behavior, app-server environment APIs, telemetry, env/config semantics, persisted state, and generated schema/protocol surfaces.

## OntoIndex

- Pre-edit impact for `normalize_exec_server_url` reported CRITICAL partial risk through environment selection and app-server environment request processors.
- The CRITICAL impact matched the approved R2B-D gate; no unapproved HIGH/CRITICAL impact was introduced.
- `ExecServerClient.exec` did not resolve by that qualified name in the current index during worker verification; the risk review recorded LOW impact and direct inventory plus the required dependent test matrix covered the call surface.

## Verification

- Passed: `cargo metadata --format-version 1 --no-deps`.
- Passed: `CARGO_BUILD_JOBS=8 just fmt`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-client`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-cli`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-core`.
- Passed with zero-test remediation: exact `CARGO_BUILD_JOBS=8 just test -p codex-core-api` compiled but exited 4 because the package has no tests; rerun with `--no-tests=pass` passed.
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-core-plugins`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-core-skills`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-mcp`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-rmcp-client`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`.
- Passed focused coverage through the required suites for exec-server environment URLs, local process execution, remote client behavior, relay behavior, sandboxed file-system operations, app-server environment APIs, command exec, and RMCP/MCP stdio paths.
- Passed: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- Passed: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Passed: `CARGO_BUILD_JOBS=8 just fix -p ontocode-exec-server`.
- Passed active stale-reference search for `ontocode-exec-server` and `codex_exec_server` except intentional runtime/test/logging strings listed below.
- Passed: `git diff --check`.
- Passed: scoped OntoIndex `gn_verify_diff`.

## Intentional Old-Name References

- `ontocode-rs/exec-server/tests/common/mod.rs` keeps the test dispatch label `ontocode-exec-server-tests`.
- `ontocode-rs/exec-server/src/remote.rs` keeps the default remote environment name `ontocode-exec-server`.
- `ontocode-rs/exec-server/src/server/transport.rs` keeps log text for `ontocode-exec-server` stdio and websocket listeners.
- `scripts/start-ontocode-exec.sh` keeps remote exec-server log and pid paths under `/tmp/ontocode-exec-server-*`.
- `codex.exec_server.*` wire/protobuf identifiers remain unchanged where present.
