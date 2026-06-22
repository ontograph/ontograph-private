# Lefties: OpenCode Tool Extensions

## Status

Moved out of core ADR

## Date

2026-06-07

## Context

These OpenCode-inspired ideas were removed from [ADR_OPENCODE_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_OPENCODE_TOOL_EXTENSIONS.md:1) because they duplicate existing Ontocode owners, need a separate ADR, or do not naturally extend the core codebase.

## Moved Items

| Original points | Reason moved |
|---|---|
| `021-040` context epoch/source runtime semantics | Current bounded context/session architecture would need a dedicated context ADR before adopting OpenCode semantics. |
| `041-060` second tool registry and managed output retention architecture | Current tool specs and truncation tests already own this behavior; a new retention store would duplicate runtime architecture. |
| `061-080` wildcard permission engine | Current shell/sandbox/approval policy owns permission decisions. |
| `101-120` executable plugin runtime, package install, dynamic import, workspace adapters | Executing external OpenCode plugins is not a natural core extension and needs an external adapter/security ADR. |
| `141-160` second provider registry, bundled provider loader, gateway behavior | Provider work belongs in native provider or external adapter ADRs, not this interop ADR. |
| `161-180` executable auth plugin import and credential persistence | Credential import is blocked without evidence, legal review, redaction, overwrite, provenance, and deletion semantics. |
| `181-200` LSP runtime code-intelligence tools | Duplicates GitNexus unless a separate ADR proves LSP adds value that GitNexus cannot provide. |
| `201-220` prompt queue rewrite | TUI/session orchestration scope; not justified by OpenCode interop. |
| `221-240` public session/app-server endpoint expansion | Public API changes require app-server v2 ADR, schema generation, compatibility tests, and product review. |
| `229` public session share URLs | Product/cloud surface outside current core-extension scope. |
| `241-260` GitHub workflows, release automation, duplicate issue tooling, broad git project automation | Repository operations or product automation, not runtime core. |
| `261-280` new patch/file mutation architecture | Current patch, filesystem, permission, and redaction owners must be extended instead. |
| `288-292` changelog, localization, commit, issue, dependency project commands | Project automation should remain command/plugin/tooling scope unless separately accepted. |
| `294-295` executable `.opencode/tool/*.ts` import | Executable imported tools are unsafe without explicit trust, sandbox, and plugin architecture. |
| `297-298` theme/glossary import | UI/docs product scope unless TUI/docs owner accepts a separate slice. |
| `321-340` install, Nix, Homebrew, signing, containers, release asset matrix | Packaging/release work, not core runtime architecture. |
| `341-360` desktop/web UI dialogs, WSL bridge, debug bar, theme preload, menu actions | Product/app UI scope outside this core ADR. |
| `361-380` broad diagnostics framework/event bus replacement | Existing doctor, feedback, status, event, and redaction owners must be extended instead. |
| `391-399` ADR automation, tracking generation, affordability splitter | Repository-only tooling belongs in the lean-ctx project tooling ADR if kept. |

## Re-entry Criteria

Any moved item can return only with:

- a concrete user problem not solved by current agents, tools, context, MCP, providers, auth, app-server, shell, diagnostics, GitNexus, external-agent, or lean-ctx owners
- a narrow ADR naming the exact owner and compatibility surface
- redaction and privacy tests for any credential, source, command, plugin, provider, LSP, or diagnostic output
- bounded context rules for anything model-visible
- explicit opt-in and sandbox rules for anything executable
- GitNexus context and impact analysis before implementation
