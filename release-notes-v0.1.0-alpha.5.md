# Ontocode CLI 0.1.0-alpha.5

This private alpha focuses on the Linux release path and install reliability.

## Included

- Linux x86_64 unsigned `ontocode` binary
- `SHA256SUMS`
- `install-ontocode-linux-x64.sh`
- `install-ontocode-linux-x64-v0.1.0-alpha.5.sh`

## Changes

- stamps the release version into `ontocode-rs/Cargo.toml` during the private alpha workflow so release binaries do not report the `0.0.0` source-build sentinel
- keeps source manifests on development placeholders until staging or tag-triggered release build time
- uses matching `release-notes-v<version>.md` files as GitHub prerelease notes when present
- documents why source builds on `main` still report `0.0.0`
- hardens the Linux installer so non-404 download failures do not silently fall back to legacy asset names
- copies existing `~/.codex` settings/history into `~/.ontocode` on first install when no Ontocode config exists

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh
```

To pin this alpha after publication:

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh -s -- --release 0.1.0-alpha.5
```
