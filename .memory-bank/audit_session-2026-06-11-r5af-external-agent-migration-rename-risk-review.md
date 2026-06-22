# R5AF External-Agent Migration Rename Risk Review

Date: 2026-06-11

## Candidate

- `codex-external-agent-migration` -> `ontocode-external-agent-migration`
- `codex_external_agent_migration` -> `ontocode_external_agent_migration`

## Current Inventory

- Cargo metadata direct reverse dependency: `ontocode-app-server`.
- Active refs: 16.
- Ref scope: root workspace metadata, app-server dependency/import/call sites, and migration crate manifest/Bazel identity.

## OntoIndex CLI Fallback Impact

- `build_mcp_config_from_external`: LOW, 20 impacted nodes, 0 affected processes.
- `import_hooks`: LOW, 0 impacted nodes, 0 affected processes.
- `import_subagents`: LOW, 0 impacted nodes, 0 affected processes.
- `import_commands`: LOW, 0 impacted nodes, 0 affected processes.
- `parse_claude_oauth_import_sample`: MEDIUM, 31 impacted nodes, 13 direct, 0 affected processes.

## Guardrails

- Only package/lib/Bazel/import identity may change.
- Preserve MCP config conversion.
- Preserve hook/script migration, subagent import, command skill import, and AGENTS.md term rewriting.
- Preserve Claude OAuth import parsing, user-consent gating, token redaction, duplicate/rejection semantics, and debug redaction behavior.
- Preserve app-server external-agent config API behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and folder path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-migration --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server external_agent_config`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_external_agent_migration|codex-external-agent-migration`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Decision

- Approved as R5AF only because it is a bounded identity-only package/lib/import rename with one direct dependent.
- Security-sensitive OAuth diagnostics must keep existing redaction behavior.
- Work must run on fallback `gpt-5.4-mini` after Spark usage-limit fallback and record that fallback in output/tracking.
