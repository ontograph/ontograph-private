use super::*;
use crate::EvidenceDomain;
use crate::EvidenceStatus;
use crate::OperationalEvidenceQuery;
use crate::OperationalEvidenceRecord;
use crate::OperationalEvidenceSummary;
use crate::OperationalEvidenceTaskClosureRequest;
use crate::OperationalEvidenceTaskClosureResult;
use crate::OperationalEvidenceTaskClosureVerdict;
use crate::RedactionStatus;
use chrono::DateTime;
use chrono::Utc;
use serde_json::Map;
use serde_json::Value;
use sqlx::QueryBuilder;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::sqlite::SqliteRow;

const DEFAULT_QUERY_LIMIT: usize = 50;
const MAX_QUERY_LIMIT: usize = 200;
const DEFAULT_QUERY_BYTE_LIMIT: usize = 64 * 1024;
const MAX_QUERY_BYTE_LIMIT: usize = 256 * 1024;
const MAX_SUMMARY_BYTES: usize = 8_192;
const MAX_SOURCE_LINKS_JSON_BYTES: usize = 16_384;
const MAX_METADATA_JSON_BYTES: usize = 16_384;
const PLAN_GATE_NAME: &str = "plan";
const DISPATCH_GATE_NAME: &str = "dispatch";
const IMPACT_GATE_NAME: &str = "impact";
const IMPLEMENTATION_GATE_NAME: &str = "implementation";
const TESTS_GATE_NAME: &str = "tests";
const DETECT_CHANGES_GATE_NAME: &str = "detect-changes";
const NO_CODE_CLOSURE_GATE_NAME: &str = "no-code-closure";
const FRESH_TARGET_HEAD_GATE_NAME: &str = "fresh_target_head";
const FRESH_PLAN_HASH_GATE_NAME: &str = "fresh_plan_hash";
const FRESH_TRACKING_HASH_GATE_NAME: &str = "fresh_tracking_hash";

fn epoch_seconds_to_datetime(seconds: i64) -> anyhow::Result<DateTime<Utc>> {
    DateTime::<Utc>::from_timestamp(seconds, 0)
        .ok_or_else(|| anyhow::anyhow!("invalid unix timestamp seconds: {seconds}"))
}

#[derive(Debug, Clone)]
struct PreparedOperationalEvidenceRecord {
    id: String,
    evidence_domain: String,
    source_tool: String,
    source_version: Option<String>,
    schema_version: i64,
    source_ref: Option<String>,
    repo: Option<String>,
    task_key: Option<String>,
    thread_id: Option<String>,
    parent_thread_id: Option<String>,
    child_thread_id: Option<String>,
    symbol_uid: Option<String>,
    symbol_name: Option<String>,
    file_path: Option<String>,
    process_label: Option<String>,
    gate_name: Option<String>,
    risk: Option<String>,
    status: String,
    summary: String,
    source_links_json: String,
    metadata_json: String,
    provenance_hash: String,
    redaction_status: String,
    target_head: Option<String>,
    graph_index_id: Option<String>,
    plan_hash: Option<String>,
    tracking_hash: Option<String>,
    created_at: i64,
    expires_at: Option<i64>,
}

#[derive(Debug)]
struct OperationalEvidenceRow {
    id: String,
    evidence_domain: String,
    source_tool: String,
    source_version: Option<String>,
    schema_version: i64,
    source_ref: Option<String>,
    repo: Option<String>,
    task_key: Option<String>,
    thread_id: Option<String>,
    parent_thread_id: Option<String>,
    child_thread_id: Option<String>,
    symbol_uid: Option<String>,
    symbol_name: Option<String>,
    file_path: Option<String>,
    process_label: Option<String>,
    gate_name: Option<String>,
    risk: Option<String>,
    status: String,
    summary: String,
    source_links_json: String,
    provenance_hash: String,
    redaction_status: String,
    target_head: Option<String>,
    graph_index_id: Option<String>,
    plan_hash: Option<String>,
    tracking_hash: Option<String>,
    created_at: i64,
    expires_at: Option<i64>,
}

impl OperationalEvidenceRow {
    fn try_from_row(row: &SqliteRow) -> anyhow::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            evidence_domain: row.try_get("evidence_domain")?,
            source_tool: row.try_get("source_tool")?,
            source_version: row.try_get("source_version")?,
            schema_version: row.try_get("schema_version")?,
            source_ref: row.try_get("source_ref")?,
            repo: row.try_get("repo")?,
            task_key: row.try_get("task_key")?,
            thread_id: row.try_get("thread_id")?,
            parent_thread_id: row.try_get("parent_thread_id")?,
            child_thread_id: row.try_get("child_thread_id")?,
            symbol_uid: row.try_get("symbol_uid")?,
            symbol_name: row.try_get("symbol_name")?,
            file_path: row.try_get("file_path")?,
            process_label: row.try_get("process_label")?,
            gate_name: row.try_get("gate_name")?,
            risk: row.try_get("risk")?,
            status: row.try_get("status")?,
            summary: row.try_get("summary")?,
            source_links_json: row.try_get("source_links_json")?,
            provenance_hash: row.try_get("provenance_hash")?,
            redaction_status: row.try_get("redaction_status")?,
            target_head: row.try_get("target_head")?,
            graph_index_id: row.try_get("graph_index_id")?,
            plan_hash: row.try_get("plan_hash")?,
            tracking_hash: row.try_get("tracking_hash")?,
            created_at: row.try_get("created_at")?,
            expires_at: row.try_get("expires_at")?,
        })
    }

    fn into_summary(self) -> anyhow::Result<OperationalEvidenceSummary> {
        Ok(OperationalEvidenceSummary {
            id: self.id,
            evidence_domain: self.evidence_domain.parse()?,
            source_tool: self.source_tool,
            source_version: self.source_version,
            schema_version: self.schema_version,
            source_ref: self.source_ref,
            repo: self.repo,
            task_key: self.task_key,
            thread_id: self.thread_id,
            parent_thread_id: self.parent_thread_id,
            child_thread_id: self.child_thread_id,
            symbol_uid: self.symbol_uid,
            symbol_name: self.symbol_name,
            file_path: self.file_path,
            process_label: self.process_label,
            gate_name: self.gate_name,
            risk: self.risk.map(|risk| risk.parse()).transpose()?,
            status: self.status.parse()?,
            summary: self.summary,
            source_links: serde_json::from_str(self.source_links_json.as_str())?,
            provenance_hash: self.provenance_hash,
            redaction_status: self.redaction_status.parse()?,
            target_head: self.target_head,
            graph_index_id: self.graph_index_id,
            plan_hash: self.plan_hash,
            tracking_hash: self.tracking_hash,
            created_at: epoch_seconds_to_datetime(self.created_at)?,
            expires_at: self.expires_at.map(epoch_seconds_to_datetime).transpose()?,
        })
    }
}

fn prepare_operational_evidence_record(
    record: &OperationalEvidenceRecord,
) -> anyhow::Result<PreparedOperationalEvidenceRecord> {
    validate_operational_evidence_record(record)?;
    let source_links_json = serde_json::to_string(&record.source_links)?;
    anyhow::ensure!(
        record.summary.len() <= MAX_SUMMARY_BYTES,
        "operational evidence summary exceeds {MAX_SUMMARY_BYTES} bytes"
    );
    anyhow::ensure!(
        source_links_json.len() <= MAX_SOURCE_LINKS_JSON_BYTES,
        "operational evidence source links exceed {MAX_SOURCE_LINKS_JSON_BYTES} bytes"
    );
    let metadata_json = serde_json::to_string(&record.metadata)?;
    anyhow::ensure!(
        metadata_json.len() <= MAX_METADATA_JSON_BYTES,
        "operational evidence metadata exceeds {MAX_METADATA_JSON_BYTES} bytes"
    );

    Ok(PreparedOperationalEvidenceRecord {
        id: record.id.clone(),
        evidence_domain: record.evidence_domain.as_ref().to_string(),
        source_tool: record.source_tool.clone(),
        source_version: record.source_version.clone(),
        schema_version: record.schema_version,
        source_ref: record.source_ref.clone(),
        repo: record.repo.clone(),
        task_key: record.task_key.clone(),
        thread_id: record.thread_id.clone(),
        parent_thread_id: record.parent_thread_id.clone(),
        child_thread_id: record.child_thread_id.clone(),
        symbol_uid: record.symbol_uid.clone(),
        symbol_name: record.symbol_name.clone(),
        file_path: record.file_path.clone(),
        process_label: record.process_label.clone(),
        gate_name: record.gate_name.clone(),
        risk: record.risk.map(|risk| risk.as_ref().to_string()),
        status: record.status.as_ref().to_string(),
        summary: record.summary.clone(),
        source_links_json,
        metadata_json,
        provenance_hash: record.provenance_hash.clone(),
        redaction_status: record.redaction_status.as_ref().to_string(),
        target_head: record.target_head.clone(),
        graph_index_id: record.graph_index_id.clone(),
        plan_hash: record.plan_hash.clone(),
        tracking_hash: record.tracking_hash.clone(),
        created_at: record.created_at.timestamp(),
        expires_at: record.expires_at.map(|value| value.timestamp()),
    })
}

fn validate_operational_evidence_record(record: &OperationalEvidenceRecord) -> anyhow::Result<()> {
    reject_disallowed_text("summary", &record.summary)?;
    reject_disallowed_text("source_tool", &record.source_tool)?;
    reject_optional_text("source_version", record.source_version.as_deref())?;
    reject_optional_text("source_ref", record.source_ref.as_deref())?;
    reject_optional_text("repo", record.repo.as_deref())?;
    reject_optional_text("task_key", record.task_key.as_deref())?;
    reject_optional_text("thread_id", record.thread_id.as_deref())?;
    reject_optional_text("parent_thread_id", record.parent_thread_id.as_deref())?;
    reject_optional_text("child_thread_id", record.child_thread_id.as_deref())?;
    reject_optional_text("symbol_uid", record.symbol_uid.as_deref())?;
    reject_optional_text("symbol_name", record.symbol_name.as_deref())?;
    reject_optional_text("file_path", record.file_path.as_deref())?;
    reject_optional_text("process_label", record.process_label.as_deref())?;
    reject_optional_text("gate_name", record.gate_name.as_deref())?;
    reject_optional_text("target_head", record.target_head.as_deref())?;
    reject_optional_text("graph_index_id", record.graph_index_id.as_deref())?;
    reject_optional_text("plan_hash", record.plan_hash.as_deref())?;
    reject_optional_text("tracking_hash", record.tracking_hash.as_deref())?;

    for (index, source_link) in record.source_links.iter().enumerate() {
        reject_disallowed_text(&format!("source_links[{index}]"), source_link)?;
    }

    reject_disallowed_json_value("metadata", &record.metadata)?;

    Ok(())
}

fn reject_optional_text(field_name: &str, value: Option<&str>) -> anyhow::Result<()> {
    if let Some(value) = value {
        reject_disallowed_text(field_name, value)?;
    }
    Ok(())
}

fn reject_disallowed_json_value(field_path: &str, value: &Value) -> anyhow::Result<()> {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) => Ok(()),
        Value::String(text) => reject_disallowed_text(field_path, text),
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                reject_disallowed_json_value(&format!("{field_path}[{index}]"), item)?;
            }
            Ok(())
        }
        Value::Object(entries) => {
            for (key, item) in entries {
                let next_field_path = format!("{field_path}.{key}");
                let normalized_key = key.to_ascii_lowercase();
                if is_credentialish_key(&normalized_key) {
                    if matches!(item, Value::String(text) if !text.trim().is_empty())
                        || matches!(item, Value::Array(items) if !items.is_empty())
                        || matches!(item, Value::Object(entries) if !entries.is_empty())
                    {
                        anyhow::bail!(
                            "operational evidence {next_field_path} contains disallowed raw credential value"
                        );
                    }
                }
                reject_disallowed_json_value(&next_field_path, item)?;
            }
            Ok(())
        }
    }
}

fn is_credentialish_key(normalized_key: &str) -> bool {
    matches!(
        normalized_key,
        "token"
            | "tokens"
            | "secret"
            | "secrets"
            | "password"
            | "passwords"
            | "credential"
            | "credentials"
            | "cookie"
            | "cookies"
            | "authorization"
            | "authorizations"
            | "api_key"
            | "api-key"
            | "access_key"
            | "access-key"
            | "access_token"
            | "access-token"
            | "refresh_token"
            | "refresh-token"
            | "client_secret"
            | "client-secret"
    ) || normalized_key.contains("_token")
        || normalized_key.contains("-token")
        || normalized_key.contains("_secret")
        || normalized_key.contains("-secret")
        || normalized_key.contains("_password")
        || normalized_key.contains("-password")
        || normalized_key.contains("_credential")
        || normalized_key.contains("-credential")
        || normalized_key.contains("_cookie")
        || normalized_key.contains("-cookie")
}

fn reject_disallowed_text(field_name: &str, value: &str) -> anyhow::Result<()> {
    if let Some(reason) = classify_disallowed_text(value) {
        anyhow::bail!("operational evidence {field_name} contains disallowed {reason}");
    }
    Ok(())
}

fn classify_disallowed_text(value: &str) -> Option<&'static str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("authorization:")
        || lower.contains("\nauthorization:")
        || lower.starts_with("proxy-authorization:")
        || lower.contains("\nproxy-authorization:")
    {
        return Some("authorization header");
    }
    if lower.starts_with("cookie:")
        || lower.contains("\ncookie:")
        || lower.starts_with("set-cookie:")
        || lower.contains("\nset-cookie:")
        || lower.starts_with("cookie=")
        || lower.contains("; cookie=")
        || lower.starts_with("set-cookie=")
    {
        return Some("cookie-like content");
    }
    if lower.contains("/library/keychains/")
        || lower.contains("\\library\\keychains\\")
        || lower.contains("keychains/")
        || lower.contains("keychain/")
    {
        return Some("keychain path");
    }
    if lower.starts_with("bearer ")
        || lower.contains("\nbearer ")
        || lower.contains(" bearer ")
        || looks_like_openai_token(trimmed)
        || looks_like_jwt(trimmed)
        || looks_like_aws_access_key(trimmed)
    {
        return Some("token-like content");
    }
    if contains_raw_credential_assignment(&lower) {
        return Some("raw credential value");
    }

    None
}

fn looks_like_openai_token(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("sk-") else {
        return false;
    };
    rest.len() >= 20 && rest.chars().all(|ch| ch.is_ascii_alphanumeric())
}

fn looks_like_aws_access_key(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("AKIA") else {
        return false;
    };
    rest.len() == 16
        && rest
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
}

fn looks_like_jwt(value: &str) -> bool {
    let mut segments = value.split('.');
    let Some(header) = segments.next() else {
        return false;
    };
    let Some(payload) = segments.next() else {
        return false;
    };
    let Some(signature) = segments.next() else {
        return false;
    };
    if segments.next().is_some() {
        return false;
    }
    [header, payload, signature]
        .into_iter()
        .all(|segment| segment.len() >= 8 && segment.chars().all(is_jwt_char))
}

fn is_jwt_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'
}

fn contains_raw_credential_assignment(value: &str) -> bool {
    let assignment_markers = [
        "api_key=",
        "api-key=",
        "access_key=",
        "access-key=",
        "access_token=",
        "access-token=",
        "refresh_token=",
        "refresh-token=",
        "client_secret=",
        "client-secret=",
        "password=",
        "secret=",
        "token=",
        "credential=",
        "credentials=",
        "api_key:",
        "api-key:",
        "access_key:",
        "access-key:",
        "access_token:",
        "access-token:",
        "refresh_token:",
        "refresh-token:",
        "client_secret:",
        "client-secret:",
        "password:",
        "secret:",
        "token:",
        "credential:",
        "credentials:",
    ];
    assignment_markers.iter().any(|marker| {
        value
            .split_once(marker)
            .is_some_and(|(_, rest)| rest.trim().len() >= 8)
    })
}

fn push_operational_evidence_values(
    builder: &mut QueryBuilder<Sqlite>,
    record: &PreparedOperationalEvidenceRecord,
) {
    let mut separated = builder.separated(", ");
    separated.push_bind(&record.id);
    separated.push_bind(&record.evidence_domain);
    separated.push_bind(&record.source_tool);
    separated.push_bind(&record.source_version);
    separated.push_bind(record.schema_version);
    separated.push_bind(&record.source_ref);
    separated.push_bind(&record.repo);
    separated.push_bind(&record.task_key);
    separated.push_bind(&record.thread_id);
    separated.push_bind(&record.parent_thread_id);
    separated.push_bind(&record.child_thread_id);
    separated.push_bind(&record.symbol_uid);
    separated.push_bind(&record.symbol_name);
    separated.push_bind(&record.file_path);
    separated.push_bind(&record.process_label);
    separated.push_bind(&record.gate_name);
    separated.push_bind(&record.risk);
    separated.push_bind(&record.status);
    separated.push_bind(&record.summary);
    separated.push_bind(&record.source_links_json);
    separated.push_bind(&record.metadata_json);
    separated.push_bind(&record.provenance_hash);
    separated.push_bind(&record.redaction_status);
    separated.push_bind(&record.target_head);
    separated.push_bind(&record.graph_index_id);
    separated.push_bind(&record.plan_hash);
    separated.push_bind(&record.tracking_hash);
    separated.push_bind(record.created_at);
    separated.push_bind(&record.expires_at);
}

fn push_operational_evidence_update_clause(builder: &mut QueryBuilder<Sqlite>) {
    builder.push(
        r#"
ON CONFLICT(provenance_hash) DO UPDATE SET
    evidence_domain = excluded.evidence_domain,
    source_tool = excluded.source_tool,
    source_version = excluded.source_version,
    schema_version = excluded.schema_version,
    source_ref = excluded.source_ref,
    repo = excluded.repo,
    task_key = excluded.task_key,
    thread_id = excluded.thread_id,
    parent_thread_id = excluded.parent_thread_id,
    child_thread_id = excluded.child_thread_id,
    symbol_uid = excluded.symbol_uid,
    symbol_name = excluded.symbol_name,
    file_path = excluded.file_path,
    process_label = excluded.process_label,
    gate_name = excluded.gate_name,
    risk = excluded.risk,
    status = excluded.status,
    summary = excluded.summary,
    source_links_json = excluded.source_links_json,
    metadata_json = excluded.metadata_json,
    redaction_status = excluded.redaction_status,
    target_head = excluded.target_head,
    graph_index_id = excluded.graph_index_id,
    plan_hash = excluded.plan_hash,
    tracking_hash = excluded.tracking_hash,
    expires_at = excluded.expires_at
"#,
    );
}

fn push_operational_evidence_filters(
    builder: &mut QueryBuilder<Sqlite>,
    query: &OperationalEvidenceQuery,
) {
    if let Some(task_key) = query.task_key.as_deref() {
        builder.push(" AND task_key = ").push_bind(task_key);
    }
    if let Some(thread_id) = query.thread_id.as_deref() {
        builder.push(" AND thread_id = ").push_bind(thread_id);
    }
    if let Some(symbol_uid) = query.symbol_uid.as_deref() {
        builder.push(" AND symbol_uid = ").push_bind(symbol_uid);
    }
    if let Some(file_path) = query.file_path.as_deref() {
        builder.push(" AND file_path = ").push_bind(file_path);
    }
    if let Some(evidence_domain) = query.evidence_domain {
        builder
            .push(" AND evidence_domain = ")
            .push_bind(evidence_domain.as_ref());
    }
    if let Some(gate_name) = query.gate_name.as_deref() {
        builder.push(" AND gate_name = ").push_bind(gate_name);
    }
    if let Some(status) = query.status {
        builder.push(" AND status = ").push_bind(status.as_ref());
    }
    if let Some(risk) = query.risk {
        builder.push(" AND risk = ").push_bind(risk.as_ref());
    }
    if let Some(target_head) = query.target_head.as_deref() {
        builder.push(" AND target_head = ").push_bind(target_head);
    }
    if let Some(fresh_at) = query.fresh_at {
        builder
            .push(" AND (expires_at IS NULL OR expires_at > ")
            .push_bind(fresh_at.timestamp())
            .push(")");
    }
}

fn normalize_query_limit(limit: Option<usize>) -> usize {
    match limit {
        Some(limit) => limit.min(MAX_QUERY_LIMIT),
        None => DEFAULT_QUERY_LIMIT,
    }
}

fn normalize_query_byte_limit(limit: Option<usize>) -> usize {
    match limit {
        Some(limit) => limit.min(MAX_QUERY_BYTE_LIMIT),
        None => DEFAULT_QUERY_BYTE_LIMIT,
    }
}

fn matches_requested_freshness(
    summary: &OperationalEvidenceSummary,
    request: &OperationalEvidenceTaskClosureRequest,
) -> bool {
    summary.target_head.as_deref() == Some(request.current_target_head.as_str())
        && summary.plan_hash.as_deref() == Some(request.current_plan_hash.as_str())
        && summary.tracking_hash.as_deref() == Some(request.current_tracking_hash.as_str())
}

fn gate_status_matches(gate_name: &str, status: EvidenceStatus) -> bool {
    match gate_name {
        PLAN_GATE_NAME => matches!(
            status,
            EvidenceStatus::Planned
                | EvidenceStatus::Dispatched
                | EvidenceStatus::Implemented
                | EvidenceStatus::Verified
                | EvidenceStatus::Done
        ),
        DISPATCH_GATE_NAME => matches!(
            status,
            EvidenceStatus::Dispatched
                | EvidenceStatus::Implemented
                | EvidenceStatus::Verified
                | EvidenceStatus::Done
        ),
        IMPACT_GATE_NAME | IMPLEMENTATION_GATE_NAME => matches!(
            status,
            EvidenceStatus::Implemented | EvidenceStatus::Verified | EvidenceStatus::Done
        ),
        TESTS_GATE_NAME | DETECT_CHANGES_GATE_NAME | NO_CODE_CLOSURE_GATE_NAME => {
            matches!(status, EvidenceStatus::Verified | EvidenceStatus::Done)
        }
        _ => false,
    }
}

fn fresh_gate_present(
    records: &[&OperationalEvidenceSummary],
    gate_name: &str,
    require_doc_or_workflow_domain: bool,
) -> bool {
    records.iter().any(|record| {
        record.gate_name.as_deref() == Some(gate_name)
            && gate_status_matches(gate_name, record.status)
            && (!require_doc_or_workflow_domain
                || matches!(
                    record.evidence_domain,
                    EvidenceDomain::Doc | EvidenceDomain::Workflow
                ))
    })
}

#[derive(Debug)]
struct RuntimeTopologySpawnDescendantRow {
    parent_thread_id: String,
    child_thread_id: String,
    status: String,
    child_agent_path: Option<String>,
    child_created_at: i64,
}

impl RuntimeTopologySpawnDescendantRow {
    fn into_record(self, root_thread_id: &str) -> anyhow::Result<OperationalEvidenceRecord> {
        let parent_thread_id = self.parent_thread_id;
        let child_thread_id = self.child_thread_id;
        let status = self.status;
        let child_agent_path = self.child_agent_path;
        let created_at = epoch_seconds_to_datetime(self.child_created_at)?;
        let mut metadata = Map::new();
        metadata.insert("edge_status".to_string(), Value::String(status.clone()));
        metadata.insert(
            "source_timestamp".to_string(),
            Value::Number(self.child_created_at.into()),
        );
        if let Some(agent_path) = child_agent_path.clone() {
            metadata.insert("agent_path".to_string(), Value::String(agent_path));
        }

        let mut summary = format!(
            "runtime topology {} descendant {} -> {}",
            status,
            parent_thread_id.as_str(),
            child_thread_id.as_str()
        );
        if let Some(agent_path) = child_agent_path.as_deref() {
            summary.push_str(" agent_path=");
            summary.push_str(agent_path);
        }

        let id = format!(
            "runtime-topology:{}:{}:{}:{}",
            root_thread_id,
            parent_thread_id.as_str(),
            child_thread_id.as_str(),
            self.child_created_at
        );
        let provenance_hash = format!(
            "runtime-topology:{}:{}:{}:{}",
            root_thread_id,
            parent_thread_id.as_str(),
            child_thread_id.as_str(),
            self.child_created_at
        );

        Ok(OperationalEvidenceRecord {
            id,
            evidence_domain: EvidenceDomain::RuntimeTopology,
            source_tool: "state-runtime".to_string(),
            source_version: None,
            schema_version: 1,
            source_ref: Some("thread_spawn_edges".to_string()),
            repo: None,
            task_key: None,
            thread_id: Some(root_thread_id.to_string()),
            parent_thread_id: Some(parent_thread_id),
            child_thread_id: Some(child_thread_id),
            symbol_uid: None,
            symbol_name: None,
            file_path: None,
            process_label: None,
            gate_name: None,
            risk: None,
            status: EvidenceStatus::Verified,
            summary,
            source_links: Vec::new(),
            metadata: Value::Object(metadata),
            provenance_hash,
            redaction_status: RedactionStatus::Clean,
            target_head: None,
            graph_index_id: None,
            plan_hash: None,
            tracking_hash: None,
            created_at,
            expires_at: None,
        })
    }
}

fn parse_runtime_topology_spawn_descendant_row(
    row: &SqliteRow,
) -> anyhow::Result<RuntimeTopologySpawnDescendantRow> {
    Ok(RuntimeTopologySpawnDescendantRow {
        parent_thread_id: row.try_get("parent_thread_id")?,
        child_thread_id: row.try_get("child_thread_id")?,
        status: row.try_get("status")?,
        child_agent_path: row.try_get("child_agent_path")?,
        child_created_at: row.try_get("child_created_at")?,
    })
}

impl StateRuntime {
    /// Upsert compact evidence for the full spawned-descendant topology rooted at `root_thread_id`.
    pub async fn upsert_runtime_topology_evidence_for_spawn_descendants(
        &self,
        root_thread_id: ThreadId,
    ) -> anyhow::Result<usize> {
        let root_thread_id = root_thread_id.to_string();
        let rows = self
            .list_runtime_topology_spawn_descendant_rows(root_thread_id.as_str())
            .await?;
        let mut written = 0usize;
        for row in rows {
            let record = row.into_record(root_thread_id.as_str())?;
            self.upsert_operational_evidence_by_provenance(&record)
                .await?;
            written += 1;
        }
        Ok(written)
    }

    async fn list_runtime_topology_spawn_descendant_rows(
        &self,
        root_thread_id: &str,
    ) -> anyhow::Result<Vec<RuntimeTopologySpawnDescendantRow>> {
        let rows = sqlx::query(
            r#"
WITH RECURSIVE subtree(parent_thread_id, child_thread_id, depth, status) AS (
    SELECT parent_thread_id, child_thread_id, 1, status
    FROM thread_spawn_edges
    WHERE parent_thread_id = ?
    UNION ALL
    SELECT edge.parent_thread_id, edge.child_thread_id, subtree.depth + 1, edge.status
    FROM thread_spawn_edges AS edge
    JOIN subtree ON edge.parent_thread_id = subtree.child_thread_id
)
SELECT
    subtree.parent_thread_id,
    subtree.child_thread_id,
    subtree.status,
    threads.agent_path AS child_agent_path,
    threads.created_at AS child_created_at
FROM subtree
JOIN threads ON threads.id = subtree.child_thread_id
ORDER BY subtree.depth ASC, subtree.child_thread_id ASC
            "#,
        )
        .bind(root_thread_id)
        .fetch_all(self.pool.as_ref())
        .await?;

        rows.iter()
            .map(parse_runtime_topology_spawn_descendant_row)
            .collect()
    }

    /// Insert a single operational evidence record.
    pub async fn insert_operational_evidence(
        &self,
        record: &OperationalEvidenceRecord,
    ) -> anyhow::Result<()> {
        self.persist_operational_evidence(record, false).await
    }

    /// Insert or refresh an operational evidence record identified by provenance.
    pub async fn upsert_operational_evidence_by_provenance(
        &self,
        record: &OperationalEvidenceRecord,
    ) -> anyhow::Result<()> {
        self.persist_operational_evidence(record, true).await
    }

    async fn persist_operational_evidence(
        &self,
        record: &OperationalEvidenceRecord,
        upsert_by_provenance: bool,
    ) -> anyhow::Result<()> {
        let record = prepare_operational_evidence_record(record)?;
        let mut builder = QueryBuilder::<Sqlite>::new(
            "INSERT INTO operational_evidence_records (id, evidence_domain, source_tool, source_version, schema_version, source_ref, repo, task_key, thread_id, parent_thread_id, child_thread_id, symbol_uid, symbol_name, file_path, process_label, gate_name, risk, status, summary, source_links_json, metadata_json, provenance_hash, redaction_status, target_head, graph_index_id, plan_hash, tracking_hash, created_at, expires_at) VALUES (",
        );
        push_operational_evidence_values(&mut builder, &record);
        builder.push(")");
        if upsert_by_provenance {
            push_operational_evidence_update_clause(&mut builder);
        }
        builder.build().execute(self.pool.as_ref()).await?;
        Ok(())
    }

    /// Return bounded evidence summaries ordered newest-first with deterministic tie-breaking.
    pub async fn query_operational_evidence(
        &self,
        query: OperationalEvidenceQuery,
    ) -> anyhow::Result<Vec<OperationalEvidenceSummary>> {
        let limit = normalize_query_limit(query.limit);
        if limit == 0 {
            return Ok(Vec::new());
        }

        let byte_limit = normalize_query_byte_limit(query.byte_limit);
        if byte_limit == 0 {
            return Ok(Vec::new());
        }

        let mut builder = QueryBuilder::<Sqlite>::new(
            "SELECT id, evidence_domain, source_tool, source_version, schema_version, source_ref, repo, task_key, thread_id, parent_thread_id, child_thread_id, symbol_uid, symbol_name, file_path, process_label, gate_name, risk, status, summary, source_links_json, provenance_hash, redaction_status, target_head, graph_index_id, plan_hash, tracking_hash, created_at, expires_at FROM operational_evidence_records WHERE 1 = 1",
        );
        push_operational_evidence_filters(&mut builder, &query);
        builder.push(" ORDER BY created_at DESC, id DESC LIMIT ");
        builder.push_bind(limit as i64);

        let rows = builder.build().fetch_all(self.pool.as_ref()).await?;
        let mut summaries = Vec::new();
        let mut used_bytes = 0usize;
        for row in rows {
            let row = OperationalEvidenceRow::try_from_row(&row)?;
            let summary = row.into_summary()?;
            let summary_bytes = serde_json::to_vec(&summary)?.len();
            if used_bytes + summary_bytes > byte_limit {
                break;
            }
            used_bytes += summary_bytes;
            summaries.push(summary);
        }

        Ok(summaries)
    }

    /// Remove evidence rows that have expired as of `now`.
    pub async fn prune_operational_evidence(&self, now: DateTime<Utc>) -> anyhow::Result<u64> {
        let result = sqlx::query(
            "DELETE FROM operational_evidence_records WHERE expires_at IS NOT NULL AND expires_at <= ?",
        )
        .bind(now.timestamp())
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected())
    }

    /// Evaluate whether compact evidence is sufficient to close a task.
    pub async fn evaluate_operational_evidence_task_closure(
        &self,
        request: OperationalEvidenceTaskClosureRequest,
    ) -> anyhow::Result<OperationalEvidenceTaskClosureResult> {
        let records = self
            .query_operational_evidence(OperationalEvidenceQuery {
                task_key: Some(request.task_key.clone()),
                byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
                limit: Some(MAX_QUERY_LIMIT),
                ..Default::default()
            })
            .await?;

        let has_any_records = !records.is_empty();
        let has_fresh_target_head = records.iter().any(|record| {
            record.target_head.as_deref() == Some(request.current_target_head.as_str())
        });
        let has_fresh_plan_hash = records
            .iter()
            .any(|record| record.plan_hash.as_deref() == Some(request.current_plan_hash.as_str()));
        let has_fresh_tracking_hash = records.iter().any(|record| {
            record.tracking_hash.as_deref() == Some(request.current_tracking_hash.as_str())
        });

        let fresh_records = records
            .iter()
            .filter(|record| matches_requested_freshness(record, &request))
            .collect::<Vec<_>>();

        let mut missing_gates = Vec::new();
        if has_any_records && !has_fresh_target_head {
            missing_gates.push(FRESH_TARGET_HEAD_GATE_NAME.to_string());
        }
        if has_any_records && !has_fresh_plan_hash {
            missing_gates.push(FRESH_PLAN_HASH_GATE_NAME.to_string());
        }
        if has_any_records && !has_fresh_tracking_hash {
            missing_gates.push(FRESH_TRACKING_HASH_GATE_NAME.to_string());
        }
        if !fresh_gate_present(&fresh_records, PLAN_GATE_NAME, false) {
            missing_gates.push(PLAN_GATE_NAME.to_string());
        }

        if request.was_dispatched && !fresh_gate_present(&fresh_records, DISPATCH_GATE_NAME, false)
        {
            missing_gates.push(DISPATCH_GATE_NAME.to_string());
        }

        if request.has_code_edits {
            for gate_name in [
                IMPACT_GATE_NAME,
                IMPLEMENTATION_GATE_NAME,
                TESTS_GATE_NAME,
                DETECT_CHANGES_GATE_NAME,
            ] {
                if !fresh_gate_present(&fresh_records, gate_name, false) {
                    missing_gates.push(gate_name.to_string());
                }
            }
        } else if !request.explicit_no_code_closure
            || !fresh_gate_present(&fresh_records, NO_CODE_CLOSURE_GATE_NAME, true)
        {
            missing_gates.push(NO_CODE_CLOSURE_GATE_NAME.to_string());
        }

        let verdict = if missing_gates.is_empty() {
            OperationalEvidenceTaskClosureVerdict::Accepted
        } else {
            OperationalEvidenceTaskClosureVerdict::Rejected
        };

        Ok(OperationalEvidenceTaskClosureResult {
            verdict,
            missing_gates,
        })
    }
}

#[cfg(test)]
#[path = "operational_evidence_tests.rs"]
mod tests;
