#!/bin/sh

set -eu

REPO="${ONTOCODE_RELEASE_REPO:-ontograph/ontograph-private}"
RELEASE="${ONTOCODE_RELEASE:-${CODEX_RELEASE:-0.1.0-alpha.1}}"
BIN_DIR="${ONTOCODE_INSTALL_DIR:-${CODEX_INSTALL_DIR:-$HOME/.local/bin}}"
BIN_PATH="$BIN_DIR/ontocode"
GITHUB_AUTH_TOKEN="${GH_TOKEN:-${GITHUB_TOKEN:-}}"

usage() {
  cat <<EOF
Usage: install.sh [--release VERSION]

Environment:
  ONTOCODE_RELEASE       Version to install; overridden by --release.
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

curl_json() {
  if [ -n "$GITHUB_AUTH_TOKEN" ]; then
    curl --http1.1 --retry 3 --retry-delay 1 -fsSL -H "Authorization: Bearer $GITHUB_AUTH_TOKEN" -H "Accept: application/vnd.github+json" "$1"
  else
    curl --http1.1 --retry 3 --retry-delay 1 -fsSL -H "Accept: application/vnd.github+json" "$1"
  fi
}

curl_file() {
  if [ -n "$GITHUB_AUTH_TOKEN" ]; then
    curl --http1.1 --retry 3 --retry-delay 1 -fsSL -H "Authorization: Bearer $GITHUB_AUTH_TOKEN" -L "$1" -o "$2"
  else
    curl --http1.1 --retry 3 --retry-delay 1 -fsSL -L "$1" -o "$2"
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
need_asset() {
  [ -s "$1" ] || { echo "Missing downloaded asset: $1" >&2; exit 1; }
}

if [ "$RELEASE" = "latest" ] || [ -z "$RELEASE" ]; then
  tag="$(curl_json "https://api.github.com/repos/$REPO/releases/latest" | json_value tag_name)"
else
  tag="rust-v$(normalize_version "$RELEASE")"
fi

version="$(normalize_version "$tag")"
release_json="$(curl_json "https://api.github.com/repos/$REPO/releases/tags/$tag")"
asset="ontocode-$version-$target"
asset_url="$(printf '%s\n' "$release_json" | awk -v asset="$asset" '
  /"name":[[:space:]]*"/ {
    name = $0
    sub(/^.*"name":[[:space:]]*"/, "", name)
    sub(/".*$/, "", name)
  }
  /"browser_download_url":[[:space:]]*"/ && name == asset {
    url = $0
    sub(/^.*"browser_download_url":[[:space:]]*"/, "", url)
    sub(/".*$/, "", url)
    print url
    exit
  }
')"

if [ -z "$asset_url" ]; then
  asset="ontocode-$target"
  asset_url="$(printf '%s\n' "$release_json" | awk -v asset="$asset" '
    /"name":[[:space:]]*"/ {
      name = $0
      sub(/^.*"name":[[:space:]]*"/, "", name)
      sub(/".*$/, "", name)
    }
    /"browser_download_url":[[:space:]]*"/ && name == asset {
      url = $0
      sub(/^.*"browser_download_url":[[:space:]]*"/, "", url)
      sub(/".*$/, "", url)
      print url
      exit
    }
  ')"
fi

checksums_url="$(printf '%s\n' "$release_json" | awk '
  /"name":[[:space:]]*"SHA256SUMS"/ { want = 1 }
  /"browser_download_url":[[:space:]]*"/ && want {
    url = $0
    sub(/^.*"browser_download_url":[[:space:]]*"/, "", url)
    sub(/".*$/, "", url)
    print url
    exit
  }
')"

[ -n "$asset_url" ] || { echo "Release $tag does not contain $asset." >&2; exit 1; }
[ -n "$checksums_url" ] || { echo "Release $tag does not contain SHA256SUMS." >&2; exit 1; }

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT INT TERM

archive="$tmp_dir/$asset"
checksums="$tmp_dir/SHA256SUMS"

echo "==> Downloading Ontocode CLI $version for $target"
curl_file "$asset_url" "$archive"
curl_file "$checksums_url" "$checksums"
need_asset "$archive"
need_asset "$checksums"

expected="$(awk -v asset="$asset" '$2 == asset { print $1; exit }' "$checksums")"
[ -n "$expected" ] || { echo "SHA256SUMS does not list $asset." >&2; exit 1; }

actual="$(sha256sum "$archive" | awk '{ print $1 }')"
[ "$actual" = "$expected" ] || { echo "Checksum mismatch for $asset." >&2; exit 1; }

mkdir -p "$BIN_DIR"
install -m 0755 "$archive" "$BIN_PATH"

echo "Ontocode CLI $version installed to $BIN_PATH"
