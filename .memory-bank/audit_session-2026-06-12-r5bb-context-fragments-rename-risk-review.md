# R5BB Context Fragments Rename Risk Review

Date: 2026-06-12

Decision:
- Dispatch `codex-context-fragments` -> `ontocode-context-fragments`.
- Dispatch `codex_context_fragments` -> `ontocode_context_fragments`.
- Scope is identity-only package/lib/Bazel/import rename.

OntoIndex:
- `Trait:ontocode-rs/context-fragments/src/fragment.rs:ContextualUserFragment`: CRITICAL, 79 impacted nodes, 30 direct, 9 affected modules, 1 affected process.
- Risk reasons: `direct_count>=30`, `total_count>=30`, `module_count>=5`.

Direct Inventory:
- Root workspace dependency metadata.
- `context-fragments` manifest and Bazel crate identity.
- Core-skills import.
- Core context imports and re-exports.
- Extension-api imports and re-export.
- Prompts permissions-instructions import.
- Cargo lock entries.

Guardrails:
- Preserve `ContextualUserFragment`, `AdditionalContextUserFragment`, `AdditionalContextDeveloperFragment`, `FragmentRegistration`, and `FragmentRegistrationProxy` behavior.
- Preserve model-context fragment caps/order/content, skill prompt fragment behavior, core diagnostic/context wiring, extension-api re-export behavior, and prompt permissions fragment behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `context-fragments` directory path.

Required Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-context-fragments --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests` or focused context/session/diagnostic tests.
- `CARGO_BUILD_JOBS=8 cargo check -p codex-core-skills --tests` if not renamed yet.
- `CARGO_BUILD_JOBS=8 cargo check -p codex-extension-api --tests` if not renamed yet.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-prompts --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_context_fragments|codex-context-fragments`.
- Cargo metadata residual count.
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

Model:
- Dispatch on `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.
