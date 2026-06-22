# Ontocode Full Legacy Migration Release And Versioning Gates

Date: 2026-06-14
Task: F0-E-C
Status: proposed

## Purpose

Define the release/versioning owner gates required before F4-F6 implementation can start in
`ONTOCODE_FULL_LEGACY_MIGRATION_PROJECT_PLAN.md`.

This file does not approve implementation. It records the minimum owner decisions needed to
unblock package migration, env/state migration, protocol/schema migration, and telemetry migration.

## Inputs

- `ONTOCODE_FULL_LEGACY_MIGRATION_PROJECT_PLAN.md`
- `ONTOCODE_FULL_LEGACY_MIGRATION_STAGE0_REVIEW.md`
- `ONTOCODE_FULL_LEGACY_MIGRATION_MATRIX_PACKAGES_STATE_PROTOCOL.md`
- `ONTOCODE_PACKAGE_IDENTITY_MIGRATION.md`
- `ONTOCODE_REMAINING_SURFACES_DISPOSITION.md`
- `ALPHA_RELEASE_READINESS.md`

OntoIndex context was checked without running `ontoindex analyze`:

- `codex-cli/scripts/build_npm_package.py::stage_sources`
- `sdk/python/scripts/update_sdk_artifacts.py::generate_v2_all`
- `ontocode-rs/state/src/runtime.rs::StateRuntime.init_inner`
- `ontocode-rs/app-server-protocol/tests/schema_fixtures.rs::assert_schema_fixtures_match_generated`
- `ontocode-rs/analytics/src/events.rs::TrackEventRequest`
- `ontocode-rs/analytics/src/client.rs::send_track_events`
- `find_codex_home` was ambiguous across config and home-dir owners and must be disambiguated before edits.

## Cross-Stage Gate

F4-F6 remain blocked until the migration tracker records named release/versioning owners for all
four gates below.

Each owner approval must specify:

- approved compatibility mode: preserve, alias, dual-publish, metapackage, or version
- release train where the behavior may ship
- rollback path
- required tests and artifact smoke checks
- exact surfaces that remain intentionally legacy-named

Default rule:

- Preserve external contracts unless a named owner approves a versioned migration with compatibility coverage.

## Gate 1: Package Migration

Stage:

- F4 package identity migration

Required owner decision:

- Name the release/package migration owner for npm, Python, TypeScript SDK, runtime carrier, native platform packages, and release asset names.
- Decide whether `@openai/ontocode`, `openai-ontocode`, and `@openai/ontocode-sdk` are in scope for dual-publish in the target release train.
- Decide whether legacy package identities stay primary, become compatibility metapackages, or dual-publish from the same source.

Default senior recommendation:

- Preserve `@openai/codex`, `openai-codex`, `openai-codex-cli-bin`, `codex_cli_bin`, and `@openai/codex-sdk` for the first migration wave.
- Prefer the `ontocode` binary and user-facing command text while package download identities remain compatible.
- Introduce new package identities only after release automation can publish, sign, test, and roll back both old and new identities from one source.
- Treat Python distribution rename and Python import-path rename as separate programs; keep `openai_codex` stable.

Compatibility requirements:

- Existing npm, PyPI, SDK, and runtime installs must keep working for at least releases N and N+1.
- Legacy package identities must not be removed before N+2, and only after adoption, support load, and release telemetry are reviewed.
- Old and new package identities must not drift in version, contents, binary behavior, schemas, or runtime layout.
- Import paths must remain stable unless a future major-version API migration ships tested aliases.
- `codex-package.json`, `codex-resources`, and `codex-path` must remain readable unless a package layout version and dual-reader support are approved.

Tests required:

- npm package staging and bin-map tests for the approved `codex` and `ontocode` behavior.
- npm optional-dependency/platform package resolution tests.
- install smoke for every published identity that remains supported.
- Python wheel/sdist metadata tests.
- Python SDK artifact workflow tests.
- Python runtime carrier layout and binary resolver tests.
- TypeScript SDK build/test/import compatibility tests if a new SDK package name is introduced.
- Release artifact signing/provenance smoke for dual-published or metapackage outputs.

Blocked without explicit release owner approval:

- Hard-renaming or unpublishing `@openai/codex`.
- Hard-renaming or unpublishing `openai-codex`.
- Hard-renaming or unpublishing `openai-codex-cli-bin`.
- Renaming `openai_codex` or `codex_cli_bin`.
- Renaming platform package families from `@openai/codex-*` to `@openai/ontocode-*`.
- Renaming `codex-package` layout files or resource directories.
- Shipping any dual-publish path without local staging and install smoke for both identities.

## Gate 2: Env And State Migration

Stage:

- F5 env, state, config, and file layout migration

Required owner decision:

- Name the state/env migration owner for home resolution, env aliases, persisted state, logs, cache, rollout/session paths, diagnostics, and managed package paths.
- Decide the deprecation window for legacy env names and whether any old state readers are ever removable.
- Decide how diagnostics present old and new names without exposing private paths or secrets.

Default senior recommendation:

- Keep `ONTOCODE_HOME` primary where it already exists and keep reading `CODEX_HOME`.
- Add `ONTOCODE_*` aliases only per owner-reviewed env var; do not rename sandbox env constants.
- Preserve existing database filenames and old state readers indefinitely unless migration tooling proves no data loss and rollback.
- Treat `.codex` user state as a compatibility root, not cleanup debt.

Compatibility requirements:

- New installs should default to Ontocode paths.
- Existing users with `CODEX_HOME`, `~/.codex`, old sessions, old rollouts, logs, cache, memories, goals, and SQLite files must continue to load without manual migration.
- Env precedence must be deterministic when both old and new names are set.
- Diagnostics must redact secrets, credentials, keychain paths, and private paths where required.
- `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` and `CODEX_SANDBOX_ENV_VAR` remain out of scope and must not be modified.

Tests required:

- home-dir precedence tests for `ONTOCODE_HOME`, `CODEX_HOME`, existing `~/.ontocode`, existing `~/.codex`, and default new-home behavior.
- config load tests for old and new home overrides.
- state migration tests for all runtime SQLite databases.
- resume-session and rollout compatibility tests from legacy state.
- app-server daemon/control-dir path tests where affected.
- cache/log/memory/goal path tests where affected.
- doctor/auth/config diagnostics tests with no-secret and no-private-path assertions.

Blocked without explicit release owner approval:

- Removing `CODEX_HOME` support.
- Stopping reads from existing `~/.codex`.
- Rewriting or deleting legacy state without a no-data-loss migration and rollback test.
- Renaming DB filenames only for branding.
- Changing sandbox env constants.
- Emitting raw credentials, tokens, keychain paths, or private user paths in migration diagnostics.

## Gate 3: Protocol And Schema Migration

Stage:

- F6 protocol, generated SDK, and schema versioning

Required owner decision:

- Name the protocol/schema versioning owner for app-server protocol schemas, generated Python and TypeScript models, schema bundle names, SDK generation, MCP/resource identifiers, and wire-visible names.
- Decide whether Ontocode names require a new protocol/schema version, generated aliases, or permanent preservation of legacy names.
- Decide whether schema bundle filenames may change and how old consumers resolve old bundles.

Default senior recommendation:

- Preserve current app-server schema bundle names, generated `Codex*` model names, SDK import paths, MCP/resource IDs, and wire method names for the first migration wave.
- Only introduce Ontocode protocol names through a versioned schema or explicit generated alias layer.
- Do not hand-edit generated output as a rename shortcut.

Compatibility requirements:

- Existing app-server clients must continue to load old schema bundles and generated model names.
- SDK generation must remain source-of-truth driven from schema generation, not local generated-file edits.
- Any new schema bundle name must ship alongside the old name until downstream consumers have migrated.
- Wire method/resource IDs must remain stable unless a new protocol version or alias is approved.
- `.codex-plugin` and similar integration contracts must remain readable unless a versioned manifest transition exists.

Tests required:

- `just test -p ontocode-app-server-protocol`.
- `just write-app-server-schema` when schema output changes.
- schema fixture tests proving old and new bundle behavior where applicable.
- Python SDK generation tests for generated model names and schema bundle lookup.
- TypeScript schema generation/import tests where affected.
- MCP integration tests if resource IDs or tool metadata change.
- Backward-compatibility tests for old clients or fixtures when a new version is introduced.

Blocked without explicit release owner approval:

- Renaming generated `Codex*` protocol model names.
- Renaming `codex_app_server_protocol*` schema bundle files.
- Changing SDK import paths as part of schema rename work.
- Renaming app-server wire methods or resource IDs.
- Removing legacy schema fixtures or generated aliases before a versioned migration window ends.
- Hand-editing generated output to make a local-only rename.

## Gate 4: Telemetry Migration

Stage:

- F6 telemetry versioning

Required owner decision:

- Name the analytics/telemetry schema owner for event request structs, event names, endpoints, runtime metadata fields, and metric names.
- Decide whether telemetry remains legacy-named, adds an Ontocode event version, or dual-writes during a migration window.
- Decide downstream analytics compatibility, dashboards, alerting, retention, and backfill behavior before any rename.

Default senior recommendation:

- Preserve existing telemetry schemas, event names, endpoints, metric names, and `codex_rs_version` metadata in the first migration wave.
- Add Ontocode telemetry only as a versioned schema or dual-write path after analytics owner approval.
- Prefer release adoption telemetry for migration decisions before removing compatibility surfaces.

Compatibility requirements:

- Existing analytics ingestion, dashboards, alerts, and retention jobs must continue to consume old event shapes.
- New event names or payload fields must have an explicit schema version and downstream owner approval.
- Dual-write must prove old and new streams are equivalent before any old stream is removed.
- Telemetry payloads must not leak tokens, cookies, authorization headers, keychain paths, or raw credentials.

Tests required:

- analytics serialization JSON/snapshot tests for current and any new event version.
- endpoint selection tests if endpoint paths change.
- event batching and send-path tests around `send_track_events`.
- DB metric name tests where `codex.*` metrics are touched.
- redaction tests proving sensitive values do not appear in analytics payloads.
- downstream contract smoke or fixture approval for renamed events.

Blocked without explicit release owner approval:

- Renaming `Codex*` telemetry request structs as a wire-visible schema change.
- Renaming analytics event names, endpoint paths, or metric prefixes.
- Renaming `codex_rs_version` or equivalent runtime metadata fields.
- Removing old telemetry stream ingestion before dual-write and dashboard parity are proven.
- Treating low code-level impact as sufficient reason to rename analytics schemas.

## F4-F6 Unblock Checklist

F4 can start only when:

- A release/package migration owner is recorded.
- The owner approves per-package compatibility mode.
- Release automation can prove every supported package identity can be staged, tested, signed, and rolled back.

F5 can start only when:

- A state/env migration owner is recorded.
- The owner approves env precedence, deprecation windows, and no-data-loss requirements.
- Legacy state and diagnostics test plans are accepted.

F6 protocol/schema work can start only when:

- A protocol/schema owner is recorded.
- The owner chooses preserve, alias, or version for every generated and wire-visible name.
- Schema generation and old-client compatibility tests are defined.

F6 telemetry work can start only when:

- An analytics/telemetry schema owner is recorded.
- The owner approves preserve, version, or dual-write behavior.
- Downstream ingestion, dashboard, alerting, and redaction test requirements are accepted.

Final removal remains blocked until:

- At least one release train validates Ontocode surfaces.
- Adoption telemetry and support load are reviewed.
- The responsible owner approves removal for the specific legacy surface.
