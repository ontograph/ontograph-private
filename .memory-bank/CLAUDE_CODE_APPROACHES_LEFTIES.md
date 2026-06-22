# Claude Code Repository Review: Lefties

Source file pruned: `CLAUDE_CODE_APPROACHES_FOR_CODEBASE.md`

This file contains approaches moved out of the core backlog because they do not naturally extend existing Codex core functionality. The retained IDs are listed in `CLAUDE_CODE_APPROACHES_FOR_CODEBASE.md`; every original ID not retained there is considered a lefty here.

## Split Rule

- Keep only work that directly strengthens provider/runtime correctness, auth safety, MCP reliability, hook governance, sandbox permissions, shell execution, session/context safety, external-agent import, diagnostics, or test harnesses.
- Move anything that primarily creates a product surface, marketplace flow, workflow command, enterprise deployment artifact, SDK compatibility promise, background-daemon product, or cosmetic UI polish.
- Reopen a lefty only when an ADR defines the owning surface and the work has a concrete core bug, security gap, or compatibility requirement.

## Moved Coverage

| Original IDs | Category | Why Moved | Reopen Trigger |
| --- | --- | --- | --- |
| 10, 15 | Provider public behavior/config extras | Provider aliases and public config behavior need compatibility evidence before implementation. | Reopen under a provider config ADR with migration and precedence tests. |
| 31-45 | Plugin and marketplace functionality | Plugin install UX, marketplace freshness, plugin validation, rollback, scaffolding, and plugin settings are product/marketplace work, not provider/runtime core. | Reopen after a plugin roadmap or plugin schema ADR exists. |
| 46, 52, 57, 60 | Hook product wrappers and policy packs | Generators, advisory packs, and broad policy bundles are higher-level UX; the core backlog keeps only hook engine validation/enforcement. | Reopen after hook policy packaging is approved separately. |
| 64-67, 69, 71-75 | Managed policy, MDM, deployment, and CI governance | These are enterprise/deployment artifacts or repository workflow policies, not core runtime behavior. | Reopen after product/security defines managed-policy and deployment requirements. |
| 76-86, 88, 90 | Background session daemon/product work | Pinning, respawn, background resume, daemon clocks, daemon attach/log APIs, and dashboard fields require a background-agent ADR. | Reopen under a background-agent daemon ADR. |
| 91-99, 101 | Review, PR, commit, release-note, and git workflow automation | These are optional workflow agents/commands, not core execution or provider functionality. | Reopen as plugins, local commands, or review-agent ADR work. |
| 102-103, 105-108, 112-116, 118, 122-123, 125-129, 131, 133-139 | TUI and input polish outside diagnostics/safety | Rendering polish, hints, menu spacing, mouse behavior, theme editing, image UI, and status-line product details do not naturally extend core. | Reopen only when the adjacent TUI component is already being modified for a core bug. |
| 147-148, 152-154, 158-159 | MCP catalog, import, managed connector, and telemetry product UX | These add catalog UX, config import/merge behavior, managed connector controls, or analytics beyond core MCP correctness. | Reopen under an MCP product/API ADR. |
| 162-163, 169-171, 173, 176-180 | Shell tooling, network policy, and broad hardening extras | These are packaging, onboarding, advisory policy, or broad stress-test efforts unless tied to a concrete permission/sandbox bug. | Reopen when a shell permission regression or platform compatibility issue requires them. |
| 182-184, 186-200 | Memory, session, status-line, and SDK/headless public API extras | Memory workflows, status JSON additions, replay flags, stream-json modes, partial streaming, SDK cancellation semantics, and SDK message contracts are public API/product surfaces. | Reopen under an SDK/headless compatibility plan or memory subsystem ADR. |
| 201-212 | Enterprise auth, managed settings, and marketplace governance | Forced login methods, org/API-key restrictions, plist/registry precedence, MDM examples, hot reload, XDG, symlink behavior, and strict marketplace policy are enterprise/product decisions. | Reopen after enterprise policy requirements and config compatibility rules are approved. |
| 216 | External-agent public detect API | Candidate detection is useful, but an app-server endpoint creates public API surface beyond import internals. | Reopen under an app-server API ADR with schema and compatibility tests. |

## Notable Explicit Exclusions

| Original ID | Topic | Challenge |
| --- | --- | --- |
| 10 | Provider-specific model alias environment variables | Environment variables create a compatibility surface; start with provider provenance/status first. |
| 31 | Plugin component summaries before install | Useful marketplace UX, but it does not prove core runtime correctness. |
| 38 | Marketplace policy controls | Looks security-related, but belongs to plugin/enterprise governance, not immediate core. |
| 65 | Managed setting to disable bypass mode | High-impact policy behavior; needs security/product ownership before code. |
| 76 | Background session pinning | Requires background-agent lifecycle semantics before isolated implementation. |
| 91 | Feature-dev workflow | Should be a plugin or local command, not a core command. |
| 100+ UI polish not retained | General TUI polish | Keep only diagnostics, permission safety, startup, session, and context rendering tests in core. |
| 153 | One-off MCP server config import | Public config import behavior needs an MCP config ADR. |
| 197 | Stream-json output mode | Headless protocol surface; must be treated as SDK/API compatibility work. |
| 200 | SDK cancellation tests | Cancellation behavior is valid, but public SDK semantics need an SDK compatibility plan. |
| 201 | Forced login method/org setting | Enterprise auth policy, not a provider-runtime prerequisite. |
| 216 | External-agent detect API | Public app-server method; keep internal detection/dry-run only until API ADR exists. |

## Advice

- Do not pull lefties back into `codex-core` just because implementation is convenient.
- Convert a lefty into core only when it has a failing test tied to an existing core surface.
- Prefer plugins/local commands for workflow automation and marketplace-oriented capabilities.
- Prefer ADRs before any moved item that changes config schema, app-server API, SDK behavior, managed policy, or enterprise deployment.
