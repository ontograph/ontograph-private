import re
import sys

def main():
    with open("ontocode-rs/core/tests/suite/agent_jobs.rs", "r") as f:
        content = f.read()

    # The function recover_running_items needs to be tested through the handler

    old_test = r"async fn agent_job_loop_recovers_running_items_after_restart\(\) \{[\s\S]*?^}"

    real_test = """async fn agent_job_loop_recovers_running_items_after_restart() {
    let server = start_mock_server().await;

    let responder = Arc::new(AgentJobsResponder::new(json!({
        "csv_path": "fake.csv",
        "instruction": "test",
    }).to_string()));

    Mock::given(method("POST"))
        .and(path_regex(r"^/v1/(chat/completions|messages)"))
        .respond_with(responder.clone())
        .mount(&server)
        .await;

    let mut config = core_test_support::test_codex::test_config().await.1;
    config.features.enable(Feature::Sqlite).unwrap();

    let mut harness = test_codex()
        .with_mock_server(&server)
        .with_config(config)
        .start()
        .await
        .expect("start");

    let db = harness.state_db().unwrap();
    let job_id = "job-1".to_string();
    let item_id = "item-1".to_string();

    db.create_agent_job(
        &ontocode_state::AgentJobCreateParams {
            id: job_id.clone(),
            name: "test".to_string(),
            instruction: "test".to_string(),
            auto_export: false,
            max_runtime_seconds: None,
            output_schema_json: None,
            input_headers: vec!["col1".to_string()],
            input_csv_path: "in.csv".to_string(),
            output_csv_path: "out.csv".to_string(),
        },
        &[ontocode_state::AgentJobItemCreateParams {
            item_id: item_id.clone(),
            row_index: 0,
            source_id: None,
            row_json: json!({"col1": "val1"}),
        }],
    ).await.unwrap();

    db.mark_agent_job_running(&job_id).await.unwrap();
    // This will cause a resume error because the dummy thread does not exist.
    db.mark_agent_job_item_running_with_thread(&job_id, &item_id, "dummy-thread-id").await.unwrap();

    // Resume the agent job loop by invoking the handler
    use ontocode_core::tools::handlers::agent_jobs::SpawnAgentsOnCsvHandler;
    use ontocode_core::function_tool::FunctionToolHandler;

    let invocation = core_test_support::tool_harness::invocation(
        harness.session().clone(),
        harness.turn_context(),
        "spawn_agents_on_csv",
        core_test_support::tool_harness::function_payload(json!({
            "csv_path": "fake.csv",
            "instruction": "test",
            "id_column": "job-1"
        })),
    );

    // We run it directly. It should fail to recover "dummy-thread-id" and mark the item failed.
    let _ = SpawnAgentsOnCsvHandler.handle(invocation).await;

    let item = db.get_agent_job_item(&job_id, &item_id).await.unwrap().unwrap();
    assert_eq!(item.status, ontocode_state::AgentJobItemStatus::Failed);
    assert!(item.last_error.unwrap().contains("failed to resume worker"));
}"""

    new_content = re.sub(old_test, real_test, content, flags=re.MULTILINE)
    with open("ontocode-rs/core/tests/suite/agent_jobs.rs", "w") as f:
        f.write(new_content)

if __name__ == "__main__":
    main()
