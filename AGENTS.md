# Rust/ontocode-rs

## Memory Bank

Use `.memory-bank/` as the project memory layer for this repository.

- At the start of any non-trivial task, read `.memory-bank/MEMORY.md` first, then open only the linked memory files relevant to the task.
- Treat `.memory-bank/MEMORY.md` as the short index; keep it concise and link-oriented.
- Treat `.memory-bank/CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md` as the authoritative dispatch/status file for the current project plan. Memory-bank files summarize and route context; they do not replace tracking files, ADRs, or GitNexus.
- Update `.memory-bank/project_plan-current.md` and `.memory-bank/project_pending-tasks.md` when project-plan status, counts, next steps, or dispatch order changes.
- Update `.memory-bank/project_architecture.md` when an architecture owner, flow, or change-home rule changes.
- Add an `.memory-bank/audit_session-YYYY-MM-DD-*.md` entry for major closure, verification, or decision events, and link it from `.memory-bank/MEMORY.md`.
- Keep memory-bank updates factual and compact; do not paste long logs, full diffs, or raw test output.
- Do not store secrets, tokens, credentials, cookies, authorization headers, keychain paths, or raw private user data in memory-bank files.
- When memory-bank content conflicts with code, GitNexus, ADRs, or tracking files, verify from the authoritative source and update the stale memory entry.

In the ontocode-rs folder where the rust code lives:

- Current source names may still use the legacy `codex-*` prefix, but the active rename goal is to move them to `ontocode-*` as compatibility allows.
- Treat `codex-core` -> `ontocode-core` as an active migration target. Do not introduce new `codex-core` references unless a compatibility boundary still requires the old name.
- When using format! and you can inline variables into {}, always do that.
- Install any commands the repo relies on (for example `just`, `rg`, or `cargo-insta`) if they aren't already available before running instructions here.
- When running build or test commands that compile code, limit build parallelism to 8 CPUs. For Cargo/Just commands, set `CARGO_BUILD_JOBS=8`; for Bazel commands, pass `--jobs=8`.
- For user-facing binary build/run guidance, build and run only the release binary. Use `cargo build --release -p ontocode-cli --bin ontocode` and the `target/release/ontocode` artifact; omit debug build/run steps unless the user explicitly asks for debug diagnostics.
- For compilation, build, packaging, install, or binary-run tasks, always deliver the exact user-facing command(s), required working directory or `--manifest-path`, and expected binary or artifact path in the final response, even if you already ran the command yourself.
- For private fork releases, do not depend on the official OpenAI release workflow, OpenAI runner labels, Azure signing secrets, npm trusted publishing, winget, or dev-site deploys. Use a minimal private-fork path on GitHub-hosted runners, build unsigned artifacts, and add macOS/Windows/platform npm only when that fork has its own release infrastructure.
- Never add or modify any code related to `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` or `CODEX_SANDBOX_ENV_VAR`.
  - You operate in a sandbox where `CODEX_SANDBOX_NETWORK_DISABLED=1` will be set whenever you use the `shell` tool. Any existing code that uses `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` was authored with this fact in mind. It is often used to early exit out of tests that the author knew you would not be able to run given your sandbox limitations.
  - Similarly, when you spawn a process using Seatbelt (`/usr/bin/sandbox-exec`), `CODEX_SANDBOX=seatbelt` will be set on the child process. Integration tests that want to run Seatbelt themselves cannot be run under Seatbelt, so checks for `CODEX_SANDBOX=seatbelt` are also often used to early exit out of tests, as appropriate.
- Always collapse if statements per https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_if
- Always inline format! args when possible per https://rust-lang.github.io/rust-clippy/master/index.html#uninlined_format_args
- Use method references over closures when possible per https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure_for_method_calls
- Avoid bool or ambiguous `Option` parameters that force callers to write hard-to-read code such as `foo(false)` or `bar(None)`. Prefer enums, named methods, newtypes, or other idiomatic Rust API shapes when they keep the callsite self-documenting.
- When you cannot make that API change and still need a small positional-literal callsite in Rust, follow the `argument_comment_lint` convention:
  - Use an exact `/*param_name*/` comment before opaque literal arguments such as `None`, booleans, and numeric literals when passing them by position.
  - Do not add these comments for string or char literals unless the comment adds real clarity; those literals are intentionally exempt from the lint.
  - The parameter name in the comment must exactly match the callee signature.
  - You can run `just argument-comment-lint` to run the lint check locally. This is powered by Bazel, so running it the first time can be slow if Bazel is not warmed up, though incremental invocations should take <15s. Most of the time, it is best to update the PR and let CI take responsibility for checking this (or run it asynchronously in the background after submitting the PR). Note CI checks all three platforms, which the local run does not.
- When possible, make `match` statements exhaustive and avoid wildcard arms.
- Newly added traits should include doc comments that explain their role and how implementations are expected to use them.
- Discourage both `#[async_trait]` and `#[allow(async_fn_in_trait)]` in Rust traits.
  - Prefer native RPITIT trait methods with explicit `Send` bounds on the returned future, as in `3c7f013f9735` / `#16630`.
  - Preferred trait shape:
    `fn foo(&self, ...) -> impl std::future::Future<Output = T> + Send;`
  - Implementations may still use `async fn foo(&self, ...) -> T` when they satisfy that contract.
  - Do not use `#[allow(async_fn_in_trait)]` as a shortcut around spelling the future contract explicitly.
- When writing tests, prefer comparing the equality of entire objects over fields one by one.
- Do not add general product or user-facing documentation to the `docs/` folder. The official Codex documentation lives elsewhere. The exception is app-server API documentation, which is covered by the app-server guidance below.
- Prefer private modules and explicitly exported public crate API.
- If you change `ConfigToml` or nested config types, run `just write-config-schema` to update `ontocode-rs/core/config.schema.json`.
- When working with MCP tool calls, prefer using `ontocode-rs/codex-mcp/src/mcp_connection_manager.rs` to handle mutation of tools and tool calls. Aim to minimize the footprint of changes and leverage existing abstractions rather than plumbing code through multiple levels of function calls.
- Do not call `reset_client_session` unnecessarily; let the incremental check logic decide whether to reuse the previous request.
- If you change Rust dependencies (`Cargo.toml` or `Cargo.lock`), run `just bazel-lock-update` from the
  repo root to refresh `MODULE.bazel.lock`, and include that lockfile update in the same change.
- After dependency changes, run `just bazel-lock-check` from the repo root so lockfile drift is caught
  locally before CI.
- Bazel does not automatically make source-tree files available to compile-time Rust file access. If
  you add `include_str!`, `include_bytes!`, `sqlx::migrate!`, or similar build-time file or
  directory reads, update the crate's `BUILD.bazel` (`compile_data`, `build_script_data`, or test
  data) or Bazel may fail even when Cargo passes.
- Do not create small helper methods that are referenced only once.
- Avoid large modules:
  - Prefer adding new modules instead of growing existing ones.
  - Target Rust modules under 500 LoC, excluding tests.
  - If a file exceeds roughly 800 LoC, add new functionality in a new module instead of extending
    the existing file unless there is a strong documented reason not to.
  - This rule applies especially to high-touch files that already attract unrelated changes, such
    as `ontocode-rs/tui/src/app.rs`, `ontocode-rs/tui/src/bottom_pane/chat_composer.rs`,
    `ontocode-rs/tui/src/bottom_pane/footer.rs`, `ontocode-rs/tui/src/chatwidget.rs`,
    `ontocode-rs/tui/src/bottom_pane/mod.rs`, and similarly central orchestration modules.
  - When extracting code from a large module, move the related tests and module/type docs toward
    the new implementation so the invariants stay close to the code that owns them.
  - Avoid adding new standalone methods to `ontocode-rs/tui/src/chatwidget.rs` unless the change is
    trivial; prefer new modules/files and keep `chatwidget.rs` focused on orchestration.
- When running Rust commands (e.g. `just fix` or `just test`) be patient with the command and never try to kill them using the PID. Rust lock can make the execution slow, this is expected.

Run `just fmt` (in the `ontocode-rs` directory) automatically after you have finished making code changes anywhere in this repository; do not ask for approval to run it. Additionally, run the tests:

1. Do not run `cargo test` directly. Use `just test` so test execution follows the repo defaults.
2. Run the test for the specific project that was changed. For example, if changes were made in `ontocode-rs/tui`, run `just test -p ontocode-tui`.
3. Once those pass, if any changes were made in common, core, or protocol, run the complete test suite with `just test`. Avoid `--all-features` for routine local runs because it expands the build matrix and can significantly increase `target/` disk usage; use it only when you specifically need full feature coverage. project-specific or individual tests can be run without asking the user, but do ask the user before running the complete test suite.

Before finalizing a large change to `ontocode-rs`, run `just fix -p <project>` (in `ontocode-rs` directory) to fix any linter issues in the code. Prefer scoping with `-p` to avoid slow workspace‑wide Clippy builds; only run `just fix` without `-p` if you changed shared crates. Do not re-run tests after running `fix` or `fmt`.

## The `codex-core` crate

Over time, the current `codex-core` crate (rename target: `ontocode-core`, defined in `ontocode-rs/core/`) has become bloated because it is the largest crate, so it is often easier to add something new to `codex-core` rather than refactor out the library code you need so your new code neither takes a dependency on, nor contributes to the size of, `codex-core`.

To that end: **resist adding code to codex-core**!

Particularly when introducing a new concept/feature/API, before adding to `codex-core`, consider whether:

- There is an existing crate other than `codex-core` that is an appropriate place for your new code to live.
- It is time to introduce a new crate to the Cargo workspace for your new functionality. Refactor existing code as necessary to make this happen.

Likewise, when reviewing code, do not hesitate to push back on PRs that would unnecessarily add code to `codex-core`.

## Architecture Reuse Rules

Before implementing provider, auth, MCP, hooks, shell, session/context, diagnostics, or external-agent import work:

- For all development tasks in this repository, default to a bounded manager loop using OntoIndex.
  Bind roles exactly as follows:
  - manager: current session
  - senior-reviewer: `gemini-pro-agent`
  - implementation-worker: `gpt-5.3-codex-spark` high, then `gpt-5.4-mini` high
  - verification-worker: `gpt-5.4-mini` high
  Use the exact model names listed above; do not truncate them.
  Continue in this order:
  - if `active_next_task` exists, execute it
  - else if the last decision was no-dispatch, reply with the exact reopen gate
  - else refuse to rewrite tracking without new evidence
- Treat every proposed change or refactor as invalid until it passes both checks:
  - it adds real new functionality, behavior, safety, compatibility, or operational value rather than cosmetic churn or duplicate plumbing
  - it extends the existing core solution and stays inline with the current architecture instead of introducing a parallel owner or side stack
- If a proposal fails either check, do not implement it as-is; inline it into the existing owner, redesign it to fit the current architecture, or drop it.
- If a request passes both checks, implement the full user-requested scope inside the existing owner when reasonably possible. Do not shrink a valid request to a narrower slice solely to reduce change size, review effort, or local verification scope.
- Reuse existing architecture first. Do not create a second provider factory, provider registry, model catalog, runtime stream abstraction, capability resolver, OAuth token parser, credential persistence layer, redactor, MCP status pipeline, hook matcher, hook registry, policy evaluator, shell permission parser, shell launcher, context injection path, or external-agent import service.
- Use GitNexus `context` on the target symbol/module and record the existing caller/callee surface before implementation. If editing a symbol, run GitNexus impact first and report the blast radius.
- Extend the existing owner when it exists: provider work belongs in `model-provider`; OAuth persistence belongs in auth/login or provider auth boundaries; MCP work belongs in `rmcp-client`, `codex-mcp`, or existing MCP processors; hook work belongs in `hooks`; shell/sandbox work belongs beside the existing runtime and sandbox modules; context work belongs in session/context modules; external-agent import work belongs in the existing migration/import services.
- Add a new module only when the current owner would become too large or mix unrelated concepts. The new module must plug into the existing owner rather than bypassing it.
- Prefer existing test harnesses and fixtures. If a new helper is necessary, document why existing helpers cannot express the case.
- Public config keys, app-server APIs, SDK behavior, schemas, dashboards, wizards, support bundles, and export paths require an ADR and compatibility tests before implementation.
- Security-sensitive diagnostics must reuse or extend shared sanitization/redaction behavior. Tests must fail if a token, cookie, authorization header, keychain path, or raw credential value appears in output.
- Anything injected into model context must use bounded context fragment architecture with hard caps.

## Third-Party Tool Migration Rule

The current project goal for external tool migration is to remove runtime
dependencies on third-party tools and upstream projects. Donor repositories are
source evidence only unless this repo explicitly adopts the code.

- Do not add a required runtime dependency on an external third-party CLI,
  daemon, package, hosted service, source checkout, or release stream.
- When moving external functionality into Ontocode, adopt the minimum required
  legacy code into this repository and expose it through a repo-owned plugin or
  existing plugin/backend owner.
- The adopted plugin must be self-contained from Ontocode's perspective: no
  dependency on the upstream project staying available, no plugin shell-out to
  an external checkout, and no hidden download step for normal use.
- Keep the plugin boundary unless an ADR proves that the functionality belongs
  in an existing native owner. Do not copy a donor runtime into `ontocode-core`
  as a shortcut.
- Strip or replace donor features that require external accounts, telemetry,
  update channels, background services, broad shell execution, or unrelated
  package managers unless a bounded ADR explicitly approves that surface.
- Preserve compatibility shims and provenance notes when adopting legacy code,
  but make the maintained Ontocode path the authority.

## Ontocode Rename Rule

- Treat `Ontocode` / `ontocode` as the target project identity for rename and migration work.
- When a task requires renaming code objects, prefer the new Ontocode name for crates, modules, types, functions, commands, package metadata, docs, and user-visible surfaces unless a compatibility boundary requires the old name.
- The current rename goal explicitly includes `codex-core` -> `ontocode-core`. Prefer `ontocode-core` in new planning, docs, and migration work unless a compatibility boundary still requires the old crate name.
- Never rename code objects with broad find-and-replace. Use GitNexus rename/impact analysis and preserve compatibility shims where external integrations, persisted state, config keys, CLI commands, app-server APIs, package names, or rollout/session data still depend on the old name.
- Before removing any old-name alias, document the migration path and verify affected execution flows with GitNexus `detect_changes`.

## Code Review Rules

### Model visible context

Codex maintains a context (history of messages) that is sent to the model in inference requests.

1. No history rewrite - the context must be built up incrementally.
2. Avoid frequent changes to context that cause cache misses.
3. No unbounded items - everything injected in the model context must have a bounded size and a hard cap.
4. No items larger than 10K tokens.
5. Highlight new individual items that can cross >1k tokens as P0. These need an additional manual review.
6. All injected fragments must be defined as structs in `core/context` and implement ContextualUserFragment trait

### Breaking changes

Search for breaking changes in external integration surfaces:

- app-server APIs
- CLI parameters
- configuration loading
- resuming sessions from existing rollouts

### Test authoring guidance

For agent changes prefer integration tests over unit tests. Integration tests are under `core/suite` and use `test_codex` to set up a test instance of codex.

Features that change the agent logic MUST add an integration test:

- Provide a list of major logic changes and user-facing behaviors that need to be tested.

If unit tests are needed, put them in a dedicated test file (\*\_tests.rs).
Avoid test-only functions in the main implementation.

Check whether there are existing helpers to make tests more streamlined and readable.

### Change size guidance (800 lines)

Unless the change is mechanical, prefer changes under 800 lines when that still satisfies the requested scope cleanly.
For complex logic changes, prefer changes under 500 lines when that still satisfies the requested scope cleanly.

If the change is larger, first check whether the full requested scope can still land safely as one coherent change. Split into stages only when staging is required by correctness, risk, dependency order, or reviewability, and make the stages map to the real requested end state rather than an arbitrarily smaller subset.
Base any staging suggestion on the actual diff, dependencies, affected call sites, and the user's requested outcome.

## TUI style conventions

See `ontocode-rs/tui/styles.md`.

## TUI code conventions

- Use concise styling helpers from ratatui’s Stylize trait.
  - Basic spans: use "text".into()
  - Styled spans: use "text".red(), "text".green(), "text".magenta(), "text".dim(), etc.
  - Prefer these over constructing styles with `Span::styled` and `Style` directly.
  - Example: patch summary file lines
    - Desired: vec!["  └ ".into(), "M".red(), " ".dim(), "tui/src/app.rs".dim()]

### TUI Styling (ratatui)

- Prefer Stylize helpers: use "text".dim(), .bold(), .cyan(), .italic(), .underlined() instead of manual Style where possible.
- Prefer simple conversions: use "text".into() for spans and vec![…].into() for lines; when inference is ambiguous (e.g., Paragraph::new/Cell::from), use Line::from(spans) or Span::from(text).
- Computed styles: if the Style is computed at runtime, using `Span::styled` is OK (`Span::from(text).set_style(style)` is also acceptable).
- Avoid hardcoded white: do not use `.white()`; prefer the default foreground (no color).
- Chaining: combine helpers by chaining for readability (e.g., url.cyan().underlined()).
- Single items: prefer "text".into(); use Line::from(text) or Span::from(text) only when the target type isn’t obvious from context, or when using .into() would require extra type annotations.
- Building lines: use vec![…].into() to construct a Line when the target type is obvious and no extra type annotations are needed; otherwise use Line::from(vec![…]).
- Avoid churn: don’t refactor between equivalent forms (Span::styled ↔ set_style, Line::from ↔ .into()) without a clear readability or functional gain; follow file‑local conventions and do not introduce type annotations solely to satisfy .into().
- Compactness: prefer the form that stays on one line after rustfmt; if only one of Line::from(vec![…]) or vec![…].into() avoids wrapping, choose that. If both wrap, pick the one with fewer wrapped lines.

### Text wrapping

- Always use textwrap::wrap to wrap plain strings.
- If you have a ratatui Line and you want to wrap it, use the helpers in tui/src/wrapping.rs, e.g. word_wrap_lines / word_wrap_line.
- If you need to indent wrapped lines, use the initial_indent / subsequent_indent options from RtOptions if you can, rather than writing custom logic.
- If you have a list of lines and you need to prefix them all with some prefix (optionally different on the first vs subsequent lines), use the `prefix_lines` helper from line_utils.

## Tests

### Test module organization

- When adding a new test module, define its contents in a separate sibling file rather than inline in the implementation file.
- Use an explicit `#[path = "..._tests.rs"]` attribute so the test filename is descriptive and easy to locate:

  ```rust
  #[cfg(test)]
  #[path = "parser_tests.rs"]
  mod tests;
  ```

- This applies only when introducing a new test module. Do not move or rewrite existing inline `#[cfg(test)] mod tests { ... }` modules solely to follow this convention.

### Snapshot tests

This repo uses snapshot tests (via `insta`), especially in `ontocode-rs/tui`, to validate rendered output.

**Requirement:** any change that affects user-visible UI (including adding new UI) must include
corresponding `insta` snapshot coverage (add a new snapshot test if one doesn't exist yet, or
update the existing snapshot). Review and accept snapshot updates as part of the PR so UI impact
is easy to review and future diffs stay visual.

When UI or text output changes intentionally, update the snapshots as follows:

- Run tests to generate any updated snapshots:
  - `just test -p ontocode-tui`
- Check what’s pending:
  - `cargo insta pending-snapshots -p ontocode-tui`
- Review changes by reading the generated `*.snap.new` files directly in the repo, or preview a specific file:
  - `cargo insta show -p ontocode-tui path/to/file.snap.new`
- Only if you intend to accept all new snapshots in this crate, run:
  - `cargo insta accept -p ontocode-tui`

If you don’t have the tool:

- `cargo install --locked cargo-insta`

### Test assertions

- Tests should use pretty_assertions::assert_eq for clearer diffs. Import this at the top of the test module if it isn't already.
- Prefer deep equals comparisons whenever possible. Perform `assert_eq!()` on entire objects, rather than individual fields.
- Avoid mutating process environment in tests; prefer passing environment-derived flags or dependencies from above.

### Spawning workspace binaries in tests (Cargo vs Bazel)

- Prefer `codex_utils_cargo_bin::cargo_bin("...")` over `assert_cmd::Command::cargo_bin(...)` or `escargot` when tests need to spawn first-party binaries.
  - Under Bazel, binaries and resources may live under runfiles; use `codex_utils_cargo_bin::cargo_bin` to resolve absolute paths that remain stable after `chdir`.
- When locating fixture files or test resources under Bazel, avoid `env!("CARGO_MANIFEST_DIR")`. Prefer `codex_utils_cargo_bin::find_resource!` so paths resolve correctly under both Cargo and Bazel runfiles.

### Integration tests (core)

- Prefer the utilities in `core_test_support::responses` when writing end-to-end Codex tests.

- All `mount_sse*` helpers return a `ResponseMock`; hold onto it so you can assert against outbound `/responses` POST bodies.
- Use `ResponseMock::single_request()` when a test should only issue one POST, or `ResponseMock::requests()` to inspect every captured `ResponsesRequest`.
- `ResponsesRequest` exposes helpers (`body_json`, `input`, `function_call_output`, `custom_tool_call_output`, `call_output`, `header`, `path`, `query_param`) so assertions can target structured payloads instead of manual JSON digging.
- Build SSE payloads with the provided `ev_*` constructors and the `sse(...)`.
- Prefer `wait_for_event` over `wait_for_event_with_timeout`.
- Prefer `mount_sse_once` over `mount_sse_once_match` or `mount_sse_sequence`

- Typical pattern:

  ```rust
  let mock = responses::mount_sse_once(&server, responses::sse(vec![
      responses::ev_response_created("resp-1"),
      responses::ev_function_call(call_id, "shell", &serde_json::to_string(&args)?),
      responses::ev_completed("resp-1"),
  ])).await;

  codex.submit(Op::UserTurn { ... }).await?;

  // Assert request body if needed.
  let request = mock.single_request();
  // assert using request.function_call_output(call_id) or request.json_body() or other helpers.
  ```

## App-server API Development Best Practices

These guidelines apply to app-server protocol work in `ontocode-rs`, especially:

- `app-server-protocol/src/protocol/common.rs`
- `app-server-protocol/src/protocol/v2.rs`
- `app-server/README.md`

### Core Rules

- All active API development should happen in app-server v2. Do not add new API surface area to v1.
- Follow payload naming consistently:
  `*Params` for request payloads, `*Response` for responses, and `*Notification` for notifications.
- Expose RPC methods as `<resource>/<method>` and keep `<resource>` singular (for example, `thread/read`, `app/list`).
- Always expose fields as camelCase on the wire with `#[serde(rename_all = "camelCase")]` unless a tagged union or explicit compatibility requirement needs a targeted rename.
- Exception: config RPC payloads are expected to use snake_case to mirror config.toml keys (see the config read/write/list APIs in `app-server-protocol/src/protocol/v2.rs`).
- Always set `#[ts(export_to = "v2/")]` on v2 request/response/notification types so generated TypeScript lands in the correct namespace.
- Never use `#[serde(skip_serializing_if = "Option::is_none")]` for v2 API payload fields.
  Exception: client->server requests that intentionally have no params may use:
  `params: #[ts(type = "undefined")] #[serde(skip_serializing_if = "Option::is_none")] Option<()>`.
- Keep Rust and TS wire renames aligned. If a field or variant uses `#[serde(rename = "...")]`, add matching `#[ts(rename = "...")]`.
- For discriminated unions, use explicit tagging in both serializers:
  `#[serde(tag = "type", ...)]` and `#[ts(tag = "type", ...)]`.
- Prefer plain `String` IDs at the API boundary (do UUID parsing/conversion internally if needed).
- Timestamps should be integer Unix seconds (`i64`) and named `*_at` (for example, `created_at`, `updated_at`, `resets_at`).
- For experimental API surface area:
  use `#[experimental("method/or/field")]`, derive `ExperimentalApi` when field-level gating is needed, and use `inspect_params: true` in `common.rs` when only some fields of a method are experimental.

### Client->server request payloads (`*Params`)

- Every optional field must be annotated with `#[ts(optional = nullable)]`. Do not use `#[ts(optional = nullable)]` outside client->server request payloads (`*Params`).
- Optional collection fields (for example `Vec`, `HashMap`) must use `Option<...>` + `#[ts(optional = nullable)]`. Do not use `#[serde(default)]` to model optional collections, and do not use `skip_serializing_if` on v2 payload fields.
- When you want omission to mean `false` for boolean fields, use `#[serde(default, skip_serializing_if = "std::ops::Not::not")] pub field: bool` over `Option<bool>`.
- For new list methods, implement cursor pagination by default:
  request fields `pub cursor: Option<String>` and `pub limit: Option<u32>`,
  response fields `pub data: Vec<...>` and `pub next_cursor: Option<String>`.

### Development Workflow

- Update app-server docs/examples when API behavior changes (at minimum `app-server/README.md`).
- Regenerate schema fixtures when API shapes change:
  `just write-app-server-schema`
  (and `just write-app-server-schema --experimental` when experimental API fixtures are affected).
- Validate with `just test -p ontocode-app-server-protocol`.
- Avoid boilerplate tests that only assert experimental field markers for individual
  request fields in `common.rs`; rely on schema generation/tests and behavioral coverage instead.

## Python Development Best Practices

### Ignore Python 2 compatibility

This project uses Python 3+. You should not use the `__future__` module.

If you need to worry about feature compatibility between different 3.xx point releases, check the
closest `pyproject.toml`'s `requires-python` field to see what minimum runtime version is supported.

<!-- gitnexus:start -->
# GitNexus — Code Intelligence

This project is indexed by GitNexus as **codex** (72144 symbols, 164215 relationships, 300 execution flows). Use the GitNexus MCP tools to understand code, assess impact, and navigate safely.

> If any GitNexus tool warns the index is stale, run `gitnexus analyze --skills --skip-agents-md` in terminal first.

## Always Do

- **MUST run impact analysis before editing any symbol.** Before modifying a function, class, or method, run `gitnexus_impact({target: "symbolName", direction: "upstream"})` and report the blast radius (direct callers, affected processes, risk level) to the user.
- **MUST run `gitnexus_detect_changes()` before committing** to verify your changes only affect expected symbols and execution flows.
- **MUST warn the user** if impact analysis returns HIGH or CRITICAL risk before proceeding with edits.
- When exploring unfamiliar code, use `gitnexus_query({query: "concept"})` to find execution flows instead of grepping. It returns process-grouped results ranked by relevance.
- When you need full context on a specific symbol — callers, callees, which execution flows it participates in — use `gitnexus_context({name: "symbolName"})`.

## Never Do

- NEVER edit a function, class, or method without first running `gitnexus_impact` on it.
- NEVER ignore HIGH or CRITICAL risk warnings from impact analysis.
- NEVER rename symbols with find-and-replace — use `gitnexus_rename` which understands the call graph.
- NEVER commit changes without running `gitnexus_detect_changes()` to check affected scope.

## Resources

| Resource | Use for |
|----------|---------|
| `gitnexus://repo/codex/context` | Codebase overview, check index freshness |
| `gitnexus://repo/codex/clusters` | All functional areas |
| `gitnexus://repo/codex/processes` | All execution flows |
| `gitnexus://repo/codex/process/{name}` | Step-by-step execution trace |

## CLI

| Task | Read this skill file |
|------|---------------------|
| Understand architecture / "How does X work?" | `.claude/skills/gitnexus/gitnexus-exploring/SKILL.md` |
| Blast radius / "What breaks if I change X?" | `.claude/skills/gitnexus/gitnexus-impact-analysis/SKILL.md` |
| Trace bugs / "Why is X failing?" | `.claude/skills/gitnexus/gitnexus-debugging/SKILL.md` |
| Rename / extract / split / refactor | `.claude/skills/gitnexus/gitnexus-refactoring/SKILL.md` |
| Tools, resources, schema reference | `.claude/skills/gitnexus/gitnexus-guide/SKILL.md` |
| Index, status, clean, wiki CLI commands | `.claude/skills/gitnexus/gitnexus-cli/SKILL.md` |

<!-- gitnexus:end -->

<!-- ontoindex:start -->
# OntoIndex — Code Intelligence

This project is indexed by OntoIndex as **codex** (82466 symbols, 214973 relationships, 300 execution flows). Use the OntoIndex MCP tools to understand code, assess impact, and navigate safely.

> If any OntoIndex tool warns the index is stale, coordinate first; exactly one process should run `ontoindex analyze`.

## Always Do

- **MUST run impact analysis before editing any symbol.** Before modifying a function, class, or method, run MCP `impact({action: "symbol", repo: "codex", target: "symbolName", direction: "upstream"})` or CLI `ontoindex impact --repo codex <symbol>`, then report the blast radius (direct callers, affected processes, risk level) to the user.
- **MUST run MCP `gn_verify_diff({repo: "codex", scope: "all"})` or CLI `ontoindex detect-changes --repo codex` before committing** to verify your changes only affect expected symbols and execution flows.
- **MUST warn the user** if impact analysis returns HIGH or CRITICAL risk before proceeding with edits.
- When exploring unfamiliar code, use MCP `search({action: "semantic", repo: "codex", query: "concept"})` to find execution flows instead of grepping. It returns process-grouped results ranked by relevance.
- When you need full context on a specific symbol — callers, callees, which execution flows it participates in — use MCP `inspect({action: "context", repo: "codex", target: "symbolName"})`.

## Never Do

- NEVER edit a function, class, or method without first running MCP `impact` or CLI `ontoindex impact` on it.
- NEVER ignore HIGH or CRITICAL risk warnings from impact analysis.
- NEVER rename symbols with find-and-replace — use MCP `refactor({action: "rename", ...})` which understands the call graph.
- NEVER commit changes without running MCP `gn_verify_diff` or CLI `ontoindex detect-changes` to check affected scope.

## Resources

| Resource | Use for |
|----------|---------|
| `ontoindex://repo/codex/context` | Codebase overview, check index freshness |
| `ontoindex://repo/codex/clusters` | All functional areas |
| `ontoindex://repo/codex/processes` | All execution flows |
| `ontoindex://repo/codex/process/{name}` | Step-by-step execution trace |

## CLI

| Task | Read this skill file |
|------|---------------------|
| Understand architecture / "How does X work?" | `.claude/skills/ontoindex/ontoindex-exploring/SKILL.md` |
| Blast radius / "What breaks if I change X?" | `.claude/skills/ontoindex/ontoindex-impact-analysis/SKILL.md` |
| Trace bugs / "Why is X failing?" | `.claude/skills/ontoindex/ontoindex-debugging/SKILL.md` |
| Rename / extract / split / refactor | `.claude/skills/ontoindex/ontoindex-refactoring/SKILL.md` |
| Tools, resources, schema reference | `.claude/skills/ontoindex/ontoindex-guide/SKILL.md` |
| Index, status, clean, wiki CLI commands | `.claude/skills/ontoindex/ontoindex-cli/SKILL.md` |

<!-- ontoindex:end -->
