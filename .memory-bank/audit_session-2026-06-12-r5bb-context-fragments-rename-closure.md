# R5BB Context Fragments Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-context-fragments` -> `ontocode-context-fragments`.
- Accepted `codex_context_fragments` -> `ontocode_context_fragments`.
- Scope remained identity-only: package, library, Bazel target, Cargo lock, and dependent imports.

## Guardrails Preserved

- `ContextualUserFragment`.
- `AdditionalContextUserFragment`.
- `AdditionalContextDeveloperFragment`.
- `FragmentRegistration`.
- `FragmentRegistrationProxy`.
- Model-context fragment caps, ordering, and content behavior.
- Skill prompt fragment behavior.
- Core diagnostic/context wiring.
- Extension API re-export behavior.
- Prompt permissions fragment behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `context-fragments` directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-context-fragments --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p codex-core-skills --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p codex-extension-api --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-prompts --tests`.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Scoped stale-reference search for `codex_context_fragments|codex-context-fragments`: clean in `ontocode-rs`.
- `git diff --check`: clean.
- OntoIndex `detect-changes --repo codex`: completed with the known broad high-risk dirty-tree envelope.

## Result

- Cargo metadata reports 19 remaining staged `codex-*` packages after R5BB.
- Work completed on `gpt-5.4-mini` after Spark usage-limit fallback.
