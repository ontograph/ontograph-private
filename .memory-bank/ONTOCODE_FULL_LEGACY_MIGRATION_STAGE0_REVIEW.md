# Ontocode Full Legacy Migration Stage 0 Review

Date: 2026-06-14
Status: no-go-for-implementation

## Inputs Reviewed

- `ONTOCODE_FULL_LEGACY_MIGRATION_PROJECT_PLAN.md`
- `ONTOCODE_FULL_LEGACY_MIGRATION_TRACKING.md`
- `ONTOCODE_FULL_LEGACY_MIGRATION_MATRIX_LAYOUT.md`
- `ONTOCODE_FULL_LEGACY_MIGRATION_MATRIX_CLI_HELPERS.md`
- `ONTOCODE_FULL_LEGACY_MIGRATION_MATRIX_PACKAGES_STATE_PROTOCOL.md`
- `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`
- `ONTOCODE_PACKAGE_IDENTITY_MIGRATION.md`
- `ONTOCODE_REMAINING_SURFACES_DISPOSITION.md`

## Manager Verdict

Stage 0 inventory is accepted.

Implementation dispatch for F1-F7 is blocked until the blockers below are cleared. This is a senior no-go, not a worker failure.

## Accepted Stage 0 Outputs

| Task | File | Verdict | Notes |
| --- | --- | --- | --- |
| F0-A | `ONTOCODE_FULL_LEGACY_MIGRATION_MATRIX_LAYOUT.md` | accepted | Covers workspace path, Cargo/Bazel/build graph, CI, package scripts, schema roots, snapshots, SDK path references, release packaging, V8/native inputs, and remote-test scripts. |
| F0-B | `ONTOCODE_FULL_LEGACY_MIGRATION_MATRIX_CLI_HELPERS.md` | accepted | Covers public CLI commands, command-name override, completions/help/resume/doctor/update text, arg0 dispatch, helper binaries, sandbox argv0, exec-server paths, app-server/TUI consumers, npm launcher/bin map, and scripts/examples. |
| F0-C | `ONTOCODE_FULL_LEGACY_MIGRATION_MATRIX_PACKAGES_STATE_PROTOCOL.md` | accepted | Covers npm/Python/SDK/native package identities, runtime carrier/import paths, generated protocol models, schema bundle names, `CODEX_*` env/state paths, telemetry, and MCP/wire IDs. |

## OntoIndex Evidence

- `ontoindex analyze --skills --skip-agents-md` was run after the manager tracking update.
- `ontoindex analyze --skills --skip-agents-md` was run after F0-A acceptance.
- `ontoindex analyze --skills --skip-agents-md` was run after F0-B acceptance.
- `ontoindex analyze --skills --skip-agents-md` was run after F0-C acceptance.
- `gn_help` after refresh reported facade tools available and recommended exact-symbol impact before implementation.

Index limitations:

- Embeddings remain absent.
- The worktree is dirty, so future implementation evidence must be scoped carefully and exact-symbol impact must be rerun immediately before edits.

## Blocking Issues

### B1: Dirty Worktree Makes F1 Unsafe

The repository currently has broad unrelated modified and untracked files, including Rust source, generated snapshots, memory-bank files, provider work, protocol files, and package metadata.

F1 requires moving `ontocode-rs/` to `ontocode-rs/`. Doing that in the current worktree would move unrelated user/worker changes and make rollback, review, and blame unreliable.

Decision:

- Do not dispatch F1 implementation while the dirty worktree remains unclassified.
- First create a clean branch or checkpoint for this migration, or explicitly classify/stage the existing work so path-only movement can be reviewed.

### B2: F4-F6 Need Release/Versioning Owners

Package names, env/state compatibility, generated protocol names, and telemetry schemas are external contracts.

Decision:

- Do not dispatch F4, F5, or F6 coding tasks without named owners for:
  - release/package migration
  - state/env migration
  - protocol/schema versioning
  - analytics/telemetry schema migration

### B3: Sub-Agent Model Choice Cannot Be Verified By Tooling

All workers were instructed to use only `gpt-5.4-mini` or `gpt-5.3-codex-spark`, but the sub-agent tool did not expose a model selector or verification field.

Decision:

- Record the limitation.
- Do not treat this as a worker-output failure because each worker explicitly reported the limitation.

## Stage Readiness

| Task | Readiness | Manager Decision |
| --- | --- | --- |
| F0 | ready to close | Stage 0 accepted after matrices, tracking, and index refreshes. |
| F1 | blocked | Needs clean/checkpointed worktree and path-only implementation branch. |
| F2 | blocked | Depends on F1 or explicit decision to cut over CLI before layout rename. Also needs exact-symbol OntoIndex impact. |
| F3 | blocked | Depends on F2 and helper runtime path impact review. |
| F4 | blocked | Needs release owner and package dual-publish/metapackage decision. |
| F5 | blocked | Needs state/env migration owner and no-data-loss plan. |
| F6 | blocked | Needs protocol/telemetry versioning owners. |
| F7 | blocked | Depends on release adoption evidence from F1-F6. |

## Required Unblock Actions

1. Create a clean migration checkpoint or branch where unrelated modified/untracked work is either committed, shelved, or explicitly accepted as part of the migration baseline.
2. Re-run `ontoindex analyze --skills --skip-agents-md` after the checkpoint.
3. Run exact OntoIndex impact for F1 owner files before moving paths:
   - root `justfile`
   - `MODULE.bazel`
   - `scripts/codex_package/cargo.py`
   - root `package.json`
   - path-sensitive CI/script/test owners from `ONTOCODE_FULL_LEGACY_MIGRATION_MATRIX_LAYOUT.md`
4. Dispatch F1 only as a path/layout-only implementation slice.
5. Keep F2-F7 blocked until their prerequisites are explicitly satisfied in tracking.

## External Compilation Command For Current Verified Binary

Until F1 changes the workspace path, external compilation remains:

```bash
cd /opt/demodb/_workfolder/ontocode
CARGO_BUILD_JOBS=8 cargo build --manifest-path ontocode-rs/Cargo.toml -p ontocode-cli --bin ontocode
```

Current debug artifact:

```text
/opt/demodb/_workfolder/ontocode/ontocode-rs/target/debug/ontocode
```

