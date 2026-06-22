# Lefties: Gemini CLI Tool Extension Review

## Status

Deferred

## Date

2026-06-07

## Source

Moved from `.memory-bank/ADR_GEMINI_CLI_TOOL_EXTENSIONS.md` during GitNexus-backed review/challenge.

## Reason

These original Gemini CLI inspired points are useful references but do not naturally extend Ontocode's current core architecture. They are UI polish, public GitHub automation, marketplace/release packaging, dashboard/export, enterprise/admin, or autonomous memory behavior.

They should not be implemented under the Gemini CLI interop ADR.

## Moved Original Points

| Original points | Reason |
|---|---|
| 51-52 | Theme and terminal setup are UI polish, not Gemini CLI interop core. |
| 73 | Autonomous memory mutation needs a separate safety ADR. |
| 94 | Manual trigger syntax is UI/interaction design, not core interop. |
| 140 | Shell UI polish is not core provider/import work. |
| 168-169, 172 | Docs/editor/IDE commands are not core interop. |
| 221-224, 235, 239 | Public GitHub automation and workflow changes require CI/security governance. |
| 255-256 | Stats export/dashboard surfaces are not core. |
| 266, 269-274 | VS Code UI, terminal setup, clipboard, theme, and vim-mode behavior are UI polish. |
| 331 | Extension marketplace/release work is not core. |
| 361-380 | Packaging, release channels, enterprise controls, and admin docs require separate product/release ADRs. |

## Reconsideration Criteria

Reconsider only if a future ADR defines:

- product ownership
- user-facing compatibility expectations
- security and privacy review
- release or marketplace governance
- test and rollout plan
