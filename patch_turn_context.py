import re

with open("ontocode-rs/core/src/session/turn_context.rs", "r") as f:
    content = f.read()

if "pub file_read_evidence:" not in content:
    content = content.replace(
        "    pub(crate) turn_metadata_state: Arc<TurnMetadataState>,\n",
        "    pub(crate) turn_metadata_state: Arc<TurnMetadataState>,\n    pub(crate) file_read_evidence: Arc<std::sync::Mutex<ontocode_protocol::read_evidence::FileReadEvidence>>,\n"
    )

with open("ontocode-rs/core/src/session/turn_context.rs", "w") as f:
    f.write(content)
