# R5BJ Execpolicy Rename Risk Review

Date: 2026-06-12

## Slice

- Rename `ontocode-execpolicy` -> `ontocode-execpolicy`.
- Rename Rust crate refs `codex_execpolicy` -> `ontocode_execpolicy`.
- Identity-only scope: package metadata, library crate name, binary/package identity where internal, Bazel target/deps, imports, and lockfiles.

## OntoIndex

- MCP `mcp__ontoindex` is still not wired to `/opt/demodb/_workfolder/ontocode`; use local OntoIndex CLI for this repo.
- `PrefixRule`: CRITICAL, 18 impacted nodes, 3 direct, 5 modules, no affected processes.
- `blocking_append_allow_prefix_rule`: HIGH, 7 impacted nodes, 5 direct, 3 modules, no affected processes.
- `blocking_append_network_rule`: LOW, 3 impacted nodes, 3 direct, 2 modules.
- `NetworkRule`: LOW, 3 impacted nodes, 2 direct, 2 modules.
- `Policy`, `PolicyParser`, `ExecPolicyCheckCommand`, and `Decision`: ambiguous/UNKNOWN.

## Guardrails

- Do not change policy parsing, prefix/network rule semantics, example validation, host executable lookup, policy merge/check behavior, amendment persistence, JSON output shape, CLI argument behavior, core exec-policy integration, config requirements-policy conversion, prompt permission text, env/config/wire/generated names, telemetry/product strings, persisted state, or the existing `execpolicy` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-execpolicy --no-tests=pass`
- Focused `ontocode-core` exec-policy/config requirements-policy checks.
- Compile-only or focused `ontocode-config` requirements-policy checks.
- Focused `codex-protocol` allow-prefix formatting checks while protocol crate remains blocked.
- Focused `ontocode-prompts` permission-instruction checks.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_execpolicy|ontocode-execpolicy`.
- Cargo metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
