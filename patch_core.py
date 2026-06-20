import re

files_to_patch = [
    "ontocode-rs/core/src/tools/handlers/dynamic.rs",
    "ontocode-rs/core/src/tools/handlers/mcp.rs",
    "ontocode-rs/core/src/tools/handlers/multi_agents.rs"
]

for file_path in files_to_patch:
    with open(file_path, "r") as f:
        code = f.read()
    
    if file_path == "ontocode-rs/core/src/tools/handlers/dynamic.rs":
        code = code.replace(
            "            self.spec(),\n            Some(ToolSearchSourceInfo {\n                name: source_name,\n                description: None,\n            }),\n        )",
            "            self.spec(),\n            Some(ToolSearchSourceInfo {\n                name: source_name,\n                description: None,\n            }),\n            None,\n            None,\n        )"
        )
    elif file_path == "ontocode-rs/core/src/tools/handlers/mcp.rs":
        code = code.replace(
            "            self.spec(),\n            source_info,\n        )",
            "            self.spec(),\n            source_info,\n            None,\n            None,\n        )"
        )
    elif file_path == "ontocode-rs/core/src/tools/handlers/multi_agents.rs":
        code = code.replace(
            "        Some(ToolSearchSourceInfo {\n            name: \"subagents\".to_string(),\n            description: Some(\n                \"Use these tools to spawn and interact with sub-agents to parallelize work.\"\n                    .to_string(),\n            ),\n        }),\n    )",
            "        Some(ToolSearchSourceInfo {\n            name: \"subagents\".to_string(),\n            description: Some(\n                \"Use these tools to spawn and interact with sub-agents to parallelize work.\"\n                    .to_string(),\n            ),\n        }),\n        None,\n        None,\n    )"
        )

    with open(file_path, "w") as f:
        f.write(code)

