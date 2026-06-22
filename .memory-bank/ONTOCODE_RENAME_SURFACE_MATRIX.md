# Ontocode Rename Surface Matrix

This matrix applies the policy in `ONTOCODE_RENAME_PROJECT_PLAN.md` to concrete, repo-visible `codex` surfaces. It is intentionally high-signal rather than exhaustive to every test string or private helper.

## Recommended Rule Legend

- `rename now`: user-facing rename can happen in the first branding pass
- `alias`: add `ontocode` support while keeping `codex` working during the transition window
- `version`: change only behind an explicit versioned migration path
- `preserve`: keep the `codex` identifier because compatibility value is higher than branding value
- `defer`: postpone until the external migration is stable and the payoff is proven

## Surface Matrix

| Surface class | Concrete surfaces / examples / paths | Recommended rule | Compatibility risk | Notes |
| --- | --- | --- | --- | --- |
| Human-facing brand | Root docs and plans such as `README*`, release notes, screenshots, help copy, package descriptions; examples already visible in [sdk/python/README.md](/opt/demodb/_workfolder/ontocode/sdk/python/README.md:1), [sdk/typescript/README.md](/opt/demodb/_workfolder/ontocode/sdk/typescript/README.md:1), [ontocode-rs/exec-server/README.md](/opt/demodb/_workfolder/ontocode/ontocode-rs/exec-server/README.md:1) | rename now | Low | Best Stage 1 target. Keep technical identifiers unchanged in the same pass unless already aliased. |
| Primary CLI executable name | Rust bin name `codex` in [ontocode-rs/cli/Cargo.toml](/opt/demodb/_workfolder/ontocode/ontocode-rs/cli/Cargo.toml:7); docs and SDKs shell out to `codex` | alias | High | Add `ontocode` alongside `codex`; do not remove `codex` until a defined release after parity and rollback tests. |
| CLI subcommands and flags | Existing command surface rooted under the `codex` binary, including `codex exec-server`, `codex mcp`, `codex login`, `codex app-server`; examples in [ontocode-rs/exec-server/README.md](/opt/demodb/_workfolder/ontocode/ontocode-rs/exec-server/README.md:9) and CLI tests under [ontocode-rs/cli/tests](/opt/demodb/_workfolder/ontocode/ontocode-rs/cli/tests:1) | alias | Medium | Prefer command-name aliasing only. Avoid renaming subcommand nouns unless there is separate product value. |
| User home and config root | `CODEX_HOME`, `~/.codex`, `config.toml`, `auth.json`, plugin and session storage; surfaced in [sdk/python/src/openai_codex/client.py](/opt/demodb/_workfolder/ontocode/sdk/python/src/openai_codex/client.py:694) and many CLI tests such as [ontocode-rs/cli/tests/login.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/cli/tests/login.rs:23) | alias | Critical | Highest split-brain risk. Read both old and new locations, define write target explicitly, and test mixed-version behavior before any public cutover. |
| Persisted session and rollout state | `~/.codex/sessions` documented in [sdk/typescript/src/codex.ts](/opt/demodb/_workfolder/ontocode/sdk/typescript/src/codex.ts:31) and [sdk/typescript/README.md](/opt/demodb/_workfolder/ontocode/sdk/typescript/README.md:100); rollout/thread resume semantics across CLI and SDKs | alias | Critical | Needs per-state migration rules, not just path fallback. Thread resume and rollback compatibility must work across old/new binaries. |
| Auth and credential state | `auth.json` under `CODEX_HOME` in [ontocode-rs/cli/tests/login.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/cli/tests/login.rs:23) plus SDK/CLI login flows | alias | Critical | Credentials are persisted state, not branding. Favor read-both/write-one or copy-on-first-run; avoid duplicate auth stores. |
| Plugin metadata and plugin cache layout | `.codex-plugin` manifests and cached plugin paths in [ontocode-rs/cli/tests/plugin_cli.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/cli/tests/plugin_cli.rs:59) and [ontocode-rs/cli/tests/plugin_cli.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/cli/tests/plugin_cli.rs:783) | preserve | High | The dot-directory name is a local integration contract. Renaming it adds migration cost with little user-visible value. Consider supporting discovery from a future `.ontocode-plugin` only if there is strong need. |
| Environment variable family | `CODEX_HOME`, `CODEX_API_KEY`, `CODEX_EXEC_SERVER_URL`, `CODEX_LOG`, `CODEX_INTERNAL_ORIGINATOR_OVERRIDE`, SDK test envs in [sdk/typescript/src/exec.ts](/opt/demodb/_workfolder/ontocode/sdk/typescript/src/exec.ts:43) and exec-server envs in [ontocode-rs/exec-server/src/environment.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/exec-server/src/environment.rs:24) | alias | High | Add `ONTOCODE_*` as preferred inputs while continuing to honor `CODEX_*`. New names should win when both are set. |
| Rust workspace crate identities | Workspace dependency graph in [ontocode-rs/Cargo.toml](/opt/demodb/_workfolder/ontocode/ontocode-rs/Cargo.toml:102) includes `codex-core`, `codex-cli`, `codex-mcp`, `ontocode-app-server-protocol`, many more | defer | High | Internal churn is large and low-value relative to external rename. Revisit only after external surfaces are stable. |
| Rust library/package names shipped to users | Cargo package names such as `codex-cli`, `ontocode-exec-server`, `ontocode-windows-sandbox` in [ontocode-rs/cli/Cargo.toml](/opt/demodb/_workfolder/ontocode/ontocode-rs/cli/Cargo.toml:2), [ontocode-rs/exec-server/Cargo.toml](/opt/demodb/_workfolder/ontocode/ontocode-rs/exec-server/Cargo.toml:2), [ontocode-rs/windows-sandbox-rs/Cargo.toml](/opt/demodb/_workfolder/ontocode/ontocode-rs/windows-sandbox-rs/Cargo.toml:5) | defer | High | These are package identities and release automation inputs. Treat as a later package-migration program, not a branding pass. |
| Python SDK distribution name | `openai-codex` in [sdk/python/pyproject.toml](/opt/demodb/_workfolder/ontocode/sdk/python/pyproject.toml:6) and distribution metadata in [sdk/python/src/openai_codex/_version.py](/opt/demodb/_workfolder/ontocode/sdk/python/src/openai_codex/_version.py:7) | version | Critical | Published package rename needs a transitional package or parallel publish plan. Do not hard-switch without upgrade-path docs. |
| Python SDK import package | `openai_codex` package path and public types in [sdk/python/src/openai_codex](/opt/demodb/_workfolder/ontocode/sdk/python/src/openai_codex:1) | preserve | High | Import-path churn is expensive for users and gives limited branding value. Keep if package distribution changes first. |
| Python runtime package and bundled asset names | `openai-codex-cli-bin`, `codex_cli_bin`, `codex-package.json`, `codex-path`, bundled `codex` binary in [sdk/python-runtime/pyproject.toml](/opt/demodb/_workfolder/ontocode/sdk/python-runtime/pyproject.toml:6) and [sdk/python-runtime/src/codex_cli_bin/__init__.py](/opt/demodb/_workfolder/ontocode/sdk/python-runtime/src/codex_cli_bin/__init__.py:4) | version | Critical | This package glues SDK installation to a pinned native runtime. Changing it breaks bootstrap and release tooling unless migrated deliberately. |
| TypeScript SDK package identity | `@openai/codex-sdk` in [sdk/typescript/package.json](/opt/demodb/_workfolder/ontocode/sdk/typescript/package.json:2) | version | High | Public npm identity should change only with explicit compatibility plan, release notes, and optional dual publish. |
| TypeScript optional native package names | `@openai/codex-linux-x64`, `@openai/codex-darwin-arm64`, etc. in [sdk/typescript/src/exec.ts](/opt/demodb/_workfolder/ontocode/sdk/typescript/src/exec.ts:48) | version | Critical | Renaming these packages is a coordinated publishing and installer exercise. It cannot be treated as a local refactor. |
| SDK public class and symbol names | `Codex`, `AsyncCodex`, `CodexExec` across [sdk/python/src/openai_codex](/opt/demodb/_workfolder/ontocode/sdk/python/src/openai_codex:1) and [sdk/typescript/src](/opt/demodb/_workfolder/ontocode/sdk/typescript/src:1) | preserve | Medium | These are code-level API identities. Keep unless product decides to accept broad source-breaking changes. |
| App-server RPC method names | Stable RPC methods such as `thread/resume`, `mcpServer/tool/call`, `config/mcpServer/reload` in generated SDK artifacts like [sdk/python/src/openai_codex/generated/v2_all.py](/opt/demodb/_workfolder/ontocode/sdk/python/src/openai_codex/generated/v2_all.py:4748) | preserve | High | These methods are not branded with `codex`; they should remain stable. Rename pressure here is low. |
| Protocol and schema crate names | `ontocode-app-server-protocol`, related generated schemas, and protocol docs referenced by exec-server and SDKs | defer | High | Internal package names can stay `codex-*`; wire compatibility matters more than crate branding. |
| Protobuf package names and checked-in generated artifacts | `package codex.exec_server.relay.v1;` in [ontocode-rs/exec-server/src/proto/codex.exec_server.relay.v1.proto](/opt/demodb/_workfolder/ontocode/ontocode-rs/exec-server/src/proto/codex.exec_server.relay.v1.proto:3) and generated `.rs`/tests | version | Critical | This is a wire identifier. Change only with explicit versioning and dual-reader support, or preserve indefinitely. |
| MCP server, resource, and memo identifiers | Test-visible URIs and names such as `memo://codex/{slug}` and `codex-memo` in [ontocode-rs/rmcp-client/tests/resources.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/rmcp-client/tests/resources.rs:24) | version | High | If these patterns exist in real clients, rename via aliases or resource-versioning; do not silently switch canonical URIs. |
| GitNexus / repo identity strings | Repo indexed as `codex` in [AGENTS.md](/opt/demodb/_workfolder/ontocode/AGENTS.md:298) and `gitnexus://repo/codex/...` references | alias | Medium | Tooling metadata can support both names during migration, but existing automation may depend on the current repo key. |
| OAuth labels, service names, analytics, and metrics | Metrics like `codex.memories.tool.call` in [ontocode-rs/ext/memories/src/metrics.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/memories/src/metrics.rs:5), service/client names like `codex-core` and `ontocode-exec-server` in [ontocode-rs/exec-server/src/client.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/exec-server/src/client.rs:101) and [ontocode-rs/exec-server/src/remote.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/exec-server/src/remote.rs:120) | preserve | Medium | Low user-facing value, potentially high observability churn. Consider display-layer branding without renaming metric keys. |
| Helper executable and sandbox names | `codex-linux-sandbox`, `ontocode-windows-sandbox`, `ontocode-command-runner`, hidden fs helper args, and arg0 dispatch in [ontocode-rs/windows-sandbox-rs/Cargo.toml](/opt/demodb/_workfolder/ontocode/ontocode-rs/windows-sandbox-rs/Cargo.toml:5) and [ontocode-rs/exec-server/src/fs_helper.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/exec-server/src/fs_helper.rs:42) | defer | High | These participate in dispatch, packaging, and platform-specific install logic. Rename only if the top-level CLI aliasing proves insufficient. |
| Repository URLs and slugs | `openai/codex` links in package metadata and docs such as [sdk/python/pyproject.toml](/opt/demodb/_workfolder/ontocode/sdk/python/pyproject.toml:17), [sdk/python-runtime/pyproject.toml](/opt/demodb/_workfolder/ontocode/sdk/python-runtime/pyproject.toml:24), [sdk/typescript/package.json](/opt/demodb/_workfolder/ontocode/sdk/typescript/package.json:7) | defer | Medium | Repo rename is an ecosystem-wide move. Update only when hosting, redirects, release tooling, and docs all align. |
| Test-only strings and temp dir prefixes | Prefixes like `codex-sdk-test-`, `codex-home`, `codex-images-`, `codex-working-dir-` in SDK and Rust tests | defer | Low | These are not user-facing migration blockers. Update opportunistically after real surfaces are settled. |

## Priority Calls

### Rename First

- Human-facing brand text and descriptions
- Installer/docs copy that can point users to `ontocode`
- New `ontocode` command entrypoint, provided `codex` remains supported

### Protect With Compatibility Design

- `CODEX_HOME` and `~/.codex`
- session, rollout, and auth persistence
- `CODEX_*` environment variables
- Python and TypeScript published package identities
- proto package names, MCP resource URIs, and other wire-visible identifiers

### Default To Preserve Or Defer

- Rust crate prefixes and most internal package names
- plugin manifest directory `.codex-plugin`
- metrics keys, analytics dimensions, helper executable names
- SDK symbol names and import package names unless a broader breaking API program is approved

## Working Assumptions

- The target end state is a user-facing `Ontocode` brand with a transition period where `codex` still works.
- Backward compatibility for persisted state is more important than internal naming consistency.
- Any surface that crosses process boundaries, package managers, or stored user state requires aliasing or versioning, not a direct rename.
