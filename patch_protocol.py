import re

with open("ontocode-rs/protocol/src/protocol.rs", "r") as f:
    content = f.read()

# Add `use crate::read_evidence::FileReadEvidence;` at the top of the file
# right after other `use`s if not there.

# Actually let's just insert it in the struct
if "pub file_read_evidence:" not in content:
    content = content.replace(
        "pub summary: ReasoningSummaryConfig,\n}",
        "pub summary: ReasoningSummaryConfig,\n    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub file_read_evidence: Option<crate::read_evidence::FileReadEvidence>,\n}"
    )

with open("ontocode-rs/protocol/src/protocol.rs", "w") as f:
    f.write(content)
