# ADR: OpenClaw-Inspired Tool Extensions Review

## Status

Challenged - OpenClaw Interop Stage 0 Only

## Date

2026-06-07

## Context

OpenClaw is a TypeScript local-first assistant platform with a gateway control plane, plugin manifests, provider catalogs, channel pairing, sandboxed execution, browser/canvas tools, skills, cron jobs, companion nodes, and diagnostics.

The first version of this ADR stored 400 candidate tools inspired by OpenClaw. After GitNexus and prior-ADR review, the catalog is not accepted as an implementation backlog. Most candidates duplicate current Ontocode owners, extend prior provider/auth/adapter/project-tooling ADRs, or are product/runtime systems that are not natural core extensions.

This ADR now owns only OpenClaw external-agent interop detection: finding OpenClaw configuration, plugin manifests, and state references and producing a redacted dry-run report. Provider, auth, plugin, MCP, channel, gateway, sandbox, browser, cron, node, diagnostics, and project-management ideas are delegated or moved to lefties.

## OpenClaw Source Evidence

Reviewed upstream at commit `66b91d78feb33d62e2f82ae1d8689c48519f5530`.

- Repository and README: <https://github.com/openclaw/openclaw/tree/66b91d78feb33d62e2f82ae1d8689c48519f5530>
- Architecture and gateway overview: <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/concepts/architecture.md>
- Gateway protocol: <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/gateway/protocol.md>
- Gateway configuration: <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/gateway/configuration.md>
- Sandboxing: <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/gateway/sandboxing.md>
- Channel and device pairing: <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/channels/pairing.md>
- Plugin architecture and manifests: <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/plugins/architecture.md>, <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/plugins/manifest.md>
- Browser and skills tools: <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/tools/browser.md>, <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/tools/skills.md>
- Cron jobs and model failover: <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/automation/cron-jobs.md>, <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/docs/concepts/model-failover.md>
- Plugin manifest metadata implementation: <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/src/plugins/manifest.ts>
- Terminal outcome mapping: <https://github.com/openclaw/openclaw/blob/66b91d78feb33d62e2f82ae1d8689c48519f5530/src/agents/agent-run-terminal-outcome.ts>

## GitNexus Challenge Evidence

GitNexus found existing Ontocode owners for the major OpenClaw-inspired surfaces:

- External-agent detection/import already has owners in [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:260), [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/external_agent_config_processor.rs:391), and [lib.rs](/opt/demodb/_workfolder/ontocode/codex-rs/external-agent-migration/src/lib.rs:1).
- Plugin manifests, skills, MCP server declarations, apps, and hooks already flow through core plugin loading: [loader.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core-plugins/src/loader.rs:555), [manifest.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core-plugins/src/manifest.rs:1), [load_outcome.rs](/opt/demodb/_workfolder/ontocode/codex-rs/plugin/src/load_outcome.rs:36).
- Provider descriptors and runtime selection already have owners: [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:7), [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:108).
- MCP OAuth and credential boundaries already have owners: [oauth.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:58), [manager.rs](/opt/demodb/_workfolder/ontocode/codex-rs/login/src/auth/manager.rs:732).
- App-server APIs and feature gates already have owners: [v2.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server-protocol/src/protocol/v2.rs:1), [features/src/tests.rs](/opt/demodb/_workfolder/ontocode/codex-rs/features/src/tests.rs:104).
- Browser controls already have a feature owner and must not be replaced by OpenClaw browser runtime ideas: [features/src/tests.rs](/opt/demodb/_workfolder/ontocode/codex-rs/features/src/tests.rs:204).
- Doctor, feedback, support, and redaction diagnostics already have owners: [output.rs](/opt/demodb/_workfolder/ontocode/codex-rs/cli/src/doctor/output.rs:1298), [feedback_doctor_report.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/feedback_doctor_report.rs:36).
- Network proxy behavior is separate and already owned: [network-proxy README](/opt/demodb/_workfolder/ontocode/codex-rs/network-proxy/README.md:1).
- Context injection must use bounded fragments, not OpenClaw transcript or memory import: [fragment.rs](/opt/demodb/_workfolder/ontocode/codex-rs/context-fragments/src/fragment.rs:46), [contextual_user_message.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/context/contextual_user_message.rs:46).

Challenge result:

- Do not implement the original 400 items as independent tools.
- Do not add another gateway, plugin runtime, provider registry, auth store, MCP manager, browser runtime, channel framework, cron scheduler, node pairing system, sandbox runtime, diagnostics framework, app-server control API, or memory/transcript importer from this ADR.
- Keep this ADR limited to a redacted OpenClaw interop detector and dry-run report.
- Delegate generic provider, auth, adapter, plugin, MCP, diagnostics, and project-tooling work to existing owners and prior ADRs.

## Prior ADR Delegation

- Provider catalog, model capability, model fallback, media capability, and provider status ideas go to [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:1).
- External-agent config, auth profile evidence, credential import, MCP OAuth readiness, and redacted migration report ideas go to [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1).
- Executable provider plugin/runtime adapter ideas go to [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md:1) only if they fit the approved adapter boundary.
- ADR tracking, source-link checking, GitNexus gates, challenge matrices, redaction templates, and low-agent task cards go to [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1).
- Gateway runtime, channel bots, daemon management, live remote control, companion apps, consumer media automation, browser/canvas runtime parity, cron scheduling, and node pairing move to [ADR_OPENCLAW_TOOL_EXTENSIONS_LEFTIES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_OPENCLAW_TOOL_EXTENSIONS_LEFTIES.md:1).

## Original Point Disposition

This table covers every original point from `001` through `400`.

| Original points | Disposition | Similar solution in core / ADR | Architecture decision |
|---|---|---|---|
| 001-020 | Moved to lefties, with metadata detection only | app-server transport and network-proxy owners | Do not implement an OpenClaw gateway, WebSocket control plane, remote role/scope system, or plugin surface host. Stage 0 may report gateway config/protocol field presence only. |
| 021-040 | Kept only as OpenClaw interop detection inputs | [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:260), external-agent migration ADRs | Stage 0 may detect `~/.openclaw/openclaw.json`, config schema hints, plugin roots, skill roots, and redacted path metadata. No config import or mutation. |
| 041-060 | Removed from current; delegated | [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:1), [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:7) | Provider catalog and model capability data must extend model-provider descriptors; no second provider catalog/registry. Stage 0 may report provider/model names only. |
| 061-080 | Removed from current; delegated | [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1), [oauth.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:58) | Auth profile and secret ideas remain evidence-only. No credential import, validation, refresh, keychain read, or auth-store mutation. |
| 081-100 | Moved to lefties, with redacted metadata only | app-server permission/policy owners | Messaging channels, DM pairing, allowlists, delivery state, and bot presence are product integrations. Stage 0 may report channel names, policy names, and counts with identity redaction. |
| 101-120 | Removed from current; delegated or lefties | session/context rules and bounded fragments | Multi-agent routing, session isolation, task binding, and context injection need existing session/context owners. Stage 0 may report agent profile names/counts only. |
| 121-140 | Moved to lefties, with metadata detection only | browser-controls feature flag, existing tool owners | Browser/canvas runtime parity and tool registries are not accepted. Stage 0 may report dedicated profile/canvas/tool field presence only. |
| 141-160 | Removed from current; delegated or lefties | shell, sandbox, exec-policy owners | Sandbox modes and conflict detection may inform diagnostics; Docker/SSH/OpenShell runtimes and alternate shell launchers are not accepted. |
| 161-180 | Removed from current; delegated | core plugin loader and manifest owners | Plugin/skill manifest scanning must reuse current plugin metadata paths. No OpenClaw plugin execution, setup, activation, or new plugin runtime. |
| 181-200 | Removed from current; delegated | rmcp-client, codex-mcp, core plugin MCP loaders | MCP config references may be reported, but transport managers, bundle installs, and MCP registry duplication are blocked. |
| 201-220 | Moved to lefties, with metadata detection only | task/session owners, no approved cron engine | Cron definitions and task metadata may be detected, but scheduling, run history import, watchdogs, and job execution are not accepted. |
| 221-240 | Moved to lefties, with redacted metadata only | provider capability ADRs and auth/redaction owners | Voice/media/node features are optional product/runtime integrations. Stage 0 may report media capability names and redacted node pairing fields only. |
| 241-260 | Removed from current; delegated | doctor/support/redaction owners | Diagnostics are useful only as acceptance criteria for the Stage 0 report and owner-specific future work; no new diagnostics framework. |
| 261-280 | Moved to lefties, with metadata detection only | network-proxy and app-server security owners | Remote access, Tailscale, SSH tunnels, trusted proxy auth, remote commands, and device trust are not accepted. Stage 0 may report remote config field presence with token redaction. |
| 281-300 | Moved to lefties or delegated to app-server/TUI ADRs | app-server v2 and TUI status owners | Migration UI/API previews need separate app-server/TUI ADRs. This ADR adds no public API, schema, dashboard, or panel. |
| 301-320 | Moved to lefties or delegated | bounded context, telemetry, support-bundle owners | Memory, transcript, state DB, and usage import are not accepted. Stage 0 may report store names/counts only, never content. |
| 321-340 | Removed from current; delegated | [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md:1), native provider ADR | Executable provider adapters must use the approved adapter boundary if ever accepted. This ADR only records manifest/provider metadata. |
| 341-360 | Removed from current; delegated as test criteria | external-agent migration tests, diagnostics tests | Conformance suites become acceptance criteria for the Stage 0 detector only. Broader tests belong to the owning feature ADR. |
| 361-380 | Moved to lefties, with package metadata detection only | packaging/release owners | Daemons, launch agents, installers, update checks, companion apps, and distribution management are not core. Stage 0 may report package/install metadata without running anything. |
| 381-400 | Removed from current; delegated | [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1), GitNexus rules | ADR challenge, source-link checking, lefties export, task-card generation, and readiness reports are project-tooling ideas, not OpenClaw runtime tools. |

## Active OpenClaw Interop Stage 0

Only these labels remain active in this ADR:

| Label | Derived original points | Status | Scope |
|---|---|---|---|
| `onto_openclaw_interop_detector` | 001-220, 221-240 metadata only, 241-260 report shape, 261-380 metadata only, 381-400 readiness field | Approved implementation | One read-only detector/report command covering config files, plugin manifests, provider/model names, channel policy names, redacted auth references, MCP references, skill roots, sandbox mode names, browser/canvas field presence, cron definitions, node pairing field presence, package metadata, and `readiness: "dry_run_only"`. |
| `onto_openclaw_interop_report_test` | 341-360, 399 | Approved test | Test fixed report shape, redaction, invalid JSON handling, invalid root behavior, deterministic ordering, and no executable import. |

Active implementation labels: 1.

Active test labels: 1.

## Lowest-Agent Stage 0 Contract

Implementation, if dispatched later, must be small enough for a low-capability coding agent:

- Edit at most `scripts/onto_openclaw_interop.py`.
- Optional tests may be added only in `scripts/tests/test_onto_openclaw_interop.py`.
- Use Python standard library only.
- Read only the target root passed by `--root`.
- Do not write to the target root, `~/.openclaw`, project config, credentials, plugin folders, browser profiles, or state stores.
- Do not modify Rust code.
- Do not add app-server APIs, config schema keys, feature flags, or model-visible tools.
- Do not call network services.
- Do not execute commands, scripts, plugins, daemons, gateway connections, browser processes, cron jobs, node pairing flows, or MCP servers discovered in OpenClaw files.
- Do not parse or print credential values, pairing codes, tokens, cookies, authorization headers, account IDs, channel identities, transcript contents, browser data, media contents, or raw memory/state contents.

Command:

```bash
python3 scripts/onto_openclaw_interop.py detect --root <path>
```

Required JSON shape:

```json
{
  "root": "<absolute path>",
  "config_files": [{"path": "<path>", "bytes": 123}],
  "plugin_manifests": [{"path": "<path>", "bytes": 123}],
  "provider_metadata": [{"name": "<provider-or-null>", "models": 0}],
  "channel_metadata": [{"name": "<channel-or-null>", "policy": "<policy-or-null>", "redacted": true}],
  "auth_references": [{"path": "<path>", "field": "<name-or-null>", "bytes": 123, "redacted": true}],
  "mcp_references": [{"path": "<path>", "count": 0}],
  "skill_roots": [{"path": "<path>", "bytes": 123}],
  "sandbox_metadata": [{"mode": "<mode-or-null>", "scope": "<scope-or-null>"}],
  "browser_metadata": [{"profile": "<profile-or-null>", "redacted": true}],
  "cron_metadata": [{"path": "<path>", "count": 0}],
  "node_metadata": [{"path": "<path>", "field": "<name-or-null>", "redacted": true}],
  "package_metadata": [{"path": "<path>", "bytes": 123}],
  "summary": {
    "config_files": 0,
    "plugin_manifests": 0,
    "provider_entries": 0,
    "channel_entries": 0,
    "auth_references": 0,
    "mcp_references": 0,
    "skill_roots": 0,
    "cron_entries": 0,
    "node_references": 0
  },
  "readiness": "dry_run_only"
}
```

Detection rules:

- Config candidates: `openclaw.json`, `.openclaw/openclaw.json`, `openclaw.config.json`, `openclaw.config.json5`, and files under `.openclaw/` ending in `.json` or `.json5`.
- Plugin manifest candidates: files named `openclaw.plugin.json`.
- Auth candidates: paths or field names containing `auth`, `oauth`, `token`, `credential`, `secret`, `cookie`, `api_key`, `keychain`, `pairing`, or `bootstrap`.
- Provider candidates: manifest/config fields named `providers`, `models`, `modelCatalog`, `modelSupport`, `providerEndpoints`, or `providerRequest`.
- Channel candidates: fields named `channels`, `dmPolicy`, `allowFrom`, `pairing`, or known channel names.
- Runtime candidates: fields named `gateway`, `sandbox`, `browser`, `canvas`, `cron`, `nodes`, `mcp`, `skills`, `plugins`, or `remote`.
- JSON/JSON5 parse failure must not fail the command; include path-only error metadata and continue.
- Missing or invalid `--root` exits non-zero with a clear stderr message.

Redaction rules:

- Never print file contents.
- Never print environment variable values.
- Never print token/key/cookie/account/pairing-code substrings.
- Never print phone numbers, handles, room IDs, channel user IDs, transcript text, browser profile paths, media paths, or credential-store paths.
- Sensitive findings must include only path, field name, byte count, count, and `redacted: true`.

## Follow-Up Requirements

Before any removed/delegated point can be implemented:

- Add a task card to the owning ADR.
- Link the existing source owner and prior ADR delegation.
- Run GitNexus context and impact before code-symbol edits.
- Add redaction, bounded-output, compatibility, and no-execution tests.
- Move the item back out of lefties only through a dedicated ADR with user-facing value, compatibility impact, and owner acceptance.
