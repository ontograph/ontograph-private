# Ontocode CLI 0.1.0-alpha.6

This private alpha keeps the shipped surface narrow while Excel tooling stays out of the current release path.

## Included

- Linux x86_64 unsigned `ontocode` binary
- `SHA256SUMS`
- `install-ontocode-linux-x64.sh`
- `install-ontocode-linux-x64-v0.1.0-alpha.6.sh`

## Changes

- keeps release version stamping in the private alpha workflow so source manifests can stay on the `0.0.0` development sentinel
- keeps Excel tools excluded from the current shipped app-server surface
- carries forward the existing private alpha Linux release flow without widening scope

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh
```

To pin this alpha after publication:

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh -s -- --release 0.1.0-alpha.6
```
