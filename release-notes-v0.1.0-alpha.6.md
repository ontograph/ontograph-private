# Ontocode CLI 0.1.0-alpha.6

This private alpha keeps the Linux release path narrow while the Excel extension work stays out of the shipped app-server surface.

## Included

- Linux x86_64 unsigned `ontocode` binary
- `SHA256SUMS`
- `install-ontocode-linux-x64.sh`
- `install-ontocode-linux-x64-v0.1.0-alpha.6.sh`

## Changes

- keeps release version stamping in the private alpha workflow so source manifests can stay on the `0.0.0` development sentinel
- excludes Excel tools from the app-server extension registry for this alpha cut
- keeps the rest of the current release surface unchanged while Excel work continues out of band

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh
```

To pin this alpha after publication:

```bash
curl -fsSL https://raw.githubusercontent.com/ontograph/ontograph-private/main/scripts/install/install.sh | sh -s -- --release 0.1.0-alpha.6
```
