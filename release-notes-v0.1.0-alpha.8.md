# Ontocode CLI 0.1.0-alpha.8

This private alpha promotes the native `lctx` and `ontograph` tool slices and keeps packaged distributions free of the removed lean-ctx backend resource.

## Included

- Linux x86_64 unsigned `ontocode` binary
- `SHA256SUMS`
- `install-ontocode-linux-x64.sh`
- `install-ontocode-linux-x64-v0.1.0-alpha.8.sh`

## Changes

- registers native `lctx.read` in the app-server extension registry
- registers native `ontograph.discover`, `ontograph.explain_module`, `ontograph.impact`, `ontograph.inspect`, and `ontograph.search` in the app-server extension registry
- carries the new `ontocode-lctx-extension` and `ontocode-ontograph-extension` crates into the workspace and app-server package set
- removes maintained lean-ctx backend packaging hooks and package metadata fields from the private alpha distribution path

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh
```

To pin this alpha after publication:

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh -s -- --release 0.1.0-alpha.8
```
