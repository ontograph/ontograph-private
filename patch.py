import re

with open("ontocode-rs/tools/src/tool_search.rs", "r") as f:
    code = f.read()

code = code.replace("pub output: LoadableToolSpec,\n}", "pub output: LoadableToolSpec,\n    pub disabled_reason: Option<String>,\n    pub source: Option<String>,\n}")
code = code.replace("let search_text = default_tool_search_text(tool_name, &spec);", "let search_text = default_tool_search_text(tool_name, &spec, None, None);")
code = code.replace("Self::from_spec(search_text, spec, source_info)", "Self::from_spec(search_text, spec, source_info, None, None)")

code = code.replace("pub fn from_spec(\n        search_text: String,\n        spec: ToolSpec,\n        source_info: Option<ToolSearchSourceInfo>,\n    )", "pub fn from_spec(\n        search_text: String,\n        spec: ToolSpec,\n        source_info: Option<ToolSearchSourceInfo>,\n        disabled_reason: Option<String>,\n        source: Option<String>,\n    )")

code = code.replace("output,\n            },\n            source_info", "output,\n                disabled_reason,\n                source,\n            },\n            source_info")

code = code.replace("pub fn default_tool_search_text(tool_name: &ToolName, spec: &ToolSpec) -> String {", "pub fn default_tool_search_text(tool_name: &ToolName, spec: &ToolSpec, disabled_reason: Option<&str>, source: Option<&str>) -> String {")

code = code.replace("parts.join(\" \")\n}", """    if let Some(reason) = disabled_reason {
        push_search_part(&mut parts, reason.to_string());
    }
    if let Some(src) = source {
        push_search_part(&mut parts, src.to_string());
    }

    parts.join(" ")
}""")

with open("ontocode-rs/tools/src/tool_search.rs", "w") as f:
    f.write(code)
