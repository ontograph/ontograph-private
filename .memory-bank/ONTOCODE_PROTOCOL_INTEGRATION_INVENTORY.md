# Ontocode Protocol And Integration Inventory

This inventory focuses on protocol, integration, and tooling surfaces where a `codex` to `ontocode` rename can affect compatibility outside a single process or a single local repo checkout.

## Classification Legend

- `safe alias`: add `ontocode` support while keeping the existing `codex` surface working
- `preserve`: keep the existing `codex` identifier because rename value is low and compatibility value is high
- `versioned break`: only change behind an explicit migration path, dual publish, or protocol version boundary

## Recommended Default

- Rebrand human-facing product text to `Ontocode`.
- Keep existing wire-visible `codex` identifiers unless there is strong product value and a versioned migration plan.
- Prefer aliasing CLI and local integration entrypoints over renaming protocol names, package identities, or resource URIs.

## Inventory Matrix

| Surface | Concrete identifiers and repo evidence | Classification | Recommendation |
| --- | --- | --- | --- |
| App-server RPC method names | Generated SDK artifacts expose methods such as `thread/resume`, `mcpServer/tool/call`, and `mcpServer/oauth/login` in [sdk/python/src/openai_codex/generated/v2_all.py](/opt/demodb/_workfolder/ontocode/sdk/python/src/openai_codex/generated/v2_all.py:4748) | preserve | These methods are not branded with `codex`; leave them unchanged. |
| App-server protocol schema filenames and generated type roots | Schema artifacts use `codex_app_server_protocol.v2.schemas.json` and generated `CodexAppServerProtocolV2` in [sdk/python/src/openai_codex/generated/v2_all.py](/opt/demodb/_workfolder/ontocode/sdk/python/src/openai_codex/generated/v2_all.py:2) | versioned break | Treat schema filename or generated package renames as a coordinated SDK/protocol migration, not a branding pass. |
| App-server binary/package name | Release packaging and dotslash outputs use `codex-app-server` and `codex-app-server-package` in [.github/dotslash-config.json](/opt/demodb/_workfolder/ontocode/.github/dotslash-config.json:31) | safe alias | Add `ontocode-app-server` only if needed for user-facing packaging, but keep `codex-app-server` working through the transition. |
| Exec-server protobuf package | Relay protocol declares `package codex.exec_server.relay.v1;` in [codex-rs/exec-server/src/proto/codex.exec_server.relay.v1.proto](/opt/demodb/_workfolder/ontocode/codex-rs/exec-server/src/proto/codex.exec_server.relay.v1.proto:3) | preserve | This is a stable wire identifier; do not rename unless a future `v2` protocol is introduced. |
| MCP host-owned server name | MCP runtime hard-codes `CODEX_APPS_MCP_SERVER_NAME = "codex_apps"` in [codex-rs/codex-mcp/src/mcp/mod.rs](/opt/demodb/_workfolder/ontocode/codex-rs/codex-mcp/src/mcp/mod.rs:44) | preserve | Keep `codex_apps` stable because it is embedded in tool namespaces, cache keys, auth metadata, and server selection logic. |
| MCP host-owned base path | Host-owned MCP defaults to `/api/codex/apps` or legacy `wham/apps` in [codex-rs/codex-mcp/src/mcp/mod.rs](/opt/demodb/_workfolder/ontocode/codex-rs/codex-mcp/src/mcp/mod.rs:431) | safe alias | Support a future `ontocode` path only through config override or server-side aliasing; keep current path valid. |
| MCP qualified tool namespaces | Model-visible names include `mcp__codex_apps__...` in [codex-rs/protocol/src/models.rs](/opt/demodb/_workfolder/ontocode/codex-rs/protocol/src/models.rs:2135) | preserve | Keep the existing namespace to avoid changing tool-call identity and cached/model-visible references. |
| MCP auth metadata keys and elicitation IDs | Auth metadata uses `_codex_apps` and `codex_apps_auth_{call_id}` in [codex-rs/codex-mcp/src/auth_elicitation.rs](/opt/demodb/_workfolder/ontocode/codex-rs/codex-mcp/src/auth_elicitation.rs:9) | preserve | These are internal integration keys with low branding value and high migration cost. |
| MCP resource URIs and templates | Integration tests expose `memo://codex/example-note`, `memo://codex/{slug}`, and `codex-memo` in [codex-rs/rmcp-client/tests/resources.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/tests/resources.rs:18) | versioned break | URI templates and template names should only change with explicit aliasing or resource versioning; otherwise preserve current forms. |
| OAuth RPC labels | Notifications such as `mcpServer/oauthLogin/completed` appear in [sdk/python/src/openai_codex/generated/notification_registry.py](/opt/demodb/_workfolder/ontocode/sdk/python/src/openai_codex/generated/notification_registry.py:103) | preserve | These labels are product-neutral and should remain unchanged. |
| OAuth token env var for connectors | MCP runtime reads `CODEX_CONNECTORS_TOKEN` in [codex-rs/codex-mcp/src/mcp/mod.rs](/opt/demodb/_workfolder/ontocode/codex-rs/codex-mcp/src/mcp/mod.rs:47) | safe alias | Add `ONTOCODE_CONNECTORS_TOKEN` as a preferred input later, but keep `CODEX_CONNECTORS_TOKEN` accepted. |
| Product SKU header | MCP forwards `X-OpenAI-Product-Sku` in [codex-rs/codex-mcp/src/mcp/mod.rs](/opt/demodb/_workfolder/ontocode/codex-rs/codex-mcp/src/mcp/mod.rs:449) | preserve | Header is not `codex`-branded and does not need rename work. |
| Custom HTTP header examples | Tests cover `x-codex-test` in [codex-rs/exec-server/tests/http_request.rs](/opt/demodb/_workfolder/ontocode/codex-rs/exec-server/tests/http_request.rs:61) | preserve | Test-only header strings are not migration blockers. |
| Telemetry metric keys | Metrics such as `codex.memories.tool.call` are defined in [codex-rs/ext/memories/src/metrics.rs](/opt/demodb/_workfolder/ontocode/codex-rs/ext/memories/src/metrics.rs:5) | preserve | Metric renames fragment dashboards and historical continuity; keep existing keys. |
| Service/client names in runtime integrations | Service labels such as `codex-core` and `codex-exec-server` appear across runtime code and package metadata, including [codex-rs/exec-server/README.md](/opt/demodb/_workfolder/ontocode/codex-rs/exec-server/README.md:1) | preserve | Treat these as internal or observability identities unless there is a separate ops migration plan. |
| npm CLI package identity | CLI package is `@openai/codex` in [codex-cli/package.json](/opt/demodb/_workfolder/ontocode/codex-cli/package.json:2) | versioned break | Rename only with dual publish or a meta-package migration plan. |
| npm native package identities | Platform packages such as `@openai/codex-linux-x64` are hard-coded in [codex-cli/bin/codex.js](/opt/demodb/_workfolder/ontocode/codex-cli/bin/codex.js:16) | versioned break | These are release-channel identities and should only change in a coordinated packaging migration. |
| TypeScript SDK package identity | Public SDK package is `@openai/codex-sdk` in [sdk/typescript/package.json](/opt/demodb/_workfolder/ontocode/sdk/typescript/package.json:2) | versioned break | Rename only with explicit source-compatibility and release guidance. |
| Python SDK distribution identity | Public SDK distribution is `openai-codex` in [sdk/python/pyproject.toml](/opt/demodb/_workfolder/ontocode/sdk/python/pyproject.toml:6) | versioned break | Rename only with parallel publish, alias package, or hard migration docs. |
| Python runtime distribution identity | Runtime distribution is `openai-codex-cli-bin` in [sdk/python/pyproject.toml](/opt/demodb/_workfolder/ontocode/sdk/python/pyproject.toml:19) | versioned break | This is installation-critical and should not be renamed in a branding phase. |
| Repo slug and install URLs | Package metadata and docs still point at `github.com/openai/codex` in [sdk/python/pyproject.toml](/opt/demodb/_workfolder/ontocode/sdk/python/pyproject.toml:17) and [sdk/typescript/package.json](/opt/demodb/_workfolder/ontocode/sdk/typescript/package.json:6) | safe alias | If the GitHub repo is ever renamed, rely on redirects first and migrate tooling separately. |
| GitNexus repo identity | Repo tooling references `gitnexus://repo/codex/...` in [AGENTS.md](/opt/demodb/_workfolder/ontocode/AGENTS.md:321) | safe alias | Add `ontocode` aliases in tooling if needed, but keep the existing repo key working for automation. |
| CI runner and artifact labels | Workflows use names like `codex-linux-x64`, `codex-windows-x64`, and `codex-package-*` in [.github/workflows/rust-release.yml](/opt/demodb/_workfolder/ontocode/.github/workflows/rust-release.yml:1037) | preserve | These are operational identifiers with high churn cost and little user-facing benefit. |
| Dotslash output identities and helper binaries | Outputs include `codex`, `codex-responses-api-proxy`, `codex-command-runner`, and `codex-windows-sandbox-setup` in [.github/dotslash-config.json](/opt/demodb/_workfolder/ontocode/.github/dotslash-config.json:2) | preserve | Preserve helper names unless top-level product packaging proves they must be user-visible. |

## Permanently Preserved `codex` Identifiers Recommended

These should stay `codex` unless there is a separate compatibility program with clear external value:

- `package codex.exec_server.relay.v1`
- `codex_apps` MCP server name
- `mcp__codex_apps__...` tool namespaces
- `_codex_apps` MCP auth metadata keys
- `memo://codex/...` resource URIs unless aliasing or resource versioning is introduced
- telemetry keys such as `codex.memories.tool.call`
- helper and operational binary names such as `codex-command-runner` and `codex-windows-sandbox-setup`

## Safe Alias Candidates

These can support `ontocode` without breaking existing clients:

- top-level CLI packaging and install copy
- app-server package entrypoint names, if dual names are published
- local env vars such as a future `ONTOCODE_CONNECTORS_TOKEN`
- repo/tooling aliases such as `gitnexus://repo/ontocode/...`
- MCP host-owned base URL path aliases, provided the current `/api/codex/apps` path remains valid

## Versioned Break Candidates

These should not change in a doc-only or CLI-alias-only stage:

- public npm package names
- public PyPI package names
- schema artifact names consumed by generated SDKs
- MCP resource URI templates if real clients persist or match them

## Bottom Line

The recommended external rename path is narrow:

- alias the local entrypoints users type
- rebrand human-facing copy
- preserve most wire-visible `codex` identifiers
- treat package identity and protocol shape changes as explicit migration programs, not cleanup tasks
