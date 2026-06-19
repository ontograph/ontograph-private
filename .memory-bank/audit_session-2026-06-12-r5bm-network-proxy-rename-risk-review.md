# R5BM Network Proxy Rename Risk Review

Date: 2026-06-12

Scope:
- `codex-network-proxy` -> `ontocode-network-proxy`.
- `codex_network_proxy` -> `ontocode_network_proxy`.
- Identity-only package/lib/Bazel/import rename; preserve the existing `network-proxy` directory path.

OntoIndex:
- Repo path verified through the OntoIndex CLI as `/opt/demodb/_workfolder/ontocode`.
- `NetworkProxyConfig`: CRITICAL, 98 impacted nodes, 30 direct, 14 modules.
- `resolve_runtime`: HIGH, 5 impacted nodes, 2 direct, 4 modules.
- `managed_ca_trust_bundle`: HIGH, 7 impacted nodes, 1 direct, 4 modules.
- `is_managed_mitm_ca_trust_bundle_path`: HIGH, 12 impacted nodes, 1 direct, 3 modules, affected shell and unified-exec runtime processes.
- `start_proxy`: LOW, 3 impacted nodes, 1 direct, 2 modules.
- `NetworkProxySettings`, `NetworkMode`, `NetworkProxyState`, and `TargetCheckedTcpConnector`: UNKNOWN due ambiguous struct/impl or enum/impl matches.
- `NetworkPolicy`: UNKNOWN because the target was not found.

Guardrails:
- Do not change network proxy config/defaults, domain allow/deny serialization, Unix-socket allowlist validation, runtime port/bind/host parsing, MITM CA/key/cert generation, managed trust bundle path handling, HTTP/SOCKS proxy behavior, upstream/connect policy, local binding restrictions, network-policy denial reasons/responses, core network proxy loader/session/unified-exec/app-server/CLI/TUI/model-provider behavior, env/config/wire/generated names, telemetry/product strings, persisted state, or folder path.

Verification required:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-network-proxy --no-tests=pass`.
- Focused config/certs/MITM/connect-policy/http/socks/runtime checks.
- Focused core network_proxy/session/unified_exec checks or compile.
- Model-provider proxy/custom-CA compile or focused checks if directly affected.
- CLI/app-server/TUI compile or focused checks if directly affected.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Stale-reference search for `codex-network-proxy|codex_network_proxy`.
- Cargo metadata residual count, expected 8 remaining `codex-*` packages after success.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`; known broad dirty-tree risk may remain outside this scoped rename.
