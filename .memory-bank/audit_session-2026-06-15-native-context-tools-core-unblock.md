---
name: Native Context Tools Core Verification Unblock
description: Resolution note for the C0 broad ontocode-core verification blocker
type: audit_session
date: 2026-06-15
status: done
---

# Native Context Tools Core Verification Unblock

Resolved blockers:

- The `test-binary-support` setup failure was caused by duplicate `ontocode-execve-wrapper` arg0 aliases. `ontocode-arg0` now installs one execve-wrapper alias, and `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0` passes.
- Broad `ontocode-core` test failures after that were environmental: a reused temp root contained `.codex`, so sibling tempdirs looked like project/config roots. Run broad core tests with a fresh temp root, for example:
  `env TMPDIR="$(mktemp -d /var/tmp/ontocode-core.XXXXXX)" CARGO_BUILD_JOBS=8 just test -p ontocode-core`.

Verification:

- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0` passed.
- `env TMPDIR="$(mktemp -d /var/tmp/ontocode-core.XXXXXX)" CARGO_BUILD_JOBS=8 just test -p ontocode-core` passed: 2666 passed, 14 skipped.

