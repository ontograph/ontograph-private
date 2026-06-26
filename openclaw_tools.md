# OpenClaw Tools Review

## Verdict

Keep only one core-extension slice: a read-only OpenClaw interop detector/report that can identify OpenClaw config, plugin manifest, provider/model, MCP, skill, sandbox, browser/canvas, cron, node, and package metadata without importing or executing anything.

All other listed OpenClaw tools are rejected as direct tool additions. They either duplicate current Ontocode core functionality, belong to an existing owner/ADR, or introduce product/runtime surfaces outside the current core architecture.

OntoIndex check: index is fresh at `8b61fd0dbfa32aa9e4a00ef15930e5b4fcb9119f` with a dirty worktree. Exploration found current owners for external-agent import in `ExternalAgentConfigRequestProcessor.import` / `ExternalAgentConfigService.import`, tool exposure in `core/src/tools/spec_plan.rs`, hosted web/image tools in `hosted_model_tool_specs`, goal/collaboration tools in `add_core_utility_tools` / `add_collaboration_tools`, and provider descriptors in `model-provider/src/descriptor.rs`.

## Keep

| Candidate | Decision | Existing owner | Scope allowed |
| --- | --- | --- | --- |
| OpenClaw interop detector/report | KEEP | `scripts/onto_openclaw_interop.py` per `.memory-bank/ADR_OPENCLAW_TOOL_EXTENSIONS.md` | Read-only detection of OpenClaw metadata. No credential import, no config mutation, no plugin execution, no gateway, no scheduler, no media/runtime tools. |
| OpenClaw interop report test | KEEP | `scripts/tests/test_onto_openclaw_interop.py` | Fixed JSON shape, deterministic ordering, invalid JSON/root handling, no-execution proof, and redaction assertions. |

## Reject Or Delegate

| Donor tool | Decision | Reason |
| --- | --- | --- |
| `createSessionsHistoryTool` | REJECT | Session history/transcripts are existing session/state surfaces; importing OpenClaw history would add a second transcript/memory path. |
| `createSessionsSendTool` | REJECT | Sending into sessions duplicates existing thread/session operations and would need app-server/API compatibility review. |
| `createTranscriptsTool` | REJECT | Transcript access risks unbounded model context and private content exposure; existing context-fragment rules require bounded fragments. |
| `createSessionsYieldTool` | REJECT | Yield/control semantics duplicate current agent/session orchestration without a proven core gap. |
| `createGatewayTool` | REJECT | Gateway/control-plane runtime is explicitly out of scope; current app-server and network proxy owners must not be bypassed. |
| `createGetGoalTool` | REJECT | Goal tools already exist in core utility tool planning; no OpenClaw-specific extension needed. |
| `createCreateGoalTool` | REJECT | Duplicate of existing goal tool surface. |
| `createUpdateGoalTool` | REJECT | Duplicate of existing goal tool surface. |
| `createUpdatePlanTool` | REJECT | Planning/status updates already exist; no new OpenClaw plan tool should be added. |
| `createNodesTool` | REJECT | Node pairing/companion runtime is product integration scope. Stage 0 may only report redacted node metadata. |
| `createPdfTool` | REJECT | PDF processing is a new media/document runtime surface, not an OpenClaw core extension. |
| `createVideoGenerateTool` | REJECT | Video generation is consumer media tooling outside current core. |
| `createSessionStatusTool` | REJECT | Session status is existing TUI/app-server/session owner territory. |
| `createSessionsListTool` | REJECT | Session listing is existing state/app-server territory; no donor-specific tool. |
| `createTtsTool` | REJECT | TTS is media/runtime scope outside current core. |
| `createSubagentsTool` | REJECT | Sub-agent spawning/status already has core collaboration owners and feature gates. |
| `createHeartbeatResponseTool` | REJECT | Heartbeat/control-plane response belongs to OpenClaw gateway runtime, not Ontocode core. |
| `createImageTool` | REJECT | Image viewing/editing/generation already has hosted/core tool owners; no OpenClaw-specific image tool. |
| `createSkillWorkshopTool` | REJECT | Skill/plugin creation belongs to the plugin/skill system and plugin tooling, not a new core OpenClaw workshop tool. |
| `createAgentsListTool` | REJECT | Agent listing overlaps existing sub-agent/job/status surfaces. |
| `createMusicGenerateTool` | REJECT | Music generation is consumer media tooling outside current core. |
| `createImageGenerateTool` | REJECT | Hosted image generation already exists in `hosted_model_tool_specs`; no duplicate tool. |
| `createMessageTool` | REJECT | Generic messaging/channel tooling is OpenClaw product integration scope. Stage 0 may only report redacted channel metadata. |
| `createWebSearchTool` | REJECT | Hosted web search already exists in `hosted_model_tool_specs`; no duplicate tool. |
| `createSessionsSpawnTool` | REJECT | Session spawning overlaps existing thread/session/sub-agent owners and would need public API review. |
| `createWebFetchTool` | REJECT | Web fetch/search belongs to existing hosted/dynamic tool owners or extension tools; no OpenClaw-specific core tool. |
| `createCronTool` | REJECT | Cron scheduling is not accepted as core. Stage 0 may only report cron definitions as metadata. |

## Accepted Follow-Up Contract

Any implementation must stay inside the existing OpenClaw ADR contract:

- edit only `scripts/onto_openclaw_interop.py`, with optional tests in `scripts/tests/test_onto_openclaw_interop.py`;
- use Python standard library only;
- read only the supplied `--root`;
- print no file contents, credentials, tokens, cookies, authorization headers, account IDs, pairing codes, channel identities, transcript text, browser data, media content, or raw memory/state content;
- do not write project config, user config, credentials, plugin folders, browser profiles, state stores, or the OpenClaw root;
- do not execute commands, plugins, MCP servers, cron jobs, gateway connections, browser processes, or node pairing flows;
- do not add Rust code, app-server APIs, config schema keys, feature flags, or model-visible tools.

Command shape retained from the ADR:

```bash
python3 scripts/onto_openclaw_interop.py detect --root <path>
```

Required output remains a deterministic redacted JSON report with `readiness: "dry_run_only"`.
