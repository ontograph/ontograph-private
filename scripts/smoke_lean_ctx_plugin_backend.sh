#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
launcher="$repo_root/scripts/run_lean_ctx_plugin_backend.sh"

host="${LEAN_CTX_HOST:-127.0.0.1}"
port="${LEAN_CTX_PORT:-7777}"
token="${LEANCTX_TOKEN:-ontocode-lean-ctx-dev}"
base_url="http://$host:$port"
start_timeout_seconds="${LEAN_CTX_PLUGIN_BACKEND_START_TIMEOUT_SECONDS:-120}"
ready=0

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/lean-ctx-plugin-backend-smoke.XXXXXX")"
stdout_log="$tmp_dir/backend.stdout"
stderr_log="$tmp_dir/backend.stderr"
server_pid=""

cleanup() {
  if [[ -n "$server_pid" ]]; then
    kill "$server_pid" >/dev/null 2>&1 || true
    wait "$server_pid" >/dev/null 2>&1 || true
  fi
  rm -rf "$tmp_dir"
}

trap cleanup EXIT INT TERM HUP

"$launcher" >"$stdout_log" 2>"$stderr_log" &
server_pid="$!"

metadata_url="$base_url/.well-known/mcp-server.json"
for _ in $(seq 1 "$((start_timeout_seconds * 20))"); do
  if curl --silent --show-error --fail \
    -H "Authorization: Bearer $token" \
    "$metadata_url" >/dev/null 2>&1; then
    ready=1
    break
  fi

  if ! kill -0 "$server_pid" >/dev/null 2>&1; then
    cat "$stderr_log" >&2 || true
    cat "$stdout_log" >&2 || true
    echo "lean-ctx plugin backend exited before becoming ready" >&2
    exit 1
  fi

  sleep 0.05
done

if [[ "$ready" -ne 1 ]]; then
  cat "$stderr_log" >&2 || true
  cat "$stdout_log" >&2 || true
  echo "lean-ctx plugin backend did not become ready within ${start_timeout_seconds}s" >&2
  exit 1
fi

response="$(
  curl --silent --show-error --fail \
    -H "Authorization: Bearer $token" \
    -H "Accept: application/json, text/event-stream" \
    -H "Content-Type: application/json" \
    -X POST \
    "$base_url/" \
    --data '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}'
)"

python3 - "$response" <<'PY'
import json
import sys

payload = json.loads(sys.argv[1])
tools = payload.get("result", {}).get("tools", [])
names = {tool.get("name") for tool in tools}
expected = {"ctx_read", "ctx_search", "ctx_summary"}

if names != expected:
    missing = sorted(expected - names)
    extra = sorted(names - expected)
    raise SystemExit(
        "unexpected tool surface: "
        f"missing={missing} extra={extra} names={sorted(names)}"
    )
PY

echo "lean-ctx plugin backend smoke passed at $base_url"
