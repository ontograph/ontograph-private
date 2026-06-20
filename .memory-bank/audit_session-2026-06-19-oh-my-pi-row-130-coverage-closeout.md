# Oh My Pi Row 130 Coverage Closeout

Date: 2026-06-19

Outcome:
- ADR row 130 was closed by adding a focused recursive local-ref regression test
  through `parse_mcp_tool` in `ontocode-rs/tools/src/mcp_tool_tests.rs`.
- No implementation changes were required.

Evidence:
- `parse_mcp_tool_handles_cyclic_local_refs` feeds a bounded cyclic
  `$defs`/`$ref` shape through the MCP wrapper and asserts the sanitized
  `ToolDefinition` matches the expected recursive schema shape.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tools` passed in this session.

OntoIndex:
- `inspect({action: "context", target: "parse_mcp_tool"})` showed the wrapper
  calls `parse_tool_input_schema`, and `impact({action: "symbol", target:
  "parse_mcp_tool", direction: "upstream"})` reported LOW risk with two direct
  callers and no affected processes.
