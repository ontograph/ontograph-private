use super::DEFAULT_QUERY_LIMIT;
use super::MAX_QUERY_BYTE_LIMIT;
use super::OperationalEvidenceQuery;
use super::*;
use crate::DirectionalThreadSpawnEdgeStatus;
use crate::EvidenceDomain;
use crate::EvidenceRisk;
use crate::EvidenceStatus;
use crate::OperationalEvidenceTaskClosureRequest;
use crate::OperationalEvidenceTaskClosureVerdict;
use crate::RedactionStatus;
use crate::ThreadMetadata;
use crate::runtime::test_support::test_thread_metadata;
use crate::runtime::test_support::unique_temp_dir;
use chrono::DateTime;
use chrono::Utc;
use pretty_assertions::assert_eq;
use serde_json::Value;
use serde_json::json;
use serde_json::to_vec;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

fn ts(seconds: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(seconds, 0).expect("timestamp")
}

fn record(id: &str, provenance_hash: &str, created_at: i64) -> OperationalEvidenceRecord {
    OperationalEvidenceRecord {
        id: id.to_string(),
        evidence_domain: EvidenceDomain::CodeGraph,
        source_tool: "gitnexus".to_string(),
        source_version: Some("1.6.2".to_string()),
        schema_version: 1,
        source_ref: Some("context".to_string()),
        repo: Some("/repo".to_string()),
        task_key: Some("task-a".to_string()),
        thread_id: Some("thread-a".to_string()),
        parent_thread_id: Some("parent-a".to_string()),
        child_thread_id: Some("child-a".to_string()),
        symbol_uid: Some("symbol-a".to_string()),
        symbol_name: Some("symbol_name_a".to_string()),
        file_path: Some("src/lib.rs".to_string()),
        process_label: Some("process-a".to_string()),
        gate_name: Some("gate-a".to_string()),
        risk: Some(EvidenceRisk::Low),
        status: EvidenceStatus::Verified,
        summary: "summary-a".to_string(),
        source_links: vec!["https://example.invalid/a".to_string()],
        metadata: json!({"kind": "context"}),
        provenance_hash: provenance_hash.to_string(),
        redaction_status: RedactionStatus::Clean,
        target_head: Some("head-a".to_string()),
        graph_index_id: Some("graph-a".to_string()),
        plan_hash: Some("plan-a".to_string()),
        tracking_hash: Some("tracking-a".to_string()),
        created_at: ts(created_at),
        expires_at: None,
    }
}

fn summary_from_record(record: &OperationalEvidenceRecord) -> OperationalEvidenceSummary {
    OperationalEvidenceSummary {
        id: record.id.clone(),
        evidence_domain: record.evidence_domain,
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
        risk: record.risk,
        status: record.status,
        summary: record.summary.clone(),
        source_links: record.source_links.clone(),
        provenance_hash: record.provenance_hash.clone(),
        redaction_status: record.redaction_status,
        target_head: record.target_head.clone(),
        graph_index_id: record.graph_index_id.clone(),
        plan_hash: record.plan_hash.clone(),
        tracking_hash: record.tracking_hash.clone(),
        created_at: record.created_at,
        expires_at: record.expires_at,
    }
}

async fn test_runtime() -> anyhow::Result<Arc<StateRuntime>> {
    StateRuntime::init(unique_temp_dir(), "test-provider".to_string()).await
}

async fn test_runtime_with_home() -> anyhow::Result<(Arc<StateRuntime>, PathBuf)> {
    let codex_home = unique_temp_dir();
    let runtime = StateRuntime::init(codex_home.clone(), "test-provider".to_string()).await?;
    Ok((runtime, codex_home))
}

fn spawn_thread(
    codex_home: &Path,
    thread_id: &str,
    created_at: i64,
    agent_path: Option<&str>,
) -> ThreadMetadata {
    let thread_id = ThreadId::try_from(thread_id.to_string()).expect("thread id");
    let mut metadata = test_thread_metadata(codex_home, thread_id, codex_home.to_path_buf());
    metadata.created_at = ts(created_at);
    metadata.updated_at = ts(created_at);
    metadata.agent_path = agent_path.map(str::to_string);
    metadata
}

fn runtime_topology_record(
    root_thread_id: &str,
    parent_thread_id: &str,
    child_thread_id: &str,
    status: &str,
    agent_path: Option<&str>,
    created_at: i64,
) -> OperationalEvidenceRecord {
    let mut metadata = serde_json::Map::new();
    metadata.insert("edge_status".to_string(), Value::String(status.to_string()));
    metadata.insert(
        "source_timestamp".to_string(),
        Value::Number(created_at.into()),
    );
    if let Some(agent_path) = agent_path {
        metadata.insert(
            "agent_path".to_string(),
            Value::String(agent_path.to_string()),
        );
    }

    let mut summary = format!(
        "runtime topology {} descendant {} -> {}",
        status, parent_thread_id, child_thread_id
    );
    if let Some(agent_path) = agent_path {
        summary.push_str(" agent_path=");
        summary.push_str(agent_path);
    }

    OperationalEvidenceRecord {
        id: format!(
            "runtime-topology:{root_thread_id}:{parent_thread_id}:{child_thread_id}:{created_at}"
        ),
        evidence_domain: EvidenceDomain::RuntimeTopology,
        source_tool: "state-runtime".to_string(),
        source_version: None,
        schema_version: 1,
        source_ref: Some("thread_spawn_edges".to_string()),
        repo: None,
        task_key: None,
        thread_id: Some(root_thread_id.to_string()),
        parent_thread_id: Some(parent_thread_id.to_string()),
        child_thread_id: Some(child_thread_id.to_string()),
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
        provenance_hash: format!(
            "runtime-topology:{root_thread_id}:{parent_thread_id}:{child_thread_id}:{created_at}"
        ),
        redaction_status: RedactionStatus::Clean,
        target_head: None,
        graph_index_id: None,
        plan_hash: None,
        tracking_hash: None,
        created_at: ts(created_at),
        expires_at: None,
    }
}

fn closure_gate_record(
    task_key: &str,
    gate_name: &str,
    evidence_domain: EvidenceDomain,
    status: EvidenceStatus,
    target_head: &str,
    plan_hash: &str,
    tracking_hash: &str,
    created_at: i64,
) -> OperationalEvidenceRecord {
    let mut record = record(
        &format!("00000000-0000-0000-0000-{created_at:012}"),
        &format!("{task_key}:{gate_name}:{created_at}"),
        created_at,
    );
    record.task_key = Some(task_key.to_string());
    record.gate_name = Some(gate_name.to_string());
    record.evidence_domain = evidence_domain;
    record.status = status;
    record.target_head = Some(target_head.to_string());
    record.plan_hash = Some(plan_hash.to_string());
    record.tracking_hash = Some(tracking_hash.to_string());
    record
}

fn closure_request(
    task_key: &str,
    current_target_head: &str,
    current_plan_hash: &str,
    current_tracking_hash: &str,
    has_code_edits: bool,
    was_dispatched: bool,
    explicit_no_code_closure: bool,
) -> OperationalEvidenceTaskClosureRequest {
    OperationalEvidenceTaskClosureRequest {
        task_key: task_key.to_string(),
        current_target_head: current_target_head.to_string(),
        current_plan_hash: current_plan_hash.to_string(),
        current_tracking_hash: current_tracking_hash.to_string(),
        has_code_edits,
        was_dispatched,
        explicit_no_code_closure,
    }
}

fn artifact_record(
    id: Option<&str>,
    evidence_domain: EvidenceDomain,
    provenance_hash: Option<&str>,
    risk: Option<EvidenceRisk>,
    summary: &str,
) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "evidenceDomain": evidence_domain,
        "provenanceHash": provenance_hash,
        "risk": risk,
        "status": EvidenceStatus::Verified,
        "summary": summary,
        "sourceLinks": ["fixture://evidence"],
        "metadata": {"fixture": summary},
        "createdAt": 1_700_000_123,
    })
}

fn workflow_artifact_record(
    id: Option<&str>,
    record_kind: &str,
    provenance_hash: Option<&str>,
    risk: Option<EvidenceRisk>,
    summary: &str,
) -> serde_json::Value {
    workflow_artifact_record_with_domain(
        id,
        EvidenceDomain::Workflow,
        record_kind,
        provenance_hash,
        risk,
        summary,
    )
}

fn workflow_artifact_record_with_domain(
    id: Option<&str>,
    evidence_domain: EvidenceDomain,
    record_kind: &str,
    provenance_hash: Option<&str>,
    risk: Option<EvidenceRisk>,
    summary: &str,
) -> serde_json::Value {
    let mut record = artifact_record(id, evidence_domain, provenance_hash, risk, summary);
    let record_object = record.as_object_mut().expect("record should be an object");
    record_object.insert(
        "recordKind".to_string(),
        Value::String(record_kind.to_string()),
    );
    record_object.insert(
        "metadata".to_string(),
        json!({
            "fixture": summary,
            "record_kind": record_kind,
        }),
    );
    record
}

fn artifact_json(schema_version: i64, records: Vec<serde_json::Value>) -> String {
    serde_json::to_string(&serde_json::json!({
        "schemaVersion": schema_version,
        "sourceTool": "fixture-importer",
        "sourceVersion": "1.0.0",
        "repo": "/repo",
        "targetHead": "head-123",
        "graphIndexId": "graph-123",
        "createdAt": 1_700_000_100,
        "records": records,
    }))
    .expect("artifact should serialize")
}

#[tokio::test]
async fn import_operational_evidence_artifact_content_accepts_low_high_and_critical_fixtures()
-> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let artifact = artifact_json(
        1,
        vec![
            artifact_record(
                Some("00000000-0000-0000-0000-000000000401"),
                EvidenceDomain::CodeGraph,
                Some("provenance-artifact-low"),
                Some(EvidenceRisk::Low),
                "low fixture",
            ),
            workflow_artifact_record(
                Some("00000000-0000-0000-0000-000000000402"),
                "task_card",
                Some("provenance-artifact-high"),
                Some(EvidenceRisk::High),
                "high fixture",
            ),
            artifact_record(
                Some("00000000-0000-0000-0000-000000000403"),
                EvidenceDomain::RuntimeTopology,
                Some("provenance-artifact-critical"),
                Some(EvidenceRisk::Critical),
                "critical fixture",
            ),
        ],
    );

    let imported = runtime
        .import_operational_evidence_artifact_content(Some(artifact.as_str()))
        .await?;
    assert_eq!(imported, 3);

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            limit: Some(200),
            ..Default::default()
        })
        .await?;

    assert_eq!(results.len(), 3);
    assert!(
        results
            .iter()
            .all(|summary| summary.source_tool == "fixture-importer")
    );
    assert!(
        results
            .iter()
            .all(|summary| summary.repo.as_deref() == Some("/repo"))
    );
    assert!(
        results
            .iter()
            .all(|summary| summary.target_head.as_deref() == Some("head-123"))
    );
    assert!(
        results
            .iter()
            .all(|summary| summary.graph_index_id.as_deref() == Some("graph-123"))
    );
    assert!(
        results
            .iter()
            .any(|summary| summary.risk == Some(EvidenceRisk::Low))
    );
    assert!(
        results
            .iter()
            .any(|summary| summary.risk == Some(EvidenceRisk::High))
    );
    assert!(
        results
            .iter()
            .any(|summary| summary.risk == Some(EvidenceRisk::Critical))
    );

    Ok(())
}

#[tokio::test]
async fn import_operational_evidence_artifact_content_rejects_missing_provenance() {
    let runtime = test_runtime().await.expect("runtime");
    let artifact = artifact_json(
        1,
        vec![artifact_record(
            Some("00000000-0000-0000-0000-000000000404"),
            EvidenceDomain::CodeGraph,
            None,
            Some(EvidenceRisk::Low),
            "missing provenance",
        )],
    );

    let err = runtime
        .import_operational_evidence_artifact_content(Some(artifact.as_str()))
        .await
        .expect_err("missing provenance should fail closed");
    assert!(err.to_string().contains("provenanceHash"));
}

#[tokio::test]
async fn import_operational_evidence_artifact_content_rejects_unsupported_schema_versions() {
    let runtime = test_runtime().await.expect("runtime");
    let artifact = artifact_json(
        2,
        vec![artifact_record(
            Some("00000000-0000-0000-0000-000000000405"),
            EvidenceDomain::CodeGraph,
            Some("provenance-unsupported-schema"),
            Some(EvidenceRisk::Low),
            "unsupported schema",
        )],
    );

    let err = runtime
        .import_operational_evidence_artifact_content(Some(artifact.as_str()))
        .await
        .expect_err("unsupported schema should fail closed");
    assert!(
        err.to_string()
            .contains("unsupported operational evidence artifact schema version")
    );
}

#[tokio::test]
async fn import_operational_evidence_artifact_content_accepts_all_workflow_record_kinds()
-> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let artifact = artifact_json(
        1,
        vec![
            workflow_artifact_record(
                Some("00000000-0000-0000-0000-000000000410"),
                "task_card",
                Some("provenance-task-card"),
                Some(EvidenceRisk::Low),
                "task card summary",
            ),
            workflow_artifact_record(
                Some("00000000-0000-0000-0000-000000000411"),
                "gate_result",
                Some("provenance-gate-result"),
                Some(EvidenceRisk::Low),
                "gate result summary",
            ),
            workflow_artifact_record_with_domain(
                Some("00000000-0000-0000-0000-000000000412"),
                EvidenceDomain::Doc,
                "doc_link_report",
                Some("provenance-doc-link-report"),
                Some(EvidenceRisk::Low),
                "doc-link report summary",
            ),
            workflow_artifact_record_with_domain(
                Some("00000000-0000-0000-0000-000000000413"),
                EvidenceDomain::Test,
                "test_summary",
                Some("provenance-test-summary"),
                Some(EvidenceRisk::Low),
                "test summary summary",
            ),
            workflow_artifact_record_with_domain(
                Some("00000000-0000-0000-0000-000000000414"),
                EvidenceDomain::Redaction,
                "redaction_report",
                Some("provenance-redaction-report"),
                Some(EvidenceRisk::Low),
                "redaction report summary",
            ),
            workflow_artifact_record(
                Some("00000000-0000-0000-0000-000000000415"),
                "readiness_summary",
                Some("provenance-readiness-summary"),
                Some(EvidenceRisk::Low),
                "readiness summary summary",
            ),
        ],
    );

    let imported = runtime
        .import_operational_evidence_artifact_content(Some(artifact.as_str()))
        .await?;
    assert_eq!(imported, 6);

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            limit: Some(200),
            ..Default::default()
        })
        .await?;

    assert_eq!(results.len(), 6);
    assert!(
        results
            .iter()
            .all(|summary| summary.source_tool == "fixture-importer"
                && summary.repo.as_deref() == Some("/repo"))
    );
    assert_eq!(
        results
            .iter()
            .filter(|summary| summary.evidence_domain == EvidenceDomain::Workflow)
            .count(),
        3
    );
    assert!(
        results
            .iter()
            .any(|summary| summary.evidence_domain == EvidenceDomain::Doc)
    );
    assert!(
        results
            .iter()
            .any(|summary| summary.evidence_domain == EvidenceDomain::Test)
    );
    assert!(
        results
            .iter()
            .any(|summary| summary.evidence_domain == EvidenceDomain::Redaction)
    );
    Ok(())
}

#[tokio::test]
async fn import_operational_evidence_artifact_content_rejects_raw_source_diff_and_graph_dump_fixtures()
 {
    let runtime = test_runtime().await.expect("runtime");
    for raw_fixture in [
        "fn main() { println!(\"raw source\"); }",
        "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs",
        "graph dump: node-a -> node-b",
    ] {
        let err = runtime
            .import_operational_evidence_artifact_content(Some(raw_fixture))
            .await
            .expect_err("raw fixture should be rejected");
        assert!(
            err.to_string()
                .contains("parse operational evidence artifact JSON")
        );
    }
}

#[tokio::test]
async fn import_operational_evidence_artifact_content_rejects_workflow_blob_carriers_and_secret_outputs()
-> anyhow::Result<()> {
    let runtime = test_runtime().await?;

    let oversized_summary = workflow_artifact_record(
        Some("00000000-0000-0000-0000-000000000416"),
        "task_card",
        Some("provenance-oversized-summary"),
        Some(EvidenceRisk::Low),
        &"x".repeat(8_193),
    );
    let oversized_summary_error = runtime
        .import_operational_evidence_artifact_content(Some(
            artifact_json(1, vec![oversized_summary]).as_str(),
        ))
        .await
        .expect_err("oversized workflow output should fail closed");
    assert!(
        oversized_summary_error
            .to_string()
            .contains("operational evidence summary exceeds")
    );

    let secret_summary = workflow_artifact_record(
        Some("00000000-0000-0000-0000-000000000417"),
        "gate_result",
        Some("provenance-secret-summary"),
        Some(EvidenceRisk::Low),
        "Authorization: Bearer abcdefghijklmnop",
    );
    let secret_summary_error = runtime
        .import_operational_evidence_artifact_content(Some(
            artifact_json(1, vec![secret_summary]).as_str(),
        ))
        .await
        .expect_err("secret-bearing workflow output should fail closed");
    assert!(
        secret_summary_error
            .to_string()
            .contains("authorization header")
    );

    for (field_name, metadata) in [
        ("output", json!({"output": "generated blob body"})),
        ("body", json!({"nested": {"body": "generated blob body"}})),
        ("cache", json!({"cache": {"session": "cached blob body"}})),
        (
            "session",
            json!({"context": {"sessionId": "session-123", "payload": "generated blob body"}}),
        ),
    ] {
        let mut record = workflow_artifact_record(
            Some("00000000-0000-0000-0000-000000000418"),
            "doc_link_report",
            Some(&format!("provenance-{field_name}")),
            Some(EvidenceRisk::Low),
            "doc-link report summary",
        );
        let record_object = record.as_object_mut().expect("record should be an object");
        record_object.insert("metadata".to_string(), metadata);

        let error = runtime
            .import_operational_evidence_artifact_content(Some(
                artifact_json(1, vec![record]).as_str(),
            ))
            .await
            .expect_err("blob-like metadata should fail closed");
        assert!(
            error.to_string().contains("blob-like payload"),
            "field {field_name} should be rejected: {error}"
        );
    }

    Ok(())
}

#[tokio::test]
async fn import_operational_evidence_artifact_content_rejects_raw_payloads_inside_valid_artifacts()
{
    let runtime = test_runtime().await.expect("runtime");
    let raw_cases = [
        (
            "raw-source",
            serde_json::json!({
                "summary": "fn main() { println!(\"raw source\"); }",
            }),
            "raw source",
        ),
        (
            "raw-diff",
            serde_json::json!({
                "summary": "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1 +1 @@",
            }),
            "raw diff",
        ),
        (
            "raw-graph",
            serde_json::json!({
                "metadata": {
                    "nodes": [{"id": "a"}],
                    "edges": [{"from": "a", "to": "b"}],
                },
            }),
            "raw graph dump",
        ),
    ];

    for (case, patch, expected_error) in raw_cases {
        let mut record = artifact_record(
            Some("00000000-0000-0000-0000-000000000406"),
            EvidenceDomain::CodeGraph,
            Some(&format!("provenance-{case}")),
            Some(EvidenceRisk::Low),
            "bounded summary",
        );
        let record_object = record.as_object_mut().expect("record should be an object");
        for (key, value) in patch.as_object().expect("patch should be an object") {
            record_object.insert(key.clone(), value.clone());
        }

        let artifact = artifact_json(1, vec![record]);
        let err = runtime
            .import_operational_evidence_artifact_content(Some(artifact.as_str()))
            .await
            .expect_err("valid JSON artifact with raw payload should be rejected");
        assert!(
            err.to_string().contains(expected_error),
            "case {case} should contain {expected_error}: {err}"
        );
    }
}

#[tokio::test]
async fn import_operational_evidence_artifact_path_treats_missing_input_as_no_evidence_and_errors_on_missing_file()
 {
    let runtime = test_runtime().await.expect("runtime");
    assert_eq!(
        runtime
            .import_operational_evidence_artifact_path(None)
            .await
            .expect("missing input should not import evidence"),
        0
    );

    let missing_path = std::env::temp_dir().join(format!(
        "ontocode-missing-operational-evidence-artifact-{}.json",
        std::process::id()
    ));
    let err = runtime
        .import_operational_evidence_artifact_path(Some(missing_path.as_path()))
        .await
        .expect_err("explicit missing path should fail");
    assert!(
        err.to_string()
            .contains("read operational evidence artifact")
    );
}

#[tokio::test]
async fn insert_operational_evidence_persists_a_record() -> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let record = record(
        "00000000-0000-0000-0000-000000000001",
        "provenance-a",
        1_700_000_000,
    );

    runtime.insert_operational_evidence(&record).await?;

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            ..Default::default()
        })
        .await?;

    assert_eq!(results, vec![summary_from_record(&record)]);
    Ok(())
}

#[tokio::test]
async fn upsert_operational_evidence_by_provenance_deduplicates_rows() -> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let inserted = record(
        "00000000-0000-0000-0000-000000000002",
        "provenance-b",
        1_700_000_001,
    );
    let duplicate = record(
        "00000000-0000-0000-0000-000000000099",
        "provenance-b",
        1_700_000_001,
    );

    runtime
        .upsert_operational_evidence_by_provenance(&inserted)
        .await?;
    runtime
        .upsert_operational_evidence_by_provenance(&duplicate)
        .await?;

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            ..Default::default()
        })
        .await?;

    assert_eq!(results, vec![summary_from_record(&inserted)]);
    Ok(())
}

#[tokio::test]
async fn query_operational_evidence_applies_filters() -> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let mut fresh = record(
        "00000000-0000-0000-0000-000000000010",
        "provenance-c",
        1_700_000_010,
    );
    fresh.task_key = Some("task-fresh".to_string());
    fresh.thread_id = Some("thread-fresh".to_string());
    fresh.symbol_uid = Some("symbol-fresh".to_string());
    fresh.file_path = Some("src/fresh.rs".to_string());
    fresh.evidence_domain = EvidenceDomain::Workflow;
    fresh.gate_name = Some("gate-fresh".to_string());
    fresh.status = EvidenceStatus::Verified;
    fresh.risk = Some(EvidenceRisk::High);
    fresh.target_head = Some("head-fresh".to_string());
    fresh.expires_at = Some(ts(1_700_000_100));

    let mut stale = record(
        "00000000-0000-0000-0000-000000000011",
        "provenance-d",
        1_700_000_011,
    );
    stale.task_key = Some("task-stale".to_string());
    stale.thread_id = Some("thread-stale".to_string());
    stale.symbol_uid = Some("symbol-stale".to_string());
    stale.file_path = Some("src/stale.rs".to_string());
    stale.evidence_domain = EvidenceDomain::RuntimeTopology;
    stale.gate_name = Some("gate-stale".to_string());
    stale.status = EvidenceStatus::Blocked;
    stale.risk = None;
    stale.target_head = Some("head-stale".to_string());
    stale.expires_at = Some(ts(1_699_999_999));

    runtime.insert_operational_evidence(&fresh).await?;
    runtime.insert_operational_evidence(&stale).await?;

    let assert_single = |results: Vec<OperationalEvidenceSummary>,
                         expected: &OperationalEvidenceRecord| {
        assert_eq!(results, vec![summary_from_record(expected)]);
    };

    assert_single(
        runtime
            .query_operational_evidence(OperationalEvidenceQuery {
                task_key: Some("task-fresh".to_string()),
                byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
                ..Default::default()
            })
            .await?,
        &fresh,
    );
    assert_single(
        runtime
            .query_operational_evidence(OperationalEvidenceQuery {
                thread_id: Some("thread-stale".to_string()),
                byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
                ..Default::default()
            })
            .await?,
        &stale,
    );
    assert_single(
        runtime
            .query_operational_evidence(OperationalEvidenceQuery {
                symbol_uid: Some("symbol-fresh".to_string()),
                byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
                ..Default::default()
            })
            .await?,
        &fresh,
    );
    assert_single(
        runtime
            .query_operational_evidence(OperationalEvidenceQuery {
                file_path: Some("src/stale.rs".to_string()),
                byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
                ..Default::default()
            })
            .await?,
        &stale,
    );
    assert_single(
        runtime
            .query_operational_evidence(OperationalEvidenceQuery {
                evidence_domain: Some(EvidenceDomain::Workflow),
                byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
                ..Default::default()
            })
            .await?,
        &fresh,
    );
    assert_single(
        runtime
            .query_operational_evidence(OperationalEvidenceQuery {
                gate_name: Some("gate-stale".to_string()),
                byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
                ..Default::default()
            })
            .await?,
        &stale,
    );
    assert_single(
        runtime
            .query_operational_evidence(OperationalEvidenceQuery {
                status: Some(EvidenceStatus::Verified),
                byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
                ..Default::default()
            })
            .await?,
        &fresh,
    );
    assert_single(
        runtime
            .query_operational_evidence(OperationalEvidenceQuery {
                risk: Some(EvidenceRisk::High),
                byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
                ..Default::default()
            })
            .await?,
        &fresh,
    );
    assert_single(
        runtime
            .query_operational_evidence(OperationalEvidenceQuery {
                target_head: Some("head-stale".to_string()),
                byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
                ..Default::default()
            })
            .await?,
        &stale,
    );

    let fresh_results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            fresh_at: Some(ts(1_700_000_050)),
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            ..Default::default()
        })
        .await?;
    assert_eq!(fresh_results, vec![summary_from_record(&fresh)]);

    Ok(())
}

#[tokio::test]
async fn query_operational_evidence_orders_ties_deterministically() -> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let first = record(
        "00000000-0000-0000-0000-000000000101",
        "provenance-e",
        1_700_000_020,
    );
    let second = record(
        "00000000-0000-0000-0000-000000000102",
        "provenance-f",
        1_700_000_020,
    );
    let third = record(
        "00000000-0000-0000-0000-000000000103",
        "provenance-g",
        1_700_000_020,
    );

    runtime.insert_operational_evidence(&first).await?;
    runtime.insert_operational_evidence(&second).await?;
    runtime.insert_operational_evidence(&third).await?;

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            limit: Some(200),
            ..Default::default()
        })
        .await?;

    assert_eq!(
        results,
        vec![
            summary_from_record(&third),
            summary_from_record(&second),
            summary_from_record(&first),
        ]
    );
    Ok(())
}

#[tokio::test]
async fn query_operational_evidence_honors_default_limit() -> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    for index in 0..(DEFAULT_QUERY_LIMIT + 1) {
        let mut evidence = record(
            &format!("00000000-0000-0000-0000-{index:012}"),
            &format!("provenance-limit-{index}"),
            1_700_000_100,
        );
        evidence.summary = format!("summary-{index}");
        runtime.insert_operational_evidence(&evidence).await?;
    }

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            ..Default::default()
        })
        .await?;

    assert_eq!(results.len(), DEFAULT_QUERY_LIMIT);
    Ok(())
}

#[tokio::test]
async fn query_operational_evidence_keeps_filtered_queries_bounded_over_many_rows()
-> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    for index in 0..600 {
        let mut evidence = record(
            &format!("00000000-0000-0000-0000-{index:012}"),
            &format!("provenance-many-{index}"),
            1_700_002_000 + index,
        );
        evidence.summary = format!("summary-{index}");
        if index % 2 == 0 {
            evidence.task_key = Some("task-hot".to_string());
            evidence.target_head = Some("head-hot".to_string());
            evidence.evidence_domain = EvidenceDomain::CodeGraph;
            evidence.status = EvidenceStatus::Verified;
            evidence.risk = Some(EvidenceRisk::High);
        } else {
            evidence.task_key = Some("task-cold".to_string());
            evidence.target_head = Some("head-cold".to_string());
            evidence.evidence_domain = EvidenceDomain::Workflow;
            evidence.status = EvidenceStatus::Blocked;
            evidence.risk = Some(EvidenceRisk::Low);
        }
        runtime.insert_operational_evidence(&evidence).await?;
    }

    let started_at = Instant::now();
    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            task_key: Some("task-hot".to_string()),
            evidence_domain: Some(EvidenceDomain::CodeGraph),
            status: Some(EvidenceStatus::Verified),
            risk: Some(EvidenceRisk::High),
            target_head: Some("head-hot".to_string()),
            limit: Some(25),
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            ..Default::default()
        })
        .await?;
    let elapsed = started_at.elapsed();

    assert_eq!(results.len(), 25);
    assert!(
        elapsed < Duration::from_secs(2),
        "filtered operational evidence query took {elapsed:?}"
    );
    assert_eq!(results[0].summary, "summary-598");
    assert_eq!(results[24].summary, "summary-550");
    assert!(results.iter().all(|summary| {
        summary.task_key.as_deref() == Some("task-hot")
            && summary.target_head.as_deref() == Some("head-hot")
            && summary.evidence_domain == EvidenceDomain::CodeGraph
            && summary.status == EvidenceStatus::Verified
            && summary.risk == Some(EvidenceRisk::High)
    }));

    Ok(())
}

#[tokio::test]
async fn query_operational_evidence_stops_at_byte_cap() -> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let mut first = record(
        "00000000-0000-0000-0000-000000000202",
        "provenance-h",
        1_700_000_200,
    );
    first.summary = "a".repeat(1_024);
    let mut second = record(
        "00000000-0000-0000-0000-000000000201",
        "provenance-i",
        1_700_000_200,
    );
    second.summary = "b".repeat(1_024);

    runtime.insert_operational_evidence(&first).await?;
    runtime.insert_operational_evidence(&second).await?;

    let byte_limit = to_vec(&summary_from_record(&first))?.len();
    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(byte_limit),
            limit: Some(200),
            ..Default::default()
        })
        .await?;

    assert_eq!(results, vec![summary_from_record(&first)]);
    Ok(())
}

#[tokio::test]
async fn prune_operational_evidence_removes_only_expired_rows() -> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let mut expired = record(
        "00000000-0000-0000-0000-000000000301",
        "provenance-j",
        1_700_000_300,
    );
    expired.expires_at = Some(ts(1_700_000_000));
    let mut active = record(
        "00000000-0000-0000-0000-000000000302",
        "provenance-k",
        1_700_000_301,
    );
    active.expires_at = Some(ts(1_700_000_500));

    runtime.insert_operational_evidence(&expired).await?;
    runtime.insert_operational_evidence(&active).await?;

    let pruned = runtime
        .prune_operational_evidence(ts(1_700_000_100))
        .await?;
    assert_eq!(pruned, 1);

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            limit: Some(200),
            ..Default::default()
        })
        .await?;

    assert_eq!(results, vec![summary_from_record(&active)]);
    Ok(())
}

#[tokio::test]
async fn insert_operational_evidence_rejects_oversized_summary_and_metadata_before_persistence()
-> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let mut oversized_summary = record(
        "00000000-0000-0000-0000-000000000401",
        "provenance-l",
        1_700_000_400,
    );
    oversized_summary.summary = "x".repeat(8_193);
    let summary_error = runtime
        .insert_operational_evidence(&oversized_summary)
        .await
        .expect_err("oversized summary should be rejected");
    assert!(
        summary_error
            .to_string()
            .contains("operational evidence summary exceeds")
    );

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            ..Default::default()
        })
        .await?;
    assert!(results.is_empty());

    let mut oversized_metadata = record(
        "00000000-0000-0000-0000-000000000402",
        "provenance-m",
        1_700_000_401,
    );
    oversized_metadata.metadata = json!({
        "payload": "y".repeat(16_385)
    });
    let metadata_error = runtime
        .insert_operational_evidence(&oversized_metadata)
        .await
        .expect_err("oversized metadata should be rejected");
    assert!(
        metadata_error
            .to_string()
            .contains("operational evidence metadata exceeds")
    );

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            ..Default::default()
        })
        .await?;
    assert!(results.is_empty());

    Ok(())
}

#[tokio::test]
async fn insert_operational_evidence_rejects_obvious_secrets_before_persistence()
-> anyhow::Result<()> {
    let runtime = test_runtime().await?;

    macro_rules! assert_rejected {
        ($record:expr, $reason:literal) => {{
            let error = runtime
                .insert_operational_evidence(&$record)
                .await
                .expect_err("secret-like content should be rejected");
            assert!(
                error.to_string().contains($reason),
                "expected {reason} rejection, got {error:?}",
                reason = $reason,
            );
        }};
    }

    let mut authorization_header = record(
        "00000000-0000-0000-0000-000000000501",
        "provenance-secret-a",
        1_700_000_500,
    );
    authorization_header.summary = "Authorization: Bearer abcdefghijklmnop".to_string();
    assert_rejected!(authorization_header, "authorization header");

    let mut cookie_header = record(
        "00000000-0000-0000-0000-000000000502",
        "provenance-secret-b",
        1_700_000_501,
    );
    cookie_header.source_ref = Some("Cookie: session_id=abcd1234".to_string());
    assert_rejected!(cookie_header, "cookie-like content");

    let mut token_like = record(
        "00000000-0000-0000-0000-000000000503",
        "provenance-secret-c",
        1_700_000_502,
    );
    token_like.file_path = Some("sk-1234567890abcdef1234".to_string());
    assert_rejected!(token_like, "token-like content");

    let mut keychain_path = record(
        "00000000-0000-0000-0000-000000000504",
        "provenance-secret-d",
        1_700_000_503,
    );
    keychain_path.process_label =
        Some("/Users/alice/Library/Keychains/login.keychain-db".to_string());
    assert_rejected!(keychain_path, "keychain path");

    let mut raw_credentials = record(
        "00000000-0000-0000-0000-000000000505",
        "provenance-secret-e",
        1_700_000_504,
    );
    raw_credentials.metadata = json!({
        "credentials": {
            "password": "super-secret-password",
            "refresh_token": "refresh-token-value",
        }
    });
    assert_rejected!(raw_credentials, "raw credential value");

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            ..Default::default()
        })
        .await?;
    assert!(results.is_empty());

    Ok(())
}

#[tokio::test]
async fn upsert_runtime_topology_evidence_records_open_descendants() -> anyhow::Result<()> {
    let (runtime, codex_home) = test_runtime_with_home().await?;
    let root = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000601",
        1_700_000_600,
        None,
    );
    let open_left = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000602",
        1_700_000_700,
        Some("agents/left"),
    );
    let open_right = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000603",
        1_700_000_700,
        None,
    );

    runtime.upsert_thread(&root).await?;
    runtime.upsert_thread(&open_left).await?;
    runtime.upsert_thread(&open_right).await?;
    runtime
        .upsert_thread_spawn_edge(
            root.id,
            open_left.id,
            DirectionalThreadSpawnEdgeStatus::Open,
        )
        .await?;
    runtime
        .upsert_thread_spawn_edge(
            root.id,
            open_right.id,
            DirectionalThreadSpawnEdgeStatus::Open,
        )
        .await?;

    let written = runtime
        .upsert_runtime_topology_evidence_for_spawn_descendants(root.id)
        .await?;
    assert_eq!(written, 2);

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            evidence_domain: Some(EvidenceDomain::RuntimeTopology),
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            limit: Some(200),
            ..Default::default()
        })
        .await?;

    assert_eq!(
        results,
        vec![
            summary_from_record(&runtime_topology_record(
                "00000000-0000-0000-0000-000000000601",
                "00000000-0000-0000-0000-000000000601",
                "00000000-0000-0000-0000-000000000603",
                "open",
                None,
                1_700_000_700,
            )),
            summary_from_record(&runtime_topology_record(
                "00000000-0000-0000-0000-000000000601",
                "00000000-0000-0000-0000-000000000601",
                "00000000-0000-0000-0000-000000000602",
                "open",
                Some("agents/left"),
                1_700_000_700,
            )),
        ]
    );

    Ok(())
}

#[tokio::test]
async fn upsert_runtime_topology_evidence_records_closed_descendants() -> anyhow::Result<()> {
    let (runtime, codex_home) = test_runtime_with_home().await?;
    let root = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000604",
        1_700_000_800,
        None,
    );
    let closed_child = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000605",
        1_700_000_810,
        Some("agents/closed"),
    );

    runtime.upsert_thread(&root).await?;
    runtime.upsert_thread(&closed_child).await?;
    runtime
        .upsert_thread_spawn_edge(
            root.id,
            closed_child.id,
            DirectionalThreadSpawnEdgeStatus::Closed,
        )
        .await?;

    let written = runtime
        .upsert_runtime_topology_evidence_for_spawn_descendants(root.id)
        .await?;
    assert_eq!(written, 1);

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            evidence_domain: Some(EvidenceDomain::RuntimeTopology),
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            limit: Some(200),
            ..Default::default()
        })
        .await?;

    assert_eq!(
        results,
        vec![summary_from_record(&runtime_topology_record(
            "00000000-0000-0000-0000-000000000604",
            "00000000-0000-0000-0000-000000000604",
            "00000000-0000-0000-0000-000000000605",
            "closed",
            Some("agents/closed"),
            1_700_000_810,
        ))]
    );

    Ok(())
}

#[tokio::test]
async fn upsert_runtime_topology_evidence_records_mixed_status_descendant_paths()
-> anyhow::Result<()> {
    let (runtime, codex_home) = test_runtime_with_home().await?;
    let root = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000610",
        1_700_000_850,
        None,
    );
    let open_child = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000611",
        1_700_000_860,
        Some("agents/open"),
    );
    let closed_grandchild = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000612",
        1_700_000_870,
        Some("agents/closed-grandchild"),
    );

    runtime.upsert_thread(&root).await?;
    runtime.upsert_thread(&open_child).await?;
    runtime.upsert_thread(&closed_grandchild).await?;
    runtime
        .upsert_thread_spawn_edge(
            root.id,
            open_child.id,
            DirectionalThreadSpawnEdgeStatus::Open,
        )
        .await?;
    runtime
        .upsert_thread_spawn_edge(
            open_child.id,
            closed_grandchild.id,
            DirectionalThreadSpawnEdgeStatus::Closed,
        )
        .await?;

    let written = runtime
        .upsert_runtime_topology_evidence_for_spawn_descendants(root.id)
        .await?;
    assert_eq!(written, 2);

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            evidence_domain: Some(EvidenceDomain::RuntimeTopology),
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            limit: Some(200),
            ..Default::default()
        })
        .await?;

    assert_eq!(
        results,
        vec![
            summary_from_record(&runtime_topology_record(
                "00000000-0000-0000-0000-000000000610",
                "00000000-0000-0000-0000-000000000611",
                "00000000-0000-0000-0000-000000000612",
                "closed",
                Some("agents/closed-grandchild"),
                1_700_000_870,
            )),
            summary_from_record(&runtime_topology_record(
                "00000000-0000-0000-0000-000000000610",
                "00000000-0000-0000-0000-000000000610",
                "00000000-0000-0000-0000-000000000611",
                "open",
                Some("agents/open"),
                1_700_000_860,
            )),
        ]
    );

    Ok(())
}

#[tokio::test]
async fn upsert_runtime_topology_evidence_orders_descendants_deterministically()
-> anyhow::Result<()> {
    let (runtime, codex_home) = test_runtime_with_home().await?;
    let root = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000606",
        1_700_000_900,
        None,
    );
    let child_a = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000607",
        1_700_000_910,
        None,
    );
    let child_b = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000608",
        1_700_000_910,
        None,
    );
    let child_c = spawn_thread(
        &codex_home,
        "00000000-0000-0000-0000-000000000609",
        1_700_000_910,
        None,
    );

    runtime.upsert_thread(&root).await?;
    runtime.upsert_thread(&child_a).await?;
    runtime.upsert_thread(&child_b).await?;
    runtime.upsert_thread(&child_c).await?;
    runtime
        .upsert_thread_spawn_edge(root.id, child_a.id, DirectionalThreadSpawnEdgeStatus::Open)
        .await?;
    runtime
        .upsert_thread_spawn_edge(root.id, child_b.id, DirectionalThreadSpawnEdgeStatus::Open)
        .await?;
    runtime
        .upsert_thread_spawn_edge(root.id, child_c.id, DirectionalThreadSpawnEdgeStatus::Open)
        .await?;

    runtime
        .upsert_runtime_topology_evidence_for_spawn_descendants(root.id)
        .await?;

    let results = runtime
        .query_operational_evidence(OperationalEvidenceQuery {
            evidence_domain: Some(EvidenceDomain::RuntimeTopology),
            byte_limit: Some(MAX_QUERY_BYTE_LIMIT),
            limit: Some(200),
            ..Default::default()
        })
        .await?;

    assert_eq!(
        results,
        vec![
            summary_from_record(&runtime_topology_record(
                "00000000-0000-0000-0000-000000000606",
                "00000000-0000-0000-0000-000000000606",
                "00000000-0000-0000-0000-000000000609",
                "open",
                None,
                1_700_000_910,
            )),
            summary_from_record(&runtime_topology_record(
                "00000000-0000-0000-0000-000000000606",
                "00000000-0000-0000-0000-000000000606",
                "00000000-0000-0000-0000-000000000608",
                "open",
                None,
                1_700_000_910,
            )),
            summary_from_record(&runtime_topology_record(
                "00000000-0000-0000-0000-000000000606",
                "00000000-0000-0000-0000-000000000606",
                "00000000-0000-0000-0000-000000000607",
                "open",
                None,
                1_700_000_910,
            )),
        ]
    );

    Ok(())
}

#[tokio::test]
async fn evaluate_operational_evidence_task_closure_accepts_code_task() -> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let task_key = "task-code-accepted";
    let current_target_head = "head-code";
    let current_plan_hash = "plan-code";
    let current_tracking_hash = "tracking-code";

    for (created_at, gate_name, evidence_domain, status) in [
        (
            1_700_001_000,
            "plan",
            EvidenceDomain::Workflow,
            EvidenceStatus::Planned,
        ),
        (
            1_700_001_001,
            "dispatch",
            EvidenceDomain::Workflow,
            EvidenceStatus::Dispatched,
        ),
        (
            1_700_001_002,
            "impact",
            EvidenceDomain::CodeGraph,
            EvidenceStatus::Verified,
        ),
        (
            1_700_001_003,
            "implementation",
            EvidenceDomain::CodeGraph,
            EvidenceStatus::Implemented,
        ),
        (
            1_700_001_004,
            "tests",
            EvidenceDomain::Test,
            EvidenceStatus::Verified,
        ),
        (
            1_700_001_005,
            "detect-changes",
            EvidenceDomain::CodeGraph,
            EvidenceStatus::Verified,
        ),
    ] {
        runtime
            .insert_operational_evidence(&closure_gate_record(
                task_key,
                gate_name,
                evidence_domain,
                status,
                current_target_head,
                current_plan_hash,
                current_tracking_hash,
                created_at,
            ))
            .await?;
    }

    let result = runtime
        .evaluate_operational_evidence_task_closure(closure_request(
            task_key,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            true,
            true,
            false,
        ))
        .await?;

    assert_eq!(
        result,
        crate::OperationalEvidenceTaskClosureResult {
            verdict: OperationalEvidenceTaskClosureVerdict::Accepted,
            missing_gates: Vec::new(),
        }
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_operational_evidence_task_closure_accepts_no_code_documentation_task()
-> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let task_key = "task-no-code-accepted";
    let current_target_head = "head-doc";
    let current_plan_hash = "plan-doc";
    let current_tracking_hash = "tracking-doc";

    runtime
        .insert_operational_evidence(&closure_gate_record(
            task_key,
            "plan",
            EvidenceDomain::Workflow,
            EvidenceStatus::Planned,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            1_700_001_010,
        ))
        .await?;
    runtime
        .insert_operational_evidence(&closure_gate_record(
            task_key,
            "no-code-closure",
            EvidenceDomain::Doc,
            EvidenceStatus::Done,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            1_700_001_011,
        ))
        .await?;

    let result = runtime
        .evaluate_operational_evidence_task_closure(closure_request(
            task_key,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            false,
            false,
            true,
        ))
        .await?;

    assert_eq!(
        result,
        crate::OperationalEvidenceTaskClosureResult {
            verdict: OperationalEvidenceTaskClosureVerdict::Accepted,
            missing_gates: Vec::new(),
        }
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_operational_evidence_task_closure_rejects_missing_tests() -> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let task_key = "task-code-missing-tests";
    let current_target_head = "head-tests";
    let current_plan_hash = "plan-tests";
    let current_tracking_hash = "tracking-tests";

    for (created_at, gate_name, evidence_domain, status) in [
        (
            1_700_001_020,
            "plan",
            EvidenceDomain::Workflow,
            EvidenceStatus::Planned,
        ),
        (
            1_700_001_021,
            "dispatch",
            EvidenceDomain::Workflow,
            EvidenceStatus::Dispatched,
        ),
        (
            1_700_001_022,
            "impact",
            EvidenceDomain::CodeGraph,
            EvidenceStatus::Verified,
        ),
        (
            1_700_001_023,
            "implementation",
            EvidenceDomain::CodeGraph,
            EvidenceStatus::Implemented,
        ),
        (
            1_700_001_024,
            "detect-changes",
            EvidenceDomain::CodeGraph,
            EvidenceStatus::Verified,
        ),
    ] {
        runtime
            .insert_operational_evidence(&closure_gate_record(
                task_key,
                gate_name,
                evidence_domain,
                status,
                current_target_head,
                current_plan_hash,
                current_tracking_hash,
                created_at,
            ))
            .await?;
    }

    let result = runtime
        .evaluate_operational_evidence_task_closure(closure_request(
            task_key,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            true,
            true,
            false,
        ))
        .await?;

    assert_eq!(
        result.verdict,
        OperationalEvidenceTaskClosureVerdict::Rejected
    );
    assert!(result.missing_gates.iter().any(|gate| gate == "tests"));
    Ok(())
}

#[tokio::test]
async fn evaluate_operational_evidence_task_closure_rejects_stale_target_head() -> anyhow::Result<()>
{
    let runtime = test_runtime().await?;
    let task_key = "task-code-stale-head";
    let current_target_head = "head-current";
    let stale_target_head = "head-stale";
    let current_plan_hash = "plan-stale";
    let current_tracking_hash = "tracking-stale";

    for (created_at, gate_name, evidence_domain, status) in [
        (
            1_700_001_030,
            "plan",
            EvidenceDomain::Workflow,
            EvidenceStatus::Planned,
        ),
        (
            1_700_001_031,
            "dispatch",
            EvidenceDomain::Workflow,
            EvidenceStatus::Dispatched,
        ),
        (
            1_700_001_032,
            "impact",
            EvidenceDomain::CodeGraph,
            EvidenceStatus::Verified,
        ),
        (
            1_700_001_033,
            "implementation",
            EvidenceDomain::CodeGraph,
            EvidenceStatus::Implemented,
        ),
        (
            1_700_001_034,
            "tests",
            EvidenceDomain::Test,
            EvidenceStatus::Verified,
        ),
        (
            1_700_001_035,
            "detect-changes",
            EvidenceDomain::CodeGraph,
            EvidenceStatus::Verified,
        ),
    ] {
        runtime
            .insert_operational_evidence(&closure_gate_record(
                task_key,
                gate_name,
                evidence_domain,
                status,
                stale_target_head,
                current_plan_hash,
                current_tracking_hash,
                created_at,
            ))
            .await?;
    }

    let result = runtime
        .evaluate_operational_evidence_task_closure(closure_request(
            task_key,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            true,
            true,
            false,
        ))
        .await?;

    assert_eq!(
        result.verdict,
        OperationalEvidenceTaskClosureVerdict::Rejected
    );
    assert!(
        result
            .missing_gates
            .iter()
            .any(|gate| gate == "fresh_target_head")
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_operational_evidence_task_closure_rejects_chat_only_completion()
-> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let task_key = "task-chat-only";
    let current_target_head = "head-chat";
    let current_plan_hash = "plan-chat";
    let current_tracking_hash = "tracking-chat";

    runtime
        .insert_operational_evidence(&closure_gate_record(
            task_key,
            "plan",
            EvidenceDomain::Workflow,
            EvidenceStatus::Planned,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            1_700_001_040,
        ))
        .await?;

    let result = runtime
        .evaluate_operational_evidence_task_closure(closure_request(
            task_key,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            false,
            false,
            true,
        ))
        .await?;

    assert_eq!(
        result.verdict,
        OperationalEvidenceTaskClosureVerdict::Rejected
    );
    assert!(
        result
            .missing_gates
            .iter()
            .any(|gate| gate == "no-code-closure")
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_operational_evidence_task_closure_rejects_plan_only_without_no_code_closure()
-> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let task_key = "task-plan-only";
    let current_target_head = "head-plan-only";
    let current_plan_hash = "plan-plan-only";
    let current_tracking_hash = "tracking-plan-only";

    runtime
        .insert_operational_evidence(&closure_gate_record(
            task_key,
            "plan",
            EvidenceDomain::Workflow,
            EvidenceStatus::Planned,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            1_700_001_050,
        ))
        .await?;

    let result = runtime
        .evaluate_operational_evidence_task_closure(closure_request(
            task_key,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            false,
            false,
            false,
        ))
        .await?;

    assert_eq!(
        result.verdict,
        OperationalEvidenceTaskClosureVerdict::Rejected
    );
    assert!(
        result
            .missing_gates
            .iter()
            .any(|gate| gate == "no-code-closure")
    );
    Ok(())
}

#[tokio::test]
async fn evaluate_operational_evidence_task_closure_does_not_let_no_code_flag_bypass_code_gates()
-> anyhow::Result<()> {
    let runtime = test_runtime().await?;
    let task_key = "task-no-code-flag-code-edits";
    let current_target_head = "head-code-flag";
    let current_plan_hash = "plan-code-flag";
    let current_tracking_hash = "tracking-code-flag";

    for (created_at, gate_name, evidence_domain, status) in [
        (
            1_700_001_060,
            "plan",
            EvidenceDomain::Workflow,
            EvidenceStatus::Planned,
        ),
        (
            1_700_001_061,
            "no-code-closure",
            EvidenceDomain::Doc,
            EvidenceStatus::Done,
        ),
    ] {
        runtime
            .insert_operational_evidence(&closure_gate_record(
                task_key,
                gate_name,
                evidence_domain,
                status,
                current_target_head,
                current_plan_hash,
                current_tracking_hash,
                created_at,
            ))
            .await?;
    }

    let result = runtime
        .evaluate_operational_evidence_task_closure(closure_request(
            task_key,
            current_target_head,
            current_plan_hash,
            current_tracking_hash,
            true,
            true,
            true,
        ))
        .await?;

    assert_eq!(
        result.verdict,
        OperationalEvidenceTaskClosureVerdict::Rejected
    );
    for gate_name in [
        "dispatch",
        "impact",
        "implementation",
        "tests",
        "detect-changes",
    ] {
        assert!(
            result.missing_gates.iter().any(|gate| gate == gate_name),
            "missing gates should include {gate_name}: {:?}",
            result.missing_gates
        );
    }
    Ok(())
}

#[test]
fn dependency_guard_rejects_forbidden_runtime_core_app_server_sdk_dependencies() {
    let manifest_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifests = [
        manifest_root.join("Cargo.toml"),
        manifest_root.join("../core/Cargo.toml"),
        manifest_root.join("../app-server/Cargo.toml"),
        manifest_root.join("../responses-api-proxy/npm/package.json"),
    ];
    let forbidden_dependencies = [
        "gitnexus",
        "@ladybugdb/core",
        "tree-sitter",
        "graphology",
        "onnx",
        "transformers",
        "express",
        "@modelcontextprotocol/sdk",
        "lean-ctx",
    ];

    for manifest_path in manifests {
        let contents = fs::read_to_string(&manifest_path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", manifest_path.display()));
        if manifest_path
            .extension()
            .and_then(|extension| extension.to_str())
            == Some("json")
        {
            assert_package_json_has_no_forbidden_dependencies(
                &manifest_path,
                &contents,
                &forbidden_dependencies,
            );
        } else {
            assert_cargo_manifest_has_no_forbidden_dependencies(
                &manifest_path,
                &contents,
                &forbidden_dependencies,
            );
        }
    }
}

fn assert_cargo_manifest_has_no_forbidden_dependencies(
    manifest_path: &std::path::Path,
    contents: &str,
    forbidden_dependencies: &[&str],
) {
    let mut in_dependencies_table = false;
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_dependencies_table = trimmed.contains("dependencies");
            continue;
        }
        if !in_dependencies_table || trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        let Some((dependency_name, _)) = trimmed.split_once('=') else {
            continue;
        };
        let dependency_name = dependency_name.trim();
        for forbidden in forbidden_dependencies {
            if dependency_name.to_ascii_lowercase().contains(forbidden) {
                panic!(
                    "{} must not gain forbidden dependency `{}`",
                    manifest_path.display(),
                    dependency_name
                );
            }
        }
    }
}

fn assert_package_json_has_no_forbidden_dependencies(
    manifest_path: &std::path::Path,
    contents: &str,
    forbidden_dependencies: &[&str],
) {
    let manifest: Value = serde_json::from_str(contents)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", manifest_path.display()));
    for section in [
        "dependencies",
        "optionalDependencies",
        "peerDependencies",
        "devDependencies",
    ] {
        let Some(entries) = manifest.get(section).and_then(Value::as_object) else {
            continue;
        };
        for dependency_name in entries.keys() {
            let dependency_name = dependency_name.to_ascii_lowercase();
            for forbidden in forbidden_dependencies {
                if dependency_name.contains(forbidden) {
                    panic!(
                        "{} must not gain forbidden dependency `{}` in {}",
                        manifest_path.display(),
                        dependency_name,
                        section
                    );
                }
            }
        }
    }
}
