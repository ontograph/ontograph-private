# Ontocode-Only CLI Hard Cutover Migration And Rollback Plan

Source ADR: `ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`

Worker: HC4

Status: HC10/HC11 preparation; do not mark HC10 or HC11 complete from this document.

## Scope

This plan covers Stage 7 migration and rollback release handling for the proposed Ontocode-only CLI hard cutover.

Write scope for this worker was limited to this file. No implementation code or manager-owned tracking files were edited.

## Recommendation

Original planning recommendation: use `soft-hard` for the first approved release slice.

- `ontocode` becomes the only successful public CLI command.
- `codex` remains for one release only as a migration-error shim.
- npm, Python, native package identities, release asset names, generated protocol bundles, import paths, telemetry keys, and persisted state keys remain unchanged unless T15 or another package-identity program is separately approved.
- A later `hard` release removes the `codex` shim only after product/release owner approval and support/telemetry evidence show acceptable breakage.

Current dispatch update, 2026-06-09: the hard-cutover tracker records release-owner acceptance of `hard` removal. For HC10/HC11, treat `hard` as the selected implementation mode and keep rollback limited to restoring `codex` launcher/bin targets only if package or startup smoke fails. Do not rename packages, generated protocols, import paths, release assets, telemetry keys, persisted state keys, or internal Rust crates as part of either cutover validation or rollback.

Reject any rollback proposal that changes package/protocol identity instead of restoring the removed command launcher/bin surface.

## Release Strategy Options

### Option A: `soft-hard`

Behavior:

- `ontocode` runs normally.
- `codex` exists only as a non-success migration shim.
- `codex --help` must not show normal command help; it must print the migration message and exit non-zero.
- Packages keep their existing identities, but installed command behavior changes according to the selected shim strategy.

Benefits:

- Gives shell scripts, CI jobs, completions, aliases, and enterprise wrappers a visible failure message.
- Keeps rollback small because package identities and internal runtime names remain stable.
- Matches the ADR manager recommendation.

Risks:

- Still breaks automation that requires `codex` to succeed.
- Requires support readiness because users will see a deliberate command failure.

### Option B: `hard`

Behavior:

- `ontocode` runs normally.
- `codex` is not installed as a public command.
- Any existing `codex` command resolution depends on the user's old installation, shell cache, or PATH state.

Benefits:

- Cleaner final state.
- Avoids carrying a temporary shim.

Risks:

- Gives users less diagnostic guidance.
- Makes package/install rollback more urgent if PATH or runtime consumers depended on `codex`.
- Should not be selected before explicit release-owner approval.

## Migration Message Text

Use the same core message across Rust CLI shim, npm launcher, native package shim, release notes, and support macros.

```text
codex has been replaced by ontocode.

Run the same command with ontocode:
  ontocode <args>

Temporary shell workaround:
  alias codex=ontocode

This codex compatibility command will be removed in the next hard-removal release.
```

For non-interactive stderr, use a compact one-line form:

```text
codex has been replaced by ontocode. Run: ontocode <args>. Temporary workaround: alias codex=ontocode
```

Do not mention renamed package names unless T15/package identity migration is separately approved.

## Release Note Outline

Title:

- `Breaking change: ontocode is now the supported CLI command`

Required sections:

- Summary: `ontocode` replaces `codex` as the user-facing command.
- Who is affected: users, CI jobs, shell aliases, scripts, completions, wrappers, and package consumers that invoke `codex`.
- What changed in `soft-hard`: `codex` prints a migration error and exits non-zero for one release.
- What changes in later `hard`: `codex` is removed from installed public command surfaces.
- What did not change: npm/Python package identities, Python import paths, runtime carrier package names, release asset names, protocol metadata, telemetry schemas, and persisted state compatibility remain unchanged unless separately announced.
- Migration steps: replace `codex ...` with `ontocode ...`.
- Temporary workaround: `alias codex=ontocode`.
- Automation migration: update CI scripts, shell completions, PATH checks, wrappers, and pinned command names.
- Validation guidance: run `ontocode --version`, `ontocode --help`, and the team's usual smoke tests.
- Rollback note: release team can restore the `codex` shim/bin without data migration if rollback triggers are met.
- Support escalation: include the rollback owner placeholder and support intake labels once approved.

## Support And Telemetry Assumptions

These assumptions must be confirmed by product/release owners before implementation:

- Support can distinguish `codex` command-removal tickets from unrelated install failures.
- Support has a public macro that includes the migration message, alias workaround, and package identity note.
- Telemetry or release observability can detect at least one of:
  - migration-shim executions;
  - package install failures after upgrade;
  - launcher failures resolving the native runtime;
  - support tickets tagged for `codex` command removal.
- No raw command arguments, tokens, credentials, keychain paths, cookies, authorization headers, or private user data are collected for this migration.
- If telemetry cannot count shim executions safely, support ticket volume and package-install failure rate become the rollback signal.

## Rollback Trigger Threshold

Placeholder threshold until release-owner approval:

- Roll back if command-removal support tickets exceed `[RELEASE_OWNER_TO_SET]` in the first `[WINDOW_TO_SET]` after release.
- Roll back if package install/runtime launch failures increase by more than `[RELEASE_OWNER_TO_SET]%` against the previous comparable release window.
- Roll back immediately if a signed package, installer, Python runtime wheel, or native archive cannot launch `ontocode` on a supported platform.
- Roll back immediately if a required internal consumer still resolves only `codex` and no supported workaround exists.

The final numeric threshold, measurement window, and telemetry source cannot be finalized until product/release owner approval exists.

## Rollback Owner

Rollback owner: `[RELEASE_OWNER_TO_ASSIGN]`

Required deputies:

- npm/package release deputy: `[TO_ASSIGN]`
- Python runtime deputy: `[TO_ASSIGN]`
- native package deputy: `[TO_ASSIGN]`
- Rust CLI/launcher deputy: `[TO_ASSIGN]`
- support/communications deputy: `[TO_ASSIGN]`

Implementation may not proceed until the rollback owner and at least one release-engineering deputy are assigned.

## HC10/HC11 Hard-Removal Rollback Validation Checklist

Use this checklist after HC8 package work and HC9 text/docs cleanup land. It is a validation checklist only; it does not close HC10 or HC11.

### Preconditions

- Confirm the manager-owned rollback threshold is still: restore `codex` launcher/bin targets if any hard-removal package or startup smoke fails.
- Confirm package identities remain frozen: `@openai/codex`, `@openai/codex-*`, `openai-codex`, `openai-codex-cli-bin`, `codex_cli_bin`, `codex-package-*`, and generated `codex_*` protocol/schema bundle names remain unchanged.
- Confirm HC8 did not rename package names, release assets, import paths, protocol metadata, telemetry schemas, persisted state keys, or internal Rust crates.
- Confirm HC9 changed only user-facing command text/examples where appropriate and did not rewrite package install names such as `npm install -g @openai/codex`.
- Confirm no compatibility `codex` command/helper remains in successful hard-removal install surfaces unless the manager explicitly approved a non-public internal compatibility exception.

### Forward Validation Before Considering Rollback

- Run the focused Rust verification matrix selected by the manager, including `codex-cli`, `codex-arg0`, `ontocode-exec`, helper/sandbox crates applicable to the host, and any app-server/TUI startup smoke that consumes runtime paths.
- Stage npm, Python runtime, and native package artifacts from the implementation diff and inspect file lists/bin maps before install smoke.
- Validate `ontocode --help` and `ontocode --version` from staged npm, Python runtime, native package, and direct Rust launcher paths.
- Validate `codex` is absent from public PATH/bin maps in hard mode for npm, Python runtime, native packages, installers, and helper manifests.
- Validate Python compatibility APIs remain import-compatible: `bundled_ontocode_path()` returns the successful executable and `bundled_codex_path()` follows the HC8-approved hard-mode behavior without requiring package identity changes.
- Validate SDK artifact generation and protocol/schema generation do not require a public `codex` executable.
- Run OntoIndex diff verification (`gn_verify_diff` or `ontoindex detect-changes --repo codex`) after implementation diffs exist and attach any caveats to the manager verification note.

### Rollback Trigger

Trigger rollback only when one of these fails in staged artifacts or startup smoke:

- `ontocode` cannot launch from a supported package/install surface.
- staged npm, Python runtime, native package, or installer artifacts cannot expose the selected hard-mode `ontocode` executable.
- SDK artifact generation or runtime discovery cannot proceed without a removed `codex` executable and no approved compatibility API behavior covers it.
- app-server, exec-server, TUI, sandbox, or helper startup smoke fails because a runtime path still requires the removed `codex` launcher/helper name.

Do not trigger rollback merely because old external automation still invokes `codex`; that breakage is accepted by the selected `hard` mode unless release/support owners change the policy.

### Rollback Patch Shape

- Restore only the minimum `codex` launcher/bin targets needed for the failing surface.
- npm rollback: restore `bin.codex` in `@openai/codex`, pointing to the approved existing launcher or a manager-approved shim; keep `bin.ontocode`.
- Python runtime rollback: restore `bin/codex` or `bin/codex.exe` only if runtime/package smoke requires it; preserve `openai-codex-cli-bin`, `codex_cli_bin`, `bundled_ontocode_path()`, and `bundled_codex_path()` compatibility.
- Native package rollback: restore a `codex` alias/launcher beside `ontocode` in `codex-package-*` layouts only for the failing package/startup path.
- Helper rollback: restore only the specific `codex-*` helper manifest/arg0 alias required by failing sandbox/exec startup smoke.
- Do not rename npm/Python/native packages, release assets, generated protocol/schema files, import namespaces, telemetry fields, persisted state, or Rust crates.
- Do not broaden rollback into a find-and-replace of `ontocode` back to `codex`.

### Rollback Validation

- Rebuild or restage only the affected rollback artifacts.
- Prove `ontocode --help` and `ontocode --version` still succeed after rollback.
- Prove the restored `codex` launcher/bin behavior matches the manager-approved rollback policy on the affected surface.
- Re-run the package/startup smoke that triggered rollback and confirm it passes.
- Re-run the smallest focused Rust/package tests for the restored launcher/helper path.
- Re-run OntoIndex diff verification after the rollback patch and report that package/protocol identities stayed unchanged.

## Exact Rollback Actions

### npm CLI Package

Rollback objective: restore `codex` command availability without changing package identity.

Actions:

- Restore the `codex` bin entry in `codex-cli/package.json` pointing to the same launcher as `ontocode`, or to a migration shim if `soft-hard` remains selected.
- Keep package identity `@openai/codex`.
- Keep platform optional dependency identities such as `@openai/codex-linux-x64`.
- Keep launcher update guidance installable through `@openai/codex` unless T15 is separately approved.
- Rebuild and stage the npm package.
- Validate the staged tarball bin map contains the rollback-approved entries.
- Validate `npx`/global install can run `codex --version` or the approved migration shim and `ontocode --version`.
- Republish or promote the rollback package according to release-engineering policy.

### Python Runtime

Rollback objective: restore runtime discovery compatibility for SDK/tooling that expects `codex` names.

Actions:

- Preserve or restore `openai-codex-cli-bin` as the runtime carrier distribution.
- Preserve or restore `codex_cli_bin` as the import package.
- Preserve or restore `bundled_codex_path()` semantics; it may return the rollback `codex` shim path or the canonical `ontocode` binary path if compatibility tests approve that behavior.
- Keep `bundled_ontocode_path()` if it already exists; do not remove it as part of rollback.
- Keep `codex-package-*` release asset lookup unless the package-identity program separately approves dual asset names.
- Rebuild the runtime wheel.
- Validate SDK artifact generation still imports `bundled_codex_path()` and resolves an executable path.
- Validate both SDK runtime smoke and package layout tests before release promotion.

### Native Packages And Archives

Rollback objective: restore a `codex` executable on PATH while keeping native package identities stable.

Actions:

- Restore a `codex` launcher/shim beside `ontocode` in native package payloads.
- Point the `codex` shim to the same native executable as `ontocode`, or to the approved migration-error shim for `soft-hard`.
- Keep native package/archive identities stable unless T15 separately approves native identity changes.
- Keep `codex-package-*` artifact names if Python runtime setup still consumes them.
- Validate Linux, macOS, and Windows package layouts expose the approved executables.
- Validate shell PATH invocation works after a clean install and after upgrade from the previous dual-bin release.
- Validate code signing/notarization/provenance covers the restored shim where applicable.

### Rust Launcher And Shim

Rollback objective: restore the public command entrypoint with minimal Rust churn.

Actions:

- Restore the `codex` binary target or launcher shim.
- For `soft-hard`, keep the shim non-success and print the approved migration message.
- For emergency compatibility rollback, allow the shim to delegate to the `ontocode` binary while preserving argv forwarding.
- Do not rename Rust crates, internal path field names, protocol metadata, telemetry schemas, or persisted state keys as part of rollback.
- Preserve internal field-name quarantine: fields such as `codex_self_exe` may point to `ontocode`/shim paths until a separate cleanup is approved.
- Validate `ontocode --help` shows `ontocode`.
- Validate `codex --help` either shows the approved migration error or succeeds only if emergency compatibility rollback was explicitly approved.
- Validate arg0/helper dispatch, sandbox, exec-server, app-server, and TUI runtime path tests required by the ADR.

## What Cannot Be Finalized Yet

These items require product/release owner approval:

- Selected removal mode: `soft-hard` or immediate `hard`.
- Exact release number and release train for migration.
- Exact duration of the migration shim.
- Numeric rollback threshold and measurement window.
- Rollback owner and deputies.
- Support-ticket labels, public support macro, and escalation routing.
- Telemetry source for migration-shim invocation or fallback support-only signal.
- Whether emergency rollback may restore successful `codex` delegation instead of only the migration-error shim.
- Whether any package identity or release asset rename is in scope; default is no.

## Validation Plan For This Document

Read/search checks used:

- Read `.memory-bank/MEMORY.md`.
- Read `.memory-bank/ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER.md`.
- Read `.memory-bank/ADR_ONTOCODE_ONLY_CLI_HARD_CUTOVER_TRACKING.md`.
- Read `.memory-bank/ONTOCODE_RENAME_TRACKING.md`.
- Read `.memory-bank/ONTOCODE_PACKAGE_IDENTITY_MIGRATION.md`.
- Searched repository for npm bin entries, npm launcher/update text, Python runtime discovery, `codex-package-*`, and CLI/helper surfaces.
- Used OntoIndex semantic search for user-facing command/help/completion/update-message surfaces; results were useful but degraded because embeddings were unavailable and worktree overlay was dirty.

Implementation validation still required before any code change:

- OntoIndex impact for target symbols in the ADR Stage 0 list.
- Removal matrix approval.
- Package/runtime validation plan approval.
- `gn_verify_diff` or `ontoindex detect-changes` after any implementation diff.

## Release Blockers

- ADR is proposed and not approved for implementation.
- Selected removal mode is unset.
- Product/release owner approval is missing.
- Rollback owner and threshold are missing.
- Package identity scope is not approved for command removal.
- Support and telemetry assumptions are not approved.
- Package/runtime validation has not been completed.
- OntoIndex evidence is available with caveats and cannot replace required implementation-time impact checks.

## Proceed/No-Proceed Decision

Implementation may not proceed unless all ADR go/no-go gates are satisfied.

Planning may continue. Code changes, package bin removal, helper alias removal, release-note publication, and rollback execution must remain blocked until the release owner approves the mode, threshold, owner, and validation plan.
