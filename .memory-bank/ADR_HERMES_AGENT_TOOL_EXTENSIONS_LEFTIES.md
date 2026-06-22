# Lefties: Hermes Agent Tool Extension Review

## Status

Non-core / product / delegated-later ideas removed from `ADR_HERMES_AGENT_TOOL_EXTENSIONS.md`.

## Date

2026-06-07

## Why These Are Lefties

These items are inspired by Hermes Agent but do not naturally extend Ontocode core in the current architecture. They are product integrations, broad platform support, media features, UI/packaging work, training-data workflows, or untrusted runtime/plugin execution surfaces. They require separate product or architecture decisions before implementation.

## Moved Items

| Original points | Reason |
|---|---|
| 150-160 | Broad messaging gateways such as Telegram, Discord, Slack, WhatsApp, Signal, Matrix, SMS, and Home Assistant are product integrations, not core runtime. |
| 169 | Natural-language cron wizard is UX/product scope; only dry-run schedule detection remains in the Hermes ADR. |
| 204-205 | Modal and Daytona serverless/hibernating runtime backends are cloud product/runtime commitments, not shell-policy hardening. |
| 217-218 | Bundled Git Bash and Termux compatibility are packaging/platform support decisions, not Hermes interop. |
| 237 | Managed browser/tool gateway is a product integration and should not bypass current tool/provider owners. |
| 241-260 | Image generation, video generation, TTS, transcription, voice mode, computer-use, and accessibility-driven desktop control are media/product/security surfaces outside the current core extension plan. |
| 262-265 | Third-party memory backends and user-modeling providers need a dedicated memory/privacy ADR before runtime support. |
| 278 | User/personality modeling beyond explicit memory requirements is not a core migration feature. |
| 281-300 | Trajectory export, training-data generation, batch runner workflows, and HuggingFace dataset compatibility are eval/training features, not core interop. |
| 315 | Dashboard analytics is product UI scope. |
| 329 | Plugin dashboard slots are product UI scope. |
| 337 | Plugin marketplace/distribution policy is product/ecosystem scope. |
| 341-360 | ACP parity, rich TUI features, desktop bootstrap, dashboard pages, OAuth modal UX, theme/polish, and config drawers need app-server/TUI/product ADRs before implementation. |

## Reconsideration Rule

A lefties item can move back only with:

- a named Ontocode owner,
- GitNexus context and impact evidence,
- an ADR proving it extends core functionality rather than duplicating product architecture,
- security/redaction tests for credential or user-content surfaces,
- compatibility tests for any public API/config/schema/UI behavior.
