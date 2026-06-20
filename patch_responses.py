import re

with open("ontocode-rs/codex-api/src/sse/responses.rs", "r") as f:
    content = f.read()

content = content.replace("#[derive(Debug, Deserialize)]\n#[allow(dead_code)]\npub use crate::error::ErrorPayload as Error;", "pub use crate::error::ErrorPayload as Error;")

with open("ontocode-rs/codex-api/src/sse/responses.rs", "w") as f:
    f.write(content)
