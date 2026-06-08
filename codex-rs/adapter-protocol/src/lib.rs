use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub const PROTOCOL_VERSION_V1: &str = "provider-adapter.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AdapterMessage {
    Handshake(Handshake),
    HandshakeResult(HandshakeResult),
    ModelList(ModelList),
    ModelListResult(ModelListResult),
    ExecuteStream(ExecuteStream),
    StreamStarted(StreamStarted),
    TextDelta(TextDelta),
    ToolCallDelta(ToolCallDelta),
    ToolCallDone(ToolCallDone),
    Usage(Usage),
    ProviderError(ProviderError),
    Completed(Completed),
    Cancel(Cancel),
    Canceled(Canceled),
    Shutdown(Shutdown),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Handshake {
    pub protocol_version: String,
    pub codex_version: String,
    pub requested_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HandshakeResult {
    pub protocol_version: String,
    pub adapter_name: String,
    pub adapter_version: String,
    pub provider_id: String,
    pub capabilities: HashMap<String, bool>,
    pub limits: AdapterLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterLimits {
    pub max_request_bytes: usize,
    pub max_event_bytes: usize,
    pub max_stderr_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelList {
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelListResult {
    pub request_id: String,
    pub models: Vec<AdapterModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterModel {
    pub id: String,
    pub display_name: String,
    pub capabilities: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecuteStream {
    pub request_id: String,
    pub model: String,
    pub conversation: AdapterConversation,
    pub tools: Vec<AdapterTool>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub credential_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterConversation {
    pub system: Option<String>,
    pub messages: Vec<AdapterMessage_>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMessage_ {
    pub role: String,
    pub content: Vec<AdapterContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AdapterContent {
    Text { text: String },
    Image { data: String, mime_type: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamStarted {
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextDelta {
    pub request_id: String,
    pub delta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCallDelta {
    pub request_id: String,
    pub call_id: String,
    pub name: Option<String>,
    pub arguments_delta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCallDone {
    pub request_id: String,
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Usage {
    pub request_id: String,
    pub input_tokens: usize,
    pub output_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProviderError {
    pub request_id: String,
    pub code: String,
    pub message: String,
    pub retryable: bool,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Completed {
    pub request_id: String,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cancel {
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Canceled {
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Shutdown {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeoutConfig {
    pub handshake_timeout_ms: u64,
    pub model_list_timeout_ms: u64,
    pub first_event_timeout_ms: u64,
    pub idle_timeout_ms: u64,
    pub shutdown_timeout_ms: u64,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            handshake_timeout_ms: 30000,
            model_list_timeout_ms: 10000,
            first_event_timeout_ms: 60000,
            idle_timeout_ms: 300000,
            shutdown_timeout_ms: 5000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AdapterCrashReason {
    HandshakeFailed,
    DiscoveryFailed,
    StreamInterrupted,
    ProtocolViolation,
    ProcessExited { exit_code: Option<i32> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamCap {
    pub max_event_count: usize,
    pub max_total_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CircuitBreakerStatus {
    Closed,
    Open { until_at: i64, reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CredentialGateState {
    HandshakePending,
    HandshakeSucceeded {
        negotiated_version: String,
        provider_id: String,
    },
    HandshakeFailed {
        error: String,
    },
}

pub struct ProtocolParser {
    max_frame_bytes: usize,
}

impl ProtocolParser {
    pub fn new(max_frame_bytes: usize) -> Self {
        Self { max_frame_bytes }
    }

    pub fn parse_line(&self, line: &str) -> Result<AdapterMessage> {
        if line.len() > self.max_frame_bytes {
            anyhow::bail!(
                "protocol frame exceeds limit of {} bytes",
                self.max_frame_bytes
            );
        }
        let msg: AdapterMessage = serde_json::from_str(line)?;
        Ok(msg)
    }

    pub fn serialize(&self, msg: &AdapterMessage) -> Result<String> {
        let s = serde_json::to_string(msg)?;
        if s.len() > self.max_frame_bytes {
            anyhow::bail!(
                "serialized message exceeds limit of {} bytes",
                self.max_frame_bytes
            );
        }
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_serialization() {
        let msg = AdapterMessage::Handshake(Handshake {
            protocol_version: PROTOCOL_VERSION_V1.to_string(),
            codex_version: "0.1.0".to_string(),
            requested_capabilities: vec!["model_list".to_string()],
        });

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"handshake\""));
        assert!(json.contains("\"protocol_version\":\"provider-adapter.v1\""));

        let deserialized: AdapterMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_parser_enforces_limit() {
        let parser = ProtocolParser::new(10);
        let res = parser.parse_line("{\"type\":\"handshake\"}");
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("exceeds limit"));
    }

    #[test]
    fn test_text_delta_serialization() {
        let msg = AdapterMessage::TextDelta(TextDelta {
            request_id: "req-1".to_string(),
            delta: "hello".to_string(),
        });
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: AdapterMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, deserialized);
    }
}
