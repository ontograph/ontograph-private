# Ontocode CLI 0.1.0-alpha.7

This private alpha keeps the distribution surface narrow while package and Excel work continues out of band.

## Included

- Linux x86_64 unsigned `ontocode` binary
- `SHA256SUMS`
- `install-ontocode-linux-x64.sh`
- `install-ontocode-linux-x64-v0.1.0-alpha.7.sh`

## Changes

- excludes the maintained lean-ctx backend from packaged release distributions
- keeps release version stamping in the private alpha workflow so source manifests can stay on the `0.0.0` development sentinel
- keeps the rest of the current release surface unchanged

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh
```

To pin this alpha after publication:

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh -s -- --release 0.1.0-alpha.7
```
