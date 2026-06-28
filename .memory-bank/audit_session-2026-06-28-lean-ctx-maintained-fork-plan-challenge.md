# Lean-ctx Maintained Fork Plan Challenge

Date: 2026-06-28
Status: reviewed

## OntoIndex review

- index fresh at current HEAD
- embeddings unavailable
- dirty worktree present, so source reads were used as the decisive evidence

## Findings

1. The reopened plan correctly reuses the existing plugin/MCP owners.
2. The plan overstates implementation readiness after `L3D-F0`.
3. Current source shows the plugin/package path can be restored cleanly, but no
   maintained-fork backend owner or contract exists yet.

## Decision

- keep `L3D-F0` open
- mark `L3D-F1`-`L3D-F3` blocked until the backend contract exists
- require explicit fork repo/home, bounded v1 tool allowlist, version owner,
  token/endpoint policy, startup runbook, and fail-closed behavior before code
  restoration starts
