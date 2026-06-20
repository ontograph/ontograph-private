import re
with open("ontocode-rs/core/src/session/review.rs", "r") as f:
    code = f.read()

# Someone else added `file_read_evidence` to TurnContext in another PR/commit, we just need to provide it (probably default).
code = code.replace(
    "auth_manager: turn_context.auth_manager.clone(),\n    };",
    "auth_manager: turn_context.auth_manager.clone(),\n        file_read_evidence: turn_context.file_read_evidence.clone(),\n    };"
)

with open("ontocode-rs/core/src/session/review.rs", "w") as f:
    f.write(code)

