# Lean-ctx In-Repo Backend Move

Date: 2026-06-28
Task: `L3D-F4`
Status: complete

## Landed changes

- imported the maintained lean-ctx backend snapshot into
  `third_party/lean-ctx-fork/`
- kept the backend outside `ontocode-rs/` and outside the repo-local plugin
  package
- rewrote current maintained-fork guidance from `../lean-ctx-fork` to
  `third_party/lean-ctx-fork`

## Validation

- verified `third_party/lean-ctx-fork/README.md` exists after import
- ran file-scoped `git diff --check` on the changed current-source docs and
  memory files
- confirmed the old `../lean-ctx-fork` path no longer appears in current
  maintained-fork guidance files

## Closure basis

The maintained backend is now owned inside this repository as a companion
subtree, so the plugin path no longer depends on a sibling checkout. The
existing plugin/MCP boundary, separate startup rule, allowlist, and fail-closed
behavior remain unchanged.
