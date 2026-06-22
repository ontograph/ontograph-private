# Ontocode Internal Crate Rename Taxonomy

Date: 2026-06-09

Source plan: `ONTOCODE_INTERNAL_CRATE_RENAME_PROJECT_PLAN.md`

## Status

- Stage: R0D taxonomy and first-slice proposal.
- Implementation status: blocked.
- Scope: planning only; no code edits outside this file.
- Supersession boundary: only deferred internal Rust Cargo package/lib/Bazel crate names are reopened.

## Final Surface Taxonomy

| Class | Examples | Rename rule | Stage |
| --- | --- | --- | --- |
| Cargo package identity | `codex-utils-elapsed`, `codex-ansi-escape` | Rename to `ontocode-*` only in staged crate-family slices. | R1-R5 |
| Rust lib crate identity | `codex_utils_elapsed`, `codex_ansi_escape` | Rename to `ontocode_*` with all Rust imports in the same slice. | R1-R5 |
| Bazel crate identity | `crate_name = "codex_utils_elapsed"` | Keep aligned with Rust lib crate name. | R1-R5 |
| Workspace dependency keys | `codex-utils-elapsed = { workspace = true }` | Rename with the package and all dependent manifests. | R1-R5 |
| Developer package selectors | `just test -p codex-utils-elapsed`, `cargo test -p codex-utils-elapsed` | Rename only after the package exists under the new name. | R1/R6 |
| Helper executable target names | `codex-linux-sandbox`, `codex-shell-escalation` | Rename only in helper/runtime slices after runtime path callers are mapped. | R2/R2B |
| Runtime package layout names | `codex-package.json`, `codex-path`, `codex-resources` | Preserve unless package-layout migration is separately approved. | forbidden |
| Public package identities | `@openai/codex`, `openai-codex`, `openai-codex-cli-bin`, `@openai/codex-sdk` | Preserve; package migration is out of scope. | forbidden |
| SDK/import package names | `openai_codex`, `codex_cli_bin`, `bundled_codex_path()` | Preserve compatibility unless SDK/package ADR approves a migration. | forbidden |
| State/env compatibility | `CODEX_HOME`, `CODEX_*`, `.codex`, `.codex-plugin` | Preserve or alias only under separate state/env migration. | forbidden |
| Wire/protocol/generated names | `codex_app_server_protocol`, protobuf packages, generated model names | Preserve by default; require protocol/schema ADR to rename. | R5B gate |
| Telemetry/analytics shapes | `CodexTurnEventRequest`, event schema field names | Preserve unless telemetry schema versioning is approved. | forbidden |
| Historical docs/ADRs | old `codex` examples in historical decision records | Do not rewrite unless the doc is active operational guidance. | R6 only |

## Forbidden Rename Classes For Low-Level Agents

Low-level implementation agents must not rename any of these strings during crate/package slices:

- `CODEX_*`, including sandbox, npm, bun, auth, and legacy compatibility inputs.
- `.codex`, `.codex-plugin`, persisted rollout/session/config/state keys, and home-directory layout names.
- `codex-package.json`, `codex-package-*`, `codex-path`, `codex-resources`, package archive names, and release asset names.
- npm/Python/SDK package identities and import paths: `@openai/codex`, `openai-codex`, `openai-codex-cli-bin`, `@openai/codex-sdk`, `openai_codex`, `codex_cli_bin`.
- SDK compatibility APIs such as `bundled_codex_path()` unless a package migration explicitly owns the change.
- App-server method names, protocol bundle filenames, protobuf packages, generated SDK protocol model names, and TypeScript schema export identities.
- Telemetry, analytics, metrics, tracing, log schema, and dashboard identifiers.
- Public CLI/runtime command behavior beyond the selected crate package/lib/Bazel rename slice.
- Historical ADR prose and tracking text unless the manager explicitly marks it as active command guidance.
- Any broad `codex` to `ontocode` find-and-replace.

## Crate Risk Buckets

| Bucket | Criteria | Examples | Notes |
| --- | --- | --- | --- |
| Leaf utility proof | No internal `codex-*` dependencies; no package/runtime/protocol/state ownership; low direct reverse-dependency count. | `codex-utils-readiness`, `codex-utils-elapsed`, `codex-utils-fuzzy-match`, `codex-utils-sleep-inhibitor`, `codex-ansi-escape` | Best R1 candidates. |
| Mechanical leaf but central test harness | No internal `codex-*` dependencies, but referenced by many test/process helpers. | `codex-utils-cargo-bin` | Not first slice despite leaf manifest shape. |
| Broad shared utility | Utility crate used across core/app/protocol/provider flows. | `codex-utils-absolute-path`, `codex-utils-string`, `codex-utils-rustls-provider`, `codex-utils-path`, `codex-utils-pty` | Defer until proof slice validates lock/Bazel/import mechanics. |
| Runtime path/package layout | Any crate that detects install method, package layout, arg0 dispatch, helper paths, sandbox runtime, or bundled resources. | `codex-install-context`, `codex-arg0`, `ontocode-exec`, `ontocode-exec-server`, `codex-sandboxing` | Must not be R1. |
| Helper executable | Helper binary crates with sandbox/escalation/platform behavior. | `codex-linux-sandbox`, `ontocode-windows-sandbox`, `codex-shell-escalation` | R2 after leaf proof. |
| CLI/app entry | User-entry orchestration and app-server runtime. | `codex-cli`, `ontocode-tui`, `ontocode-app-server*` | High blast radius. |
| Provider/auth/MCP support | Provider, auth, plugin, MCP, hooks, config owners. | `codex-model-provider`, `codex-login`, `codex-mcp`, `codex-rmcp-client`, `codex-config`, `codex-hooks` | Requires owner-specific tests. |
| Core/shared | Core agent, API/client, state, rollout, tools. | `codex-core`, `codex-core-api`, `codex-api`, `codex-client` | Late-stage only. |
| Protocol/generated | Protocol crates and generated schema/model identities. | `codex-protocol`, `ontocode-app-server-protocol` | Preserve by default; separate ADR required. |

## Challenge Of Manager Recommendation

Manager recommendation:

1. `codex-utils-absolute-path` -> `ontocode-utils-absolute-path`
2. `codex-utils-cargo-bin` -> `ontocode-utils-cargo-bin`
3. `codex-install-context` -> `ontocode-install-context`

Disposition:

| Candidate | Direct evidence | OntoIndex evidence | R1 decision |
| --- | --- | --- | --- |
| `codex-utils-absolute-path` | No internal `codex-*` dependencies, but `cargo metadata` shows 43 direct reverse dependencies across analytics, app-server, protocol, core, config, exec-server, TUI, hooks, plugins, and more. | `AbsolutePathBuf` struct impact returned LOW/0, which conflicts with direct import inventory and is therefore not sufficient. | Defer from first slice; use after smaller proof slice validates mechanics. |
| `codex-utils-cargo-bin` | No internal `codex-*` dependencies, but `cargo metadata` shows 15 direct reverse dependencies and direct search shows first-party binary/test-resource lookup across CLI, app-server, core, rmcp-client, apply-patch, and test harnesses. | `cargo_bin` impact returned CRITICAL: 162 impacted nodes, 52 direct, modules `Tests`, `Suite`, `V2`, `Unified_exec`. | Defer from first slice; safe only after R1 proves test-harness package selectors and Bazel runfiles updates. |
| `codex-install-context` | Depends on `codex-utils-absolute-path` and `codex-utils-home-dir`; `cargo metadata` shows 6 direct reverse dependencies: arg0, CLI, core, linux-sandbox, thread-store, TUI. Source owns `InstallContext`, `CodexPackageLayout`, `codex-package.json`, `codex-path`, `codex-resources`, npm/bun install detection, standalone release layout, and bundled resource lookup. | `InstallContext` impact returned partial LOW/0 with traversal warning `Write operations are not allowed. The pool adapter is read-only`, so graph evidence is unreliable here. | Move out of R1 to R2B runtime-path/package-layout gate. |

Conclusion: the manager’s proposed list is too risky for the first implementation slice. `codex-install-context` must move to R2B, `codex-utils-cargo-bin` must move to a later test-harness utility slice, and `codex-utils-absolute-path` should wait until a small leaf utility proof passes.

## Proposed First Implementation Slice: R1A Leaf Utility Proof

Exact recommended crate list:

| Old package | New package | Old lib crate | New lib crate | Why low-risk |
| --- | --- | --- | --- | --- |
| `codex-utils-readiness` | `ontocode-utils-readiness` | `codex_utils_readiness` | `ontocode_utils_readiness` | `cargo metadata` shows 0 direct reverse dependencies and no internal `codex-*` dependencies. |
| `codex-ansi-escape` | `ontocode-ansi-escape` | `codex_ansi_escape` | `ontocode_ansi_escape` | `cargo metadata` shows 1 direct reverse dependency (`ontocode-tui`); OntoIndex `ansi_escape` impact is LOW with 3 impacted nodes. |
| `codex-utils-elapsed` | `ontocode-utils-elapsed` | `codex_utils_elapsed` | `ontocode_utils_elapsed` | `cargo metadata` shows 1 direct reverse dependency (`ontocode-tui`) and no internal `codex-*` dependencies. |
| `codex-utils-fuzzy-match` | `ontocode-utils-fuzzy-match` | `codex_utils_fuzzy_match` | `ontocode_utils_fuzzy_match` | `cargo metadata` shows 1 direct reverse dependency (`ontocode-tui`) and import use is confined to TUI picker/filter helpers. |
| `codex-utils-sleep-inhibitor` | `ontocode-utils-sleep-inhibitor` | `codex_utils_sleep_inhibitor` | `ontocode_utils_sleep_inhibitor` | `cargo metadata` shows 1 direct reverse dependency (`ontocode-tui`) and no package/runtime/protocol/state ownership. |

Why this slice is a better proof:

- All selected crates are leaf utilities with no internal `codex-*` package dependencies.
- The slice validates all required mechanics: package rename, lib crate rename, root workspace dependency key update, dependent Cargo manifest update, Rust import update, Bazel crate name/deps update, `Cargo.lock`, and `MODULE.bazel.lock` regeneration.
- The dependent surface is intentionally narrow: mostly `ontocode-tui` plus one zero-revdep utility.
- It avoids runtime path detection, package layout, SDK binary lookup, app-server wire/schema identities, provider/auth/MCP owners, and core orchestration.
- It keeps the diff reviewable and avoids the central test-harness blast radius of `codex-utils-cargo-bin`.

Recommended R1A verification:

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-utils-readiness`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-ansi-escape`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-utils-elapsed`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-utils-fuzzy-match`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-utils-sleep-inhibitor`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-tui` until the TUI package itself is renamed; use `ontocode-tui` only after the TUI crate rename stage.
- `cd ontocode-rs && just bazel-lock-update`
- `cd ontocode-rs && just bazel-lock-check`
- OntoIndex `gn_verify_diff` or CLI `detect-changes` for the exact changed files.

## R1 Follow-On Candidates After R1A Passes

| Candidate | Condition |
| --- | --- |
| `codex-utils-absolute-path` | Only after R1A validates Cargo/Bazel/lockfile mechanics; expect broad dependent manifest/import churn. |
| `codex-utils-cargo-bin` | Only after R1A and a test-harness command map exists; OntoIndex CRITICAL impact must be acknowledged before editing. |
| `codex-utils-home-dir` and `codex-utils-path` | Only after `absolute-path` migration is stable because they depend on it. |
| `codex-utils-string`, `codex-utils-template`, `codex-utils-rustls-provider` | Later utility slices; they feed core/protocol/provider/network paths and should not be mixed with R1A. |

## Go/No-Go Checklist For Starting R1

Go only if all are true:

- R0A/R0B/R0C inventories are present or the manager explicitly accepts this R0D proposal as sufficient to start R1A.
- The selected R1 crate list is exactly the R1A list above or a manager-approved subset.
- No runtime-path, package-layout, SDK package, protocol/generated, telemetry, `CODEX_*`, `.codex`, or package identity strings are in scope.
- Every selected crate has an old/new package name, old/new lib crate name, Bazel crate name, dependent manifest list, import list, and scoped test command.
- The worker commits to updating Cargo manifests, Rust imports, Bazel deps, `Cargo.lock`, and `MODULE.bazel.lock` together.
- The worker runs OntoIndex impact before editing any symbol and warns if HIGH/CRITICAL appears.
- The worker plans OntoIndex diff verification before closing the task.
- The worker keeps the change under the project change-size guidance and splits if generated lockfile churn or import updates exceed reviewable size.

No-go if any are true:

- The slice includes `codex-install-context`, `codex-arg0`, `ontocode-exec`, `ontocode-exec-server`, `codex-sandboxing`, `codex-cli`, `codex-core`, `codex-protocol`, or `ontocode-app-server-protocol`.
- The slice requires broad `codex` string replacement.
- The slice renames package manager identities, SDK import paths, generated protocol names, wire identifiers, telemetry shapes, `CODEX_*`, `.codex`, `codex-package-*`, `codex-path`, or `codex-resources`.
- OntoIndex reports HIGH/CRITICAL risk and the manager has not explicitly approved proceeding.
- Bazel lock regeneration or scoped package tests are not planned.

## OntoIndex And Direct Evidence Caveats

- OntoIndex repo snapshot reports repo path `/opt/demodb/_workfolder/ontocode`, repo name `codex`, indexed at `2026-06-09T05:31:30.221Z`.
- OntoIndex package-name modeling is weak for this task: package names may resolve as folders/docs rather than Cargo package dependency edges.
- OntoIndex `AbsolutePathBuf` returned LOW/0 despite direct metadata showing `codex-utils-absolute-path` has 43 direct reverse dependencies; direct Cargo/Bazel inventory must override that graph gap.
- OntoIndex `InstallContext` returned partial LOW/0 with a traversal warning; direct source evidence shows package-layout/runtime-path ownership, so it remains high risk for R1.
- OntoIndex `cargo_bin` returned CRITICAL with 162 impacted nodes and 52 direct impacts; this is a real warning for the `codex-utils-cargo-bin` first-slice proposal.
- Sidecar enrichment was unavailable with reason `missing-store`; no docs/enrichment facts were used.
- Direct evidence used here: `.memory-bank/ontocode_internal_crate_rename-cargo-metadata.json`, `ontocode-rs/**/Cargo.toml`, `ontocode-rs/**/BUILD.bazel`, direct source reads of `install-context`, `utils/absolute-path`, `utils/cargo-bin`, and `ansi-escape`, plus direct reference searches.

## Recommended R1 Exact Crate List

Start R1A with exactly:

1. `codex-utils-readiness` -> `ontocode-utils-readiness`
2. `codex-ansi-escape` -> `ontocode-ansi-escape`
3. `codex-utils-elapsed` -> `ontocode-utils-elapsed`
4. `codex-utils-fuzzy-match` -> `ontocode-utils-fuzzy-match`
5. `codex-utils-sleep-inhibitor` -> `ontocode-utils-sleep-inhibitor`

Do not include `codex-install-context`, `codex-utils-cargo-bin`, or `codex-utils-absolute-path` in R1A.
