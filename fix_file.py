with open("ontocode-rs/tui/src/multi_agents.rs", "r") as f:
    lines = f.readlines()

new_lines = []
skip = False
for i, line in enumerate(lines):
    if "fn agent_label_line" in line:
        new_lines.append(line)
        new_lines.append("    agent_label_spans(agent).into()\n")
        new_lines.append("}\n\n")
        new_lines.append("fn agent_label_spans(agent: AgentLabel<'_>) -> Vec<Span<'static>> {\n")
        new_lines.append("    let mut spans = Vec::new();\n")
        new_lines.append("    let nickname = agent\n")
        new_lines.append("        .nickname\n")
        new_lines.append("        .map(str::trim)\n")
        new_lines.append("        .filter(|nickname| !nickname.is_empty());\n")
        new_lines.append("    let role = agent.role.map(str::trim).filter(|role| !role.is_empty());\n\n")
        new_lines.append("    if let Some(nickname) = nickname {\n")
        new_lines.append("        spans.push(Span::from(nickname.to_string()).cyan().bold());\n")
        new_lines.append("        if let Some(role) = role {\n")
        new_lines.append("            spans.push(Span::from(\" \").dim());\n")
        new_lines.append("            spans.push(Span::from(format!(\"[{role}]\")).dim());\n")
        new_lines.append("        }\n")
        new_lines.append("    } else if let Some(thread_id) = agent.thread_id {\n")
        new_lines.append("        spans.push(Span::from(thread_id.to_string()).cyan());\n")
        new_lines.append("        if let Some(role) = role {\n")
        new_lines.append("            spans.push(Span::from(\" \").dim());\n")
        new_lines.append("            spans.push(Span::from(format!(\"[{role}]\")).dim());\n")
        new_lines.append("        }\n")
        new_lines.append("    } else {\n")
        new_lines.append("        spans.push(Span::from(\"agent\").cyan());\n")
        new_lines.append("    }\n\n")
        new_lines.append("    spans\n")
        new_lines.append("}\n\n")
        new_lines.append("fn spawn_request_spans(spawn_request: Option<&SpawnRequestSummary>) -> Vec<Span<'static>> {\n")
        skip = True
        continue
    if skip and "fn spawn_request_spans" in line:
        skip = False
        continue
    if skip:
        continue
    new_lines.append(line)

with open("ontocode-rs/tui/src/multi_agents.rs", "w") as f:
    f.writelines(new_lines)
