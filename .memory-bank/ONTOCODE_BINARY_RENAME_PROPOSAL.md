# Ontocode Binary Rename Proposal

Date: 2026-06-08

## Recommendation

Keep both CLI binaries during the current migration:

- `ontocode` is the preferred public command.
- `codex` remains a compatibility command.
- Docs, examples, and install guidance should prefer `ontocode`.
- Rust crate, helper binary, package, telemetry, protocol, and persisted-state identifiers should stay `codex-*` unless a separate versioned migration is approved.

This preserves the visible Ontocode rename while avoiding breakage for existing scripts, package installs, SDK runtime lookup, persisted state, and tests.

## Current State

- `codex-rs/cli/Cargo.toml` already defines both binaries:
  - `codex`
  - `ontocode`
- `codex-rs/cli/src/main.rs` already supports command-name detection through arg0 and `ONTOCODE_CLI_COMMAND_NAME`.
- Existing rename tracking marks the public CLI alias as complete.
- Existing disposition docs defer internal crate/helper renames and preserve package/wire identities.

## GitNexus Evidence

GitNexus context for `command_name_override_from_env` shows the CLI command-name path flows into `command_name_from_arg0`.

GitNexus impact for `command_name_override_from_env` reported:

- risk: `HIGH`
- impacted count: `10`
- direct callers: `1`
- affected process: `cli_main`
- affected modules: `3`

Any change to canonical command-name behavior should therefore be small, tested, and compatibility-preserving.

GitNexus query for the alias and packaging surface also found related code outside the immediate CLI parser:

- alias tests in `codex-rs/cli/tests/ontocode_alias.rs`
- install-context package layout checks in `codex-rs/install-context/src/lib.rs`
- Python runtime binary resolution tests in `sdk/python/tests/test_artifact_workflow_and_binaries.py`
- source binary packaging helpers in `scripts/codex_package/cargo.py`
- doctor installation/update checks in `codex-rs/cli/src/doctor.rs`
- arg0 helper alias logic in `codex-rs/arg0/src/lib.rs`

This confirms the binary name is not only a display concern. Packaging, install detection, SDK runtime lookup, helper discovery, and tests all participate in the migration.

## Review And Challenge

### Finding 1: Stage 1 Is Safe Only If It Stays CLI-Local

Earlier proposal wording allowed `ontocode` updates in generated user-facing text. That is too broad because it can pull in protocol SDK artifacts and schema output.

Decision:

- Limit Stage 1 to CLI help/display and hand-written docs/examples.
- Do not edit generated SDK models, generated protocol files, schema output, or wire identifiers.

### Finding 2: Packaging Alias Work Is Not A Small Task

The proposal says to ensure npm, Python, and native runtime packaging install both names. GitNexus shows this touches install-context, Python runtime lookup, package scripts, and doctor checks.

Decision:

- Treat package aliasing as its own implementation slice.
- A small external agent may inventory and propose exact packaging changes, but should not implement broad package changes in the same pass.

### Finding 3: Package Rename Is Explicitly Out Of Scope

The existing package migration policy preserves package identities in the first wave and only allows dual-publish after release tooling is ready.

Decision:

- Do not rename `@openai/codex`, `openai-codex`, `@openai/codex-sdk`, `codex-cli`, or runtime carrier packages in this task.
- Do not add `@openai/ontocode`, `openai-ontocode`, or `@openai/ontocode-sdk` until a release-program task explicitly approves dual-publish.

### Finding 4: Helper Binary Renames Should Stay Deferred

Helper binaries such as `codex-exec`, `codex-exec-server`, sandbox helpers, and `codex-execve-wrapper` participate in dispatch, packaging, and platform-specific lookup.

Decision:

- Do not rename helper binaries in the public CLI canonicalization slice.
- If a helper rename is later required, add an alias first and run GitNexus impact on each helper lookup path before editing.

## Proposed Stages

### Stage 1: Canonical CLI Display

- Keep both binaries.
- Make `ontocode` the preferred public spelling in CLI help plus hand-written docs and examples where safe.
- Preserve `codex` help/version behavior when invoked through the `codex` binary.
- Keep tests for both command names.
- Scope for a small implementation slice:
  - CLI help/display behavior
  - hand-written CLI docs and examples
  - tests under `codex-rs/cli/tests`
- Explicitly out of scope:
  - package manager identities
  - generated SDK/protocol artifacts
  - helper executable names
  - telemetry, metrics, protocol, or persisted-state identifiers

### Stage 2: Packaging Alias

- Ensure npm, Python, and native runtime packaging install both `codex` and `ontocode` where applicable.
- Keep existing package identities in the first wave:
  - `@openai/codex`
  - `openai-codex`
  - `@openai/codex-sdk`
  - `codex-cli`
- Publish instructions may say: install the existing package, run `ontocode`.

### Stage 3: Optional Package Rename

Only start after release tooling supports dual publish or compatibility packages.

Candidate new identities:

- `@openai/ontocode`
- `openai-ontocode`
- `@openai/ontocode-sdk`

Compatibility requirements:

- Old package identities remain installable for at least two releases.
- Import paths and runtime carrier package names remain stable unless a major-version program approves source-breaking changes.
- Release automation, provenance, signing, changelog, and artifact publishing are validated before rollout.

### Stage 4: Internal Helper Rename

Defer helper and internal binary renames unless there is concrete product or operational value.

Examples to defer:

- `codex-exec`
- `codex-exec-server`
- `codex-linux-sandbox`
- `codex-windows-sandbox`
- `codex-execve-wrapper`
- Rust crates and library names prefixed `codex-*`

If revisited, rename subsystem-by-subsystem with aliases and GitNexus impact analysis before edits.

## External Small Agent Prompt

Use this prompt for a small implementation agent:

```text
Task: tighten Ontocode CLI canonicalization without broad package or helper renames.

Repository: /opt/demodb/_workfolder/ontocode

Read first:
- .memory-bank/MEMORY.md
- .memory-bank/ONTOCODE_BINARY_RENAME_PROPOSAL.md
- .memory-bank/ONTOCODE_RENAME_SURFACE_MATRIX.md
- .memory-bank/ONTOCODE_PACKAGE_IDENTITY_MIGRATION.md
- .memory-bank/ONTOCODE_REMAINING_SURFACES_DISPOSITION.md

Use GitNexus:
- Run `gitnexus status` and confirm the index is up to date.
- Query the CLI alias surface with:
  `gitnexus query 'ontocode binary alias command_name_from_arg0 CLI help packaging tests' -r codex`
- Before editing any Rust symbol, run `gitnexus impact <symbol> --direction upstream -r codex` and report the blast radius.

Allowed scope:
- CLI help/display behavior for the existing `ontocode` alias.
- Hand-written docs/examples that tell users which command to run.
- Focused tests for `codex` and `ontocode` binary parity.

Forbidden scope:
- Do not remove the `codex` binary.
- Do not rename Rust crates or Cargo packages.
- Do not rename helper binaries such as `codex-exec`, `codex-exec-server`, sandbox helpers, or `codex-execve-wrapper`.
- Do not edit generated SDK/protocol artifacts.
- Do not rename npm/Python/TypeScript package identities.
- Do not rename protocol, telemetry, metrics, persisted-state, or wire identifiers.

Expected implementation:
- Verify the current `ontocode` wrapper still resolves the sibling `codex` binary and preserves `ONTOCODE_CLI_COMMAND_NAME=ontocode`.
- If CLI help/display still says `codex` when invoked via `ontocode`, fix only the minimal CLI command-name path.
- Add or update focused tests under `codex-rs/cli/tests` for both binary names.
- Update only hand-written docs/examples where the command shown to users should be `ontocode`.

Verification:
- Run `CARGO_BUILD_JOBS=8 just fmt` from `codex-rs` after code changes.
- Run `CARGO_BUILD_JOBS=8 just test -p codex-cli`.
- Run `gitnexus detect-changes --scope all --repo codex` before finalizing.

Final report:
- Summarize changed files.
- Report GitNexus impact findings for any edited Rust symbols.
- Report tests run and any blockers.
```

## Do Not Do

- Do not remove the `codex` binary in the current migration.
- Do not rename Rust crates with broad search-and-replace.
- Do not rename helper binaries before package, install, and test lookup code supports both names.
- Do not rename protocol, telemetry, metrics, persisted-state, or wire identifiers without versioned compatibility.

## Decision

The next implementation work should make `ontocode` more visibly canonical at the docs/package/display layer while keeping `codex` as a stable compatibility binary.

Broad binary/package/internal object renames should remain a separate, explicitly approved migration program.
