---
name: Find Doubled Code Tool
description: ADR for a thin jscpd-backed duplicate-code advisory tool
type: adr
date: 2026-06-21
status: challenged - helper first, runtime tool gated
---

# ADR: Find Doubled Code Tool

## Context

Donor source: [tmp/jscpd-main](../tmp/jscpd-main).

jscpd provides copy/paste detection, compact AI-oriented clone output, ignore
globs, minimum line/token thresholds, JSON/Markdown reports, and CI-friendly
statistics. Relevant donor evidence:

- [skills/jscpd/SKILL.md](../tmp/jscpd-main/skills/jscpd/SKILL.md) documents
  `npx jscpd --reporters ai <path>` and compact clone-pair output.
- [skills/dry-refactoring/SKILL.md](../tmp/jscpd-main/skills/dry-refactoring/SKILL.md)
  treats clone findings as refactor hints that still require reading both
  fragments and proving a real shared concept.
- [docs/typescript.md](../tmp/jscpd-main/docs/typescript.md) documents
  thresholds, modes, reporters, `.gitignore`, `skipLocal`, and structured
  reports.
- [docs/ci-and-hooks.md](../tmp/jscpd-main/docs/ci-and-hooks.md) documents
  CI outputs such as clone count, duplicated lines, percentage, and report path.

Existing Ontocode jscpd review already rejected a native duplicate detector,
MCP server, REST API, background scanner, and repo-wide CI gate. The retained
value is advisory duplicate discovery for manager review, ADR cleanup, prompt
cleanup, and refactor planning.

OntoIndex challenge evidence:

- Current code-mode command tests already cover bounded external command output
  and handler error surfacing in [code_mode.rs](../ontocode-rs/core/tests/suite/code_mode.rs).
- Existing repo helper precedent is a small read-only script plus focused tests:
  [onto_memory_tools.py](../scripts/onto_memory_tools.py) and
  [test_onto_memory_tools.py](../sdk/python/tests/test_onto_memory_tools.py).
- `ToolRegistry` is a core runtime surface in
  [registry.rs](../ontocode-rs/core/src/tools/registry.rs). Adding a
  model-visible tool there is a wider behavior change than a donor-advisory
  scan needs.

## Decision

Do not start with a model-visible runtime tool. The first acceptable shape is a
repo-local helper named `find_doubled_code` that wraps an already available
`jscpd` or `cpd` executable and emits bounded advisory output.

The helper should:

- scan explicit paths supplied by the caller;
- default to `.gitignore`-aware scanning and ignore generated, vendor, build,
  snapshot, and donor scratch trees;
- use meaningful defaults such as `minLines=5`, `minTokens=50`, and weak mode
  for docs/tests unless the caller asks for strict code scanning;
- return compact bounded output optimized for agents: clone pairs, summary
  counts, duplicated lines, duplication percentage, and optional report path;
- treat findings as review/refactor hints, not automatic extraction tasks;
- fail closed with installation guidance when neither `jscpd` nor `cpd` exists.

Runtime exposure as a model-visible tool is gated. It needs a second ADR or ADR
amendment proving that shelling out through the existing command path is not
enough, that the tool registry is the correct owner, and that the new surface is
worth the added model-visible behavior.

The helper must not embed a Rabin-Karp detector, tokenizer, persistent store,
reporter framework, or daemon.

## Proposed Interface

Helper name: `find_doubled_code`.

Inputs:

- `paths`: required list of files or directories to scan.
- `min_lines`: optional, default `5`.
- `min_tokens`: optional, default `50`.
- `mode`: optional `weak`, `mild`, or `strict`; default `weak`.
- `skip_local`: optional, default `false`.
- `format`: optional language/format filter.
- `report`: optional `compact`, `json`, or `markdown`; default `compact`.

Output:

- `status`: `ok`, `duplicates_found`, `tool_missing`, or `scan_failed`.
- `summary`: clone count, duplicated lines, duplicated percentage.
- `clones`: bounded list of clone pairs with file ranges.
- `report_path`: optional path if a file report was requested.
- `next_action`: one short recommendation, usually inspect both fragments
  before proposing extraction.

## Non-Goals

- Do not build a native duplicate detector in Ontocode.
- Do not add a model-visible runtime tool, MCP server, app-server API, CI gate,
  pre-commit hook, or background scanner in this ADR.
- Do not inject full clone snippets into model context by default.
- Do not auto-refactor duplicated code.
- Do not scan the whole repository unless the user explicitly asks for it.
- Do not scan `target/`, `node_modules/`, `dist/`, generated schemas,
  snapshots, or `tmp/` donor repositories by default.

## Acceptance Criteria

- Missing `jscpd`/`cpd` produces a clear `tool_missing` result and exact install
  guidance instead of a panic or silent success.
- Default scan respects `.gitignore` and default ignore globs.
- Output is bounded and contains clone pairs plus summary counts.
- A test or fixture proves large clone lists are truncated with a visible
  omitted-count marker.
- A test proves clone findings are advisory and do not trigger automatic code
  edits.
- Any future Rust/tool-registry implementation requires separate approval and
  must run through the existing tool registry/handler owner; no new tool
  registry or scanner service is introduced.

## Review Rule

Before acting on a clone, read both fragments, run OntoIndex impact on any
symbol to be edited, and only extract when the duplicate represents a real
shared concept. Similar code in tests, snapshots, generated output, or parallel
domain owners may be correct and should often stay duplicated.

## Challenge Result

Keep the idea, narrow the surface. `find_doubled_code` is useful as a local
advisory helper for review and ADR cleanup. It is not yet approved as a
model-visible runtime tool because current shell-command execution plus a small
repo helper can cover the need with less architecture.
