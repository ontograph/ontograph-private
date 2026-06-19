#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/codex/scripts/debug-codex.sh" in VSCode settings to always get the 
# latest ontocode-rs binary when debugging Ontocode Extension.


set -euo pipefail

CODEX_RS_DIR=$(realpath "$(dirname "$0")/../ontocode-rs")
(cd "$CODEX_RS_DIR" && cargo run --quiet --bin ontocode -- "$@")
