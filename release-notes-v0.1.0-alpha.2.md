# Ontocode CLI 0.1.0-alpha.2

This private alpha release focuses on release hygiene and installer reliability.

## Included

- Linux x86_64 unsigned `ontocode` binary
- `SHA256SUMS`
- `install-ontocode-linux-x64.sh`
- `install-ontocode-linux-x64-v0.1.0-alpha.2.sh`

## Changes

- fixed public PR CI jobs that still depended on private runner labels
- fixed the Cargo manifest policy checker to validate current `ontocode-*` package naming
- fixed `ontocode-adapter-protocol` workspace metadata so `cargo-deny` no longer fails on missing license data
- updated the Linux installer to:
  - resolve the newest prerelease by default
  - use bounded curl timeouts and retries
  - show download progress for large binary fetches
  - fall back to the older unversioned binary asset name used by `0.1.0-alpha.1`
- updated the main-branch install command in `README.md` to use the repo install script directly

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh
```
