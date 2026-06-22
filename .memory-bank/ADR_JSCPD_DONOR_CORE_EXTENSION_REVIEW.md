name: jscpd Donor Core Extension Review
desc: ADR gate for jscpd donor ideas, keeping only new proposals that extend Ontocode core behavior
type: adr
date: 2026-06-16
status: proposed

# ADR: jscpd Donor Core Extension Review

## Context

Source review: [JSCPD_DONOR_40_PROPOSALS_REVIEW.md](JSCPD_DONOR_40_PROPOSALS_REVIEW.md).

The donor project `tmp/jscpd-main` is useful as a copy/paste detection and report-quality reference, but most ideas from it are external tooling, CI policy, or documentation hygiene. Ontocode should not grow a native duplicate detector, second report pipeline, second MCP service, or broad refactor engine from this donor.

This ADR keeps only proposals that pass both gates:

- **New:** not already covered by current bounded output reducers, hook output spill handling, prompt planning rules, or OntoIndex/code-intelligence workflows.
- **Core-extending:** improves existing generated code/text behavior, tool output stability, prompt surface quality, or parser/test safety in current owners.

## OntoIndex Evidence

OntoIndex index check was fresh at commit `73ba3040e201390b3b6b0bc05f7d8d33e9c215b6`.

Relevant owner checks:

- `ontocode-rs/core/src/tools/handlers/apply_patch.rs` owns apply-patch interception and is already 623 LoC. Extend through tests first.
- `ontocode-rs/core/tests/suite/apply_patch_cli.rs` owns apply-patch CLI regression coverage and is the best first home for generated-code duplicate/error-output cases.
- `ontocode-rs/core/src/session/turn.rs` owns turn/prompt/tool construction and is already 2252 LoC. Do not add donor logic directly here.
- `ontocode-rs/hooks/src/engine/discovery.rs` owns hook discovery and is already 1087 LoC. Do not add a second hook system.
- `ontocode-rs/core/src/tools/spec_plan.rs` owns tool routing and is already 996 LoC. Do not add a donor-inspired tool registry.
- Existing output truncation/spill code already covers generic bounded-output behavior, so generic path compression and report compactness ideas are not new core work by themselves.

## Decision

Keep only five jscpd donor proposals as active core-extension candidates. All are test-first or guard-first. None approve a native clone detector.

| ID | Keep Scope | Best Current Owner | Why It Extends Core | Challenge / Narrowing |
| --- | --- | --- | --- | --- |
| JSC-31 | Add regression tests for repeated apply-patch failure text and duplicated generated-code diagnostics. | `ontocode-rs/core/tests/suite/apply_patch_cli.rs` | Generated patch text is a core coding path; duplicated diagnostics waste model/user context and can hide the real error. | Only add tests when there is a concrete repeated-output fixture. Do not add detector code unless a failing test proves the current formatter cannot be fixed locally. |
| JSC-32 | Add hook-output tests that assert duplicate spill text remains bounded, redacted, and attributed once. | `ontocode-rs/hooks/src/output_spill.rs`; hook engine tests | Hook output can be injected into agent-visible context; caps and redaction are core safety behavior. | Reuse existing output spill/truncation helpers. Do not add a new hook reporter or discovery path. |
| JSC-33 | Add a prompt-surface duplication guard for base instruction sections that affect model behavior. | `ontocode-rs/protocol/src/prompts/base_instructions/default.md`; prompt validation tests if present | The base prompt directly shapes coding behavior; accidental repeated rules increase token cost and can create conflicting guidance. | Keep this as a small static check or review fixture. Do not build a repo-wide markdown clone scanner. |
| JSC-39 | Use adversarial/fuzz-style fixtures for apply-patch and generated-text parsers. | Apply-patch parser/CLI tests; existing parser test harnesses | Parser robustness directly affects generated code application. Donor value is the test strategy, not clone detection. | Prefer existing test harnesses. Add a fuzz target only if an existing fuzz setup is already available or a real parser bug justifies it. |
| JSC-40 | Add golden/snapshot parity tests for model-visible or CI-visible generated reports. | Existing tool/report/snapshot tests near the output owner | Stable generated text is core behavior when the model or user consumes it. Golden fixtures catch accidental verbosity, duplication, and shape drift. | Limit to outputs already owned by Ontocode. Do not introduce a generic donor-style reporter pipeline. |

## Removed From Active Scope

All other jscpd donor rows are removed from active implementation scope for this ADR.

- **External report or CI policy, not core:** JSC-01, JSC-03, JSC-04, JSC-05, JSC-06, JSC-07, JSC-08, JSC-12, JSC-13, JSC-14, JSC-15, JSC-16, JSC-17, JSC-25, JSC-26, JSC-27, JSC-30, JSC-38.
- **Already covered by current architecture or only useful as review advice:** JSC-02, JSC-09, JSC-10, JSC-11, JSC-18, JSC-19, JSC-20, JSC-21, JSC-22, JSC-23, JSC-24, JSC-28, JSC-29, JSC-34, JSC-35.
- **Rejected as parallel product surface:** JSC-36, JSC-37.

If one of those rows becomes backed by a concrete product requirement or failing regression, it should get a new ADR or project-plan slice with an owner-specific design. It should not be revived as a generic duplicate-detection initiative.

## Implementation Notes

Preferred first slice:

1. Add one apply-patch regression fixture for duplicated failure output, if current behavior demonstrates it.
2. Add one prompt duplication check scoped only to `base_instructions/default.md`.
3. Add one golden fixture around an already model-visible generated report.

Stop conditions:

- The change requires a new MCP service, REST API, report registry, or scanner daemon.
- The change adds production code to `session/turn.rs`, `tools/spec_plan.rs`, or hook discovery without a failing owner-local test.
- The change injects unbounded duplicate-scan output into model context.

## Consequences

This keeps jscpd as an idea donor for generated-code/text quality, not as a subsystem donor. The accepted value is narrower but more durable: fewer duplicated diagnostics, stable generated output shapes, and better parser/prompt regression coverage inside existing Ontocode owners.
