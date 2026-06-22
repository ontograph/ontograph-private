# Native Heterogeneous Provider Engines Tracking

Source ADR: `ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md`

## Status Key

- `pending`
- `in_progress`
- `blocked`
- `done`
- `deferred`

## Task Queue

| ID | Task | Scope | Status | Notes |
| --- | --- | --- | --- | --- |
| N0 | Tracking bootstrap | Create tracker and classify runnable, blocked, and deferred ADR stages | done | Initial manager pass created this tracker |
| N1A | Built-in descriptor selection | Add descriptor-level selection for native built-in providers without public config/schema changes | done | Worker added private native placeholder engines and `NativeRuntimePending`; `just fmt` and `just test -p codex-model-provider` passed 45/45; manager review found real runtime factory still lacks provider-id input, so this is descriptor-placeholder only |
| N1B | Runtime execution seam design | Identify the minimal protocol-neutral runtime seam and produce an implementation plan/test map | done | Worker recommended dispatch at `ModelClientSession::stream`, leaving `stream_responses_api` unchanged and reusing `map_response_events` |
| N1C | Runtime execution seam scaffold | Implement the minimal unsupported-native-runtime dispatch scaffold and tests | done | Runtime seam patch present; warning cleanup applied; `just fmt`, `CARGO_BUILD_JOBS=1 just test -p codex-model-provider`, `CARGO_BUILD_JOBS=1 just test -p codex-core client`, and `git diff --check` passed |
| N2 | Claude API-key native engine | Implement Anthropic Messages runtime after N1C is complete | done | API-key text streaming, function tool declarations, tool-use streaming, and tool-result history verified |
| N2A | Claude text streaming slice | Implement API-key Anthropic request/SSE text runtime through the native seam | done | `just fmt`, `CARGO_BUILD_JOBS=1 just test -p codex-core native_provider::anthropic`, `CARGO_BUILD_JOBS=1 just test -p codex-model-provider`, and `CARGO_BUILD_JOBS=1 just test -p codex-core client` passed |
| N2B | Claude tool translation slice | Add Anthropic tool declaration, tool-use, and tool-result translation tests/runtime support | done | `just fmt`, `CARGO_BUILD_JOBS=1 just test -p codex-core native_provider::anthropic`, `CARGO_BUILD_JOBS=1 just test -p codex-model-provider`, and `CARGO_BUILD_JOBS=1 just test -p codex-core client` passed |
| N3 | Gemini API-key native engine | Implement Google `generateContent` runtime after N1C is complete | done | API-key text streaming, function declarations, function-call streaming, and tool-result history verified |
| N4 | GitHub Copilot API engine | Implement Copilot API runtime and token exchange after N1C is complete | done | Copilot chat request translation, token exchange, SSE text/tool-call streaming, and focused/provider/core-client tests verified |
| N5 | Internal registry hardening | Consolidate duplicated built-in metadata after native engines prove the pattern | done | Consolidated native provider selection into production-owned descriptor metadata; removed test-only placeholder selector; `just fmt` and `CARGO_BUILD_JOBS=1 just test -p codex-model-provider` passed |
| N6 | External adapter ADR | Draft separate ADR for out-of-process provider adapters | done | Created and hardened `ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md`; stdio-first protocol ADR direction is proposed, runtime implementation remains future work |
| N6A | External adapter ADR draft | Create `ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md` with options, decision, protocol, trust model, and acceptance criteria | done | Standalone ADR drafted; scoped diff check passed |
| N6B | External adapter ADR challenge | Review and harden the drafted ADR for security, config, streaming, cancellation, and testability gaps | done | Challenge pass hardened opt-in, credential handoff, framing, cancellation/crash semantics, non-goals, and conformance fixtures |
| N6C | External adapter ADR finalization | Apply review fixes, update tracker, run doc checks, and run GitNexus detect | done | Cross-links updated; scoped diff checks passed; GitNexus detect remains CRITICAL for broad dirty worktree noise |
| N7 | Final manager verification | Consolidate worker results, run targeted tests, update tracker, and run GitNexus detect | done | Final focused verification passed for model-provider, Anthropic, Gemini, Copilot, and core client; GitNexus detect remains CRITICAL for full dirty worktree noise |

## Dispatch Log

| Step | Tracking Update | Action |
| --- | --- | --- |
| 1 | Created tracker; marked `N1A` in progress; marked `N2`-`N5` blocked and `N6` deferred | Starting Stage 1A implementation and parallel runtime-seam analysis |
| 2 | Updated `N1A` risk notes and marked `N1B` in progress | Dispatching disjoint workers: one for descriptor-only implementation, one for runtime-seam analysis |
| 3 | Marked `N1A` done and `N7` in progress | N1A worker reported `just fmt` and `just test -p codex-model-provider` passed; manager reviewing patch |
| 4 | Clarified `N1A` completion scope | Descriptor placeholder selection exists, but provider-id-aware runtime factory wiring is deferred until the runtime seam stage |
| 5 | Marked `N1B` done and added `N1C` in progress | Runtime-seam worker recommended dispatch at `ModelClientSession::stream`; preparing high-risk implementation scaffold |
| 6 | Updated `N1C` impact notes | Dispatching scaffold worker with explicit constraint to avoid `create_model_provider` changes |
| 7 | Updated `N1C` verification status | Worker patch is present and model-provider tests passed; core verification remains blocked by active `just test -p codex-core client` compile/link process |
| 8 | Ran GitNexus detect | `gitnexus_detect_changes` reports HIGH for the full dirty worktree, including many unrelated existing changes; no provider-engine implementation should start until N1C verification resolves |
| 9 | Marked `N1C` and `N7` done; moved `N2`-`N4` to pending | Senior unblock pass reran `just fmt`, `CARGO_BUILD_JOBS=1 just test -p codex-model-provider`, and `CARGO_BUILD_JOBS=1 just test -p codex-core client`; all passed |
| 10 | Marked `N2` in progress | Dispatching Claude API-key native engine worker; `N3` and `N4` remain pending to avoid concurrent edits to shared runtime code |
| 11 | Confirmed `N2` remains in progress | Senior implementation pass starting for Claude API-key native engine; shared runtime seam edits require GitNexus impact before code changes |
| 12 | Split `N2` into `N2A` done and `N2B` in progress | N2A text streaming slice verified; N2 remains open because ADR requires Claude tool-use/tool-result translation |
| 13 | Marked `N2` and `N2B` done; marked `N3` in progress | Claude native engine verified; GitNexus detect remains CRITICAL for full dirty worktree noise and provider factory flows |
| 14 | Confirmed `N3` remains in progress | Senior implementation pass starting for Gemini API-key native engine; scope limited to native module, focused tests, and existing dispatch seam |
| 15 | Marked `N3` done, `N4` in progress, and `N5` pending | Gemini native engine verified with focused/provider/core-client tests; dispatching Copilot next |
| 16 | Confirmed `N4` remains in progress | Senior implementation pass starting for Copilot token-exchange/request/stream slice; scope limited to native module, tests, module export, and existing runtime dispatch |
| 17 | Marked `N4` done and `N5` in progress | Copilot verified with focused/provider/core-client tests; GitNexus detect remains CRITICAL for full dirty worktree noise and provider factory flows |
| 18 | Marked `N5` done | Internal registry hardening completed; native descriptor matching now uses production metadata and focused model-provider verification passed |
| 19 | Updated `N7` final verification notes | Manager reran `just fmt`, `CARGO_BUILD_JOBS=1 just test -p codex-model-provider`, Anthropic/Gemini/Copilot native focused tests, `CARGO_BUILD_JOBS=1 just test -p codex-core client`, `git diff --check`, and GitNexus detect |
| 20 | Marked `N6` and `N6A` in progress; added `N6B` and `N6C` | GitNexus query found no existing adapter flow; dispatching stdio-first external adapter ADR draft |
| 21 | Marked `N6A` done and `N6B` in progress | Draft ADR created and diff-check passed; dispatching challenge pass before finalization |
| 22 | Marked `N6B` done and `N6C` in progress | Challenge pass hardened security/protocol/testability gaps; starting manager finalization |
| 23 | Marked `N6` and `N6C` done | Created standalone external adapter ADR, updated cross-links, ran scoped doc checks, and ran GitNexus detect; detect remains CRITICAL for full dirty worktree noise |
