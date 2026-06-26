use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::unified_exec::resolve_max_tokens;
use ontocode_protocol::models::FunctionCallOutputContentItem;
use ontocode_utils_output_truncation::TruncationPolicy;
use ontocode_utils_output_truncation::formatted_truncate_text;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct ExecCommandToolOutput {
    pub event_call_id: String,
    pub chunk_id: String,
    pub wall_time: Duration,
    /// Raw bytes returned for this unified exec call before any truncation.
    pub raw_output: Vec<u8>,
    pub truncation_policy: TruncationPolicy,
    pub max_output_tokens: Option<usize>,
    pub process_id: Option<i32>,
    pub exit_code: Option<i32>,
    pub original_token_count: Option<usize>,
    pub hook_command: Option<String>,
}

impl ToolOutput for ExecCommandToolOutput {
    fn log_preview(&self) -> String {
        super::context::telemetry_preview(&self.response_text())
    }

    fn success_for_logging(&self) -> bool {
        true
    }

    fn to_response_item(
        &self,
        call_id: &str,
        payload: &ToolPayload,
    ) -> ontocode_protocol::models::ResponseInputItem {
        super::context::function_tool_response(
            call_id,
            payload,
            vec![FunctionCallOutputContentItem::InputText {
                text: self.response_text(),
            }],
            Some(true),
        )
    }

    fn post_tool_use_id(&self, call_id: &str) -> String {
        if self.event_call_id.is_empty() {
            call_id.to_string()
        } else {
            self.event_call_id.clone()
        }
    }

    fn post_tool_use_input(&self, _payload: &ToolPayload) -> Option<JsonValue> {
        self.hook_command
            .as_ref()
            .map(|command| serde_json::json!({ "command": command }))
    }

    fn post_tool_use_response(&self, _call_id: &str, _payload: &ToolPayload) -> Option<JsonValue> {
        if self.process_id.is_some() || self.hook_command.is_none() {
            return None;
        }

        Some(JsonValue::String(
            self.truncated_output(self.model_output_max_tokens()),
        ))
    }

    fn code_mode_result(&self, _payload: &ToolPayload) -> JsonValue {
        #[derive(Serialize)]
        struct UnifiedExecCodeModeResult {
            #[serde(skip_serializing_if = "Option::is_none")]
            chunk_id: Option<String>,
            wall_time_seconds: f64,
            #[serde(skip_serializing_if = "Option::is_none")]
            exit_code: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            session_id: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            original_token_count: Option<usize>,
            output: String,
        }

        let result = UnifiedExecCodeModeResult {
            chunk_id: (!self.chunk_id.is_empty()).then(|| self.chunk_id.clone()),
            wall_time_seconds: self.wall_time.as_secs_f64(),
            exit_code: self.exit_code,
            session_id: self.process_id,
            original_token_count: self.original_token_count,
            output: match self.max_output_tokens {
                Some(max_tokens) => self.truncated_output(max_tokens),
                None => String::from_utf8_lossy(&self.raw_output).to_string(),
            },
        };

        serde_json::to_value(result).unwrap_or_else(|err| {
            JsonValue::String(format!("failed to serialize exec result: {err}"))
        })
    }
}

impl ExecCommandToolOutput {
    fn model_output_max_tokens(&self) -> usize {
        resolve_max_tokens(self.max_output_tokens).min(self.truncation_policy.token_budget())
    }

    pub(crate) fn truncated_output(&self, max_tokens: usize) -> String {
        let text = String::from_utf8_lossy(&self.raw_output).to_string();

        if let Some(count) = self.original_token_count
            && count > max_tokens
        {
            let result = formatted_truncate_text(&text, TruncationPolicy::Tokens(max_tokens));
            if !result.contains("tokens truncated") {
                let removed = count.saturating_sub(max_tokens);
                let total_lines = text.lines().count();
                if text.len() <= TruncationPolicy::Tokens(max_tokens).byte_budget() {
                    return format!(
                        "Total output lines: {total_lines}\n\n{text}\n…{removed} tokens truncated…"
                    );
                } else {
                    return format!("{result}\n…{removed} tokens truncated…");
                }
            }
        }
        formatted_truncate_text(&text, TruncationPolicy::Tokens(max_tokens))
    }

    fn response_text(&self) -> String {
        let mut sections = Vec::new();

        if !self.chunk_id.is_empty() {
            sections.push(format!("Chunk ID: {}", self.chunk_id));
        }

        let wall_time_seconds = self.wall_time.as_secs_f64();
        sections.push(format!("Wall time: {wall_time_seconds:.4} seconds"));

        if let Some(exit_code) = self.exit_code {
            sections.push(format!("Process exited with code {exit_code}"));
        }

        if let Some(process_id) = &self.process_id {
            sections.push(format!("Process running with session ID {process_id}"));
        }

        if let Some(original_token_count) = self.original_token_count {
            sections.push(format!("Original token count: {original_token_count}"));
        }

        sections.push("Output:".to_string());
        sections.push(self.truncated_output(self.model_output_max_tokens()));

        sections.join("\n")
    }
}
