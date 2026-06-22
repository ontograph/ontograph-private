# Ontocode CLI

Ontocode CLI is an independent private fork of the Codex CLI codebase. It is
published from `ontograph/ontograph-private` and is not an official OpenAI,
Azure, ChatGPT, npm, Homebrew, or IDE distribution.

The canonical private binary is `ontocode`. The legacy `codex` command may
still exist in source and compatibility paths while the rename is completed.

## Alpha Status

This repository is in private alpha. Treat public OpenAI Codex installers,
package-manager commands, release workflows, and hosted documentation as
upstream references only; they do not publish this fork.

Current private alpha scope:

- unsigned Linux release binary first
- macOS, Windows, and platform npm packages later when needed
- release builds run in single-build mode with `CARGO_BUILD_JOBS=8`

## Build From Source

Install the current private Linux x64 alpha release:

```bash
gh auth login
scripts/install/install.sh --release 0.1.0-alpha.1
```

Run from the repository root:

```bash
cd ontocode-rs
CARGO_BUILD_JOBS=8 cargo build --release -p ontocode-cli --bin ontocode
```

Expected binary:

```text
ontocode-rs/target/release/ontocode
```

Run it with:

```bash
./target/release/ontocode --help
```

## Private Alpha Release

The private alpha workflow is `.github/workflows/private-alpha-release.yml`.
It builds an unsigned Linux `ontocode` binary on GitHub-hosted runners and
uploads workflow artifacts. Tag-triggered runs can upload assets to the private
GitHub prerelease.

## Development

Rust workspace:

```bash
cd ontocode-rs
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p <crate-you-touched>
```

Do not use broad OpenAI/Azure release workflows for this private fork unless a
dedicated compatibility task explicitly re-enables them.

## License

This repository is licensed under the [Apache-2.0 License](LICENSE).
