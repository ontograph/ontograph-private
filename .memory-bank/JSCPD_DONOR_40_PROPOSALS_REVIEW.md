name: jscpd Donor 40 Proposals Review
description: Forty useful jscpd-inspired proposals mapped to Ontocode owner areas
type: donor_review
date: 2026-06-16
status: proposed

# jscpd Donor 40 Proposals Review

## Context

Donor source: `tmp/jscpd-main`.

jscpd is a copy/paste detector with TypeScript and Rust engines. Useful donor ideas come from:

- Rabin-Karp duplicate detection and clone statistics in `packages/core`.
- File walking, gitignore handling, and reporter orchestration in `packages/finder`.
- Multi-format tokenization and cross-format block handling in `packages/tokenizer`.
- Rust engine crates under `rust/crates/cpd-*`.
- AI reporter, SARIF reporter, GitHub Action, CI thresholding, and agent skills.

OntoIndex owner checks used for current Ontocode fit:

- `ontocode-rs/core/tests/suite/apply_patch_cli.rs` already owns apply-patch regression tests and is a good first home for duplication-driven patch safety checks.
- `ontocode-rs/core/src/tools/handlers/apply_patch.rs` owns apply-patch interception and is 623 LoC; prefer tests before production changes.
- `ontocode-rs/core/src/session/turn.rs` owns prompt/turn/context behavior and is 2252 LoC; avoid growing it except through focused tests.
- `ontocode-rs/hooks/src/engine/discovery.rs` owns hook discovery and is 1087 LoC; avoid second hook systems.
- `ontocode-rs/core/src/tools/spec_plan.rs` owns tool routing and is 996 LoC; avoid new tool registries.

## Decision Frame

These are useful proposals, not approved implementation tasks. Keep them as donor ideas until each one passes the existing architecture gate:

- It extends an existing owner.
- It adds safety, testability, review quality, or bounded generated-code/text value.
- It does not create a second scanner service, MCP server, hook system, report pipeline, or refactoring engine.

## Proposals

| ID | Donor idea | Ontocode owner | Proposal | Suitability |
| --- | --- | --- | --- | --- |
| JSC-01 | AI reporter with compact clone lines | `core/tests`, memory-bank review docs | Add an optional "duplicate hotspots" review format for human/agent triage that emits only file ranges and summary counts. | Useful if produced by an external check or script; do not inject into model context by default. |
| JSC-02 | Common-path-prefix compression | output reducers / review summaries | Use common-prefix compression for repeated file paths in generated audit/review summaries. | Useful for token reduction; fits existing bounded-output rules. |
| JSC-03 | Threshold reporter | CI/project-plan checks | Add project-plan guidance for duplication thresholds per touched area, not a global hard gate. | Useful as review policy; avoid blocking all work on repo-wide historical duplication. |
| JSC-04 | SARIF reporter | release/CI docs | Consider SARIF as an export target for future static-analysis findings. | Useful but deferred; needs security review and schema compatibility. |
| JSC-05 | JSON report | audit/session artifacts | Store clone findings as bounded JSON artifacts for manager review. | Useful for tooling; keep out of runtime core unless an ADR accepts it. |
| JSC-06 | Markdown reporter | memory-bank summaries | Generate compact markdown clone summaries for ADR/audit notes. | Useful for docs workflows; no runtime changes. |
| JSC-07 | Badge reporter | release dashboards | Add duplication badge only if a release dashboard already exists. | Low priority; cosmetic unless tied to release readiness. |
| JSC-08 | GitHub Action inputs/outputs | CI/release workflow | Mirror jscpd's explicit CI outputs for future quality checks: clone count, duplicated lines, percentage, report path. | Useful for automation plans; not core runtime. |
| JSC-09 | `--min-lines` and `--min-tokens` | review policy | Define minimum clone size for meaningful refactor suggestions to avoid noise. | Strong fit for agent-review guidance. |
| JSC-10 | `--max-size` and `--max-lines` | scan scripts / CI | Skip generated or huge files by hard caps before analysis. | Useful and cheap; matches bounded-output principles. |
| JSC-11 | `.gitignore` aware scanning | scripts / repo helper | Respect `.gitignore` by default when scanning donor/current code for duplicates. | Useful; avoid scanning generated/vendor trees. |
| JSC-12 | Explicit ignore globs | review config docs | Document ignore defaults for `target/`, generated schemas, snapshots, fixtures, and donor `tmp/`. | Useful as policy; no code needed first. |
| JSC-13 | `--skip-local` | refactor triage | Separate local same-directory clones from cross-module clones. | Useful: cross-module clones are stronger architecture signals. |
| JSC-14 | Detection modes: mild/weak/strict | review workflow | Use modes as triage levels: strict for code, weak for tests/docs, mild for first scan. | Useful if using jscpd externally; do not build custom detector. |
| JSC-15 | Blame comparison | review triage | Use blame only to prioritize ownership, not to assign fault. | Useful for reviewer routing; avoid adding blame into model context. |
| JSC-16 | Worker control | local scan scripts | Cap duplication-scan parallelism like Rust build parallelism. | Useful for repo hygiene scripts; avoids developer-machine overload. |
| JSC-17 | Rust engine packaging | release plans | Study prebuilt-binary matrix only for future Ontocode release packaging. | Useful reference; not relevant to core behavior now. |
| JSC-18 | Drop-in CLI compatibility | Ontocode rename/alias plans | Use jscpd's `jscpd`/`cpd` alias model as a reference for Ontocode/Codex binary compatibility. | Useful for rename planning; fits existing compatibility-first migration. |
| JSC-19 | Config compatibility across engines | config/schema migrations | Preserve config compatibility when replacing internals. | Useful principle for provider/config migrations. |
| JSC-20 | File walker separated from detector | tooling scripts | Keep scanning/file selection separate from duplicate analysis. | Useful if a repo helper is added; avoid mixing with core session code. |
| JSC-21 | Tokenizer separated from detector | context tooling | Separate normalization/tokenization from reporting if duplicate analysis is introduced. | Useful design guard; no new crate until needed. |
| JSC-22 | Cross-format tokenization | prompt/docs checks | Detect duplication across Markdown, TOML, Rust, and TypeScript docs/config when reviewing donor plans. | Useful for docs hygiene; likely external script only. |
| JSC-23 | Format-name mapping | repo helper config | Support filename-based classification for `Dockerfile`, `Makefile`, `BUILD.bazel`, `AGENTS.md`. | Useful if external scan config is adopted. |
| JSC-24 | SFC/Markdown block extraction | prompt/memory docs | Treat Markdown fenced code and prose sections separately when checking duplication. | Useful for memory-bank/ADR cleanup; no runtime context injection. |
| JSC-25 | Clone statistics by format | audit reports | Report duplication by owner area or file type. | Useful for manager review; avoid broad CI gate at first. |
| JSC-26 | Statistics by source file | hotspot reports | Use duplicated-line/token percentage to rank large files for refactor review. | Useful; aligns with existing large-module rules. |
| JSC-27 | Highest-impact-first refactor skill | pre-junior plans | Add guidance to address the largest clone first only after owner/impact checks. | Useful for future pre-junior docs. |
| JSC-28 | Dry-refactoring workflow | refactor review process | Require read-both-fragments, understand behavior, extract only when a real shared concept exists, then verify. | Strong fit for refactor discipline. |
| JSC-29 | Avoid speculative abstraction | refactor process | Invert dry-refactoring: not every clone deserves extraction. | Strong fit with project and Ponytail/YAGNI rules. |
| JSC-30 | Test clone detection | test helper cleanup | Use clone findings in tests to identify missing fixtures/helpers. | Useful; should be advisory only. |
| JSC-31 | Clone findings in apply-patch tests | `core/tests/suite/apply_patch_cli.rs` | Add future tests ensuring repeated apply-patch error text remains bounded and not duplicated across paths. | Useful but needs a concrete failing test before code. |
| JSC-32 | Hook output duplication checks | `hooks` tests | Check repeated hook diagnostics for redaction/cap consistency. | Useful; extend existing hook tests only. |
| JSC-33 | Prompt instruction duplicate check | `protocol/src/prompts` tests/docs | Scan prompt markdown for repeated policy blocks before adding more base instructions. | Useful; could prevent prompt bloat. |
| JSC-34 | Memory-bank duplicate scope check | `.memory-bank` docs | Use clone checks to find duplicate ADR/project-plan sections and consolidate stale donor rows. | Useful; directly matches current donor-doc cleanup work. |
| JSC-35 | Token-efficient report for code intelligence | OntoIndex review docs | Keep code-intelligence outputs compact like jscpd's AI reporter. | Useful principle; do not duplicate OntoIndex. |
| JSC-36 | MCP server for duplicate checks | MCP docs | Reject by default: Ontocode already has MCP/code-intelligence owners; use external jscpd if needed. | Useful as a negative decision. |
| JSC-37 | REST API for duplicate scan | app-server docs | Defer: a duplication API is not current app-server core scope. | Useful only if product requirement appears. |
| JSC-38 | Pre-commit hook | contributor docs | Add optional local pre-commit duplication check after a threshold/config is accepted. | Useful but should be optional and documented. |
| JSC-39 | Fuzz targets for detector/tokenizer | parser/tooling tests | Use fuzzing idea for parsers or patch grammar, not clone detection. | Useful for apply-patch/parser safety if scoped. |
| JSC-40 | Golden report parity tests | tool/report tests | Use golden fixtures to keep reports stable when output is model-visible or CI-visible. | Useful; fits existing snapshot/golden-test patterns. |

## Best First Slice

The lowest-risk first slice is documentation/tooling only:

1. Add an optional external duplication scan note for memory-bank and prompt reviews.
2. Define default ignore globs for generated/vendor/donor directories.
3. Use AI-style compact output for any clone report pasted into ADRs.
4. Treat clone findings as refactor hints, not automatic abstraction tasks.

Do not implement a native duplicate detector, MCP server, or CI gate yet.
