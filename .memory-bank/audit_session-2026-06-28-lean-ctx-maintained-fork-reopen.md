# Lean-ctx Maintained Fork Reopen

Date: 2026-06-28
Status: replanned

## Decision

The lean-ctx project is reopened with a new goal:

`Local maintained fork as plugin backend`

This does not undo the earlier shutdown-removal pass. It changes the target
again:

- no upstream dependency restoration
- no native core port
- yes to a forked backend owned by Ontocode
- yes to keeping the plugin and MCP boundary

## Active next task

`L3D-F0` — define the maintained-fork backend contract, carried tool surface,
release/version owner, and fail-closed behavior.
