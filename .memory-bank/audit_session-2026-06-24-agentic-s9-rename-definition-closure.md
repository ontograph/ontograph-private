# Audit Session: Agentic S9 Rename Definition Closure

Date: 2026-06-24

## Scope

Close `AGENTIC-S9`: repo-local `/agent` definition rename.

## Implementation

- Added picker rename rows only for role definitions loaded from repo-root `.codex/agents/*.toml`.
- Added a rename prompt seeded with the current role name.
- Added scaffold rename logic that validates the source path, moves the role file to the new slug, updates only the internal `name = ...` field, rejects destination collisions, and leaves reload semantics unchanged.

## Verification

- Passed: `CARGO_BUILD_JOBS=8 just fmt`
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-tui -- open_agent_picker_exposes_rename_action_for_repo_local_role_definition open_agent_picker_exposes_copy_action_for_repo_local_role_definition rename_agent_definition_scaffold_moves_repo_local_role_file rename_agent_definition_scaffold_rejects_collision copy_agent_definition_scaffold_duplicates_repo_local_role_file`
- Passed: `CARGO_BUILD_JOBS=8 just fix -p ontocode-tui`
- Passed: OntoIndex `gn_test_gap` for the intended TUI files and focused test evidence.
- Passed: scoped OntoIndex `gn_verify_diff` with only the intended AGENTIC-S9 code and memory-bank files.

## Residual Risk

Repo-wide OntoIndex `gn_verify_diff` still fails because the dirty worktree contains many unrelated changes outside this slice. Full `ontocode-tui` verification remains out of scope because the dirty worktree still carries unrelated snapshot drift. Broader `/agent` dispatch/profile/job surfaces remain blocked by the ADR.
