# Ontocode Persisted-State Migration

This document defines the persisted-state migration and rollback policy for the `codex` to `ontocode` transition.

## Goals

- Prevent split-brain state between `~/.codex` and `~/.ontocode`.
- Allow existing `codex` users to upgrade without manual migration.
- Allow rollback from an `ontocode` release to a `codex` release without losing critical state.
- Keep compatibility rules explicit per state class instead of using a single global policy.

## Global Rules

- Legacy home:
  - `~/.codex`
- New home:
  - `~/.ontocode`
- Legacy env var family:
  - `CODEX_*`
- New env var family:
  - `ONTOCODE_*`
- Transition releases:
  - `N`: first release that ships `ontocode`
  - `N+1`: dual-name support remains required
  - `N+2`: removal of selected aliases may be evaluated, but persisted-state read compatibility should remain unless a separate migration program is approved

## Global Precedence

When more than one source is available, resolve in this order:

1. Explicit new env var
2. Explicit legacy env var
3. New home on disk
4. Legacy home on disk
5. Default new home path

Additional rules:

- New names win when both new and legacy names are set.
- Reads may inspect both homes when the state class allows it.
- Writes must target one canonical location per state class.
- Do not sync files bidirectionally on every run.
- Do not delete legacy state automatically.

## Canonical Write Policy

The migration must avoid creating two active mutable homes. The canonical write target is:

- If `ONTOCODE_HOME` is set: write there
- Else if `CODEX_HOME` is set: write there
- Else if `~/.ontocode` exists: write there
- Else if `~/.codex` exists: keep writing there for rollback-sensitive state classes that preserve legacy storage
- Else: create and write `~/.ontocode`

This rule is intentionally overridden for some state classes below.

## State-Class Policy Matrix

| State class | Examples | Migration rule | Canonical write target | Rationale |
| --- | --- | --- | --- | --- |
| Config files | `config.toml`, user settings, profile selection | read both, write new | `~/.ontocode/config.toml` unless explicit home env var points elsewhere | Config should converge on the new home, but legacy config must remain readable during transition. |
| Auth and credentials | `auth.json`, refresh tokens, API auth state | copy on first run, then write new | `~/.ontocode/auth.json` unless explicit home env var points elsewhere | Credentials are high value and must survive upgrade; copy-once avoids long-term dual-store drift. |
| Session and rollout state | threads, resumes, rollout metadata, local conversation state | read both, write new | `~/.ontocode/sessions/` and related new-home state | New releases should converge on one home, but must still discover legacy resumable state. |
| Plugin state | installed plugin metadata, plugin cache, plugin settings under user home | preserve old indefinitely | keep existing plugin state under legacy location unless a future plugin-specific migration is approved | Plugin state has high compatibility risk and low branding value; avoid moving it in this program. |
| MCP config | MCP server definitions, tool config, MCP auth/config fragments | read both, write new | `~/.ontocode/...` MCP config path unless explicit home env var points elsewhere | MCP configuration is user-authored config and should converge with main config. |
| Caches and indexes | search indexes, repo indexes, temporary caches, derived caches | preserve old indefinitely | existing cache location; new cache files may be created in new home without migrating old cache contents | Derived state can be regenerated, so migration effort should be minimized and rollback kept simple. |

## Detailed Rules By State Class

## 1. Config Files

Policy:

- Rule: read both, write new
- Read order:
  1. explicit new-home config
  2. explicit legacy-home config
  3. `~/.ontocode/config.toml`
  4. `~/.codex/config.toml`
- If both new and legacy configs exist:
  - use the new config as authoritative
  - do not merge automatically
  - emit a one-time warning that both exist and the new config wins
- If only legacy config exists:
  - read it
  - write subsequent updates to new config
  - optionally create new config on first mutating write, not necessarily on first read

Rollback:

- A rollback to an older `codex` binary may not see settings changed only in `~/.ontocode/config.toml`.
- To reduce surprise, the first release `N` should avoid rewriting config unless the user mutates config or a new default must be persisted.
- If rollback-safe config mirroring is later desired, it should be a separate scoped project.

## 2. Auth and Credentials

Policy:

- Rule: copy on first run, then write new
- On first successful `ontocode` auth read:
  - if new auth is missing and legacy auth exists, copy legacy auth to new home
  - mark copy completion with a migration marker in the new home
- After copy:
  - reads prefer new auth
  - writes go only to new auth
- If both auth files exist:
  - new auth wins
  - legacy auth is left untouched

Mixed-version behavior:

- Old `codex` login followed by new `ontocode` run:
  - new run copies or reads legacy auth, then converges to new auth
- New `ontocode` login followed by rollback:
  - old `codex` will continue using legacy auth if present
  - if legacy auth is absent, rollback may require re-login

Rollback:

- Do not delete or overwrite legacy auth during migration.
- Copy-on-first-run preserves the best chance of successful rollback without continuous dual writes.

## 3. Session and Rollout State

Policy:

- Rule: read both, write new
- Reads:
  - search new-home session state first
  - if requested session/thread is not present, fall back to legacy state
- Writes:
  - new sessions and updated session state are written only to new home
- Discovery:
  - session listings may include legacy sessions when no migrated copy exists
  - if identical logical session IDs exist in both homes, the new-home copy wins

Mixed-version behavior:

- Old binary writes, new binary reads:
  - supported through legacy read fallback
- New binary writes, old binary reads:
  - not guaranteed for newly created sessions unless the old binary also knows the new home

Rollback:

- Existing legacy sessions remain readable by old versions.
- Sessions created only after migration may not be visible to old versions.
- This is acceptable because rollback compatibility prioritizes preserving pre-existing user state, not perfect forward-backward synchronization.

## 4. Plugin State

Policy:

- Rule: preserve old indefinitely
- Keep plugin state and plugin-local persisted data in the legacy location for this migration program.
- New branding may refer to `Ontocode`, but plugin storage contracts remain `codex`-shaped unless a separate plugin migration is approved.

Rationale:

- Plugin ecosystems often hardcode paths and manifest assumptions.
- Moving plugin state creates a high risk of breakage for low user-visible value.
- A future `.ontocode-plugin` scheme, if ever needed, should be introduced as an additive capability rather than a forced rename.

Rollback:

- Full rollback safety is preserved because plugin state remains where old versions expect it.

## 5. MCP Config

Policy:

- Rule: read both, write new
- Treat MCP config as config, not cache.
- If both old and new MCP configs exist:
  - new wins
  - no auto-merge
  - emit a one-time warning if they materially differ
- If only legacy MCP config exists:
  - read it
  - first mutating write creates new-home MCP config

Mixed-version behavior:

- Existing old MCP setups continue to load under the new binary.
- New MCP changes made only in the new home may not be visible to older binaries unless they support the new home path.

Rollback:

- Preserve old MCP config files unchanged.
- Accept that post-migration edits may not fully roll back without manual copy.

## 6. Caches and Indexes

Policy:

- Rule: preserve old indefinitely
- Do not migrate or copy existing caches/indexes from legacy home to new home.
- New binaries may:
  - continue using legacy cache/index locations, or
  - create fresh derived caches in new locations, depending on implementation needs
- Cached data must always be considered disposable.

Rationale:

- Derived state is not worth migration complexity.
- Copying indexes increases disk usage and migration time.
- Regeneration is simpler than maintaining cache coherence.

Rollback:

- Old caches remain intact.
- New caches may be ignored by old versions.

## Mixed-Version Scenarios

## Upgrade From `codex` To `ontocode`

- If only legacy home exists:
  - config and MCP config are read from legacy
  - auth is copied to new home on first successful access
  - sessions are discovered from legacy and new sessions are written to new home
  - plugin state stays in legacy home
  - caches remain where they already are

## Running Both Binaries In The Same Period

- `codex` continues to use legacy state.
- `ontocode` converges config, auth, session writes, and MCP writes toward the new home.
- Users may observe:
  - shared visibility for old sessions
  - partial divergence for new sessions and new config edits
- This is acceptable during the transition window and should be documented clearly.

## Rollback From `ontocode` To `codex`

- Pre-existing legacy config/auth/session/plugin state remains available.
- New config-only edits and new sessions created only in new home may not be visible after rollback.
- Rollback is expected to preserve old state, not to provide full bidirectional synchronization.

## Conflict Handling

When both old and new state exist for the same class:

- New wins for:
  - config
  - auth
  - session identity collisions
  - MCP config
- Legacy remains canonical for:
  - plugin state
  - preserved caches/indexes when implementations continue to use them

The system should emit concise warnings for:

- both config homes present
- both MCP configs present and different
- both auth files present but materially different

Warnings should be one-time or rate-limited to avoid log spam.

## Migration Markers

Where copy-on-first-run or one-time warnings are used, the new home should store lightweight migration markers.

Allowed uses:

- auth copied from legacy
- legacy/new conflict warning already shown

Markers must:

- be bounded in size
- be purely local metadata
- not be required for correctness

## Test Matrix

| Scenario | Expected result |
| --- | --- |
| Legacy config only, first `ontocode` read | Config loads successfully from legacy. |
| Legacy config only, first config mutation under `ontocode` | New config is created in new home and becomes write target. |
| New config only | New config loads and writes remain in new home. |
| Both configs present | New config wins; one-time warning is emitted. |
| Legacy auth only | First successful auth access copies auth to new home. |
| Both auth files present | New auth wins; legacy auth is untouched. |
| Legacy session only | New binary can discover and resume it. |
| New session created by `ontocode`, then rollback to `codex` | Old binary may not see that new session; pre-existing legacy sessions remain visible. |
| Legacy plugin state only | New binary still uses legacy plugin state. |
| Legacy MCP config only | New binary loads it; first mutating write creates new-home MCP config. |
| Both MCP configs present | New MCP config wins; conflict warning may be emitted. |
| Legacy caches only | New binary either continues using them or regenerates new caches without migration failure. |
| `ONTOCODE_HOME` and `CODEX_HOME` both set | `ONTOCODE_HOME` wins. |
| `ONTOCODE_*` and `CODEX_*` env vars both set for same meaning | `ONTOCODE_*` value wins. |
| No old or new home exists | New home is created and used as default. |
| Mixed-version period with alternating `codex` and `ontocode` runs | No corruption, no auto-delete, documented divergence limited to state classes that write new only. |

## Acceptance Criteria

- Existing `codex` users can upgrade without manually moving config or credentials.
- No automatic migration deletes legacy state.
- The migration does not require continuous dual writes.
- Plugin state remains rollback-safe.
- Auth migration is one-time, explicit, and testable.
- Session resume preserves access to pre-existing legacy sessions.
- Conflict precedence is deterministic and documented.

## Non-Goals

- Full bidirectional synchronization between old and new homes
- Automatic merging of conflicting config files
- Renaming plugin manifest directories in this migration
- Migrating derived caches purely for branding consistency
