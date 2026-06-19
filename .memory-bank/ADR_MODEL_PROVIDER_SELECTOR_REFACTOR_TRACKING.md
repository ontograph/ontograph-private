# Model Provider Selector Refactor Tracking
src ADR: `ADR_MODEL_PROVIDER_SELECTOR_REFACTOR.md`
## Status Key
- `pending`
- `in_progress`
- `blocked`
- `done`
- `deferred`
## Task Queue
| ID | Task | Scope | Status | Notes |
| --- | --- | --- | --- | --- |
| P1 | Provider selector impl slice | Introduce internal provider-kind selector and route `create_model_provider` through it without behavior changes | done | Added private `ProviderKind` routing in `ontocode-rs/model-provider/src/provider.rs`; `just fmt`, `just test -p codex-model-provider`, and `just fix -p codex-model-provider` passed |
| P2 | External provider predicate audit | Classify provider/auth predicates outside `ontocode-rs/model-provider` | done | Audit captured in `MODEL_PROVIDER_SELECTOR_PREDICATE_AUDIT.md`; no must-migrate-now blockers found |
| P3 | Selector verification review | Validate tests, GitNexus blast radius, and missing coverage after P1/P2 | done | Verified P1/P2 as behavior-preserving; GitNexus still rates `create_model_provider` CRITICAL due broad callers, but public factory API and runtime branch semantics stayed unchanged |
| P4 | Claude MCP live credential validation | Run redaction helper and opt-in validator on a real Claude MCP credential src file | blocked | Fresh 2026-06-06 rediscovery found no local Claude MCP credential source; requires an external machine with authenticated Claude MCP connector credentials |
| P5 | Provider selector capability API | Add selector-owned outputs for provider auth, catalog, account-state, and diagnostics so external callers stop copying provider predicates | done | Added provider-owned auth/probe capabilities and migrated CLI doctor auth/reachability diagnostics off direct `requires_openai_auth` / `is_amazon_bedrock` reads |
| P6 | Post-P5 selector verification | Re-audit predicate leaks, review P5 diff, and confirm validation after implementation landed | done | No direct CLI doctor auth/reachability predicate leaks remain; `ontocode-rs/cli/src/main.rs` has an unrelated CLI alias/help diff and should stay out of selector scope |
| P7 | App-server auth status provider capability migration | Move app-server auth status requirement logic off raw `ModelProviderInfo.requires_openai_auth` and onto provider-owned account/capability output | done | Auth status now uses provider capabilities; targeted app-server auth-status tests passed 8/8 and `just fix -p ontocode-app-server` passed |
| P8 | Selector-owned runtime constructors | Move configured-provider and Bedrock construction behind selector-owned constructor helpers without changing the public factory API | done | Added private selector-owned construction helper; `just fmt`, `just test -p codex-model-provider`, and `just fix -p codex-model-provider` passed |
| P9 | TUI auth/status provider capability migration | Move TUI login/status/rate-limit decisions off raw `requires_openai_auth` and onto provider-owned capability/account output | done | Runtime TUI predicate migration implemented; focused TUI login tests passed 7/7 and `just fix -p ontocode-tui` passed |
| P10 | CLI model-route probe policy migration | Move remaining CLI Azure `/models` probe exception into provider-owned diagnostic capability/policy | done | Provider-owned Azure `/models` probe policy implemented; focused model-provider and CLI reachability tests passed, and scoped fixes passed |
| P11 | Extensible provider-class descriptor decision | Decide whether the static selector should become a small provider-class descriptor/registry before adding more heterogeneous providers | done | Decision: defer descriptor/registry until a third heterogeneous provider class exists; current `ProviderKind` plus provider-owned capabilities are sufficient after P9/P10 |
| P12 | Option 3 descriptor contract | Define the hybrid descriptor + built-in engine scope and smallest implementation slice for provider growth without per-provider Rust changes | done | Decision: first slice keeps `ProviderEngine` internal/derived; defer public `ModelProviderInfo.engine` until a real selectable engine needs config/schema support |
| P13 | Provider engine descriptor model | Add internal descriptor types for built-in provider engines and descriptor-owned capabilities without changing existing runtime behavior | done | Added private `descriptor` module with internal `ProviderEngine` and `ProviderDescriptor` |
| P14 | Configured-provider descriptor application | Route configured OpenAI-compatible providers through descriptor-owned capabilities/engine metadata while preserving Bedrock behavior | done | `ProviderKind` and provider capabilities now route through descriptor metadata while preserving the public factory API |
| P15 | Descriptor validation tests | Add focused tests proving descriptor-driven OpenAI-compatible providers can vary capabilities/probe behavior without new provider classes | done | `just test -p codex-model-provider` passed 41/41 after descriptor tests were added |
| P16 | Option 3 final verification | Run formatter, targeted tests/fixes, GitNexus detect, and remaining predicate audit | done | Verified descriptor slice is private/internal, no public config/schema engine field was added, provider factory signature is unchanged, predicate audit is clean, and diff-check passed |
## Dispatch Log
| Step | Tracking Update | Action |
| --- | --- | --- |
| 1 | Created tracking file and marked `P1`/`P2` in progress | Dispatching provider-selector impl and external predicate audit as disjoint sub-agent lanes |
| 2 | Marked `P2` done | External predicate audit completed with leave-as-is, migrate-later, and must-migrate-now classifications |
| 3 | Marked `P1` done | Provider factory now routes through a private selector result and the model-provider validation lane passed |
| 4 | Marked `P3` in progress | Dispatching verification review for GitNexus blast radius, test evidence, and residual readiness gaps |
| 5 | Marked `P3` done | Verification found no Rust blocker; no tests rerun because no Rust code changes were made during P3 |
| 6 | Marked `P5` in progress | Dispatching capability-API design/implementation and diagnostic-predicate migration lanes |
| 7 | Recorded `P5B` audit blocker | Verification found the CLI doctor provider-predicate leak still present and P5 needs another implementation loop before it can be marked done |
| 8 | Marked `P5` done | P5A added `ProviderCapabilities.requires_openai_auth` and `supports_models_route_probe`; `just fmt`, `just test -p codex-model-provider`, reduced-jobs CLI diagnostic filters, and scoped `just fix` passed |
| 9 | Marked `P6` in progress | Dispatching a fresh post-implementation verification pass because the earlier P5B audit completed before P5A landed |
| 10 | Marked `P6` done | Re-audit found no direct CLI doctor `config.model_provider.requires_openai_auth` or `is_amazon_bedrock()` diagnostic leaks; GitNexus remains workspace-critical due broad dirty scope |
| 11 | Marked `P7` in progress | Dispatching app-server auth status migration to provider-owned auth requirement output |
| 12 | Recorded `P7B` audit blocker | App-server `get_auth_status_response` still reads `self.config.model_provider.requires_openai_auth`; no Rust edits made and no tests run during audit |
| 13 | Updated `P7` after implementation landed | `get_auth_status_response` now reads `provider.capabilities().requires_openai_auth`; validation is still pending on a long-running app-server test compile |
| 14 | Marked `P7` blocked on validation | App-server test validation is still active after more than 50 minutes with long-running rustc/linker processes; no failure output is available and the process was left running |
| 15 | Marked `P7` done | Re-ran `CARGO_BUILD_JOBS=1 just test -p ontocode-app-server get_auth_status` after build artifacts warmed; 8/8 passed, then `just fix -p ontocode-app-server` passed |
| 16 | Marked `P8` in progress | Dispatching a selector-owned constructor/registry slice to reduce hard-coded factory branching while preserving runtime behavior |
| 17 | Marked `P8` done | P8 implementation kept the public factory API unchanged and moved configured-provider/Bedrock construction behind `ProviderKind`; model-provider validation passed |
| 18 | Marked `P9` in progress | Dispatching TUI auth/status predicate migration as a separate UI/runtime lane because raw OpenAI-auth checks still block broad provider readiness |
| 19 | Marked `P10` in progress | Dispatching CLI Azure `/models` probe policy migration as a separate diagnostics lane |
| 20 | Recorded `P9` high-impact gate | GitNexus rated TUI startup/auth symbols HIGH, so the first worker stopped before edits; dispatching a constrained redo because this is the expected auth-gating blast radius |
| 21 | Applied `P9` implementation | TUI login/status/rate-limit runtime decisions now use provider capabilities; `just fmt` passed, focused TUI validation is queued behind a pre-existing Cargo artifact lock |
| 22 | Marked `P9` blocked on validation | TUI runtime predicate migration landed, but focused TUI validation is queued behind a long-running Cargo artifact lock; no runtime `config.model_provider.requires_openai_auth` TUI leaks remain |
| 23 | Marked `P10` blocked on validation | Provider-owned Azure `/models` probe policy landed, but focused CLI/model-provider validation is queued behind the same Cargo artifact lock; `should_probe_models_route` was removed |
| 24 | Marked `P9` done | After Cargo lock cleared, focused TUI login tests passed 7/7 and `just fix -p ontocode-tui` passed |
| 25 | Marked `P10` done | Focused model-provider Azure probe test and CLI provider-reachability tests passed; `just fix -p codex-model-provider`, `just fix -p codex-cli`, and `just fix -p ontocode-tui` passed |
| 26 | Marked `P11` in progress | Dispatching final descriptor/registry readiness decision now that runtime predicate leaks are closed |
| 27 | Marked `P11` done | Registry implementation is deferred until a real third heterogeneous provider validates the abstraction; future provider-specific predicates must stay inside `ontocode-rs/model-provider` |
| 28 | Marked `P12` in progress | User selected Option 3, so dispatching hybrid descriptor + built-in engine planning before any new code edits |
| 29 | Marked `P12` done | P12 chose a private derived `ProviderDescriptor`/`ProviderEngine` slice and deferred public config/schema engine fields |
| 30 | Marked `P13`/`P14`/`P15` in progress | Dispatching one implementation worker for the shared `ontocode-rs/model-provider` write set |
| 31 | Marked `P13`/`P14`/`P15` done | Private descriptor module landed; model-provider formatter, tests, and scoped fix passed |
| 32 | Marked `P16` in progress | Running final manager verification for Option 3 descriptor slice |
| 33 | Marked `P16` done | Final verification passed; note `descriptor.rs` and this tracking file are untracked and must be included when staging/committing |
