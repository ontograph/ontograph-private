use super::MAX_OUTPUT_STRING_CHARS;
use super::MAX_REQUIRED_ROLE_NAME_CHARS;
use super::MAX_REQUIRED_ROLES;
use super::MAX_TRACKING_FILE_BYTES;
use super::ManagerLoopNextResult;
use crate::session::tests::make_session_and_context;
use crate::tools::ToolRouter;
use crate::tools::context::ToolPayload;
use crate::tools::router::ToolCall;
use crate::tools::router::ToolCallSource;
use crate::tools::router::ToolRouterParams;
use crate::turn_diff_tracker::TurnDiffTracker;
use core_test_support::PathExt;
use ontocode_protocol::models::FunctionCallOutputBody;
use ontocode_protocol::models::ResponseInputItem;
use pretty_assertions::assert_eq;
use serde_json::json;
use serial_test::serial;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

static TRACKING_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn write_tracking_file(
    turn: &crate::session::turn_context::TurnContext,
    tracking_path: &str,
    body: &str,
) {
    let path = turn
        .environments
        .primary()
        .expect("primary turn environment")
        .cwd
        .as_path()
        .join(tracking_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create tracking parent");
    }
    fs::write(path, body).expect("write tracking file");
}

fn unique_tracking_path(tracking_path: &str) -> String {
    let counter = TRACKING_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let process_id = std::process::id();
    let path = Path::new(tracking_path);
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("tracking");
    let extension = path.extension().and_then(|extension| extension.to_str());
    let file_name = match extension {
        Some(extension) => format!("{stem}-{process_id}-{counter}.{extension}"),
        None => format!("{stem}-{process_id}-{counter}"),
    };

    match path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        Some(parent) => parent.join(file_name).to_string_lossy().into_owned(),
        None => file_name,
    }
}

async fn invoke_manager_loop_next(body: &str, tracking_path: &str) -> ResponseInputItem {
    let temp_dir = TempDir::new().expect("create manager_loop_next test workspace");
    let (session, mut turn) = make_session_and_context().await;
    let cwd = temp_dir.path().abs();
    turn.environments
        .turn_environments
        .first_mut()
        .expect("primary turn environment")
        .cwd = cwd;
    let tracking_path = unique_tracking_path(tracking_path);
    write_tracking_file(&turn, &tracking_path, body);

    let router = ToolRouter::from_turn_context(
        &turn,
        ToolRouterParams {
            deferred_mcp_tools: None,
            mcp_tools: None,
            discoverable_tools: None,
            extension_tool_executors: Vec::new(),
            dynamic_tools: turn.dynamic_tools.as_slice(),
        },
    );

    let call = ToolCall {
        tool_name: ontocode_tools::ToolName::plain("manager_loop_next"),
        call_id: "call-manager-loop".to_string(),
        payload: ToolPayload::Function {
            arguments: json!({
                "tracking_path": tracking_path,
                "mode": "strict",
            })
            .to_string(),
        },
    };

    router
        .dispatch_tool_call_with_code_mode_result(
            Arc::new(session),
            Arc::new(turn),
            CancellationToken::new(),
            Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new())),
            call,
            ToolCallSource::Direct,
        )
        .await
        .expect("dispatch manager_loop_next")
        .into_response()
}

fn parse_result(response: ResponseInputItem) -> ManagerLoopNextResult {
    let ResponseInputItem::FunctionCallOutput { output, .. } = response else {
        panic!("expected function output");
    };
    let FunctionCallOutputBody::Text(text) = output.body else {
        panic!("expected text body");
    };
    serde_json::from_str(&text).expect("manager_loop_next output should be json")
}

fn strict_tracking_block(body: &str) -> String {
    format!("---\nname: test\n---\n\n# Tracking\n\n```yaml\n{body}\n```\n")
}

#[tokio::test]
#[serial]
async fn manager_loop_next_executes_active_next_task_first() {
    let response = invoke_manager_loop_next(
        &strict_tracking_block(
            r#"manager_loop:
  authority: true
  active_next_task: R3
  last_decision: dispatch
  reopen_gate: null
  required_roles:
    senior-reviewer:
      model: gemini-pro-agent
    implementation-worker:
      models:
        - gpt-5.3-codex-spark
        - gpt-5.4-mini
      reasoning_effort: high
  tasks:
    - id: R1
      status: CLOSED
      classification: implementation-ready
    - id: R3
      status: OPEN
      classification: implementation-ready
      depends_on: [R1]"#,
        ),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "execute_active_next_task",
            "task_id": "R3",
            "reason": "active_next_task is set and classified implementation-ready",
            "required_roles": [
                {
                    "role": "implementation-worker",
                    "required": true
                },
                {
                    "role": "senior-reviewer",
                    "required": true
                }
            ],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_returns_exact_no_dispatch_reopen_gate() {
    let response = invoke_manager_loop_next(
        &strict_tracking_block(
            r#"manager_loop:
  authority: true
  active_next_task: null
  last_decision: no-dispatch
  reopen_gate: reopen only after fresh failing proof
  required_roles:
    senior-reviewer:
      model: gemini-pro-agent
  tasks:
    - id: R1
      status: OPEN
      classification: implementation-ready"#,
        ),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "no_dispatch",
            "task_id": null,
            "reason": "last_decision is no-dispatch",
            "required_roles": [
                {
                    "role": "senior-reviewer",
                    "required": true
                }
            ],
            "reopen_gate": "reopen only after fresh failing proof"
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_promotes_first_dependency_ready_open_task() {
    let response = invoke_manager_loop_next(
        &strict_tracking_block(
            r#"manager_loop:
  authority: true
  active_next_task: null
  last_decision: dispatch
  reopen_gate: null
  required_roles: {}
  tasks:
    - id: D1
      status: OPEN
      classification: implementation-ready
    - id: R2
      status: OPEN
      classification: docs/design-only
    - id: R3
      status: OPEN
      classification: implementation-ready
      depends_on: [D1]
    - id: R4
      status: OPEN
      classification: implementation-ready
      depends_on: []
    - id: R5
      status: OPEN
      classification: proof-only"#,
        ),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "promote_next_open",
            "task_id": "D1",
            "reason": "first dependency-ready OPEN task with classification implementation-ready",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_skips_tasks_with_blocked_dependencies() {
    let response = invoke_manager_loop_next(
        &strict_tracking_block(
            r#"manager_loop:
  authority: true
  active_next_task: null
  last_decision: dispatch
  reopen_gate: null
  required_roles: {}
  tasks:
    - id: B1
      status: BLOCKED
      classification: implementation-ready
    - id: R2
      status: OPEN
      classification: implementation-ready
      depends_on: [B1]
    - id: R3
      status: OPEN
      classification: implementation-ready
      depends_on: []"#,
        ),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "promote_next_open",
            "task_id": "R3",
            "reason": "first dependency-ready OPEN task with classification implementation-ready",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_returns_complete_when_nothing_implementation_ready_remains() {
    let response = invoke_manager_loop_next(
        &strict_tracking_block(
            r#"manager_loop:
  authority: true
  active_next_task: null
  last_decision: dispatch
  reopen_gate: null
  required_roles: {}
  tasks:
    - id: R1
      status: CLOSED
      classification: implementation-ready
    - id: R2
      status: OPEN
      classification: proof-only
    - id: R3
      status: OPEN
      classification: docs/design-only"#,
        ),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "complete",
            "task_id": null,
            "reason": "nothing left in scope",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_allows_task_metadata_fields() {
    let response = invoke_manager_loop_next(
        &strict_tracking_block(
            r#"manager_loop:
  authority: true
  active_next_task: null
  last_decision: dispatch
  reopen_gate: null
  required_roles: {}
  tasks:
    - id: R1
      status: CLOSED
      classification: implementation-ready
      owner: existing multi-agent owners
      verification:
        - CARGO_BUILD_JOBS=8 just test -p ontocode-core manager_loop_next"#,
        ),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "complete",
            "task_id": null,
            "reason": "nothing left in scope",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_rejects_unknown_task_fields() {
    let response = invoke_manager_loop_next(
        &strict_tracking_block(
            r#"manager_loop:
  authority: true
  active_next_task: null
  last_decision: dispatch
  reopen_gate: null
  required_roles: {}
  tasks:
    - id: R1
      status: OPEN
      classification: implementation-ready
      depends-on: [R0]"#,
        ),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "invalid_tracking",
            "task_id": null,
            "reason": "unsupported task field `depends-on: [R0]`",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_returns_invalid_tracking_for_missing_strict_block() {
    let response = invoke_manager_loop_next(
        "# Tracking\n\nNo fenced yaml block here.\n",
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "invalid_tracking",
            "task_id": null,
            "reason": "missing strict fenced `manager_loop` YAML block",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_rejects_non_memory_bank_paths() {
    let response = invoke_manager_loop_next(
        &strict_tracking_block(
            r#"manager_loop:
  authority: true
  active_next_task: null
  last_decision: dispatch
  reopen_gate: null
  required_roles: {}
  tasks: []"#,
        ),
        "tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "invalid_tracking",
            "task_id": null,
            "reason": "tracking_path must stay under `.memory-bank/`",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_rejects_oversized_tracking_files() {
    let response = invoke_manager_loop_next(
        &"x".repeat(MAX_TRACKING_FILE_BYTES as usize + 1),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "invalid_tracking",
            "task_id": null,
            "reason": "tracking file exceeds maximum size",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_rejects_oversized_reopen_gate() {
    let response = invoke_manager_loop_next(
        &strict_tracking_block(&format!(
            r#"manager_loop:
  authority: true
  active_next_task: null
  last_decision: no-dispatch
  reopen_gate: {}
  required_roles: {{}}
  tasks: []"#,
            "x".repeat(MAX_OUTPUT_STRING_CHARS + 1)
        )),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "invalid_tracking",
            "task_id": null,
            "reason": "reopen_gate exceeds maximum length",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_rejects_too_many_required_roles() {
    let roles = (0..=MAX_REQUIRED_ROLES)
        .map(|index| format!("    role-{index}:\n      model: test\n"))
        .collect::<String>();
    let response = invoke_manager_loop_next(
        &strict_tracking_block(&format!(
            r#"manager_loop:
  authority: true
  active_next_task: null
  last_decision: dispatch
  reopen_gate: null
  required_roles:
{roles}  tasks: []"#
        )),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "invalid_tracking",
            "task_id": null,
            "reason": "required_roles exceeds maximum count",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_rejects_oversized_required_role_names() {
    let role_name = "x".repeat(MAX_REQUIRED_ROLE_NAME_CHARS + 1);
    let response = invoke_manager_loop_next(
        &strict_tracking_block(&format!(
            r#"manager_loop:
  authority: true
  active_next_task: null
  last_decision: dispatch
  reopen_gate: null
  required_roles:
    {role_name}:
      model: test
  tasks: []"#
        )),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "invalid_tracking",
            "task_id": null,
            "reason": "required role exceeds maximum length",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}

#[tokio::test]
#[serial]
async fn manager_loop_next_rejects_missing_authority_flag() {
    let response = invoke_manager_loop_next(
        &strict_tracking_block(
            r#"manager_loop:
  active_next_task: null
  last_decision: dispatch
  reopen_gate: null
  required_roles: {}
  tasks: []"#,
        ),
        ".memory-bank/tracking.md",
    )
    .await;

    assert_eq!(
        parse_result(response),
        serde_json::from_value(json!({
            "decision": "invalid_tracking",
            "task_id": null,
            "reason": "manager_loop block must set `authority: true`",
            "required_roles": [],
            "reopen_gate": null
        }))
        .expect("deserialize expected result")
    );
}
