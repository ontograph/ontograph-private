# R5BM Network Proxy Rename Closure

Date: 2026-06-12

Result:
- Accepted `codex-network-proxy` -> `ontocode-network-proxy`.
- Accepted `codex_network_proxy` -> `ontocode_network_proxy`.
- Identity-only package/lib/Bazel/import rename is complete.
- Residual `codex-*` Cargo package count is 8.

Verification:
- Worker passed `CARGO_BUILD_JOBS=8 just test -p ontocode-network-proxy --no-tests=pass`.
- Worker passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`.
- Worker passed `CARGO_BUILD_JOBS=8 just fmt`.
- Worker passed `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- Worker passed `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Manager confirmed `git diff --check` is clean.
- Manager confirmed Cargo metadata now lists 8 remaining `codex-*` packages.
- Manager confirmed active old refs are intentional compatibility/docs/advisory surfaces only.
- OntoIndex `detect-changes --repo codex` reports the known broad high-risk dirty tree.

Preserved:
- Network proxy config/defaults and TOML/serde shapes.
- Domain allow/deny serialization.
- Unix-socket allowlist validation.
- Runtime port/bind/host parsing.
- MITM CA/key/cert generation and managed trust bundle path handling.
- HTTP/SOCKS proxy behavior.
- Upstream/connect policy.
- Local binding restrictions.
- Network-policy denial reasons/responses.
- Core network proxy loader/session/unified-exec/app-server/CLI/TUI/model-provider behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, public command compatibility, and the existing `network-proxy` directory path.

Notes:
- Manager normalized leading whitespace in `ontocode-rs/Cargo.toml` workspace dependency metadata after worker verification; this did not change package identity or behavior.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
