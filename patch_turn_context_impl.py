import re

with open("ontocode-rs/core/src/session/turn_context.rs", "r") as f:
    content = f.read()

# Update `with_model`
if "file_read_evidence: self.file_read_evidence.clone()," not in content:
    content = content.replace(
        "            turn_metadata_state: self.turn_metadata_state.clone(),\n",
        "            turn_metadata_state: self.turn_metadata_state.clone(),\n            file_read_evidence: self.file_read_evidence.clone(),\n"
    )

# Update `make_turn_context`
if "file_read_evidence: Arc::new(std::sync::Mutex::new(" not in content:
    content = content.replace(
        "            turn_metadata_state,\n",
        "            turn_metadata_state,\n            file_read_evidence: Arc::new(std::sync::Mutex::new(Default::default())),\n"
    )

# Update `to_turn_context_item`
if "file_read_evidence: Some(self.file_read_evidence" not in content:
    content = content.replace(
        "            effort: self.reasoning_effort,\n",
        "            effort: self.reasoning_effort,\n            file_read_evidence: Some(self.file_read_evidence.lock().unwrap().clone()),\n"
    )

with open("ontocode-rs/core/src/session/turn_context.rs", "w") as f:
    f.write(content)
