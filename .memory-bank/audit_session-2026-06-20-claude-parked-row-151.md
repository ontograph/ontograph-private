# Claude Parked Row 151 Review

Date: 2026-06-20

## Decision

Row 151 stays parked.

## Source

- ADR row 151: `Partial / Non-core / NARROW / Skill command polish belongs in skill manager/UI.`
- Donor row 151: `Add plugin marketplace command. | core-plugins / app-server | Manages plugins without manual files. | Plugin command test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing TUI owners already cover `/plugins`, plugin list/detail UI, marketplace add, marketplace remove, and marketplace upgrade flows.
- Existing app-server protocol and processors already expose plugin and marketplace RPCs including `plugin/list`, `plugin/install`, `plugin/uninstall`, `marketplace/add`, `marketplace/remove`, and `marketplace/upgrade`.
- Existing app-server tests cover plugin list, install, uninstall, marketplace add/remove/upgrade, and remote/curated marketplace cases.
- Existing TUI plugin popup interaction and snapshot tests cover marketplace browse/add/remove/detail flows.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
