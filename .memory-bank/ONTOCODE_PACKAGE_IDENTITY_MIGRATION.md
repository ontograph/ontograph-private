# Ontocode Package Identity Migration

## Goal

Define a concrete package identity migration plan for the `codex` to `ontocode` rename across published package surfaces without forcing a one-release hard break.

This document covers:

- npm CLI package(s)
- Python distribution(s)
- TypeScript SDK package(s)
- native runtime packages
- release automation implications
- user upgrade paths

This is a breaking-change program design, not an implementation spec.

## Migration Rules

Each published package must be assigned one of these treatments:

- `alias`: same artifact or equivalent behavior under a new user-facing command/name
- `dual-publish`: publish old and new package identities in parallel from the same source
- `metapackage`: publish a thin compatibility package that depends on the canonical package
- `version`: use a new major-version or protocol version to carry the rename
- `preserve`: keep the existing identity because the compatibility cost is higher than the rename value

Default policy:

- Human-facing install docs should prefer `ontocode` as soon as compatible artifacts exist.
- Existing `codex` package identities must remain installable for at least releases `N` and `N+1`.
- Removal of legacy package identities must not happen before `N+2`, and only after upgrade telemetry and support load are reviewed.
- Import paths and binary names must not be renamed in the same step unless the repo also ships a compatibility alias.

## Package Inventory

### Primary user-facing packages

- npm CLI package: `@openai/codex`
- Python SDK distribution: `openai-codex`
- TypeScript SDK package: `@openai/codex-sdk`

### Runtime and support packages

- Python runtime carrier: `openai-codex-cli-bin`
- npm sidecar binary: `@openai/ontocode-responses-api-proxy`

### Internal and native build/runtime packages

- Rust workspace crates such as `codex-cli`, `codex-core`, `ontocode-app-server`, `ontocode-windows-sandbox`
- helper binaries and native runtime crates used internally by installers or packaged artifacts

These internal/native package names are not a user-facing migration priority and should be preserved unless a later release demonstrates concrete value in changing them.

## Decisions By Package Family

### 1. npm CLI package

Current identity:

- package: `@openai/codex`
- binary names: `codex`, `ontocode`

Decision:

- `preserve` the package identity `@openai/codex` for the first migration wave
- `alias` the executable name by shipping both `codex` and `ontocode`
- optionally `dual-publish` `@openai/ontocode` later, but only after installer and release automation are ready

Rationale:

- npm package renames are breaking for pinned installs, lockfiles, enterprise mirrors, and scripted `npx` or `npm install -g` flows.
- The binary name change gives users the visible rename benefit with much lower risk than a package rename.

Canonical path:

- Phase 1 canonical install remains `@openai/codex`
- Docs prefer running `ontocode` after install
- Phase 2 may introduce `@openai/ontocode` as the preferred install target

Upgrade path:

- Existing users of `@openai/codex` upgrade in place and gain the `ontocode` command
- New users may install `@openai/codex` initially, then `@openai/ontocode` once dual-publish exists
- If `@openai/ontocode` launches, `@openai/codex` should become a compatibility package or continue dual-publishing for at least one additional release train

Recommendation:

- Do not hard-rename the primary npm CLI package in the first rename program

### 2. Python SDK distribution

Current identity:

- distribution: `openai-codex`
- import package: `openai_codex`

Decision:

- `dual-publish` distributions during transition:
  - keep `openai-codex`
  - introduce `openai-ontocode` only if PyPI namespace and release tooling are ready
- `preserve` the Python import path `openai_codex` for the transition window
- only consider a new import path in a separate major-version program

Rationale:

- Python distribution names are moderately migratable, but import path changes are far more disruptive.
- Users can tolerate a new pip package name if the import surface stays stable.

Canonical path:

- Package download identity may move to `openai-ontocode`
- Runtime Python import remains `openai_codex` until a separate major-version decision

Upgrade path:

- `pip install --upgrade openai-codex` continues to work during transition
- New docs can prefer `pip install openai-ontocode` once dual-publish exists
- Both distributions should resolve to the same import package and equivalent runtime behavior

Recommendation:

- Treat distribution rename and import rename as separate programs

### 3. Python runtime carrier

Current identity:

- distribution: `openai-codex-cli-bin`
- installed payload namespace: `codex_cli_bin`

Decision:

- `preserve` the existing package identity through the main rename program
- optionally add a later `metapackage` or `dual-publish` only if the Python SDK distribution moves and packaging UX suffers

Rationale:

- This package is a support/runtime artifact, not a primary user-selected brand surface.
- Renaming it early adds packaging churn without meaningful brand payoff.

Upgrade path:

- Existing SDK dependency pins continue unchanged
- If the Python SDK later publishes as `openai-ontocode`, it may still depend on `openai-codex-cli-bin` until a packaging-only follow-up release

Recommendation:

- Preserve for `N` through `N+2`

### 4. TypeScript SDK package

Current identity:

- package: `@openai/codex-sdk`

Decision:

- `dual-publish` if the TypeScript SDK is intended to become an Ontocode-branded external product
- otherwise `preserve` the package identity and update only descriptions/docs in the first wave
- do not rename exports/import specifiers unless a major-version release is planned

Rationale:

- JavaScript package renames force import-site churn across downstream applications.
- If the product positioning of the SDK is still tied to Codex APIs, a cosmetic package rename is not worth the downstream cost.

Canonical path:

- If renamed, new canonical package becomes `@openai/ontocode-sdk`
- Old package remains published and points to the same code for at least `N` and `N+1`

Upgrade path:

- Existing imports from `@openai/codex-sdk` continue to work
- New applications may adopt `@openai/ontocode-sdk` after dual-publish begins
- Import-path migration should be opt-in and documented, not forced by surprise

Recommendation:

- Only dual-publish if product management explicitly wants the SDK package name to carry the Ontocode brand

### 5. npm sidecar and support binaries

Current identity:

- `@openai/ontocode-responses-api-proxy`
- binary: `ontocode-responses-api-proxy`

Decision:

- `preserve`

Rationale:

- This is an integration/support package, not a primary product entrypoint.
- Renaming low-visibility support packages creates downstream churn in automation and deployment without improving the main user journey.

Upgrade path:

- No rename in the main program
- Revisit only if the main CLI package later hard-renames and support package consistency becomes operationally necessary

### 6. Rust/internal/native runtime packages

Current identities:

- Rust crates and native helpers with `codex-*` names
- platform-specific binaries such as sandbox and proxy helpers

Decision:

- `preserve`

Rationale:

- These are mostly internal build graph or helper identities.
- Renaming them creates high churn across Cargo manifests, release jobs, packaging metadata, and native installers for minimal user-facing benefit.

Upgrade path:

- None in this program

## Release Program

### Release N

- Rebrand user-facing docs to `Ontocode`
- Ship `ontocode` binary alias
- Keep published package identities unchanged by default
- Optional: prepare hidden or preview dual-publish channels for selected packages

### Release N+1

- Introduce any approved dual-published package identities:
  - `@openai/ontocode`
  - `openai-ontocode`
  - `@openai/ontocode-sdk` if approved
- Mark legacy package names as supported but deprecated in release notes and install docs
- Keep all import paths and runtime carrier packages stable

### Release N+2 or later

- Review adoption, support burden, and ecosystem breakage
- Decide per package whether to:
  - keep dual-publishing
  - turn old package into a compatibility metapackage
  - preserve old package indefinitely
  - remove old package only with a major-version or clearly announced breaking release

## Release Automation Implications

Any dual-publish or metapackage plan must be validated against release tooling before rollout.

Required checks:

- npm publish jobs can publish one or two package identities from the same source tree
- PyPI release jobs can publish one or two distributions from the same source tree
- artifact signing, provenance, changelog generation, and tagging continue to work
- install docs, release notes, and package metadata point users to the correct canonical package
- smoke tests cover install and upgrade flows for both old and new package identities

Operational rules:

- Use one source of truth for version numbers across old/new package identities where dual-publishing
- Do not let old and new package lines drift functionally
- Avoid package-specific feature divergence during the migration window

## User Upgrade Paths

### npm CLI users

- Supported path:
  - keep `npm install -g @openai/codex`
  - start running `ontocode`
- Later optional path:
  - switch install command to `npm install -g @openai/ontocode`

### Python SDK users

- Supported path:
  - continue `pip install openai-codex`
  - keep `import openai_codex`
- Later optional path:
  - install `openai-ontocode`
  - keep the same import path during transition

### TypeScript SDK users

- Supported path:
  - continue `npm install @openai/codex-sdk`
- Later optional path if dual-published:
  - new projects may adopt `@openai/ontocode-sdk`
  - existing projects migrate imports when ready

## What Must Not Happen

- Do not rename all package identities in one release.
- Do not change package download identity and import identity in the same step unless the package already has a tested compatibility shim.
- Do not rename internal Rust/native packages merely for brand consistency during the external migration.
- Do not force users to manually uninstall old packages before the new branded path works.

## Recommended End State

- Primary product branding is `Ontocode`
- `ontocode` is the preferred command name
- Legacy package identities remain installable long enough to avoid ecosystem breakage
- New package identities are introduced only where they materially improve user-facing clarity
- Internal/native package identities mostly remain `codex-*`

## Acceptance Criteria

- A current user can upgrade without breaking existing install scripts immediately.
- New users see a coherent Ontocode-branded install path.
- Old and new package identities do not drift in behavior during transition.
- Import-path breaks are isolated to explicit future major-version programs.
- Release automation can publish and test every supported package identity reliably.
