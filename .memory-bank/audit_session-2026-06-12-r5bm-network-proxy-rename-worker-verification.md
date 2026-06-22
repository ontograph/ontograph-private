# R5BM Network Proxy Rename Worker Verification

Date: 2026-06-12

Scope:
- `codex-network-proxy` -> `ontocode-network-proxy`.
- `codex_network_proxy` -> `ontocode_network_proxy`.
- Identity-only package/lib/Bazel/import rename; preserve the existing `network-proxy` directory path and all network proxy behavior guardrails.

Fallback:
- Work completed on `gpt-5.4-mini` after Spark usage-limit fallback.

Implementation:
- Renamed the package/lib/Bazel/import identity surfaces for the `network-proxy` crate.
- Left network proxy config/defaults, TOML/serde shapes, domain allow/deny serialization, Unix-socket allowlist validation, runtime port/bind/host parsing, MITM CA/key/cert generation, managed trust bundle paths, HTTP/SOCKS proxy behavior, upstream/connect policy, local binding restrictions, network-policy denial reasons/responses, and product/runtime strings unchanged.

Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-network-proxy --no-tests=pass` passed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- `cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name | select(startswith("codex-"))' | sort | wc -l` returned `8`.
- `git diff --check` passed.
- `OntoIndex detect-changes --repo codex` reported the known high-risk pre-existing dirty-tree noise.

Residual refs:
- Old-name references remain only in compatibility/docs surfaces such as `ontocode-rs/network-proxy/README.md` and `ontocode-rs/deny.toml`.
