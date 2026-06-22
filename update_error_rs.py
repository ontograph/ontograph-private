import re

with open("ontocode-rs/codex-api/src/error.rs", "r") as f:
    content = f.read()

new_struct = """
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct ErrorPayload {
    pub r#type: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
    pub plan_type: Option<String>,
    pub resets_at: Option<i64>,
}

impl ErrorPayload {
    pub fn is_context_window_error(&self) -> bool {
        self.code.as_deref() == Some("context_length_exceeded")
    }

    pub fn is_quota_exceeded_error(&self) -> bool {
        self.code.as_deref() == Some("insufficient_quota")
    }

    pub fn is_usage_not_included(&self) -> bool {
        self.code.as_deref() == Some("usage_not_included")
    }

    pub fn is_invalid_prompt_error(&self) -> bool {
        self.code.as_deref() == Some("invalid_prompt")
    }

    pub fn is_cyber_policy_error(&self) -> bool {
        self.code.as_deref() == Some("cyber_policy")
    }

    pub fn is_server_overloaded_error(&self) -> bool {
        self.code.as_deref() == Some("server_is_overloaded")
    }
}
"""

content = content + new_struct

with open("ontocode-rs/codex-api/src/error.rs", "w") as f:
    f.write(content)
