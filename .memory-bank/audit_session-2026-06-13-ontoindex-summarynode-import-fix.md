# OntoIndex SummaryNode Import Fix

- Scope: local OntoIndex repair for `/opt/demodb/_workfolder/ontocode` indexing.
- Symptom: `ontoindex analyze` failed after clean rebuild with `COPY failed for SummaryNode: Binder exception: Table SummaryNode does not contain column startLine.`
- Root cause: `ontoindex/src/core/lbug/lbug-adapter.ts` still mapped `SummaryNode` through the generic code-element `getCopyQuery()` fallback, which imports `startLine,endLine,content` columns instead of the dedicated summary schema.
- Fix: added explicit `SummaryNode` handling in `getCopyQuery()` and in `insertNodeToLbug()` fallback creation logic.
- Verification:
  - rebuilt `ontoindex` and `ontoindex-native`
  - `ontoindex analyze` succeeded for `/opt/demodb/_workfolder/ontocode`
  - `ontoindex status` reports `✅ up-to-date`
  - MCP `discover({action:"repos"})` returns repo `codex -> /opt/demodb/_workfolder/ontocode`
  - MCP `gn_tool_contract({includeFacades:true})` reports `status: ok`
