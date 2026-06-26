use super::*;
use chrono::Utc;
use ontocode_state::AgentJobItem;
use ontocode_state::AgentJobItemStatus;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn parse_csv_supports_quotes_and_commas() {
    let input = "id,name\n1,\"alpha, beta\"\n2,gamma\n";
    let (headers, rows) = parse_csv(input).expect("csv parse");
    assert_eq!(headers, vec!["id".to_string(), "name".to_string()]);
    assert_eq!(
        rows,
        vec![
            vec!["1".to_string(), "alpha, beta".to_string()],
            vec!["2".to_string(), "gamma".to_string()]
        ]
    );
}

#[test]
fn csv_escape_quotes_when_needed() {
    assert_eq!(csv_escape("simple"), "simple");
    assert_eq!(csv_escape("a,b"), "\"a,b\"");
    assert_eq!(csv_escape("a\"b"), "\"a\"\"b\"");
}

#[test]
fn render_job_csv_includes_job_status_columns() {
    let now = Utc::now();
    let item = AgentJobItem {
        job_id: "job-1".to_string(),
        item_id: "item-1".to_string(),
        row_index: 7,
        source_id: Some("source-1".to_string()),
        row_json: json!({ "path": "src/lib.rs" }),
        status: AgentJobItemStatus::Completed,
        assigned_thread_id: Some("thread-1".to_string()),
        attempt_count: 2,
        result_json: Some(json!({ "ok": true })),
        last_error: Some("last error".to_string()),
        created_at: now,
        updated_at: now,
        completed_at: Some(now),
        reported_at: Some(now),
    };

    let csv = render_job_csv(&["path".to_string()], &[item]).expect("csv renders");

    assert!(csv.starts_with(
        "path,job_id,item_id,row_index,source_id,status,attempt_count,last_error,result_json,reported_at,completed_at\n"
    ));
    assert!(csv.contains("src/lib.rs,job-1,item-1,7,source-1,completed,2,last error"));
    assert!(csv.contains("{\"\"ok\"\":true}"));
}

#[test]
fn render_instruction_template_expands_placeholders_and_escapes_braces() {
    let row = json!({
        "path": "src/lib.rs",
        "area": "test",
        "file path": "docs/readme.md",
    });
    let rendered = render_instruction_template(
        "Review {path} in {area}. Also see {file path}. Use {{literal}}.",
        &row,
    );
    assert_eq!(
        rendered,
        "Review src/lib.rs in test. Also see docs/readme.md. Use {literal}."
    );
}

#[test]
fn render_instruction_template_leaves_unknown_placeholders() {
    let row = json!({
        "path": "src/lib.rs",
    });
    let rendered = render_instruction_template("Check {path} then {missing}", &row);
    assert_eq!(rendered, "Check src/lib.rs then {missing}");
}

#[test]
fn ensure_unique_headers_rejects_duplicates() {
    let headers = vec!["path".to_string(), "path".to_string()];
    let Err(err) = ensure_unique_headers(headers.as_slice()) else {
        panic!("expected duplicate header error");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel("csv header path is duplicated".to_string())
    );
}
