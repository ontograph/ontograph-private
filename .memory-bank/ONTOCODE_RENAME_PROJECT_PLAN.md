# Ontocode Rename Project Plan

Recommended strategy: a staged dual-brand migration from `codex` to `ontocode`, with compatibility first and irreversible renames last.

## Guiding Decision

- Treat `ontocode` as the new user-facing identity first.
- Treat deep internal renames as optional and high-cost.
- Do not rename stable external or persisted identifiers without an explicit compatibility and rollback design.
- Default end state:
  - user-facing brand: `Ontocode`
  - supported CLI names: `ontocode` and `codex` during transition
  - internal crate prefixes and low-value internal identifiers may remain `codex-*`
  - old persisted state remains readable until a defined removal release

## Surface Taxonomy

Every rename candidate must be classified before implementation:

- Human-facing:
  - docs, titles, help text, screenshots, descriptions
- Local persisted:
  - `~/.codex`, `~/.ontocode`, session state, rollout state, credentials, plugins, local caches
- Published package identity:
  - CLI package names, Python distribution names, install commands
- Wire/protocol:
  - app-server payloads, MCP identifiers, proto package names, resource URIs, headers
- Runtime compatibility:
  - CLI command names, env vars, config keys, config discovery paths
- Internal-only:
  - crate names, Rust module names, internal helper names, test fixtures
- Telemetry/ops:
  - metric names, OAuth labels, service names, logs, analytics dimensions

Each class needs an explicit rule: rename now, alias, version, preserve indefinitely, or defer.

## 1. Define Rename Scope and Compatibility Policy

- Decide what must become `ontocode` immediately versus what can remain `codex` internally.
- Freeze the external compatibility promise:
  - Keep old CLI `codex` working for a defined transition window.
  - Keep reading legacy config/env/session state for a defined transition window.
  - Do not break existing MCP/app-server clients without an alias path.
- Define removal and support horizons up front:
  - `N`: first release that introduces `ontocode`
  - `N+1`: both names fully supported, deprecation warnings allowed
  - `N+2` or later: evaluate removal of selected legacy aliases
- Produce a persisted-state migration policy per state type:
  - config files
  - auth/credentials
  - session/rollout state
  - plugin state
  - MCP config
  - caches/indexes
- For each persisted state type, choose one rule:
  - read old, write old
  - read old, write new
  - read both, write new
  - copy on first run, then write new
  - preserve old indefinitely with no migration
- Produce a rename matrix:
  - User-facing brand
  - CLI/bin names
  - Package names
  - Config paths
  - Env vars
  - Protocol/wire names
  - Internal crate/module names

## 2. Stage 1: User-Facing Rebrand Only

- Change visible product strings from `Codex` to `Ontocode`.
- Update docs, README, install text, screenshots, help output, app titles, and package descriptions.
- Keep all technical identifiers unchanged in this stage.
- Before landing public branding, complete a protocol/integration inventory so the project does not promise a rename while major client-visible `codex` surfaces remain unknown.
- Goal: make the product read as `Ontocode` without breaking anything.

Deliverables:

- docs/README/install copy updated
- CLI/help branding updated
- protocol and integration inventory created
- release notes describing no compatibility break

## 3. Stage 2: Add `ontocode` Aliases

- Add an `ontocode` executable alongside `codex`.
- Make installers and docs prefer `ontocode`.
- Keep `codex` as a supported alias.
- Add tests that both commands invoke equivalent flows.

Deliverables:

- `ontocode` binary entrypoint
- command alias tests
- packaging/install updates
- explicit deprecation wording and support window for `codex`

## 4. Stage 3: Config and Environment Compatibility Layer

- Add support for `~/.ontocode` while still reading `~/.codex`.
- Add support for `ONTOCODE_*` env vars while preserving `CODEX_*`.
- Define precedence rules:
  - new name wins when both are set
  - legacy name remains accepted
- Avoid breaking session resume and rollout storage.
- Add explicit mixed-version behavior rules:
  - old binary writes, new binary reads
  - new binary writes, old binary reads
  - both homes exist
  - both env var families are set
- Avoid split-brain state between `~/.codex` and `~/.ontocode`.

Deliverables:

- config loader fallback/precedence rules
- persisted-state migration design
- migration notes
- tests for legacy/new config resolution
- rollback behavior documented

## 5. Stage 4: External Package and SDK Rename

- Treat package identity changes as a separate breaking-change program, not a routine follow-on task.
- Rename published package identities where needed:
  - Python package metadata
  - CLI package names
  - install references
- Keep compatibility shims where practical.
- Audit any downstream automation that shells out to `codex`.
- Define per-package migration mechanism:
  - alias package
  - transitional metapackage
  - parallel publish
  - hard rename with versioned release notes
- Do not change import/distribution identity until installer, docs, release automation, and upgrade path are validated.

Deliverables:

- package metadata migration
- package compatibility design
- deprecation notices
- install/upgrade path docs

## 6. Stage 5: Protocol and Integration Surface Audit

- Review app-server, MCP, memo/resource URIs, OAuth labels, headers, telemetry/service names, proto package names.
- Split into:
  - safe aliases
  - hard breaks requiring versioning
- Only rename wire-level identifiers if aliases or versioning exist.
- Preserve stable identifiers when the rename value is cosmetic and the compatibility cost is high.

Deliverables:

- integration inventory
- break/no-break classification
- versioning or alias design for unsafe surfaces
- explicit list of permanently preserved `codex` wire identifiers, if any

## 7. Stage 6: Internal Rename Decision

- Re-evaluate whether renaming internal crate prefixes `codex-*` to `ontocode-*` is worth the churn.
- Only proceed if there is strong value after external migration is stable.
- Default answer is no unless there is demonstrated value beyond brand consistency.
- If yes, do it as multiple mechanical PRs by workspace area.

Deliverables:

- go/no-go decision doc
- staged crate rename sequence if approved

## 8. Testing and Rollout

- Add regression coverage for:
  - `codex` and `ontocode` command parity
  - legacy/new config path loading
  - legacy/new env var loading
  - session resume compatibility
  - MCP/app-server compatibility
- Add migration matrix coverage for:
  - old config only
  - new config only
  - both configs present
  - old env vars only
  - new env vars only
  - both env var families set
  - old binary writes, new binary reads
  - new binary writes, old binary reads
  - upgrade from `N-1` to `N`
  - rollback from `N` to `N-1`
- Roll out in phases:
  - internal/dev preview
  - public dual-name release
  - deprecation window
  - optional cleanup release

## Suggested PR Breakdown

1. Surface inventory and compatibility matrix
2. Branding-only copy changes
3. `ontocode` CLI alias and installer updates
4. Config/env dual-read compatibility
5. Persisted-state migration and rollback handling
6. Package identity migration design
7. Protocol/integration aliases or versioning changes
8. Optional internal crate renames

## Risks

- Breaking scripts that invoke `codex`
- Breaking config discovery via `CODEX_HOME` or `~/.codex`
- Breaking persisted sessions/rollouts
- Breaking MCP/app-server clients on renamed wire identifiers
- Publishing/package churn across Python and Rust surfaces

## Recommendation

Stop after Stage 5 unless there is a compelling reason to rename internal crate prefixes. That gets the product to `Ontocode` with much lower risk than a full internal rewrite.

## Adopted Closeout

The project is now considered complete at the public-surface boundary.

- Adopted option:
  - `Ontocode` is the user-facing brand
  - public SDK primary names are `Ontocode*`
  - `ontocode` CLI alias exists alongside `codex`
- Intentionally preserved by design:
  - `codex-*` Rust crate names
  - generated protocol model names
  - published package/runtime identities
  - `CODEX_*` environment variables
  - `~/.codex` persisted state
  - telemetry/event schema names
  - wire/protobuf/MCP/resource identifiers
- Deferred:
  - deeper internal Rust/package rename work remains a separate breaking-change program, not part of this completed rename project

## Acceptance Criteria

- A user can install and run `ontocode` without losing access to existing `codex` state.
- Existing `codex` users can upgrade without manual data migration.
- Mixed legacy/new state does not create duplicate or divergent session/config behavior.
- Published package migration has a documented upgrade path.
- External clients either continue to work unchanged or have a versioned migration path.
- The project has a defined release where each deprecated legacy surface is re-evaluated, rather than left indefinite by accident.
