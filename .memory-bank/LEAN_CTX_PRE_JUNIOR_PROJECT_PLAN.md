---
name: Lean-ctx Pre-Junior Project Plan
description: Step-by-step plan for very junior implementers to build only the approved repository-script slice from ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md
type: project_plan
date: 2026-06-15
status: complete
---

# Lean-ctx Pre-Junior Project Plan

## Goal

Implement the smallest approved lean-ctx-inspired repository helper slice without adding runtime architecture, model-visible tools, app-server APIs, new dependencies, or a second memory/search/context system.

This plan is written for pre-junior implementers. Each task must be small, checkable, and reversible.

## Completion Status

Completed on 2026-06-15.

Implementation:

- [scripts/onto_memory_tools.py](../scripts/onto_memory_tools.py)
- [sdk/python/tests/test_onto_memory_tools.py](../sdk/python/tests/test_onto_memory_tools.py)
- [LEAN_CTX_PRE_JUNIOR_TRACKING.md](LEAN_CTX_PRE_JUNIOR_TRACKING.md)
- [audit_session-2026-06-15-lean-ctx-pre-junior-preflight.md](audit_session-2026-06-15-lean-ctx-pre-junior-preflight.md)

Known follow-up:

- `doc-link-check` currently reports pre-existing broken `.memory-bank` links in older ADRs. The helper is complete; fixing that documentation debt is a separate cleanup task.

## Source ADR

Authoritative source: [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md)

Binding interpretation:

- Lean-ctx stays external workflow tooling.
- Ontocode must not vendor lean-ctx or copy its runtime.
- Stage 0 may add exactly one repository-only Python script and optional tests.
- The only approved repository tools are:
  - `onto_memory_status_digest`
  - `onto_tracking_count_left`
  - `onto_diff_doc_link_check`
- Everything else in the ADR catalog is backlog, not approved work.

## Challenge Review

Review date: 2026-06-15.

OntoIndex was used to check existing repository script and test patterns before updating this plan.

Findings:

- Existing root-level Python script pattern: [scripts/format.py](/opt/demodb/_workfolder/ontocode/scripts/format.py:1) uses a single-file script, `argparse`, `Path(__file__).resolve()`, `main() -> int`, and no package scaffolding.
- Existing Python script test pattern: [test_artifact_workflow_and_binaries.py](/opt/demodb/_workfolder/ontocode/sdk/python/tests/test_artifact_workflow_and_binaries.py:17) loads repository scripts as modules to test real helper functions.
- The current plan was safe but incomplete: it did not require implementers to inspect the donor lean-ctx source before copying workflow ideas.
- The plan must explicitly block implementation until the donor source has been reviewed and summarized.

Challenge result:

- Keep the one-script Stage 0 scope.
- Add a mandatory donor-source preflight before implementation.
- Copy only patterns: bounded output, read-only file inspection, clear command errors, deterministic summaries.
- Do not copy lean-ctx runtime mechanics: MCP server, caches, compression engine, session store, knowledge store, graph index, shell wrapper, or tool registry.

## Donor Source Preflight

This stage is mandatory before writing `scripts/onto_memory_tools.py`.

Purpose: make the implementer inspect how lean-ctx solved similar workflow problems before writing a local helper, while preventing them from copying lean-ctx internals into Ontocode.

Read these mandatory donor files:

- [donor lean-ctx rules](/opt/demodb/_workfolder/_donors/lean-ctx/LEAN-CTX.md:1)
- [donor skill manifest](/opt/demodb/_workfolder/_donors/lean-ctx/skills/lean-ctx/SKILL.md:1)
- [donor ctx_read](/opt/demodb/_workfolder/_donors/lean-ctx/rust/src/tools/ctx_read/mod.rs:1)
- [donor ctx_search](/opt/demodb/_workfolder/_donors/lean-ctx/rust/src/tools/ctx_search.rs:1)
- [donor ctx_shell](/opt/demodb/_workfolder/_donors/lean-ctx/rust/src/tools/ctx_shell.rs:1)

Reviewer-only donor context for rejected runtime patterns:

- [donor MCP registry](/opt/demodb/_workfolder/_donors/lean-ctx/rust/src/server/registry.rs:122)
- [donor tool dispatch](/opt/demodb/_workfolder/_donors/lean-ctx/rust/src/server/call_tool.rs:272)
- [donor ctx_session](/opt/demodb/_workfolder/_donors/lean-ctx/rust/src/tools/ctx_session.rs:1)
- [donor ctx_knowledge](/opt/demodb/_workfolder/_donors/lean-ctx/rust/src/tools/ctx_knowledge/mod.rs:1)
- [donor shell compression](/opt/demodb/_workfolder/_donors/lean-ctx/rust/src/shell/compress/engine.rs:1)

Write a short implementation note before coding:

```text
Donor source checked:
- files read:
- patterns reused:
- patterns explicitly rejected:
- why this remains one read-only repository script:
```

Allowed reused patterns:

- resolve the project root once
- read files with explicit UTF-8 handling
- return bounded summaries instead of raw file dumps
- print clear errors with stable exit codes
- keep command output deterministic

Rejected donor patterns:

- MCP tool registration
- long-running daemon behavior
- compressed output archive storage
- session or knowledge persistence
- graph index or call graph storage
- shell command execution wrappers
- background workers

Acceptance:

- The implementation PR includes the short donor-source note in the PR body or audit entry.
- The new script does not import, vendor, shell out to, or depend on donor lean-ctx code.
- Any proposed behavior not found in the donor preflight or ADR must be dropped unless a reviewer approves a separate task card.

## Non-Goals

- Do not edit Rust code.
- Do not add a crate, MCP tool, app-server method, config key, migration, or model-visible tool.
- Do not call GitNexus/OntoIndex from the new script.
- Do not parse LadybugDB, lean-ctx caches, `.gitnexus`, or `.ontoindex` storage.
- Do not mutate ADR or tracking status from the script.
- Do not add third-party Python dependencies.

## Deliverable

One script:

- `scripts/onto_memory_tools.py`

Optional tests:

- `sdk/python/tests/test_onto_memory_tools.py`

The script must be standard-library only and read-only by default.

## Command Interface

Use simple subcommands:

```bash
python3 scripts/onto_memory_tools.py status-digest
python3 scripts/onto_memory_tools.py count-left
python3 scripts/onto_memory_tools.py doc-link-check
```

Deferred flags:

Do not implement `--root`, `--json`, or other feature flags in Stage 0. Add them later only with a separate task card.

## Stage 0: Guardrails

Purpose: make it hard for a junior implementer to build the wrong thing.

Tasks:

- Complete the donor-source preflight and record the short implementation note.
- Create `scripts/onto_memory_tools.py`.
- Add an argparse CLI with exactly three subcommands.
- Add a top-level docstring saying this is repository-only, read-only, standard-library-only tooling.
- Resolve paths relative to repository root, not current working directory.
- Exit non-zero only for real failures:
  - missing required file
  - broken markdown link
  - invalid CLI args

Acceptance:

- Running `python3 scripts/onto_memory_tools.py --help` lists exactly the three approved commands.
- The Stage 0 CLI accepts no optional feature flags other than normal argparse help.
- The script imports only Python standard-library modules.
- The script does not write files.
- The donor-source note exists before code review.

Pre-junior checklist:

- Do not import `requests`, `yaml`, `toml`, `rich`, `click`, or repo Rust code.
- Do not create a package folder.
- Do not add config files.

## Stage 1: `status-digest`

Purpose: print a short project status digest from memory-bank files.

Inputs:

- `.memory-bank/MEMORY.md`
- `.memory-bank/project_plan-current.md`
- `.memory-bank/project_pending-tasks.md`

Behavior:

- Confirm all three files exist.
- Print:
  - memory-bank title count from `MEMORY.md`
  - first status-like line from `project_plan-current.md`
  - number of non-empty checklist or bullet lines in `project_pending-tasks.md`
  - top three pending lines, truncated to 160 characters each

Implementation hints:

- Use `pathlib.Path`.
- Use `read_text(encoding="utf-8")`.
- Count memory entries by lines starting with `- [`.
- Treat pending lines as lines starting with `- ` or `- [ ]`.
- Use a small `truncate(text, max_len=160)` helper.

Acceptance:

- Output is under 80 lines.
- Missing files produce a clear error.
- No raw large file content is printed.

Manual check:

```bash
python3 scripts/onto_memory_tools.py status-digest
```

## Stage 2: `count-left`

Purpose: count obvious task statuses without changing tracking files.

Inputs:

- `.memory-bank/project_pending-tasks.md`
- `.memory-bank/CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md`

Behavior:

- Read both files if present.
- Count lines containing these markers:
  - done: `[x]`, `done`, `completed`, `accepted`
  - pending: `[ ]`, `pending`, `todo`
  - blocked: `blocked`
  - in progress: `in progress`, `in-progress`
- Print a compact table with one row per file.

Implementation hints:

- Use lowercase matching.
- Count only markdown table rows and task/list lines starting with `|`, `-`, or `*`.
- Avoid clever parsing. This is a rough digest, not source of truth.
- Make counts deterministic by scanning files in a fixed order.

Acceptance:

- The command never edits tracking files.
- The output says counts are heuristic.
- If the tracking file is missing, report it and continue with the pending file.

Manual check:

```bash
python3 scripts/onto_memory_tools.py count-left
```

## Stage 3: `doc-link-check`

Purpose: validate local markdown links inside `.memory-bank`.

Inputs:

- Markdown files under `.memory-bank/`

Behavior:

- Find markdown links shaped like `[label](target)`.
- Ignore:
  - `http://...`
  - `https://...`
  - `mailto:...`
  - anchors only, like `#section`
- For relative links, resolve from the markdown file's parent directory.
- For absolute local links, check that the target path exists.
- Strip line suffixes like `:123` only for absolute file targets.
- Strip anchor suffixes after `#`.
- Print broken links and return exit code `1` if any are broken.

Implementation hints:

- Use `re.finditer`.
- Use `Path.exists()`.
- Keep the parser intentionally simple; markdown edge cases are not worth a framework.

Acceptance:

- Valid links pass.
- At least one deliberately broken link in a temp test file fails.
- Output lists file path and target for each broken link.

Manual check:

```bash
python3 scripts/onto_memory_tools.py doc-link-check
```

## Stage 4: Minimal Tests

Purpose: leave one runnable check behind.

Tasks:

- Add `sdk/python/tests/test_onto_memory_tools.py`.
- Use only Python standard-library test helpers such as `unittest`, `tempfile`, `pathlib`, `importlib.util`, `io`, and `contextlib`.
- Load `scripts/onto_memory_tools.py` as a module using the existing root-script test pattern in `sdk/python/tests/test_artifact_workflow_and_binaries.py`.
- Test:
  - help lists the three commands
  - `doc-link-check` passes for a valid temp memory-bank
  - `doc-link-check` fails for a broken temp link
  - status counting helper handles simple marker lines

Acceptance:

```bash
python3 -m unittest sdk/python/tests/test_onto_memory_tools.py
```

Do not add pytest.

## Stage 5: Memory-Bank Update

Purpose: record the new helper without overstating it.

Tasks:

- Add a one-line entry to `.memory-bank/MEMORY.md` only after the script exists.
- If implementation changes workflow behavior, add a dated audit file.

Acceptance:

- Memory entry says the script is repository-only and read-only.
- No ADR status changes.

## Pre-Junior Task Cards

### PJ-PRE Donor Source Preflight

Files:

- `.memory-bank/LEAN_CTX_PRE_JUNIOR_PROJECT_PLAN.md`
- donor files listed in "Donor Source Preflight"

Steps:

- Read the mandatory donor files.
- Skim the reviewer-only donor context only to understand what must not be copied.
- Read [scripts/format.py](/opt/demodb/_workfolder/ontocode/scripts/format.py:1) for the local one-file script pattern.
- Read [test_artifact_workflow_and_binaries.py](/opt/demodb/_workfolder/ontocode/sdk/python/tests/test_artifact_workflow_and_binaries.py:17) for the local script-as-module test pattern.
- Write the short donor-source note.
- Confirm no donor runtime mechanic is needed for the approved Stage 0 commands.

Done when:

- A reviewer can see exactly what was checked before implementation.

### PJ-0 Create CLI Skeleton

Files:

- `scripts/onto_memory_tools.py`

Steps:

- Add argparse with three subcommands.
- Add `repo_root()` helper.
- Add `main(argv=None)`.
- Return process codes from `main`.

Done when:

- `python3 scripts/onto_memory_tools.py --help` works.

### PJ-1 Add Status Digest

Files:

- `scripts/onto_memory_tools.py`

Steps:

- Add `status_digest(root)`.
- Do not implement JSON output in this stage.
- Read only the three approved memory files.
- Print bounded summary.

Done when:

- `python3 scripts/onto_memory_tools.py status-digest` prints under 80 lines.

### PJ-2 Add Count Left

Files:

- `scripts/onto_memory_tools.py`

Steps:

- Add `count_left(root)`.
- Count simple status markers only in markdown table rows and task/list lines.
- Print heuristic table.

Done when:

- `python3 scripts/onto_memory_tools.py count-left` works even if one optional tracking file is missing.

### PJ-3 Add Doc Link Check

Files:

- `scripts/onto_memory_tools.py`

Steps:

- Add markdown link scanner.
- Ignore external URLs.
- Resolve local links.
- Return exit `1` for broken links.

Done when:

- `python3 scripts/onto_memory_tools.py doc-link-check` reports broken links or exits cleanly.

### PJ-4 Add Minimal Tests

Files:

- `sdk/python/tests/test_onto_memory_tools.py`

Steps:

- Add unittest tests for CLI help and link checking.
- Keep fixtures in temporary directories.

Done when:

- `python3 -m unittest sdk/python/tests/test_onto_memory_tools.py` passes.

## Review Rules

Reviewer must reject the change if it:

- adds dependencies
- writes memory-bank files automatically
- changes ADR status
- adds runtime Rust code
- adds model-visible or app-server surfaces
- creates more than one helper script without a new ADR
- shells out to GitNexus, OntoIndex, lean-ctx, cargo, just, or git
- skips the donor-source preflight
- copies lean-ctx runtime mechanics instead of only small script patterns

## Final Verification

Before implementation, verify the donor source files still exist from the repository root:

```bash
cd /opt/demodb/_workfolder/ontocode
test -f ../_donors/lean-ctx/LEAN-CTX.md
test -f ../_donors/lean-ctx/skills/lean-ctx/SKILL.md
test -f ../_donors/lean-ctx/rust/src/tools/ctx_read/mod.rs
test -f ../_donors/lean-ctx/rust/src/tools/ctx_shell.rs
test -f ../_donors/lean-ctx/rust/src/tools/ctx_search.rs
test -f ../_donors/lean-ctx/rust/src/server/registry.rs
test -f ../_donors/lean-ctx/rust/src/server/call_tool.rs
test -f ../_donors/lean-ctx/rust/src/tools/ctx_session.rs
test -f ../_donors/lean-ctx/rust/src/tools/ctx_knowledge/mod.rs
test -f ../_donors/lean-ctx/rust/src/shell/compress/engine.rs
```

After implementation, run:

```bash
cd /opt/demodb/_workfolder/ontocode
python3 scripts/onto_memory_tools.py --help
python3 scripts/onto_memory_tools.py status-digest
python3 scripts/onto_memory_tools.py count-left
python3 scripts/onto_memory_tools.py doc-link-check
python3 -m unittest sdk/python/tests/test_onto_memory_tools.py
```

Current note: `doc-link-check` is expected to exit `1` until the existing broken links in older `.memory-bank` ADRs are cleaned up.

No Rust formatting or Rust tests are required for this repository-script-only slice.
