# ADR: External-Agent Interop Detector Consolidation

## Status

Accepted for planning - consolidates Gemini CLI, Hermes Agent, and GBrain interop ADRs into one Stage 0 detector contract.

## Date

2026-06-08

## Context

The following ADRs independently converged on the same architecture decision:

- [Gemini CLI Tool Extensions](ADR_GEMINI_CLI_TOOL_EXTENSIONS.md) retained only read-only Gemini CLI artifact detection and a redacted dry-run report.
- [Hermes Agent Tool Extensions](ADR_HERMES_AGENT_TOOL_EXTENSIONS.md) retained only read-only Hermes artifact detection and a redacted dry-run report.
- [GBrain Tool Extensions](ADR_GBRAIN_TOOL_EXTENSIONS.md) retained only read-only GBrain artifact detection and a redacted dry-run report.

All three rejected the same unsafe implementation patterns:

- no new provider registry
- no new credential store or auth broker
- no new MCP manager
- no new tool registry
- no new shell runtime, scheduler, or background worker
- no new context/memory/search substrate
- no model-visible tools or app-server APIs in Stage 0
- no command/plugin/skill execution from detected files
- no credential import, content import, or runtime mutation

## Decision

Consolidate Gemini CLI, Hermes Agent, and GBrain interop into one external-agent interop detector family. The consolidated detector family may produce only redacted, bounded, read-only reports in Stage 0.

The source ADRs remain historical source-evidence files. This ADR is the dispatch and implementation authority for all retained Stage 0 detector work from those three ADRs.

## Codebase Owner Mapping

| Surface | Existing owner | Source link | Consolidated requirement |
|---|---|---|---|
| External-agent detection | `ExternalAgentConfigService` | [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:104) | Future Rust integration must extend this owner, not create a second detector service. |
| App-server request processing | `ExternalAgentConfigRequestProcessor` | [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/external_agent_config_processor.rs:46) | No new app-server API in Stage 0; later API changes require separate compatibility ADR/tests. |
| External-agent import helpers | `external-agent-migration` crate | [lib.rs](/opt/demodb/_workfolder/ontocode/codex-rs/external-agent-migration/src/lib.rs:111) | Stage 0 must not call import helpers that write hooks, commands, subagents, MCP config, or skills. |
| MCP | `McpConnectionManager` and RMCP owners | [connection_manager.rs](/opt/demodb/_workfolder/ontocode/codex-rs/codex-mcp/src/connection_manager.rs:104) | Detect MCP config presence only; do not mutate MCP config or create a parallel MCP registry. |
| Model context | `ContextualUserFragment` | [fragment.rs](/opt/demodb/_workfolder/ontocode/codex-rs/context-fragments/src/fragment.rs:46) | No Stage 0 context injection; later context must use bounded fragments with hard caps. |
| Provider runtime | `model-provider` descriptors/engines | [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:108) | Provider mapping is out of scope; delegate to provider ADRs. |
| Credentials/OAuth | login/RMCP auth stores and provider auth ADRs | [oauth.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:58) | Report auth artifact presence only; never import or print token values. |
| Operational evidence | consolidated evidence backbone | [ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md](ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md) | Detector output may later be imported as `workflow` or `architecture` evidence only through the unified evidence model. |

## Consolidated Stage 0 Contract

Stage 0 may implement one repository-only detector command or three source-specific subcommands. Both shapes must use the same report envelope.

Preferred command shape:

```bash
python3 scripts/onto_external_agent_interop.py detect --source gemini --root <path>
python3 scripts/onto_external_agent_interop.py detect --source hermes --root <path>
python3 scripts/onto_external_agent_interop.py detect --source gbrain --root <path>
```

Compatibility allowance:

- Existing source-specific Stage 0 scripts may remain as thin wrappers only if they emit the shared envelope.
- Do not implement separate parsers with divergent redaction or report semantics.

Shared top-level report shape:

```json
{
  "schema_version": 1,
  "source": "gemini|hermes|gbrain",
  "root": "<absolute path>",
  "readiness": "dry_run_only",
  "summary": {
    "artifact_count": 0,
    "auth_artifact_count": 0,
    "mcp_artifact_count": 0,
    "unsupported_count": 0,
    "error_count": 0
  },
  "artifacts": [],
  "unsupported": [],
  "errors": []
}
```

Artifact records must be bounded:

```json
{
  "kind": "config|memory|command|skill|extension|mcp|auth|ignore|plugin|session|job|gateway|trajectory|schema_pack|source_registry|operation_registry|audit",
  "path": "<path>",
  "bytes": 123,
  "redacted": true,
  "metadata": {}
}
```

Allowed metadata:

- names, IDs, kinds, counts, booleans, schema versions, size classes, hostnames, transport kinds, schedule strings, and declared env var names
- never file contents, token values, raw URLs with credentials, cookies, authorization headers, DB URLs, private key paths, raw message content, source content, shell output, media content, or trajectory bodies

## Source-Specific Requirements

### Gemini CLI

Retained requirements:

- Detect `.gemini/` directory by path only.
- Detect `GEMINI.md` as `memory` by path and byte size only.
- Detect `.gemini/commands/**` as `command` by path and byte size only.
- Detect `.gemini/skills/**` and `skills/**` as `skill` by path and byte size only.
- Detect `.gemini/extensions/**` and `extensions/**` as `extension` by path and byte size only.
- Detect `.gemini/settings.json`, `.gemini/config.yaml`, and `.gemini/mcp/**` as `mcp` candidates by path and byte size only.
- Detect auth candidate filenames containing `auth`, `oauth`, `token`, `credential`, `key`, or `service-account` as redacted `auth` artifacts.
- Detect `.geminiignore` as `ignore` by path and byte size only.

Blocked:

- importing Gemini commands, skills, extensions, MCP config, auth, context, or memory
- executing Gemini commands
- mapping provider runtime behavior
- changing app-server or model context

### Hermes Agent

Retained requirements:

- Detect Hermes home/profile/repo surfaces.
- Report provider IDs, auth type names, base-url hostnames, model IDs, plugin manifest presence, and secret env var names only.
- Report MCP server IDs, transport kinds, command names, URL hostnames, and OAuth metadata presence only.
- Report skill names, declared tool/env requirements, size class, and helper-script presence only.
- Report plugin names, kinds, manifest paths, and quarantine recommendation.
- Report session DB presence, schema version if safely readable, and row counts only.
- Report cron job count, schedule strings, and target platform type only.
- Report gateway platform kinds and allowlist/pairing config presence only.
- Report trajectory JSONL file count, approximate size, and schema-shape status only.

Blocked:

- plugin execution
- provider/gateway runtime enablement
- session content import
- media/chat gateway integrations
- cron/job execution
- trajectory export or training-data use

### GBrain

Retained requirements:

- Detect GBrain installation/config presence: `~/.gbrain/`, workspace `gbrain.*`, package metadata, and documented config paths.
- Detect MCP connection presence: server name, transport kind, endpoint host only, and operation count.
- Detect schema-pack presence: pack name, version, declared type count, and validation errors.
- Detect skillpack presence: name, version, manifest path, command count, and routing rule count.
- Detect source registry presence: source IDs, source kinds, enabled/disabled state, and file counts.
- Detect operation registry presence: operation IDs, local-only flags, MCP exposure flags, and schema names.
- Detect minion/job/scheduler presence: job IDs, schedule shape, disabled/enabled state, and command count.
- Detect audit/eval/log presence: file counts, last-modified timestamps, and redacted category names.
- Detect storage/backend configuration kind: local, remote, sqlite, postgres, or unknown.
- Detect brain page/timeline/compiled-truth content shape: counts and section names only.

Blocked:

- hybrid search, RRF, reranking, graph traversal, embeddings, query expansion
- memory content import
- schema-pack runtime activation
- source ingestion/enrichment pipeline
- minion execution or job queues
- guardrail runtime, MCP manager, eval framework, provider gateway

## Unified Delegation Rules

| Proposal class | Destination |
|---|---|
| Provider descriptors, provider capabilities, native Gemini/Claude/Copilot behavior | [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md) |
| Provider/auth persistence, OAuth, credential import, provenance, overwrite/delete semantics | [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md) |
| External adapter runtime/plugin transport | [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md) |
| Operational evidence, dependency boundaries, GitNexus/lean-ctx evidence import | [ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md](ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md) |
| Memory-bank validators, task-card generation, readiness gates | [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md) |
| Product gateways, dashboards, media, voice, marketplace, package/release work | Source-specific lefties files |

## Implementation Rules

- Stage 0 implementation must be read-only and standard-library-first.
- Stage 0 may edit at most one repository script and one test file unless a stage card explicitly widens scope.
- Stage 0 must not edit Rust, app-server protocol, public config, SDKs, provider runtime, MCP runtime, context fragments, credential stores, hooks, shell runtime, or memory persistence.
- Stage 0 must not call network services.
- Stage 0 must not execute commands described by detected config files.
- Stage 0 must not write to the target root.
- Stage 0 must preserve stable output ordering for deterministic tests.
- Unreadable files must produce bounded error records without leaking content.
- Malformed JSON/YAML/TOML must not fail detection unless the file itself cannot be read.
- Any later Rust integration must run GitNexus impact first on the exact owner symbol and record the blast radius.

## Required Tests For First Slice

Fixture tests must cover:

- Gemini `.gemini`, `GEMINI.md`, commands, skills, extensions, MCP, auth, and ignore artifacts.
- Hermes provider, MCP, skills, plugins, sessions, cron, gateway, and trajectory artifacts.
- GBrain config, MCP, schema pack, skillpack, source registry, operation registry, minion/job, audit/eval/log, backend, and content-shape artifacts.
- Token-like strings, cookies, authorization headers, database URLs, credential paths, and private keys do not appear in output.
- Output ordering is deterministic.
- Missing root exits non-zero with a clear error.
- Invalid root exits non-zero with a clear error.

## Source ADR Disposition

| Source ADR | New status | Dispatch rule |
|---|---|---|
| `ADR_GEMINI_CLI_TOOL_EXTENSIONS.md` | Historical source evidence | Do not dispatch directly; use this consolidation ADR. |
| `ADR_HERMES_AGENT_TOOL_EXTENSIONS.md` | Historical source evidence | Do not dispatch directly; use this consolidation ADR. |
| `ADR_GBRAIN_TOOL_EXTENSIONS.md` | Historical source evidence | Do not dispatch directly; use this consolidation ADR. |

## Final Recommendation

Implement a single redacted external-agent interop detector family instead of three independent detector stacks. Treat Gemini CLI, Hermes Agent, and GBrain as source-specific profiles feeding one shared report contract. Keep Stage 0 repository-only and read-only. Deeper import, runtime mutation, context injection, app-server exposure, credential persistence, provider mapping, MCP mutation, command execution, and plugin activation remain blocked until separate ADRs approve exact owner, schema, tests, compatibility, and redaction behavior.
