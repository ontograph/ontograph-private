import re

with open(".memory-bank/ADR_FIVE_CONCURRENT_CODING_SUBAGENTS.md", "r") as f:
    text = f.read()

# Update the status and outcome as instructed
text = re.sub(r"status: accepted", "status: changes-required", text)

with open(".memory-bank/ADR_FIVE_CONCURRENT_CODING_SUBAGENTS.md", "w") as f:
    f.write(text)

print("Status updated.")
