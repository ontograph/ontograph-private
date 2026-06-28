use super::*;
use chrono::Utc;
use csv::ReaderBuilder;
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
fn render_job_csv_preserves_agentgym_style_json_fields() {
    let now = Utc::now();
    let first_conversations = serde_json::to_string(&json!([
        {
            "from": "human",
            "loss": serde_json::Value::Null,
            "value": "You are web shopping.\nThought:\nplan carefully\n\nAction:\nsearch[gym shorts]",
        },
        {
            "from": "gpt",
            "loss": true,
            "value": "Thought:\nI should search for the requested shorts.\n\nAction:\nsearch[men's shorts elastic waist navy x-large]",
        }
    ]))
    .expect("serialize first AgentGym-style conversations");
    let second_conversations = serde_json::to_string(&json!([
        {
            "from": "human",
            "loss": serde_json::Value::Null,
            "value": "Observation: product page with Buy Now",
        },
        {
            "from": "gpt",
            "loss": true,
            "value": "Thought:\nI found the right item.\n\nAction:\nclick[Buy Now]",
        }
    ]))
    .expect("serialize second AgentGym-style conversations");
    let items = vec![
        AgentJobItem {
            job_id: "job-agentgym".to_string(),
            item_id: "webshop_6".to_string(),
            row_index: 0,
            source_id: Some("webshop_6".to_string()),
            row_json: json!({
                "item_id": "webshop_6",
                "conversations": first_conversations,
            }),
            status: AgentJobItemStatus::Completed,
            assigned_thread_id: Some("thread-1".to_string()),
            attempt_count: 1,
            result_json: Some(json!({ "item_id": "webshop_6" })),
            last_error: None,
            created_at: now,
            updated_at: now,
            completed_at: Some(now),
            reported_at: Some(now),
        },
        AgentJobItem {
            job_id: "job-agentgym".to_string(),
            item_id: "webshop_7".to_string(),
            row_index: 1,
            source_id: Some("webshop_7".to_string()),
            row_json: json!({
                "item_id": "webshop_7",
                "conversations": second_conversations,
            }),
            status: AgentJobItemStatus::Completed,
            assigned_thread_id: Some("thread-2".to_string()),
            attempt_count: 1,
            result_json: Some(json!({ "item_id": "webshop_7" })),
            last_error: None,
            created_at: now,
            updated_at: now,
            completed_at: Some(now),
            reported_at: Some(now),
        },
    ];

    let csv = render_job_csv(
        &["item_id".to_string(), "conversations".to_string()],
        &items,
    )
    .expect("csv renders");

    let mut reader = ReaderBuilder::new().from_reader(csv.as_bytes());
    let headers = reader.headers().expect("csv headers").clone();
    let item_id_index = headers
        .iter()
        .position(|header| header == "item_id")
        .expect("item_id column");
    let conversations_index = headers
        .iter()
        .position(|header| header == "conversations")
        .expect("conversations column");
    let source_id_index = headers
        .iter()
        .position(|header| header == "source_id")
        .expect("source_id column");
    let row_index_index = headers
        .iter()
        .position(|header| header == "row_index")
        .expect("row_index column");
    let result_json_index = headers
        .iter()
        .position(|header| header == "result_json")
        .expect("result_json column");

    let records = reader
        .records()
        .collect::<Result<Vec<_>, _>>()
        .expect("csv records");
    assert_eq!(records.len(), 2);
    assert_eq!(&records[0][item_id_index], "webshop_6");
    assert_eq!(&records[0][source_id_index], "webshop_6");
    assert_eq!(&records[0][row_index_index], "0");
    assert_eq!(
        &records[0][result_json_index],
        "{\"item_id\":\"webshop_6\"}"
    );
    assert_eq!(&records[0][conversations_index], first_conversations);

    assert_eq!(&records[1][item_id_index], "webshop_7");
    assert_eq!(&records[1][source_id_index], "webshop_7");
    assert_eq!(&records[1][row_index_index], "1");
    assert_eq!(
        &records[1][result_json_index],
        "{\"item_id\":\"webshop_7\"}"
    );
    assert_eq!(&records[1][conversations_index], second_conversations);
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
