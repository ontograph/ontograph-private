# Prompt: OntoIndex MCP Runtime Defects

Use this prompt for the OntoIndex team.

```text
Task: Fix OntoIndex MCP runtime defects found during ontocode integration smoke test.

Environment:
- Target repo: /opt/demodb/_workfolder/ontocode
- OntoIndex repo/tool path: /opt/demodb/_workfolder/OntoIndex/ontoindex
- Runtime: ontoindex 1.9.0
- Node: v22.22.3
- MCP repo label: codex
- Expected repoPath: /opt/demodb/_workfolder/ontocode

Smoke Test Result:
- Direct MCP tools exposed in Codex session are callable.
- Internal OntoIndex registry reports:
  - 61 callable tools
  - 53 super-functions
  - 8 facades
  - no advertised/callable contract drift
- Core graph tools correctly target repoPath /opt/demodb/_workfolder/ontocode:
  - inspect
  - impact
  - search
  - gn_explore
  - gn_verify_diff
  - gn_diff_impact
  - gn_review_diff

Issues To Fix:

1. gn_pre_commit_audit is advertised but broken at runtime
- Call:
  gn_pre_commit_audit({ repo: "codex", scope: "staged" })
- Error:
  Cannot find module '/opt/demodb/_workfolder/OntoIndex/ontoindex/dist/mcp/super/pre-commit-audit.js'
  imported from /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/mcp/super/dispatch.js
- Expected:
  Either the module is built and callable, or the tool is not advertised.
- Required fix:
  Ensure `pre-commit-audit.js` is emitted into `dist/mcp/super/`, included in package/build artifacts, and covered by a runtime MCP smoke test.

2. gn_taint_trace does not resolve repo-relative paths
- Failed call:
  gn_taint_trace({
    repo: "codex",
    path: "ontocode-rs/cli/src/main.rs",
    source: "args",
    sink: "command_name_from_arg0"
  })
- Error behavior:
  It tries to open the path relative to the MCP process cwd and returns ENOENT.
- Working call:
  gn_taint_trace({
    repo: "codex",
    path: "/opt/demodb/_workfolder/ontocode/ontocode-rs/cli/src/main.rs",
    source: "args",
    sink: "command_name_from_arg0"
  })
- Expected:
  When `repo` is provided, repo-relative `path` must resolve against that repo's `repoPath`.
- Required fix:
  Normalize relative paths using the resolved repo root, reject paths escaping the repo, and include `repoPath` plus normalized path in diagnostics.

3. Docs tools are callable but degraded because Markdown sidecar is missing
- Calls:
  gn_docs({ repo: "codex", action: "readiness" })
  docs({ repo: "codex", action: "context" })
- Current result:
  sidecarStatus = "missing"
  skipReasons = ["sidecar-missing"]
- Expected:
  This is acceptable only if setup/docs clearly explain how to generate it.
- Required fix:
  Add clear setup guidance and a first-class command path:
  `ontoindex docs refresh` or `ontoindex analyze --markdown-sidecar`.
  Consider making MCP readiness recommend the exact command including repo path.

4. Embeddings are missing/degraded
- Search works, but reports enrichment/embeddings unavailable.
- Expected:
  The degraded state should be obvious and actionable.
- Required fix:
  Improve readiness/help output to recommend:
  `ontoindex analyze --embeddings`
  or the correct current command for this repo.

5. Tool contract says 61 callable tools, but Codex session exposes fewer direct wrappers
- `gn_tool_contract` reports the full internal registry.
- Codex-visible direct MCP namespace exposes a smaller subset.
- Expected:
  Agents need to know which functions are actually callable from their MCP client.
- Required fix:
  Add an MCP-facing "visible frontier" report or include client-visible tool names when possible.
  At minimum, document that `gn_tool_contract` reports OntoIndex internal registry, not necessarily the host client's generated direct wrappers.

Acceptance Criteria:
- `gn_pre_commit_audit({ repo: "codex", scope: "staged" })` returns a structured response, not module-not-found.
- `gn_taint_trace({ repo: "codex", path: "ontocode-rs/cli/src/main.rs", ... })` resolves against `/opt/demodb/_workfolder/ontocode`.
- Relative path traversal like `../outside` is rejected safely.
- Docs readiness gives exact remediation commands.
- Embeddings/readiness output gives exact remediation commands.
- Contract/help output clearly distinguishes internal callable registry from host-exposed direct MCP tools.
- Add automated MCP smoke tests for all advertised tools so no missing `dist` module can ship again.
```
