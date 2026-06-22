use super::*;
use crate::session::tests::make_session_and_context;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn validates_final_output_json_schema_shape() {
    assert_eq!(
        validate_final_output_json_schema_update(Some(Some(json!({
            "type": "object",
            "properties": {
                "answer": { "type": "string" }
            }
        })))),
        Ok(Some(Some(json!({
            "type": "object",
            "properties": {
                "answer": { "type": "string" }
            }
        }))))
    );
    assert!(validate_final_output_json_schema_update(Some(Some(json!({})))).is_err());
    assert!(validate_final_output_json_schema_update(Some(Some(json!("object")))).is_err());
    assert!(validate_final_output_json_schema_update(Some(None)).is_ok());
    assert!(validate_final_output_json_schema_update(None).is_ok());
}

#[tokio::test]
async fn turn_context_records_bounded_tool_result_evidence() {
    let (_session, turn) = make_session_and_context().await;

    turn.record_tool_result_evidence(
        "bash",
        Some(&json!({
            "command": "CARGO_BUILD_JOBS=8 just test -p ontocode-core",
        })),
        Some(&json!("ok")),
    );
    turn.record_tool_result_evidence(
        "bash",
        Some(&json!({
            "command": "CARGO_BUILD_JOBS=8 just fmt",
        })),
        Some(&json!("ok")),
    );
    turn.record_tool_result_evidence(
        "apply_patch",
        Some(&json!({
            "command": "*** Begin Patch\n*** Update File: src/input.txt\n*** Move to: dist/output.txt\n*** End Patch",
        })),
        Some(&json!("patched")),
    );

    let evidence = turn
        .file_read_evidence
        .lock()
        .expect("evidence lock")
        .clone();

    assert_eq!(evidence.symbol_touches.get("bash"), Some(&2));
    assert_eq!(
        evidence
            .tests_run
            .get("CARGO_BUILD_JOBS=8 just test -p ontocode-core"),
        Some(&1)
    );
    assert_eq!(
        evidence.policy_checks.get("CARGO_BUILD_JOBS=8 just fmt"),
        Some(&1)
    );
    assert_eq!(
        evidence
            .source_references
            .get("CARGO_BUILD_JOBS=8 just test -p ontocode-core"),
        Some(&1)
    );
    assert_eq!(
        evidence
            .source_references
            .get("CARGO_BUILD_JOBS=8 just fmt"),
        Some(&1)
    );
    assert_eq!(evidence.source_references.get("src/input.txt"), Some(&1));
    assert_eq!(evidence.source_references.get("dist/output.txt"), Some(&1));
}
