# Lean-ctx Upstream Bootstrap Removal

Date: 2026-06-28
Scope: maintained-fork backend runtime path in `third_party/lean-ctx-fork`

## Decision

Keep the plugin boundary and the in-repo maintained fork, but remove the last
inherited operator path that still depended on upstream release hosting.

## Evidence

- The repo-owned runtime path already works through:
  - `just lean-ctx-plugin-backend-build`
  - `just lean-ctx-plugin-backend-start`
  - `just lean-ctx-plugin-backend-smoke`
- The adopted fork still shipped `third_party/lean-ctx-fork/install.sh` with
  `api.github.com` and `github.com/yvgude/lean-ctx/releases` download logic.
- The adopted fork README still presented upstream install/onboard as the
  primary operator path.

## Change

- `third_party/lean-ctx-fork/install.sh` now fails closed for `--download`
  instead of reaching upstream release infrastructure.
- `third_party/lean-ctx-fork/README.md` now puts the Ontocode-maintained
  runtime flow first and labels the inherited upstream guidance as donor docs.
- Root plugin docs now explicitly reject the inherited upstream downloader as a
  supported path.

## Validation

- `third_party/lean-ctx-fork/install.sh --download` now exits non-zero with
  maintained-fork guidance.
- `just lean-ctx-plugin-backend-smoke` still passes against the in-repo backend.

## Non-blocking decision

Do not investigate the broad `cargo test tool_profiles` host hang in this
slice. The exact relevant scoped profile proof already passed, and this runtime
hardening work did not reproduce a correctness blocker.
