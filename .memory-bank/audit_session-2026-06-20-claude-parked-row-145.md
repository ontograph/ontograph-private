# Claude Parked Row 145 Review

Date: 2026-06-20

## Decision

Row 145 stays parked.

## Source

- ADR row 145: `Partial / Non-core / NARROW / Eval command can live in tooling first.`
- Donor row 145: `Add /fast mode as config overlay. | model-provider / config | Easier low-latency mode. | Config overlay test.`

## Evidence

- Requested sub-agent model `gemini-3-flash` was unavailable in the current tool surface; `gpt-5.4-mini` was used as the available fallback and recorded in the tracker.
- Existing config owners already cover `service_tier`, Fast Mode feature gating, canonical `fast` config writes, default/legacy fast loading, and arbitrary service-tier preservation.
- Existing TUI owners already expose model service tiers as catalog-driven slash commands, including `/fast`, and route them through the same `OverrideTurnContext` and `PersistServiceTierSelection` events as the Fast Mode keybinding.
- Existing app-server/session plumbing forwards service-tier overrides, and existing status tests render Fast Mode state.
- No exactly-one existing-owner model-provider/config test or doc gap was found for NARROW promotion.

## Outcome

No implementation dispatch. Adding a separate `/fast` mode would duplicate the current service-tier config and slash-command surface rather than extend core functionality. No Rust tests were run because no product code changed.
