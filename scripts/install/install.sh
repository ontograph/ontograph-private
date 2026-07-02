#!/bin/sh

set -eu

REPO="${ONTOCODE_RELEASE_REPO:-ontograph/ontograph-private}"
RELEASE="${ONTOCODE_RELEASE:-${CODEX_RELEASE:-}}"
BIN_DIR="${ONTOCODE_INSTALL_DIR:-${CODEX_INSTALL_DIR:-$HOME/.local/bin}}"
BIN_PATH="$BIN_DIR/ontocode"
GITHUB_AUTH_TOKEN="${GH_TOKEN:-${GITHUB_TOKEN:-}}"
ONTOCODE_HOME_DIR="${ONTOCODE_HOME:-$HOME/.ontocode}"
LEGACY_CODEX_HOME_DIR="$HOME/.codex"

usage() {
  cat <<EOF
Usage: install.sh [--release VERSION|latest]

Environment:
  ONTOCODE_RELEASE       Version to install. Default: latest private alpha.
                         Overridden by --release.
  ONTOCODE_RELEASE_REPO  GitHub repo to download from. Default: $REPO
  ONTOCODE_INSTALL_DIR   Install directory. Default: $BIN_DIR
  GH_TOKEN/GITHUB_TOKEN  Optional GitHub token for private release downloads.
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --release)
      [ "$#" -ge 2 ] || { echo "--release requires a value." >&2; exit 1; }
      RELEASE="$2"
      shift
      ;;
    --help | -h)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 1
      ;;
  esac
  shift
done

case "$(uname -s):$(uname -m)" in
  Linux:x86_64 | Linux:amd64) target="x86_64-unknown-linux-gnu" ;;
  *)
    echo "Ontocode private alpha installer currently supports Linux x86_64 only." >&2
    echo "Build locally instead: cd ontocode-rs && CARGO_BUILD_JOBS=8 cargo build --release -p ontocode-cli --bin ontocode" >&2
    exit 1
    ;;
esac

need() {
  command -v "$1" >/dev/null 2>&1 || { echo "$1 is required." >&2; exit 1; }
}

normalize_version() {
  case "$1" in
    rust-v*) printf '%s\n' "${1#rust-v}" ;;
    v*) printf '%s\n' "${1#v}" ;;
    *) printf '%s\n' "$1" ;;
  esac
}

CURL_COMMON_OPTS="--http1.1 --retry 3 --retry-delay 1 --connect-timeout 15 --max-time 600"

curl_json() {
  if [ -n "$GITHUB_AUTH_TOKEN" ]; then
    curl $CURL_COMMON_OPTS -fsSL -H "Authorization: Bearer $GITHUB_AUTH_TOKEN" -H "Accept: application/vnd.github+json" "$1"
  else
    curl $CURL_COMMON_OPTS -fsSL -H "Accept: application/vnd.github+json" "$1"
  fi
}

curl_file() {
  if [ -n "$GITHUB_AUTH_TOKEN" ]; then
    curl $CURL_COMMON_OPTS -fL# -H "Authorization: Bearer $GITHUB_AUTH_TOKEN" "$1" -o "$2"
  else
    curl $CURL_COMMON_OPTS -fL# "$1" -o "$2"
  fi
}

json_value() {
  sed -n "s/.*\"$1\":[[:space:]]*\"\\([^\"]*\\)\".*/\\1/p" | head -n 1
}

need curl
need sed
need awk
need mktemp
need chmod
need install
need sha256sum
need cp
need_asset() {
  [ -s "$1" ] || { echo "Missing downloaded asset: $1" >&2; exit 1; }
}

if [ "$RELEASE" = "latest" ] || [ -z "$RELEASE" ]; then
  if command -v gh >/dev/null 2>&1; then
    tag="$(gh release list --repo "$REPO" --limit 1 | cut -f3)"
  else
    tag="$(curl_json "https://api.github.com/repos/$REPO/releases?per_page=1" | json_value tag_name)"
  fi
else
  tag="rust-v$(normalize_version "$RELEASE")"
fi
[ -n "$tag" ] || { echo "Could not resolve an Ontocode release tag." >&2; exit 1; }

version="$(normalize_version "$tag")"
asset="ontocode-$version-$target"
asset_url="https://github.com/$REPO/releases/download/$tag/$asset"
legacy_asset="ontocode-$target"
legacy_asset_url="https://github.com/$REPO/releases/download/$tag/$legacy_asset"
checksums_url="https://github.com/$REPO/releases/download/$tag/SHA256SUMS"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT INT TERM

archive="$tmp_dir/$asset"
checksums="$tmp_dir/SHA256SUMS"

echo "==> Downloading Ontocode CLI $version for $target"
if [ -s "./$asset" ]; then
  echo "==> Using local release asset ./$asset"
  cp "./$asset" "$archive"
elif [ -s "./$legacy_asset" ]; then
  asset="$legacy_asset"
  archive="$tmp_dir/$asset"
  echo "==> Using local legacy release asset ./$asset"
  cp "./$asset" "$archive"
elif command -v gh >/dev/null 2>&1 &&
  gh release download "$tag" --repo "$REPO" --pattern "$asset" --dir "$tmp_dir" &&
  gh release download "$tag" --repo "$REPO" --pattern SHA256SUMS --dir "$tmp_dir"; then
  :
elif curl_file "$asset_url" "$archive"; then
  :
else
  curl_exit=$?
  if [ "$curl_exit" -eq 22 ]; then
    asset="$legacy_asset"
    archive="$tmp_dir/$asset"
    echo "==> Falling back to legacy release asset name $asset"
    curl_file "$legacy_asset_url" "$archive"
  else
    exit "$curl_exit"
  fi
fi
if [ ! -s "$archive" ]; then
  if command -v gh >/dev/null 2>&1 &&
    gh release download "$tag" --repo "$REPO" --pattern "$legacy_asset" --dir "$tmp_dir"; then
    asset="$legacy_asset"
    archive="$tmp_dir/$asset"
    echo "==> Falling back to legacy release asset name $asset"
  else
    need_asset "$archive"
  fi
fi
if [ -s "./SHA256SUMS" ]; then
  echo "==> Using local SHA256SUMS"
  cp ./SHA256SUMS "$checksums"
elif [ ! -s "$checksums" ]; then
  curl_file "$checksums_url" "$checksums"
fi
need_asset "$archive"
need_asset "$checksums"

expected="$(awk -v asset="$asset" '$2 == asset { print $1; exit }' "$checksums")"
[ -n "$expected" ] || { echo "SHA256SUMS does not list $asset." >&2; exit 1; }

actual="$(sha256sum "$archive" | awk '{ print $1 }')"
[ "$actual" = "$expected" ] || { echo "Checksum mismatch for $asset." >&2; exit 1; }

mkdir -p "$BIN_DIR"
install -m 0755 "$archive" "$BIN_PATH"

if [ ! -f "$ONTOCODE_HOME_DIR/config.toml" ] && [ -d "$LEGACY_CODEX_HOME_DIR" ] && [ "$ONTOCODE_HOME_DIR" != "$LEGACY_CODEX_HOME_DIR" ]; then
  echo "==> Copying settings and history from $LEGACY_CODEX_HOME_DIR to $ONTOCODE_HOME_DIR"
  mkdir -p "$ONTOCODE_HOME_DIR"
  cp -R "$LEGACY_CODEX_HOME_DIR"/. "$ONTOCODE_HOME_DIR"/
fi

echo "Ontocode CLI $version installed to $BIN_PATH"
