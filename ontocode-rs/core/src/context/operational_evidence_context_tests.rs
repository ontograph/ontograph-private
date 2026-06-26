use super::OperationalEvidenceContextFragment;
use crate::context::ContextualUserFragment;
use crate::context::is_contextual_user_fragment;
use chrono::DateTime;
use chrono::Utc;
use ontocode_protocol::models::ContentItem;
use ontocode_state::EvidenceDomain;
use ontocode_state::EvidenceRisk;
use ontocode_state::EvidenceStatus;
use ontocode_state::OperationalEvidenceSummary;
use ontocode_state::RedactionStatus;
use pretty_assertions::assert_eq;

fn ts(seconds: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(seconds, 0).expect("timestamp")
}

fn summary_record(summary: &str) -> OperationalEvidenceSummary {
    OperationalEvidenceSummary {
        id: "evidence-1".to_string(),
        evidence_domain: EvidenceDomain::CodeGraph,
        source_tool: "fixture-importer".to_string(),
        source_version: Some("1.0.0".to_string()),
        schema_version: 1,
        source_ref: Some("context".to_string()),
        repo: Some("/repo".to_string()),
        task_key: None,
        thread_id: Some("thread-123".to_string()),
        parent_thread_id: None,
        child_thread_id: None,
        symbol_uid: None,
        symbol_name: None,
        file_path: None,
        process_label: None,
        gate_name: Some("gate-a".to_string()),
        risk: Some(EvidenceRisk::Low),
        status: EvidenceStatus::Verified,
        summary: summary.to_string(),
        source_links: Vec::new(),
        provenance_hash: "hash-1".to_string(),
        redaction_status: RedactionStatus::Clean,
        target_head: Some("head-123".to_string()),
        graph_index_id: Some("graph-123".to_string()),
        plan_hash: None,
        tracking_hash: None,
        created_at: ts(1_700_000_000),
        expires_at: None,
    }
}

#[test]
fn renders_operational_evidence_context_fragment_as_bounded_user_context() {
    let fragment = OperationalEvidenceContextFragment {
        thread_id: "thread-123".to_string(),
        records: vec![summary_record("bounded summary")],
    };

    let rendered = fragment.render();

    assert_eq!(
        rendered,
        "<operational_evidence_context>\n  <thread_id>thread-123</thread_id>\n  <record evidence_domain=\"code_graph\" status=\"verified\" risk=\"low\" source_tool=\"fixture-importer\" source_ref=\"context\" gate_name=\"gate-a\" target_head=\"head-123\" graph_index_id=\"graph-123\" provenance_hash=\"hash-1\">\n    <summary>bounded summary</summary>\n  </record>\n</operational_evidence_context>"
    );
    assert!(is_contextual_user_fragment(&ContentItem::InputText {
        text: rendered
    }));
}

#[test]
fn truncates_operational_evidence_context_fragment_to_the_byte_cap() {
    let long_value = "x".repeat(900);
    let fragment = OperationalEvidenceContextFragment {
        thread_id: "thread-123".to_string(),
        records: vec![
            summary_record("first bounded summary"),
            OperationalEvidenceSummary {
                id: "evidence-2".to_string(),
                evidence_domain: EvidenceDomain::Workflow,
                source_tool: long_value.clone(),
                source_version: Some("1.0.0".to_string()),
                schema_version: 1,
                source_ref: Some(long_value.clone()),
                repo: Some("/repo".to_string()),
                task_key: Some(long_value.clone()),
                thread_id: Some("thread-123".to_string()),
                parent_thread_id: None,
                child_thread_id: None,
                symbol_uid: None,
                symbol_name: Some(long_value.clone()),
                file_path: Some(long_value.clone()),
                process_label: None,
                gate_name: Some(long_value.clone()),
                risk: Some(EvidenceRisk::High),
                status: EvidenceStatus::Done,
                summary: long_value.clone(),
                source_links: Vec::new(),
                provenance_hash: long_value.clone(),
                redaction_status: RedactionStatus::Clean,
                target_head: Some(long_value.clone()),
                graph_index_id: Some(long_value.clone()),
                plan_hash: None,
                tracking_hash: None,
                created_at: ts(1_700_000_001),
                expires_at: None,
            },
        ],
    };

    let rendered = fragment.render();

    assert!(rendered.contains("first bounded summary"), "{rendered}");
    assert!(
        rendered.contains("<truncated reason=\"byte_cap\" />"),
        "{rendered}"
    );
    assert!(!rendered.contains(&long_value), "{rendered}");
}
