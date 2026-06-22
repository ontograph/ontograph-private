# Ontocode Internal Crate Rename Reference Inventory

Task: R0C Stage 0 Rust import, script, and developer-command reference inventory.

Date: 2026-06-09

Scope: inventory only. No implementation proposal beyond Stage 0.

## Inputs Read

- `.memory-bank/MEMORY.md`
- `.memory-bank/ONTOCODE_INTERNAL_CRATE_RENAME_PROJECT_PLAN.md`
- `.memory-bank/ONTOCODE_INTERNAL_CRATE_RENAME_TRACKING.md`

## Commands Run

- `rg -n --glob '*.rs' '(^|[^A-Za-z0-9_])(use codex_[A-Za-z0-9_]*(::|[;,{])|extern crate codex_[A-Za-z0-9_]*|codex_[A-Za-z0-9_]*::)' ontocode-rs | wc -l`
- `rg -n --glob '*.rs' '(^|[^A-Za-z0-9_])(use codex_[A-Za-z0-9_]*(::|[;,{])|extern crate codex_[A-Za-z0-9_]*|codex_[A-Za-z0-9_]*::)' ontocode-rs | awk ... | sort | uniq -c | sort -nr | head -60`
- `rg -n --glob '*.rs' '(^|[^A-Za-z0-9_])(use codex_[A-Za-z0-9_]*(::|[;,{])|extern crate codex_[A-Za-z0-9_]*|codex_[A-Za-z0-9_]*::)' ontocode-rs | awk -F: ...`
- `rg -n '(just\s+[^\n]*-p\s+codex-[A-Za-z0-9_-]+|cargo\s+[^\n]*-p\s+codex-[A-Za-z0-9_-]+|codex-[A-Za-z0-9_-]+)' justfile scripts sdk .github .memory-bank AGENTS.md ontocode-rs --glob '!ontocode-rs/target/**' | wc -l`
- `rg -o 'codex-[A-Za-z0-9_-]+' justfile scripts sdk .github .memory-bank AGENTS.md ontocode-rs --glob '!ontocode-rs/target/**' | sed 's/^.*://g' | sort | uniq -c | sort -nr | head -100`
- `rg -n 'just\s+[^\n]*-p\s+codex-[A-Za-z0-9_-]+' justfile scripts sdk .github .memory-bank AGENTS.md ontocode-rs --glob '!ontocode-rs/target/**'`
- `rg -n 'cargo\s+[^\n]*-p\s+codex-[A-Za-z0-9_-]+' justfile scripts sdk .github .memory-bank AGENTS.md ontocode-rs --glob '!ontocode-rs/target/**'`
- `rg -n 'codex-(package|path|resources|cli-bin|command-runner|windows-sandbox-setup|responses-api-proxy|sdk)' scripts sdk .github ontocode-rs --glob '!ontocode-rs/target/**'`
- `rg -n 'CODEX_[A-Z0-9_]+|\.codex\b|codex_app_server_protocol|codex\.exec_server|openai-codex|@openai/codex|ontocode-responses-api-proxy|codex-package\.json|codex-path|codex-resources' ontocode-rs scripts sdk .github .memory-bank AGENTS.md --glob '!ontocode-rs/target/**' | wc -l`
- `rg -o 'CODEX_[A-Z0-9_]+' ontocode-rs scripts sdk .github .memory-bank AGENTS.md --glob '!ontocode-rs/target/**' | sed 's/^.*://g' | sort | uniq -c | sort -nr | head -80`

Note: lean-ctx blocked shell redirect writes, so the companion `.txt` files are compact derived summaries created through the owned-file edit path, not complete raw `rg` dumps.

## Rust Import And Usage Inventory

Total Rust hits for `use codex_*`, `codex_*::`, and `extern crate codex_*`: `17,041`.

Owner class counts:

| Owner class | Hits |
| --- | ---: |
| Rust source | 10,015 |
| Tests | 6,935 |
| Rust source/bin | 91 |

Top path clusters:

| Path cluster | Hits | Owner |
| --- | ---: | --- |
| `ontocode-rs/core` | 6,165 | core/shared and tests |
| `ontocode-rs/app-server` | 3,121 | app-server source/tests |
| `ontocode-rs/tui` | 1,973 | TUI source/tests/snapshots |
| `ontocode-rs/ext` | 535 | extension API/tests |
| `ontocode-rs/app-server-protocol` | 422 | protocol/generated/schema |
| `ontocode-rs/cli` | 376 | CLI source/tests |
| `ontocode-rs/exec` | 325 | exec/runtime |
| `ontocode-rs/hooks` | 244 | hooks source/tests |
| `ontocode-rs/analytics` | 229 | analytics/tests |
| `ontocode-rs/exec-server` | 217 | exec-server/runtime |
| `ontocode-rs/thread-store` | 215 | thread state |
| `ontocode-rs/codex-api` | 210 | API crate |
| `ontocode-rs/rollout` | 200 | rollout/persisted state |
| `ontocode-rs/core-plugins` | 181 | core plugin integration |
| `ontocode-rs/windows-sandbox-rs` | 157 | Windows helper/runtime |
| `ontocode-rs/config` | 148 | config/shared |
| `ontocode-rs/memories` | 140 | memory/state |
| `ontocode-rs/codex-mcp` | 133 | MCP owner |
| `ontocode-rs/app-server-transport` | 126 | app transport |
| `ontocode-rs/app-server-client` | 116 | app client |
| `ontocode-rs/mcp-server` | 105 | MCP server |
| `ontocode-rs/model-provider` | 98 | provider owner |
| `ontocode-rs/linux-sandbox` | 98 | sandbox helper |
| `ontocode-rs/login` | 96 | auth/login |

Top Rust crate-name clusters:

| Crate token | Hits | Stage/Risk Note |
| --- | ---: | --- |
| `codex_protocol` | 5,590 | Core/protocol dependency; high blast radius; protocol decision gate applies. |
| `codex_app_server_protocol` | 3,694 | Protocol/generated identity; preserve unless separate protocol ADR approves. |
| `codex_config` | 1,068 | Shared config; schema compatibility risk. |
| `codex_core` | 564 | Highest-risk core/shared crate; late-stage only. |
| `codex_tools` | 482 | Core/tooling integration; model-context and agent behavior risk. |
| `codex_state` | 474 | Persisted state/runtime risk. |
| `codex_utils_absolute_path` | 469 | Leaf utility candidate but broad import fanout. |
| `codex_login` | 321 | Auth/login owner; compatibility risk. |
| `codex_extension_api` | 321 | Extension API surface; external integration risk. |
| `codex_exec_server` | 308 | Runtime helper/high-risk path. |
| `codex_otel` | 203 | Telemetry; preserve schema names. |
| `codex_features` | 187 | Shared feature flags. |
| `codex_api` | 172 | API/core edge. |
| `codex_core_plugins` | 145 | Plugin/core integration. |
| `codex_rollout` | 144 | Persisted rollout/session risk. |
| `codex_analytics` | 120 | Analytics/telemetry risk. |
| `codex_network_proxy` | 119 | Runtime/network. |
| `codex_model_provider_info` | 113 | Provider metadata. |
| `codex_client` | 113 | Client/core edge. |
| `codex_windows_sandbox` | 102 | Helper/runtime packaging. |
| `codex_sandboxing` | 100 | Runtime/sandbox policy. |
| `codex_thread_store` | 95 | Persisted thread state. |
| `codex_cloud_tasks_client` | 91 | Cloud/client edge. |
| `codex_code_mode` | 90 | Agent/runtime. |
| `codex_mcp` | 88 | MCP owner. |
| `codex_hooks` | 86 | Hooks owner. |
| `codex_utils_cargo_bin` | 74 | Leaf utility candidate; test-binary lookup risk. |
| `codex_execpolicy` / `codex_execpolicy_legacy` | 151 combined | Policy tooling/docs. |
| `codex_plugin` | 72 | Plugin owner. |
| `codex_home` | 67 | Home/state compatibility risk. |
| `codex_git_utils` | 66 | Utility/helper. |
| `codex_utils_pty` | 62 | Utility/helper. |
| `codex_core_api` | 53 | Core API edge. |
| `codex_app_server` | 53 | App-server source edge. |
| `codex_exec` | 50 | Runtime helper. |
| `codex_model_provider` | 46 | Provider owner. |
| `codex_rmcp_client` | 40 | MCP/auth owner. |
| `codex_arg0` | 40 | Runtime/package path risk. |
| `codex_utils_cli` | 37 | Leaf utility candidate. |
| `codex_tui` | 35 | TUI snapshots/UI risk. |

## Developer Command And Script Reference Inventory

Total command/package-name hits for `just test -p codex-*`, `cargo ... -p codex-*`, and `codex-*`: `4,627`.

Owner class counts:

| Owner class | Hits |
| --- | ---: |
| Cargo/Bazel/fixtures under `ontocode-rs` | 1,929 |
| Rust source/tests | 1,277 |
| Memory-bank historical/planning docs | 880 |
| GitHub CI/release/signing | 413 |
| Scripts | 154 |
| SDK/package surfaces | 113 |
| `AGENTS.md` | 27 |
| Root `justfile` | 15 |

Live developer command selectors found:

| Owner | Examples |
| --- | --- |
| `AGENTS.md` | `just test -p ontocode-tui`, `just test -p ontocode-app-server-protocol`, `cargo insta ... -p ontocode-tui` |
| Root `justfile` | `cargo build -p codex-cli`, `cargo run -p ontocode-app-server-test-client`, `cargo run -p ontocode-mcp-server`, `cargo run -p codex-core`, `cargo run -p ontocode-app-server-protocol`, `cargo run -p codex-hooks`, `cargo run -p codex-state` |
| Scripts | `scripts/start-ontocode-exec.sh`, `scripts/run_tui_with_exec_server.sh`, `scripts/test-remote-env.sh` build or run `codex-cli` / `ontocode-tui` package selectors. |
| Rust READMEs/tests | `ontocode-rs/app-server-test-client/README.md`, `ontocode-rs/execpolicy*/README.md`, `ontocode-rs/network-proxy/README.md`, `ontocode-rs/thread-manager-sample/README.md`, TUI tests/snapshots and core command canonicalization tests embed `cargo test -p codex-*`. |
| CI | `.github/workflows/v8-canary.yml` runs `cargo ... -p codex-v8-poc`; release workflows package `codex-*` helper/runtime artifacts. |
| Memory-bank | Historical verification logs and active rename plans include many `just test -p codex-*` references; do not rewrite historical audit facts during implementation stages. |

Top `codex-*` package/string clusters:

| Token | Hits | Classification |
| --- | ---: | --- |
| `ontocode-rs` | 3,853 | Repo path/doc identity; not a crate package selector by itself. |
| `codex-protocol` | 364 | Protocol/wire decision gate. |
| `codex-plugin` | 271 | Plugin owner. |
| `codex-core` | 269 | Core/shared; highest risk. |
| `codex-utils-absolute-path` | 266 | Leaf utility candidate with broad fanout. |
| `codex-login` | 171 | Auth/login. |
| `ontocode-app-server-protocol` | 165 | Protocol/generated decision gate. |
| `codex-config` | 137 | Config/schema compatibility. |
| `codex-api` | 125 | API/core edge. |
| `codex-cli` | 123 | CLI/app; high risk. |
| `ontocode-tui` | 118 | TUI/UI/snapshot risk. |
| `codex-otel` | 116 | Telemetry; preserve schemas. |
| `ontocode-exec-server` | 109 | Runtime helper. |
| `codex-model-provider` | 106 | Provider owner. |
| `codex-utils-cargo-bin` | 102 | Leaf utility candidate with test-binary lookup risk. |
| `ontocode-app-server` | 85 | App-server. |
| `codex-mcp` | 85 | MCP owner. |
| `codex-linux-sandbox` | 79 | Helper/runtime. |
| `codex-arg0` | 78 | Runtime dispatch. |
| `codex-resources` | 77 | Package layout; must not be renamed in this program. |
| `codex-extension-api` | 73 | Extension API. |
| `codex-client` | 73 | Client/core edge. |
| `ontocode-responses-api-proxy` | 70 | Package/binary identity; preserve unless separate package plan. |
| `codex-state` | 67 | Persisted state. |
| `codex-cli-bin` | 59 | SDK runtime package identity; preserve. |
| `ontocode-command-runner` | 53 | Windows helper/runtime/package. |
| `codex-sandboxing` | 51 | Runtime/sandboxing. |
| `ontocode-windows-sandbox` | 50 | Helper/runtime. |
| `codex-install-context` | 50 | Runtime package layout; high risk. |
| `codex-path` | 45 | Package layout; must not be renamed in this program. |
| `codex-package` | 45 | Package archive/layout; must not be renamed in this program. |

## References That Must Not Be Renamed In This Program

The preservation search returned `6,840` hits across env/state, package manager IDs, protocol/generated names, SDK runtime layout, telemetry, and historical docs.

Do not rename these classes during internal crate/package rename stages:

| Class | Examples | Reason |
| --- | --- | --- |
| Package manager IDs | `@openai/codex`, `@openai/codex-sdk`, `@openai/ontocode-responses-api-proxy`, `openai-codex`, `openai-codex-cli-bin`, `codex_cli_bin` | External package identity is explicitly out of scope. |
| Runtime package layout | `codex-package.json`, `codex-package-*`, `codex-path`, `codex-resources` | SDK/install/runtime layout compatibility boundary. |
| Compatibility env/state | `CODEX_HOME`, `.codex`, `CODEX_API_KEY`, `CODEX_EXEC_SERVER_URL`, `CODEX_SANDBOX*`, `CODEX_APPS_*`, `CODEX_PACKAGE_COMPONENT`, `CODEX_MANAGED_PACKAGE_ROOT` | Persisted state, environment contracts, test gates, and compatibility inputs. |
| Protocol/generated names | `codex_app_server_protocol`, `codex_app_server_protocol.v2.schemas.json`, `codex.exec_server.*` | Protocol/schema/generated identity requires separate protocol-versioning ADR. |
| Telemetry/analytics names | `codex_otel`, analytics event/schema names, `CODEX_TURN_METADATA_HEADER` | Telemetry schema compatibility boundary. |
| Historical memory-bank audit facts | Existing audit/session notes with `just test -p codex-*` | Historical records should not be rewritten as cleanup. |
| Helper binary/package identities already covered by compatibility | `ontocode-command-runner`, `ontocode-windows-sandbox-setup`, `ontocode-responses-api-proxy` | Helper/package/runtime boundary; rename only if a scoped helper/package stage explicitly owns it. |

Top `CODEX_*` env/state clusters:

| Token | Hits |
| --- | ---: |
| `CODEX_HOME` | 256 |
| `CODEX_APPS_MCP_SERVER_NAME` | 136 |
| `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` | 35 |
| `CODEX_LINUX_SANDBOX_ARG0` | 20 |
| `CODEX_CA_CERT_ENV` | 19 |
| `CODEX_CLI_VERSION` | 18 |
| `CODEX_THREAD_ID_ENV_VAR` | 17 |
| `CODEX_SANDBOX_ENV_VAR` | 16 |
| `CODEX_EXEC_SERVER_URL_ENV_VAR` | 15 |
| `CODEX_API_KEY_ENV_VAR` | 15 |
| `CODEX_THREAD_ID` | 14 |
| `CODEX_EXEC_SERVER_URL` | 14 |
| `CODEX_TURN_METADATA_HEADER` | 13 |
| `CODEX_API_KEY` | 12 |

## Highest-Risk Reference Clusters

| Risk | Cluster | Why It Is High Risk |
| --- | --- | --- |
| Critical | Protocol/generated: `codex_protocol`, `codex_app_server_protocol`, schema files, `codex.exec_server.*` | Large fanout plus external wire/generated compatibility; default should preserve pending separate ADR. |
| Critical | Package manager and SDK runtime IDs: `@openai/codex*`, `openai-codex*`, `codex_cli_bin`, `codex-package.json`, `codex-path`, `codex-resources` | External installs, SDK binary discovery, release archive layout, and package smoke tests depend on old names. |
| High | Core/shared: `codex_core`, `codex_config`, `codex_state`, `codex_tools`, `codex_home` | Broad source/test fanout and persisted state/config/schema behavior. |
| High | CLI/app/TUI: `codex-cli`, `ontocode-tui`, `ontocode-app-server*` | Developer commands, snapshots, app-server docs, CI, and runtime launch paths all refer to package names. |
| High | Runtime/helper: `ontocode-exec*`, `codex-arg0`, `codex-install-context`, `codex-sandboxing`, `codex-linux-sandbox`, `ontocode-windows-sandbox` | Arg0 dispatch, bundled helper lookup, package layout, install scripts, and platform-specific CI intersect. |
| Medium | Leaf utilities with broad fanout: `codex-utils-absolute-path`, `codex-utils-cargo-bin`, `codex-utils-cli`, `codex-ansi-escape` | Lower semantic risk but many imports/dependency keys/tests must move together. |
| Medium | Memory-bank/AGENTS historical command refs | Active instructions may need future R6 cleanup, but historical audits should not be rewritten. |

## Blockers And Notes

- No implementation blockers for R0C inventory.
- Complete raw `rg` dumps were not written because lean-ctx blocks redirect writes; compact companion `.txt` summaries were generated instead.
- The inventory confirms direct package-name references are too broad for a mechanical rename.
- Stage 0 should remain blocked until R0A/R0B/R0D outputs are reconciled and a first crate-family slice is selected by exact crate list.

