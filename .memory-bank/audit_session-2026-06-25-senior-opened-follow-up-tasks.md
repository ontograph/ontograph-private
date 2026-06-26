# Senior Opened Follow-Up Tasks

Date: 2026-06-25
Status: opened-bounded-followups

## Scope

- `F5-L` / post-fix legacy-migration verification
- `Alpha Release Readiness`

## Evidence

- OntoIndex freshness stayed current at commit `2e72a6d25e147f0619863e7721107b6f11a87fc2`.
- `F5-L` already proved the remote-compaction blocker and passed the three focused `ontocode-core` regressions after removing the provider-name override in `Session.new_turn_context_from_configuration`.
- The tracker still left the broader `ontocode-core` rerun implicit instead of opening it as a concrete next task.
- Alpha readiness still has one local follow-up available that does not depend on a real `CLAUDE_OAUTH_REDACTED_SAMPLE`: verify the remaining native release-artifact/staging path for `0.1.0-alpha.1`.

## Senior Decision

- Open `F5-M` as the next explicit legacy-migration task: run a package-wide `ontocode-core` rerun with a fresh isolated `TMPDIR`, and do not reopen CRITICAL config/state owners unless that rerun isolates a new smaller failing owner.
- Open one bounded alpha publish-prep task: stage or verify the remaining publish path that depends on `rust-v0.1.0-alpha.1` release artifacts or an explicit workflow artifact URL.
- Keep `Claude OAuth Live Validation` blocked on a real `CLAUDE_OAUTH_REDACTED_SAMPLE`.
- Keep `F6` and `F7` blocked on their recorded versioning/adoption gates.

## Commands

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
env TMPDIR="$(mktemp -d /var/tmp/ontocode-core.XXXXXX)" CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

## Notes

- This was a tracking-only senior opening pass. No new implementation slice was dispatched.
