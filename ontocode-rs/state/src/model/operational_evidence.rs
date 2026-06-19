use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use strum::AsRefStr;
use strum::Display;
use strum::EnumString;

/// Evidence domains stored in the operational evidence ledger.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsRefStr, Display, EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EvidenceDomain {
    CodeGraph,
    Workflow,
    Test,
    Doc,
    Redaction,
    Architecture,
    RuntimeTopology,
}

/// Lifecycle status for an operational evidence record.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsRefStr, Display, EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EvidenceStatus {
    Planned,
    Dispatched,
    Implemented,
    Verified,
    Stale,
    Blocked,
    Done,
    Rejected,
}

/// Risk classification attached to an operational evidence record.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsRefStr, Display, EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EvidenceRisk {
    None,
    Low,
    Medium,
    High,
    Critical,
    Unknown,
}

/// Redaction state for persisted evidence.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsRefStr, Display, EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RedactionStatus {
    Clean,
    Redacted,
    Rejected,
}

/// Full operational evidence row as stored in the state ledger.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalEvidenceRecord {
    pub id: String,
    pub evidence_domain: EvidenceDomain,
    pub source_tool: String,
    pub source_version: Option<String>,
    pub schema_version: i64,
    pub source_ref: Option<String>,
    pub repo: Option<String>,
    pub task_key: Option<String>,
    pub thread_id: Option<String>,
    pub parent_thread_id: Option<String>,
    pub child_thread_id: Option<String>,
    pub symbol_uid: Option<String>,
    pub symbol_name: Option<String>,
    pub file_path: Option<String>,
    pub process_label: Option<String>,
    pub gate_name: Option<String>,
    pub risk: Option<EvidenceRisk>,
    pub status: EvidenceStatus,
    pub summary: String,
    pub source_links: Vec<String>,
    pub metadata: Value,
    pub provenance_hash: String,
    pub redaction_status: RedactionStatus,
    pub target_head: Option<String>,
    pub graph_index_id: Option<String>,
    pub plan_hash: Option<String>,
    pub tracking_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Input for creating or updating an operational evidence row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewOperationalEvidenceRecord {
    pub evidence_domain: EvidenceDomain,
    pub source_tool: String,
    pub source_version: Option<String>,
    pub schema_version: i64,
    pub source_ref: Option<String>,
    pub repo: Option<String>,
    pub task_key: Option<String>,
    pub thread_id: Option<String>,
    pub parent_thread_id: Option<String>,
    pub child_thread_id: Option<String>,
    pub symbol_uid: Option<String>,
    pub symbol_name: Option<String>,
    pub file_path: Option<String>,
    pub process_label: Option<String>,
    pub gate_name: Option<String>,
    pub risk: Option<EvidenceRisk>,
    pub status: EvidenceStatus,
    pub summary: String,
    pub source_links: Vec<String>,
    pub metadata: Value,
    pub provenance_hash: String,
    pub redaction_status: RedactionStatus,
    pub target_head: Option<String>,
    pub graph_index_id: Option<String>,
    pub plan_hash: Option<String>,
    pub tracking_hash: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Bounded filters for internal operational evidence queries.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OperationalEvidenceQuery {
    pub task_key: Option<String>,
    pub thread_id: Option<String>,
    pub symbol_uid: Option<String>,
    pub file_path: Option<String>,
    pub evidence_domain: Option<EvidenceDomain>,
    pub gate_name: Option<String>,
    pub status: Option<EvidenceStatus>,
    pub risk: Option<EvidenceRisk>,
    pub target_head: Option<String>,
    pub fresh_at: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
    pub byte_limit: Option<usize>,
}

/// Compact summary view returned from bounded evidence queries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalEvidenceSummary {
    pub id: String,
    pub evidence_domain: EvidenceDomain,
    pub source_tool: String,
    pub source_version: Option<String>,
    pub schema_version: i64,
    pub source_ref: Option<String>,
    pub repo: Option<String>,
    pub task_key: Option<String>,
    pub thread_id: Option<String>,
    pub parent_thread_id: Option<String>,
    pub child_thread_id: Option<String>,
    pub symbol_uid: Option<String>,
    pub symbol_name: Option<String>,
    pub file_path: Option<String>,
    pub process_label: Option<String>,
    pub gate_name: Option<String>,
    pub risk: Option<EvidenceRisk>,
    pub status: EvidenceStatus,
    pub summary: String,
    pub source_links: Vec<String>,
    pub provenance_hash: String,
    pub redaction_status: RedactionStatus,
    pub target_head: Option<String>,
    pub graph_index_id: Option<String>,
    pub plan_hash: Option<String>,
    pub tracking_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Request for a compact planned-versus-done closure evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalEvidenceTaskClosureRequest {
    pub task_key: String,
    pub current_target_head: String,
    pub current_plan_hash: String,
    pub current_tracking_hash: String,
    pub has_code_edits: bool,
    pub was_dispatched: bool,
    pub explicit_no_code_closure: bool,
}

/// Verdict for a compact planned-versus-done closure evaluation.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsRefStr, Display, EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum OperationalEvidenceTaskClosureVerdict {
    Accepted,
    Rejected,
}

/// Result from a compact planned-versus-done closure evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalEvidenceTaskClosureResult {
    pub verdict: OperationalEvidenceTaskClosureVerdict,
    pub missing_gates: Vec<String>,
}

#[cfg(test)]
#[path = "operational_evidence_tests.rs"]
mod tests;
