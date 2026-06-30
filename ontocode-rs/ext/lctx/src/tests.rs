use std::path::Path;
use std::sync::Arc;

use ontocode_core::config::Config;
use ontocode_extension_api::ConversationHistory;
use ontocode_extension_api::ExtensionData;
use ontocode_extension_api::ExtensionRegistryBuilder;
use ontocode_extension_api::FunctionCallError;
use ontocode_extension_api::NoopTurnItemEmitter;
use ontocode_extension_api::ToolCall;
use ontocode_extension_api::ToolExecutor;
use ontocode_extension_api::ToolName;
use ontocode_extension_api::ToolPayload;
use ontocode_extension_api::TurnInputContext;
use ontocode_extension_api::TurnInputEnvironment;
use ontocode_utils_output_truncation::TruncationPolicy;
use pretty_assertions::assert_eq;
use serde_json::json;
use tempfile::tempdir;

use crate::install;
use crate::tool::LCTX_NAMESPACE;
use crate::tool::LctxReadResult;
use crate::tool::MAX_OUTPUT_BYTES;
use crate::tool::RAW_ONLY_LIMIT_NOTE;
use crate::tool::READ_TOOL_NAME;

#[test]
fn install_registers_one_lctx_tool() {
    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();

    let tool_names = registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| {
            contributor.tools(
                &ExtensionData::new("session"),
                &ExtensionData::new("thread"),
            )
        })
        .map(|tool| tool.tool_name())
        .collect::<Vec<_>>();

    assert_eq!(
        tool_names,
        vec![ToolName::namespaced(LCTX_NAMESPACE, READ_TOOL_NAME)]
    );
}

#[test]
fn repo_does_not_restore_plugin_forwarding_paths() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root");
    let forbidden_paths = [
        "plugins/ontocode-lean-ctx/.codex-plugin/plugin.json",
        "plugins/ontocode-lean-ctx/.mcp.json",
        "plugins/ontocode-lean-ctx/README.md",
        "ontocode-rs/lean-ctx-mcp",
        "scripts/run_lean_ctx_plugin_backend.sh",
        "scripts/smoke_lean_ctx_plugin_backend.sh",
    ];

    for path in forbidden_paths {
        assert!(!repo_root.join(path).exists(), "{path} must stay absent");
    }
}

#[tokio::test]
async fn lctx_read_reads_relative_file_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("src")).expect("create workspace");
    std::fs::write(workspace.join("src/sample.txt"), "alpha\nbeta\n").expect("write source");

    let output = call_lctx_read(
        &workspace,
        json!({
            "path": "./src/sample.txt"
        }),
    )
    .await
    .expect("lctx.read should succeed");

    assert_eq!(
        output,
        LctxReadResult {
            path: "src/sample.txt".to_string(),
            content: "alpha\nbeta\n".to_string(),
            resolved_mode: "raw".to_string(),
            limits: vec![
                format!("content output is capped at {MAX_OUTPUT_BYTES} bytes"),
                RAW_ONLY_LIMIT_NOTE.to_string(),
            ],
        }
    );
}

#[tokio::test]
async fn lctx_read_rejects_parent_paths() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");

    let err = call_lctx_read(&workspace, json!({ "path": "../Cargo.toml" }))
        .await
        .expect_err("parent path should be rejected");

    assert!(
        err.to_string()
            .contains("relative and stay within the current working directory")
    );
}

#[tokio::test]
async fn lctx_read_rejects_unsupported_modes() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(workspace.join("sample.txt"), "alpha\n").expect("write source");

    let err = call_lctx_read(
        &workspace,
        json!({
            "path": "sample.txt",
            "mode": "full"
        }),
    )
    .await
    .expect_err("unsupported mode should be rejected");

    assert!(err.to_string().contains("supports only mode=\"raw\""));
}

#[tokio::test]
async fn lctx_read_rejects_content_over_max_bytes() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(workspace.join("sample.txt"), "abcdef").expect("write source");

    let err = call_lctx_read(
        &workspace,
        json!({
            "path": "sample.txt",
            "max_bytes": 4
        }),
    )
    .await
    .expect_err("oversized content should be rejected");

    assert!(err.to_string().contains("exceeds requested max_bytes"));
}

#[cfg(unix)]
#[tokio::test]
async fn lctx_read_rejects_symlink_traversal() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(dir.path().join("outside.txt"), "outside\n").expect("write source");
    std::os::unix::fs::symlink(dir.path().join("outside.txt"), workspace.join("linked.txt"))
        .expect("create symlink");

    let err = call_lctx_read(&workspace, json!({ "path": "linked.txt" }))
        .await
        .expect_err("symlink path should be rejected");

    assert!(err.to_string().contains("must not traverse symlinks"));
}

#[tokio::test]
async fn lctx_read_clears_cwd_when_turn_has_no_environment() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(workspace.join("sample.txt"), "alpha\n").expect("write source");

    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");
    let contributor = &registry.turn_input_contributors()[0];

    contributor
        .contribute(
            TurnInputContext {
                turn_id: "turn-1".to_string(),
                user_input: Vec::new(),
                environments: vec![TurnInputEnvironment {
                    environment_id: "local".to_string(),
                    cwd: workspace,
                    is_primary: true,
                }],
            },
            &session_store,
            &thread_store,
            &ExtensionData::new("turn-1"),
        )
        .await;
    contributor
        .contribute(
            TurnInputContext {
                turn_id: "turn-2".to_string(),
                user_input: Vec::new(),
                environments: Vec::new(),
            },
            &session_store,
            &thread_store,
            &ExtensionData::new("turn-2"),
        )
        .await;

    let tool = registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .find(|tool| tool.tool_name() == ToolName::namespaced(LCTX_NAMESPACE, READ_TOOL_NAME))
        .expect("lctx.read tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "sample.txt" }).to_string(),
    };
    let result = tool
        .handle(ToolCall {
            turn_id: "turn-2".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(LCTX_NAMESPACE, READ_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload,
        })
        .await;

    let err = match result {
        Ok(_) => panic!("tool should reject calls without turn workspace context"),
        Err(err) => err,
    };
    assert!(
        err.to_string()
            .contains("workspace context is unavailable for this turn")
    );
}

async fn install_and_find_tool(workspace: &Path) -> Arc<dyn ToolExecutor<ToolCall>> {
    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");

    for contributor in registry.turn_input_contributors() {
        let input = TurnInputContext {
            turn_id: "turn-1".to_string(),
            user_input: Vec::new(),
            environments: vec![TurnInputEnvironment {
                environment_id: "local".to_string(),
                cwd: workspace.to_path_buf(),
                is_primary: true,
            }],
        };
        contributor
            .contribute(
                input,
                &session_store,
                &thread_store,
                &ExtensionData::new("turn"),
            )
            .await;
    }

    registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .find(|tool| tool.tool_name() == ToolName::namespaced(LCTX_NAMESPACE, READ_TOOL_NAME))
        .expect("lctx.read tool")
}

async fn call_lctx_read(
    workspace: &Path,
    arguments: serde_json::Value,
) -> Result<LctxReadResult, FunctionCallError> {
    let tool = install_and_find_tool(workspace).await;
    let payload = ToolPayload::Function {
        arguments: arguments.to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(LCTX_NAMESPACE, READ_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await?;
    let value = output
        .post_tool_use_response("call-1", &payload)
        .expect("json response");
    Ok(serde_json::from_value(value).expect("deserialize result"))
}
