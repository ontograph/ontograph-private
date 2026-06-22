# Lean-ctx Pre-Junior Tracking

Source plan: [LEAN_CTX_PRE_JUNIOR_PROJECT_PLAN.md](LEAN_CTX_PRE_JUNIOR_PROJECT_PLAN.md)

Date opened: 2026-06-15
Status: complete

## Ledger

| ID | Task | Status | Owner | Notes |
| --- | --- | --- | --- | --- |
| PJ-PRE | Donor source preflight | done | gpt-5.4-mini worker | Recorded `audit_session-2026-06-15-lean-ctx-pre-junior-preflight.md`; no donor code copied. |
| PJ-0 | Create CLI skeleton | done | gpt-5.4-mini worker | Added stdlib-only read-only script: `scripts/onto_memory_tools.py`. |
| PJ-1 | Add status-digest | done | gpt-5.4-mini worker | Verified `status-digest` prints bounded memory-bank summary. |
| PJ-2 | Add count-left | done | gpt-5.4-mini worker | Verified heuristic counts over approved tracking files. |
| PJ-3 | Add doc-link-check | done | gpt-5.4-mini worker | Verified local `.memory-bank` markdown link checker; current repo has pre-existing broken links. |
| PJ-4 | Add minimal tests | done | gpt-5.4-mini worker | Added `unittest` coverage for command surface and temp-fixture link/count behavior; local test run passed. |
| PJ-5 | Memory-bank update | done | manager | Added `MEMORY.md` link to `scripts/onto_memory_tools.py`. |
| PJ-6 | Final verification and OntoIndex refresh | done | manager | Helper tests and `git diff --check` passed; `doc-link-check` correctly reports pre-existing broken memory-bank links. Final OntoIndex refresh completed. |

## Guardrails

- Keep this as one repository-only Python script plus optional tests.
- Do not add dependencies, Rust code, app-server APIs, MCP tools, config keys, or model-visible tools.
- Do not write memory-bank files from the script.
