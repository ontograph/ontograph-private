with open("ontocode-rs/tui/src/multi_agents.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if line.strip() == "} else {" and len(new_lines) > 0 and "spans.push(Span::from(\"agent\").cyan());" in new_lines[-2]:
        # This seems to be the start of the duplication
        pass
    new_lines.append(line)

# This is still error prone.
# Let's just use `git checkout` to revert and start over.
