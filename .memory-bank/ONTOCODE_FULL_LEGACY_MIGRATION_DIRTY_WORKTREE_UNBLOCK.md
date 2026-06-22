# Ontocode Full Legacy Migration Dirty Worktree Unblock

Date: 2026-06-14
Task: F0-E-A
Status: blocker-classified

## Scope

This note classifies why implementation of `.memory-bank/ONTOCODE_FULL_LEGACY_MIGRATION_PROJECT_PLAN.md`, especially F1 `ontocode-rs/` -> `ontocode-rs/`, must not proceed in the current worktree.

No code was edited for this classification. This report is the only intended file written by F0-E-A.

## OntoIndex Status

OntoIndex was checked with `gn_diagnose` only. `ontoindex analyze` was not run.

- Repo key/label: `codex`
- Repo path: `/opt/demodb/_workfolder/ontocode`
- Branch: `main`
- Worktree mode: dirty-worktree overlay
- Dirty worktree: true
- OntoIndex dirty file count: 2130
- OntoIndex source dirty counts: 1419 unstaged source files, 15 untracked source files, 0 staged source files
- Index freshness: reported stale in the top-level freshness check, with degraded confidence for freshness-sensitive work
- Embeddings: unavailable

Planning context is usable as advisory evidence, but F1 implementation still requires a clean or intentionally scoped worktree plus fresh OntoIndex impact on exact edit targets before any symbol edits.

## Current Git Dirty Counts

Observed with `git status --porcelain=v1` after this report file was created:

| Count | Value |
| --- | ---: |
| Total dirty paths | 2128 |
| Staged paths | 0 |
| Unstaged tracked paths | 1689 |
| Untracked paths | 439 |
| Modified paths | 1688 |
| Deleted paths | 1 |
| Added paths | 0 |
| Renamed paths | 0 |
| Copied paths | 0 |

Status-code summary:

| Status | Count |
| --- | ---: |
| ` M` | 1688 |
| `??` | 439 |
| ` D` | 1 |

The single tracked deletion is `ontocode-rs/cli/src/bin/ontocode.rs`, which directly overlaps public CLI behavior and makes F1 layout-only verification unsafe to interpret.

## Top Dirty Path Groups

Top path groups after this report file was created:

| Count | Path group |
| ---: | --- |
| 407 | `ontocode-rs/core` |
| 351 | `.memory-bank` |
| 254 | `ontocode-rs/tui` |
| 138 | `ontocode-rs/app-server` |
| 65 | `ontocode-rs/ext` |
| 62 | `ontocode-rs/utils` |
| 46 | `ontocode-rs/app-server-protocol` |
| 38 | `ontocode-rs/exec-server` |
| 38 | `ontocode-rs/cli` |
| 33 | `ontocode-rs/core-plugins` |
| 33 | `ontocode-rs/codex-api` |
| 31 | `ontocode-rs/config` |
| 27 | `ontocode-rs/windows-sandbox-rs` |
| 24 | `ontocode-rs/tools` |
| 23 | `ontocode-rs/login` |
| 22 | `ontocode-rs/memories` |
| 22 | `ontocode-rs/hooks` |
| 21 | `ontocode-rs/otel` |
| 20 | `ontocode-rs/thread-store` |
| 19 | `ontocode-rs/exec` |
| 19 | `ontocode-rs/app-server-transport` |
| 18 | `ontocode-rs/rollout` |
| 17 | `ontocode-rs/rmcp-client` |
| 17 | `ontocode-rs/protocol` |
| 17 | `ontocode-rs/core-skills` |

Additional dirty-shape markers:

- 344 untracked `.memory-bank` paths after this report, dominated by rename closure/risk/verification audit files and internal crate rename inventories.
- 81 untracked `*.snap.new` files, mostly under `ontocode-rs/tui`.
- 12 generated schema files under `ontocode-rs/app-server-protocol/schema`.
- 8 untracked `.memory-bank/ontocode_internal_crate_rename-*` inventory files.
- Untracked provider and routing implementation files exist under `ontocode-rs/provider-auth`, `ontocode-rs/model-provider`, `ontocode-rs/protocol`, `ontocode-rs/login`, and `ontocode-rs/rmcp-client`.

## Why F1 Is Unsafe In Place

F1 is supposed to be layout-only: move `ontocode-rs/` to `ontocode-rs/` and update path references without changing package identities, protocol/generated names, telemetry, persisted state, or public CLI behavior.

The current worktree already changes the same verification surfaces F1 must rely on:

- Build graph and workspace manifests: `ontocode-rs/Cargo.toml`, `ontocode-rs/Cargo.lock`, many `BUILD.bazel` files, and many crate manifests are dirty.
- Runtime and CLI behavior: `ontocode-rs/cli` is dirty and `ontocode-rs/cli/src/bin/ontocode.rs` is deleted.
- Protocol and generated outputs: app-server protocol Rust files, JSON schemas, TypeScript schemas, and schema fixture tests are dirty.
- TUI snapshots: many `*.snap.new` files are untracked, so UI/test diffs cannot be attributed to F1.
- Package/provider work: new provider-auth and routing files overlap provider, auth, protocol, login, and RMCP ownership boundaries.
- Memory-bank evidence: hundreds of untracked or modified memory-bank audit and inventory files make plan/tracking status ambiguous.

Running `git mv ontocode-rs ontocode-rs` or applying path edits in this state would mix F1 with unrelated rename, package, schema, snapshot, provider, and CLI changes. Verification would not be able to prove that build/test failures, generated diffs, or residual `ontocode-rs` matches came from the layout rename rather than the existing overlay.

Rollback would also be unsafe because the move would need to preserve 1689 tracked edits, 437 untracked paths, and one tracked deletion inside the tree being moved.

Classification: hard implementation blocker for in-place F1.

## Safe Unblock Options

1. Clean sibling worktree for F1, leaving this dirty worktree untouched.
   - Best option because it preserves the current overlay while giving F1 a clean, reviewable baseline.
   - F1 counts, OntoIndex impact, path edits, and verification become attributable to the F1 branch only.

2. Land or intentionally shelve the existing dirty stack before F1.
   - Safe if a manager owns the current changes and can review, commit, stash, or discard them as a separate decision.
   - More expensive than option 1 because it requires classifying 2126 paths before F1 can start.

3. Create a patch/archive backup of the dirty worktree, then reset to a clean tree for F1.
   - Operationally possible, but higher risk because untracked files and generated outputs can be missed unless archived carefully.
   - Only suitable if the manager confirms the dirty overlay is disposable or fully backed up.

4. Continue F1 in place with strict file filters.
   - Not recommended.
   - The overlap with `ontocode-rs/core`, `ontocode-rs/tui`, `ontocode-rs/app-server`, `ontocode-rs/app-server-protocol`, `ontocode-rs/cli`, generated schemas, snapshots, and package/provider files is too broad for reliable attribution.

## Recommended Option

Use option 1: create a clean sibling worktree from the current intended base, run F1 there, and leave `/opt/demodb/_workfolder/ontocode` untouched until the dirty overlay is separately owned.

Exact recommendation:

- Stop F1 implementation in `/opt/demodb/_workfolder/ontocode`.
- Manager creates `/opt/demodb/_workfolder/ontocode-f1-layout` from the approved base commit or branch.
- Manager runs F0/F1 inventory and OntoIndex freshness checks in that clean worktree.
- F1 implementation proceeds only after `git status --porcelain=v1` is empty in the F1 worktree.

## Commands The Manager/User Would Run If Needed

Inspect current blocker state:

```bash
cd /opt/demodb/_workfolder/ontocode
git status --short
git status --porcelain=v1 | wc -l
git status --porcelain=v1 | cut -c1-3 | sort | uniq -c | sort -nr
```

Create a clean F1 worktree without touching the dirty overlay:

```bash
cd /opt/demodb/_workfolder/ontocode
git worktree add ../ontocode-f1-layout HEAD
cd ../ontocode-f1-layout
git status --short
```

If the manager wants to preserve the current dirty overlay before any cleanup:

```bash
cd /opt/demodb/_workfolder/ontocode
git diff > /tmp/ontocode-dirty-tracked-2026-06-14.patch
git ls-files --others --exclude-standard -z | tar --null -T - -czf /tmp/ontocode-dirty-untracked-2026-06-14.tgz
```

If the manager owns the dirty overlay and wants to park it in Git:

```bash
cd /opt/demodb/_workfolder/ontocode
git switch -c dirty-legacy-migration-overlay-2026-06-14
git add -A
git commit -m "WIP: preserve legacy migration dirty overlay"
```

If the manager chooses stash instead of a branch:

```bash
cd /opt/demodb/_workfolder/ontocode
git stash push -u -m "dirty legacy migration overlay before F1 layout rename"
```

After the F1 worktree is clean and exactly one process is assigned to refresh the index:

```bash
cd /opt/demodb/_workfolder/ontocode-f1-layout
ontoindex analyze --skip-agents-md
```

F1 must still run exact OntoIndex impact/context on each symbol before any symbol edit. The layout move itself is not a symbol edit, but any code changes made to repair build, package, CLI, or test behavior are symbol edits and must use OntoIndex impact first.

## Stop Conditions

Stop F1 immediately if any of these are true:

- `git status --porcelain=v1` in the F1 implementation worktree is non-empty before F1 begins.
- OntoIndex reports a stale index, dirty-worktree overlay, or low-confidence scope for the F1 worktree after the manager-designated refresh point.
- Any proposed F1 patch changes public CLI behavior, package identities, protocol/generated names, telemetry names, `CODEX_*` env/state semantics, SDK import paths, or release assets.
- Any F1 patch includes unreviewed `*.snap.new` files or generated schema changes not produced by the required F1 verification commands.
- Any build/test failure cannot be attributed to the layout rename alone.
- OntoIndex impact for a required symbol edit returns HIGH or CRITICAL risk and the manager has not explicitly accepted that blast radius.
- `git diff --check -- .memory-bank/ONTOCODE_FULL_LEGACY_MIGRATION_DIRTY_WORKTREE_UNBLOCK.md` fails.

## Verification

Required local verification for this report:

```bash
cd /opt/demodb/_workfolder/ontocode
git diff --check -- .memory-bank/ONTOCODE_FULL_LEGACY_MIGRATION_DIRTY_WORKTREE_UNBLOCK.md
```
