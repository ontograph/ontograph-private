use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::EvidenceDomain;
use crate::EvidenceRisk;
use crate::EvidenceStatus;
use crate::OperationalEvidenceRecord;
use crate::RedactionStatus;

use super::StateRuntime;

const SUPPORTED_ARTIFACT_SCHEMA_VERSION: i64 = 1;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum WorkflowRecordKind {
    TaskCard,
    GateResult,
    DocLinkReport,
    TestSummary,
    RedactionReport,
    ReadinessSummary,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OperationalEvidenceArtifact {
    schema_version: i64,
    source_tool: String,
    source_version: String,
    repo: String,
    target_head: String,
    graph_index_id: String,
    created_at: i64,
    records: Vec<OperationalEvidenceArtifactRecord>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OperationalEvidenceArtifactRecord {
    id: Option<String>,
    evidence_domain: EvidenceDomain,
    record_kind: Option<WorkflowRecordKind>,
    summary: String,
    provenance_hash: Option<String>,
    status: Option<EvidenceStatus>,
    risk: Option<EvidenceRisk>,
    source_ref: Option<String>,
    task_key: Option<String>,
    thread_id: Option<String>,
    parent_thread_id: Option<String>,
    child_thread_id: Option<String>,
    symbol_uid: Option<String>,
    symbol_name: Option<String>,
    file_path: Option<String>,
    process_label: Option<String>,
    gate_name: Option<String>,
    source_links: Option<Vec<String>>,
    metadata: Option<Value>,
    redaction_status: Option<RedactionStatus>,
    plan_hash: Option<String>,
    tracking_hash: Option<String>,
    created_at: Option<i64>,
    expires_at: Option<i64>,
}

impl StateRuntime {
    /// Import bounded operational evidence artifacts from an explicit content payload.
    pub async fn import_operational_evidence_artifact_content(
        &self,
        artifact_content: Option<&str>,
    ) -> Result<usize> {
        let Some(artifact_content) = artifact_content
            .map(str::trim)
            .filter(|content| !content.is_empty())
        else {
            return Ok(0);
        };

        let artifact = parse_operational_evidence_artifact(artifact_content)?;
        self.import_operational_evidence_artifact(artifact).await
    }

    /// Import bounded operational evidence artifacts from an explicit file path.
    pub async fn import_operational_evidence_artifact_path(
        &self,
        artifact_path: Option<&Path>,
    ) -> Result<usize> {
        let Some(artifact_path) = artifact_path else {
            return Ok(0);
        };

        let artifact_content = tokio::fs::read_to_string(artifact_path)
            .await
            .with_context(|| {
                format!(
                    "read operational evidence artifact {}",
                    artifact_path.display()
                )
            })?;
        self.import_operational_evidence_artifact_content(Some(artifact_content.as_str()))
            .await
    }

    async fn import_operational_evidence_artifact(
        &self,
        artifact: OperationalEvidenceArtifact,
    ) -> Result<usize> {
        validate_artifact_envelope(&artifact)?;

        let schema_version = artifact.schema_version;
        let source_tool = artifact.source_tool;
        let source_version = artifact.source_version;
        let repo = artifact.repo;
        let target_head = artifact.target_head;
        let graph_index_id = artifact.graph_index_id;
        let created_at = artifact.created_at;
        let records = artifact.records;

        let mut imported = 0usize;
        for (index, record) in records.into_iter().enumerate() {
            let record = build_operational_evidence_record(
                schema_version,
                &source_tool,
                &source_version,
                &repo,
                &target_head,
                &graph_index_id,
                created_at,
                record,
                index,
            )?;
            self.upsert_operational_evidence_by_provenance(&record)
                .await?;
            imported += 1;
        }

        Ok(imported)
    }
}

fn parse_operational_evidence_artifact(content: &str) -> Result<OperationalEvidenceArtifact> {
    serde_json::from_str(content)
        .with_context(|| "parse operational evidence artifact JSON".to_string())
}

fn validate_artifact_envelope(artifact: &OperationalEvidenceArtifact) -> Result<()> {
    anyhow::ensure!(
        artifact.schema_version == SUPPORTED_ARTIFACT_SCHEMA_VERSION,
        "unsupported operational evidence artifact schema version {}",
        artifact.schema_version
    );
    ensure_non_empty("sourceTool", artifact.source_tool.as_str())?;
    ensure_non_empty("sourceVersion", artifact.source_version.as_str())?;
    ensure_non_empty("repo", artifact.repo.as_str())?;
    ensure_non_empty("targetHead", artifact.target_head.as_str())?;
    ensure_non_empty("graphIndexId", artifact.graph_index_id.as_str())?;
    Ok(())
}

fn build_operational_evidence_record(
    schema_version: i64,
    source_tool: &str,
    source_version: &str,
    repo: &str,
    target_head: &str,
    graph_index_id: &str,
    default_created_at: i64,
    record: OperationalEvidenceArtifactRecord,
    index: usize,
) -> Result<OperationalEvidenceRecord> {
    validate_artifact_record_is_bounded(&record)?;

    let provenance_hash = record
        .provenance_hash
        .ok_or_else(|| anyhow!("operational evidence artifact record is missing provenanceHash"))?;
    ensure_non_empty("provenanceHash", provenance_hash.as_str())?;

    let metadata = match (record.evidence_domain, record.record_kind) {
        (
            EvidenceDomain::Workflow
            | EvidenceDomain::Test
            | EvidenceDomain::Doc
            | EvidenceDomain::Redaction,
            Some(record_kind),
        ) => {
            let mut metadata = record.metadata.unwrap_or_else(|| serde_json::json!({}));
            let metadata_object = metadata.as_object_mut().ok_or_else(|| {
                anyhow!("workflow evidence artifact metadata must be a JSON object")
            })?;
            metadata_object.insert("record_kind".to_string(), serde_json::json!(record_kind));
            metadata
        }
        (EvidenceDomain::Workflow, None) => {
            return Err(anyhow!(
                "workflow evidence artifact record is missing recordKind"
            ));
        }
        (_, Some(_)) => {
            return Err(anyhow!(
                "recordKind is only allowed for workflow, test, doc, or redaction evidence"
            ));
        }
        (_, None) => record.metadata.unwrap_or_else(|| serde_json::json!({})),
    };

    let record_created_at = record.created_at.unwrap_or(default_created_at);
    let created_at = unix_timestamp_to_datetime(record_created_at)?;
    let expires_at = match record.expires_at {
        Some(expires_at) => Some(unix_timestamp_to_datetime(expires_at)?),
        None => None,
    };

    Ok(OperationalEvidenceRecord {
        id: record
            .id
            .filter(|id| !id.trim().is_empty())
            .unwrap_or_else(|| format!("artifact:{provenance_hash}:{index}")),
        evidence_domain: record.evidence_domain,
        source_tool: source_tool.to_string(),
        source_version: Some(source_version.to_string()),
        schema_version,
        source_ref: record.source_ref,
        repo: Some(repo.to_string()),
        task_key: record.task_key,
        thread_id: record.thread_id,
        parent_thread_id: record.parent_thread_id,
        child_thread_id: record.child_thread_id,
        symbol_uid: record.symbol_uid,
        symbol_name: record.symbol_name,
        file_path: record.file_path,
        process_label: record.process_label,
        gate_name: record.gate_name,
        risk: record.risk,
        status: record.status.unwrap_or(EvidenceStatus::Verified),
        summary: record.summary,
        source_links: record.source_links.unwrap_or_default(),
        metadata,
        provenance_hash,
        redaction_status: record.redaction_status.unwrap_or(RedactionStatus::Clean),
        target_head: Some(target_head.to_string()),
        graph_index_id: Some(graph_index_id.to_string()),
        plan_hash: record.plan_hash,
        tracking_hash: record.tracking_hash,
        created_at,
        expires_at,
    })
}

fn ensure_non_empty(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!(
            "operational evidence artifact {label} must not be empty"
        ))
    } else {
        Ok(())
    }
}

fn validate_artifact_record_is_bounded(record: &OperationalEvidenceArtifactRecord) -> Result<()> {
    reject_raw_artifact_text("summary", record.summary.as_str())?;
    reject_optional_raw_artifact_text("sourceRef", record.source_ref.as_deref())?;
    reject_optional_raw_artifact_text("taskKey", record.task_key.as_deref())?;
    reject_optional_raw_artifact_text("threadId", record.thread_id.as_deref())?;
    reject_optional_raw_artifact_text("parentThreadId", record.parent_thread_id.as_deref())?;
    reject_optional_raw_artifact_text("childThreadId", record.child_thread_id.as_deref())?;
    reject_optional_raw_artifact_text("symbolUid", record.symbol_uid.as_deref())?;
    reject_optional_raw_artifact_text("symbolName", record.symbol_name.as_deref())?;
    reject_optional_raw_artifact_text("filePath", record.file_path.as_deref())?;
    reject_optional_raw_artifact_text("processLabel", record.process_label.as_deref())?;
    reject_optional_raw_artifact_text("gateName", record.gate_name.as_deref())?;
    reject_optional_raw_artifact_text("planHash", record.plan_hash.as_deref())?;
    reject_optional_raw_artifact_text("trackingHash", record.tracking_hash.as_deref())?;

    if let Some(source_links) = &record.source_links {
        for (index, source_link) in source_links.iter().enumerate() {
            reject_raw_artifact_text(&format!("sourceLinks[{index}]"), source_link)?;
        }
    }

    if let Some(metadata) = &record.metadata {
        reject_raw_artifact_value("metadata", metadata)?;
    }

    Ok(())
}

fn reject_optional_raw_artifact_text(field_name: &str, value: Option<&str>) -> Result<()> {
    if let Some(value) = value {
        reject_raw_artifact_text(field_name, value)?;
    }
    Ok(())
}

fn reject_raw_artifact_value(field_path: &str, value: &Value) -> Result<()> {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) => Ok(()),
        Value::String(text) => reject_raw_artifact_text(field_path, text),
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                reject_raw_artifact_value(&format!("{field_path}[{index}]"), item)?;
            }
            Ok(())
        }
        Value::Object(items) => {
            if items.contains_key("nodes") && items.contains_key("edges") {
                anyhow::bail!(
                    "operational evidence artifact {field_path} looks like a raw graph dump"
                );
            }

            for (key, item) in items {
                let normalized_key = key.to_ascii_lowercase();
                if normalized_key.contains("output")
                    || normalized_key.contains("body")
                    || normalized_key.contains("cache")
                    || normalized_key.contains("session")
                {
                    anyhow::bail!(
                        "operational evidence artifact {field_path}.{key} looks like a blob-like payload"
                    );
                }
                reject_raw_artifact_text(&format!("{field_path} key"), key)?;
                reject_raw_artifact_value(&format!("{field_path}.{key}"), item)?;
            }
            Ok(())
        }
    }
}

fn reject_raw_artifact_text(field_name: &str, value: &str) -> Result<()> {
    if looks_like_raw_diff(value) {
        anyhow::bail!("operational evidence artifact {field_name} looks like a raw diff");
    }
    if looks_like_raw_source(value) {
        anyhow::bail!("operational evidence artifact {field_name} looks like raw source");
    }
    if looks_like_raw_graph_dump(value) {
        anyhow::bail!("operational evidence artifact {field_name} looks like a raw graph dump");
    }
    Ok(())
}

fn looks_like_raw_diff(value: &str) -> bool {
    let trimmed = value.trim_start();
    trimmed.starts_with("diff --git ")
        || (value.contains("\n@@ ") && (value.contains("\n--- ") || value.contains("\n+++ ")))
}

fn looks_like_raw_source(value: &str) -> bool {
    let trimmed = value.trim_start();
    (trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") || trimmed.contains("\nfn "))
        && value.contains('{')
}

fn looks_like_raw_graph_dump(value: &str) -> bool {
    value.to_ascii_lowercase().contains("graph dump:")
}

fn unix_timestamp_to_datetime(seconds: i64) -> Result<DateTime<Utc>> {
    DateTime::<Utc>::from_timestamp(seconds, 0)
        .ok_or_else(|| anyhow!("invalid unix timestamp {seconds}"))
}
