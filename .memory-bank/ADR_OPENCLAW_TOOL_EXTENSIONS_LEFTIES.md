# Lefties: OpenClaw Tool Extensions

## Status

Deferred

## Date

2026-06-07

## Source

Moved from [ADR_OPENCLAW_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_OPENCLAW_TOOL_EXTENSIONS.md:1) during GitNexus challenge.

## Reason

These items may be useful product integrations or optional runtime systems, but they do not naturally extend Ontocode core provider, auth, MCP, plugin, shell, sandbox, context, diagnostics, memory-bank, or external-agent migration architecture.

They should not be implemented from the OpenClaw ADR. Reconsider only through a dedicated ADR with an explicit owner, user-facing value, compatibility impact, security model, and tests.

## Moved Items

| Original points | Items | Reason |
|---|---|---|
| 001-020 | Gateway runtime, WebSocket control plane, remote role/scope system, plugin surface hosting, heartbeat/reconnect behavior, Tailscale/trusted-proxy gateway auth | Duplicates app-server/network ownership and would create a second gateway control plane. |
| 081-100 | Messaging channel transports, DM pairing execution, allowlist mutation, delivery state, bot presence, typing, channel commands | Product/channel framework scope, not core external-agent interop. |
| 101-120 | Multi-agent routing, agent runtime profiles, session isolation engine, task-agent binding, task status ownership | Requires session/task architecture decisions outside OpenClaw detection. |
| 121-140 | Browser runtime parity, canvas/A2UI surfaces, tool registry, screenshot/PDF/browser action execution | Duplicates or bypasses existing browser/tool owners and introduces product UI/runtime scope. |
| 141-160 | Docker/SSH/OpenShell sandbox backends, alternate shell launchers, permission prompts, runtime cleanup engine | Duplicates shell/sandbox/policy ownership and has high security risk. |
| 161-180 | OpenClaw plugin activation, setup, runtime loading, onboarding, hook-only plugin behavior, marketplace-compatible loading | Duplicates core plugin loading and would execute untrusted plugin code. |
| 181-200 | MCP transport managers, bundle installs, MCP registry behavior, tool/resource registration, OAuth bridge execution | Duplicates existing MCP owners and requires separate auth/security review. |
| 201-220 | Cron scheduler, scheduled task execution, job history import, watchdogs, run reconciliation | No approved Ontocode cron engine; personal automation is product scope. |
| 221-240 | Voice wake/talk mode, realtime voice runtime, media generation, transcript ingestion, node pairing execution, remote node commands | Optional consumer media/device integrations with privacy and security risk. |
| 261-280 | Live remote access, Tailscale/SSH tunnel management, trusted-proxy auth, replay protection service, remote command execution, remote audit system | Remote-control platform scope, not core interop detection. |
| 281-300 | Migration dashboards, TUI panels, app-server scan/import APIs, control-surface permissions, companion-app parity | Requires separate app-server/TUI/product ADR and compatibility tests. |
| 301-320 | Memory database import, transcript import, usage archive import, personal memory search, retention/budget policies | Risks unbounded context and private data import; existing context rules do not allow it. |
| 361-380 | Daemon/service install, launch agents, companion app packaging, update checks, install script auditing as runtime behavior, distribution management | Release/deployment/product scope; not a natural core extension. |

## Reconsideration Criteria

Reconsider a moved item only if a later ADR defines:

- a concrete Ontocode owner
- user-facing behavior that cannot be achieved through existing surfaces
- compatibility and migration impact
- explicit security and privacy model
- integration or snapshot tests for UI/API behavior
- redaction and policy tests for any diagnostic, credential, channel, remote, browser, media, or network output
