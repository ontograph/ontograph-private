# R5AN Process Hardening Rename Risk Review

Date: 2026-06-12
Status: approved for identity-only dispatch with process/security guardrails
Model fallback: `gpt-5.4-mini` because the required Spark model is unavailable or usage-limited.

## Scope

- Rename Cargo package `codex-process-hardening` to `ontocode-process-hardening`.
- Rename Rust crate import `codex_process_hardening` to `ontocode_process_hardening`.
- Update workspace metadata, dependent imports, README crate references, and Bazel crate identity.
- Preserve the existing `process-hardening` folder path.

## Direct Inventory

- Direct reverse dependencies: `ontocode-linux-sandbox`, `ontocode-responses-api-proxy`.
- Active refs are in workspace metadata, the process-hardening manifest/Bazel/README identity, linux-sandbox dependency/proxy-routing usage, and responses-api-proxy dependency/main/README usage.

## OntoIndex Impact

- `pre_main_hardening`: LOW, 1 impacted symbol, 1 direct, no affected processes.
- `disable_process_dumping`: LOW, 5 impacted symbols, 1 direct, 2 modules, no affected processes.

## Guardrails

- Identity-only rename: do not change prctl, ptrace, setrlimit, env-var removal, exit-code, or platform cfg behavior.
- Preserve process dump disabling, core dump disabling, LD_/DYLD_ stripping, and non-UTF-8 env-key tests.
- Preserve public proxy binary/npm/docs/config compatibility surfaces.
- Do not touch sandbox environment variable code.
- Do not rename other residual `codex-*` packages.
- Run package tests, linux-sandbox and responses-api-proxy dependent checks, fmt, Bazel lock update/check, stale-reference classification, `git diff --check`, and OntoIndex diff detection before closure.
