import re

with open("ontocode-rs/core/src/session/turn_context.rs", "r") as f:
    content = f.read()

impl_str = """
    pub(crate) fn record_file_read(&self, path: &AbsolutePathBuf) {
        if let Ok(mut evidence) = self.file_read_evidence.lock() {
            *evidence.paths.entry(path.clone()).or_insert(0) += 1;
        }
    }
"""

if "pub(crate) fn record_file_read" not in content:
    content = content.replace(
        "    pub(crate) fn compact_prompt(&self) -> &str {\n",
        impl_str + "\n    pub(crate) fn compact_prompt(&self) -> &str {\n"
    )

with open("ontocode-rs/core/src/session/turn_context.rs", "w") as f:
    f.write(content)
