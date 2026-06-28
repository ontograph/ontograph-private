#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest_path="$repo_root/third_party/lean-ctx-fork/rust/Cargo.toml"
binary_path="$repo_root/third_party/lean-ctx-fork/rust/target/release/lean-ctx"

host="${LEAN_CTX_HOST:-127.0.0.1}"
port="${LEAN_CTX_PORT:-7777}"
token="${LEANCTX_TOKEN:-ontocode-lean-ctx-dev}"
profile="${LEAN_CTX_TOOL_PROFILE:-ontocode}"
build_jobs="${CARGO_BUILD_JOBS:-8}"

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  if [[ ! -x "$binary_path" ]]; then
    CARGO_BUILD_JOBS="$build_jobs" cargo build --release --manifest-path "$manifest_path" --bin lean-ctx
  fi
  exec "$binary_path" serve --help
fi

if [[ ! -x "$binary_path" ]]; then
  CARGO_BUILD_JOBS="$build_jobs" cargo build --release --manifest-path "$manifest_path" --bin lean-ctx
fi

export LEAN_CTX_TOOL_PROFILE="$profile"

exec "$binary_path" serve --host "$host" --port "$port" --auth-token "$token" "$@"
