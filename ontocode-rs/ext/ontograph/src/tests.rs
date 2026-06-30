use std::path::Path;
use std::sync::Arc;

use ontocode_core::config::Config;
use ontocode_extension_api::ConversationHistory;
use ontocode_extension_api::ExtensionData;
use ontocode_extension_api::ExtensionRegistryBuilder;
use ontocode_extension_api::NoopTurnItemEmitter;
use ontocode_extension_api::ToolCall;
use ontocode_extension_api::ToolName;
use ontocode_extension_api::ToolPayload;
use ontocode_extension_api::TurnInputContext;
use ontocode_extension_api::TurnInputEnvironment;
use ontocode_utils_output_truncation::TruncationPolicy;
use pretty_assertions::assert_eq;
use serde_json::json;
use tempfile::tempdir;

use crate::indexer::RustIndexRequest;
use crate::indexer::index_rust_workspace;
use crate::install;
use crate::tool::DISCOVER_TOOL_NAME;
use crate::tool::DiscoverResult;
use crate::tool::DiscoveredRepoSummary;
use crate::tool::EXPLAIN_MODULE_TOOL_NAME;
use crate::tool::ExplainModuleResult;
use crate::tool::IMPACT_TOOL_NAME;
use crate::tool::INSPECT_TOOL_NAME;
use crate::tool::ImpactSymbolResult;
use crate::tool::ImpactSymbolSummary;
use crate::tool::ImplementedToolSummary;
use crate::tool::InspectContextResult;
use crate::tool::InspectSymbolSummary;
use crate::tool::MAX_FILE_BYTES;
use crate::tool::ONTOGRAPH_NAMESPACE;
use crate::tool::SEARCH_TOOL_NAME;
use crate::tool::SearchAction;
use crate::tool::SearchRepomapFileSummary;
use crate::tool::SearchResult;
use crate::tool::SearchSymbolSummary;

#[test]
fn install_registers_five_ontograph_tools() {
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
        vec![
            ToolName::namespaced(ONTOGRAPH_NAMESPACE, DISCOVER_TOOL_NAME),
            ToolName::namespaced(ONTOGRAPH_NAMESPACE, EXPLAIN_MODULE_TOOL_NAME),
            ToolName::namespaced(ONTOGRAPH_NAMESPACE, IMPACT_TOOL_NAME),
            ToolName::namespaced(ONTOGRAPH_NAMESPACE, INSPECT_TOOL_NAME),
            ToolName::namespaced(ONTOGRAPH_NAMESPACE, SEARCH_TOOL_NAME),
        ]
    );
}

#[test]
fn repo_does_not_restore_removed_ontoindex_plugin_path() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root");
    let forbidden_paths = [
        "plugins/ontocode-ontoindex",
        "ontocode-rs/ontoindex-mcp",
        "ontocode-rs/ontoindex-backend",
        "ontocode-rs/ontoindex-query",
    ];

    for path in forbidden_paths {
        assert!(!repo_root.join(path).exists(), "{path} must stay absent");
    }
}

#[tokio::test]
async fn explain_module_tool_reads_relative_rust_file_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("src")).expect("create workspace");
    std::fs::write(
        workspace.join("src/sample.rs"),
        "pub struct Demo;\nfn helper() {}\nimpl Demo {\n    fn nested() {}\n}\npub(crate) enum Mode { A }\n",
    )
    .expect("write source");

    let tool = install_and_find_tool(&workspace).await;
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "./src/sample.rs",
            "max_symbols": 8
        })
        .to_string(),
    };

    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, EXPLAIN_MODULE_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should explain module");

    let result = serde_json::from_value::<ExplainModuleResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize result");

    assert_eq!(
        result,
        ExplainModuleResult {
            path: "src/sample.rs".to_string(),
            language: Some("rust".to_string()),
            line_count: 6,
            top_level_symbols: vec![
                "Demo".to_string(),
                "helper".to_string(),
                "Mode".to_string()
            ],
            limits: vec![
                format!("file reads are capped at {MAX_FILE_BYTES} bytes"),
                "top_level_symbols are collected only from top-level Rust lines using a simple line-based scan".to_string()
            ],
        }
    );
}

#[tokio::test]
async fn discover_tools_reports_only_live_native_ontograph_tools() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");

    let payload = ToolPayload::Function {
        arguments: json!({ "action": "tools" }).to_string(),
    };
    let output = install_and_find_named_tool(&workspace, DISCOVER_TOOL_NAME)
        .await
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, DISCOVER_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("discover should report live tools");

    let result = serde_json::from_value::<DiscoverResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize result");

    assert_eq!(
        result,
        DiscoverResult {
            namespace: "ontograph".to_string(),
            tools: vec![
                ImplementedToolSummary {
                    full_name: "ontograph.discover".to_string(),
                    description:
                        "Report the live native Ontograph tool surface and checked-in parity metadata."
                            .to_string(),
                },
                ImplementedToolSummary {
                    full_name: "ontograph.explain_module".to_string(),
                    description:
                        "Inspect one local UTF-8 file and return bounded module facts without building an index."
                            .to_string(),
                },
                ImplementedToolSummary {
                    full_name: "ontograph.impact".to_string(),
                    description:
                        "Read bounded symbol impact from the active native Ontograph source index."
                            .to_string(),
                },
                ImplementedToolSummary {
                    full_name: "ontograph.inspect".to_string(),
                    description:
                        "Read bounded context from the active native Ontograph source index."
                            .to_string(),
                },
                ImplementedToolSummary {
                    full_name: "ontograph.search".to_string(),
                    description:
                        "Read bounded symbol matches from the active native Ontograph source index."
                            .to_string(),
                },
            ],
            repos: Vec::new(),
            parity_matrix_path: ".memory-bank/knowledge-hub/ONTOGRAPH_DONOR_PARITY_MATRIX.md"
                .to_string(),
            donor_actions_total: 62,
            donor_actions_implemented: 6,
            donor_actions_intentional_divergence: 1,
            donor_actions_missing: 55,
            notes: vec![
                "Only live native Ontograph tools are listed.".to_string(),
                "Donor facade placeholders are intentionally excluded.".to_string(),
            ],
        }
    );
}

#[tokio::test]
async fn discover_repos_reports_active_manifest_metadata() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(
        workspace.join("lib.rs"),
        "fn caller() {\n    callee();\n}\nfn callee() {}\n",
    )
    .expect("write source");
    index_rust_workspace(RustIndexRequest {
        repo_root: workspace.clone(),
        generation_id: "gen-1".to_string(),
        target_head: "abc123".to_string(),
        implementation_version: "0.0.0-test".to_string(),
        timestamp_unix_seconds: 1_725_000_000,
    })
    .expect("index workspace");

    let payload = ToolPayload::Function {
        arguments: json!({ "action": "repos" }).to_string(),
    };
    let output = install_and_find_named_tool(&workspace, DISCOVER_TOOL_NAME)
        .await
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, DISCOVER_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("discover repos should read active manifest");

    let result = serde_json::from_value::<DiscoverResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize result");

    assert_eq!(
        result,
        DiscoverResult {
            namespace: "ontograph".to_string(),
            tools: Vec::new(),
            repos: vec![DiscoveredRepoSummary {
                root: workspace.display().to_string(),
                vcs: "git".to_string(),
                remote_url: None,
                active_generation_id: "gen-1".to_string(),
                target_head: "abc123".to_string(),
                files: 1,
                symbols: 2,
                relationships: 1,
            }],
            parity_matrix_path: ".memory-bank/knowledge-hub/ONTOGRAPH_DONOR_PARITY_MATRIX.md"
                .to_string(),
            donor_actions_total: 62,
            donor_actions_implemented: 6,
            donor_actions_intentional_divergence: 1,
            donor_actions_missing: 55,
            notes: vec![
                "Only the current workspace active native Ontograph artifact is listed."
                    .to_string(),
                "No donor repo registry, daemon, sync, or group model is used.".to_string(),
            ],
        }
    );
}

#[tokio::test]
async fn impact_symbol_tool_reads_active_source_index_relationships() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(
        workspace.join("lib.rs"),
        "fn caller() {\n    callee();\n}\nfn callee() {}\n",
    )
    .expect("write source");
    index_rust_workspace(RustIndexRequest {
        repo_root: workspace.clone(),
        generation_id: "gen-1".to_string(),
        target_head: "abc123".to_string(),
        implementation_version: "0.0.0-test".to_string(),
        timestamp_unix_seconds: 1_725_000_000,
    })
    .expect("index workspace");

    let payload = ToolPayload::Function {
        arguments: json!({
            "action": "symbol",
            "symbol": "callee",
            "max_items": 1
        })
        .to_string(),
    };
    let output = install_and_find_named_tool(&workspace, IMPACT_TOOL_NAME)
        .await
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, IMPACT_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("impact should read active source index");

    let result = serde_json::from_value::<ImpactSymbolResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize result");

    assert_eq!(
        result,
        ImpactSymbolResult {
            symbol: "callee".to_string(),
            upstream: vec![ImpactSymbolSummary {
                symbol: "caller".to_string(),
                path: "lib.rs".to_string(),
                relationship: "CALLS".to_string(),
            }],
            downstream: Vec::new(),
            upstream_truncated: false,
            downstream_truncated: false,
            limits: vec![
                "impact items are capped at 200 per direction".to_string(),
                "impact.symbol reads only the active native source-index artifact".to_string(),
            ],
        }
    );
}

#[tokio::test]
async fn search_semantic_tool_reads_active_source_index_symbols() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(
        workspace.join("lib.rs"),
        "fn call_alpha() {}\nfn call_beta() {}\nfn other() {}\n",
    )
    .expect("write source");
    index_rust_workspace(RustIndexRequest {
        repo_root: workspace.clone(),
        generation_id: "gen-1".to_string(),
        target_head: "abc123".to_string(),
        implementation_version: "0.0.0-test".to_string(),
        timestamp_unix_seconds: 1_725_000_000,
    })
    .expect("index workspace");

    let payload = ToolPayload::Function {
        arguments: json!({
            "action": "semantic",
            "query": "call",
            "max_items": 1
        })
        .to_string(),
    };
    let output = install_and_find_named_tool(&workspace, SEARCH_TOOL_NAME)
        .await
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, SEARCH_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("search should read active source index");

    let result = serde_json::from_value::<SearchResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize result");

    assert_eq!(
        result,
        SearchResult {
            action: SearchAction::Semantic,
            query: "call".to_string(),
            matches: vec![SearchSymbolSummary {
                symbol: "call_alpha".to_string(),
                kind: "fn".to_string(),
                path: "lib.rs".to_string(),
            }],
            files: Vec::new(),
            truncated: true,
            limits: vec![
                "search matches are capped at 200 items".to_string(),
                "search.semantic currently uses native source-index symbol matching only"
                    .to_string(),
            ],
        }
    );
}

#[tokio::test]
async fn search_repomap_tool_reads_active_source_index_files() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("src")).expect("create workspace");
    std::fs::write(
        workspace.join("src/alpha.rs"),
        "fn alpha_entry() {}\nfn alpha_helper() {}\n",
    )
    .expect("write alpha source");
    std::fs::write(workspace.join("src/beta.rs"), "fn beta_entry() {}\n")
        .expect("write beta source");
    index_rust_workspace(RustIndexRequest {
        repo_root: workspace.clone(),
        generation_id: "gen-1".to_string(),
        target_head: "abc123".to_string(),
        implementation_version: "0.0.0-test".to_string(),
        timestamp_unix_seconds: 1_725_000_000,
    })
    .expect("index workspace");

    let payload = ToolPayload::Function {
        arguments: json!({
            "action": "repomap",
            "query": "alpha",
            "max_items": 1
        })
        .to_string(),
    };
    let output = install_and_find_named_tool(&workspace, SEARCH_TOOL_NAME)
        .await
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, SEARCH_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("repomap should read active source index");

    let result = serde_json::from_value::<SearchResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize result");

    assert_eq!(
        result,
        SearchResult {
            action: SearchAction::Repomap,
            query: "alpha".to_string(),
            matches: Vec::new(),
            files: vec![SearchRepomapFileSummary {
                path: "src/alpha.rs".to_string(),
                language: "rust".to_string(),
                symbols: vec![SearchSymbolSummary {
                    symbol: "alpha_entry".to_string(),
                    kind: "fn".to_string(),
                    path: "src/alpha.rs".to_string(),
                }],
            }],
            truncated: false,
            limits: vec![
                "repomap files are capped at 200 items".to_string(),
                "repomap symbols per file are capped at 200 items".to_string(),
                "search.repomap currently uses native source-index file and symbol facts only"
                    .to_string(),
            ],
        }
    );
}

#[tokio::test]
async fn inspect_context_tool_reads_active_source_index() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(
        workspace.join("lib.rs"),
        "fn caller() {\n    callee();\n}\nfn callee() {}\n",
    )
    .expect("write source");
    index_rust_workspace(RustIndexRequest {
        repo_root: workspace.clone(),
        generation_id: "gen-1".to_string(),
        target_head: "abc123".to_string(),
        implementation_version: "0.0.0-test".to_string(),
        timestamp_unix_seconds: 1_725_000_000,
    })
    .expect("index workspace");

    let payload = ToolPayload::Function {
        arguments: json!({
            "action": "context",
            "path": "lib.rs",
            "max_items": 1
        })
        .to_string(),
    };
    let output = install_and_find_named_tool(&workspace, INSPECT_TOOL_NAME)
        .await
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, INSPECT_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("inspect should read active source index");

    let result = serde_json::from_value::<InspectContextResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize result");

    assert_eq!(
        result,
        InspectContextResult {
            path: "lib.rs".to_string(),
            language: Some("rust".to_string()),
            symbols: vec![InspectSymbolSummary {
                symbol: "caller".to_string(),
                kind: "fn".to_string(),
                path: "lib.rs".to_string(),
            }],
            truncated: true,
            limits: vec![
                "context symbols are capped at 200 items".to_string(),
                "inspect.context reads only the active native source-index artifact".to_string(),
            ],
        }
    );
}

#[tokio::test]
async fn explain_module_tool_reports_non_rust_utf8_file_without_symbols() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("docs")).expect("create workspace");
    std::fs::write(workspace.join("docs/notes.md"), "# Demo\n\ntext\n").expect("write notes");

    let output = call_explain_module(&workspace, json!({ "path": "docs/notes.md" }))
        .await
        .expect("tool should explain markdown file");
    let result = serde_json::from_value::<ExplainModuleResult>(
        output
            .post_tool_use_response(
                "call-1",
                &ToolPayload::Function {
                    arguments: json!({ "path": "docs/notes.md" }).to_string(),
                },
            )
            .expect("json response"),
    )
    .expect("deserialize result");

    assert_eq!(
        result,
        ExplainModuleResult {
            path: "docs/notes.md".to_string(),
            language: Some("markdown".to_string()),
            line_count: 3,
            top_level_symbols: Vec::new(),
            limits: vec![
                format!("file reads are capped at {MAX_FILE_BYTES} bytes"),
                "top_level_symbols are collected only from top-level Rust lines using a simple line-based scan".to_string()
            ],
        }
    );
}

#[tokio::test]
async fn explain_module_tool_clamps_symbol_count() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("src")).expect("create workspace");
    let mut content = String::new();
    for index in 0..40 {
        content.push_str(&format!("pub struct S{index};\n"));
    }
    std::fs::write(workspace.join("src/many.rs"), content).expect("write source");

    let tool = install_and_find_tool(&workspace).await;
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "src/many.rs",
            "max_symbols": 100
        })
        .to_string(),
    };

    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, EXPLAIN_MODULE_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should explain module");

    let result = serde_json::from_value::<ExplainModuleResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize result");
    assert_eq!(result.top_level_symbols.len(), 32);
    assert_eq!(result.top_level_symbols.first(), Some(&"S0".to_string()));
    assert_eq!(result.top_level_symbols.last(), Some(&"S31".to_string()));
}

#[tokio::test]
async fn explain_module_tool_rejects_parent_paths() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");

    let tool = install_and_find_tool(&workspace).await;
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "../Cargo.toml"
        })
        .to_string(),
    };

    let result = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, EXPLAIN_MODULE_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload,
        })
        .await;
    let err = match result {
        Ok(_) => panic!("parent path should be rejected"),
        Err(err) => err,
    };

    assert!(
        err.to_string()
            .contains("relative and stay within the current working directory")
    );
}

#[tokio::test]
async fn explain_module_tool_rejects_missing_paths() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");

    let err = expect_tool_error(
        call_explain_module(&workspace, json!({ "path": "missing.rs" })).await,
        "missing path should be rejected",
    );

    assert!(
        err.to_string()
            .contains("ontograph.explain_module path does not exist")
    );
}

#[tokio::test]
async fn explain_module_tool_rejects_directories() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("src")).expect("create workspace");

    let err = expect_tool_error(
        call_explain_module(&workspace, json!({ "path": "src" })).await,
        "directory path should be rejected",
    );

    assert!(
        err.to_string()
            .contains("ontograph.explain_module path must point to a file")
    );
}

#[cfg(unix)]
#[tokio::test]
async fn explain_module_tool_rejects_symlink_traversal() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(dir.path().join("outside.rs"), "pub struct Outside;\n").expect("write source");
    std::os::unix::fs::symlink(dir.path().join("outside.rs"), workspace.join("linked.rs"))
        .expect("create symlink");

    let err = expect_tool_error(
        call_explain_module(&workspace, json!({ "path": "linked.rs" })).await,
        "symlink path should be rejected",
    );

    assert!(
        err.to_string()
            .contains("ontograph.explain_module path must not traverse symlinks")
    );
}

#[tokio::test]
async fn explain_module_tool_rejects_oversized_files() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(workspace.join("large.rs"), vec![b'a'; MAX_FILE_BYTES + 1])
        .expect("write source");

    let err = expect_tool_error(
        call_explain_module(&workspace, json!({ "path": "large.rs" })).await,
        "oversized file should be rejected",
    );

    assert!(
        err.to_string()
            .contains("ontograph.explain_module path is too large")
    );
}

#[tokio::test]
async fn explain_module_tool_clears_workspace_context_when_turn_has_no_environment() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    std::fs::write(workspace.join("sample.rs"), "pub struct Demo;\n").expect("write source");

    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");
    let turn_input_contributor = &registry.turn_input_contributors()[0];
    turn_input_contributor
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
    turn_input_contributor
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
        .find(|tool| {
            tool.tool_name() == ToolName::namespaced(ONTOGRAPH_NAMESPACE, EXPLAIN_MODULE_TOOL_NAME)
        })
        .expect("ontograph explain module tool");
    let err = expect_tool_error(
        tool.handle(ToolCall {
            turn_id: "turn-2".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, EXPLAIN_MODULE_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: ToolPayload::Function {
                arguments: json!({ "path": "sample.rs" }).to_string(),
            },
        })
        .await,
        "tool should reject stale workspace context",
    );

    assert!(
        err.to_string()
            .contains("workspace context is unavailable for this turn")
    );
}

async fn call_explain_module(
    cwd: &Path,
    arguments: serde_json::Value,
) -> Result<Box<dyn ontocode_extension_api::ToolOutput>, ontocode_extension_api::FunctionCallError>
{
    let tool = install_and_find_tool(cwd).await;
    tool.handle(ToolCall {
        turn_id: "turn-1".to_string(),
        call_id: "call-1".to_string(),
        tool_name: ToolName::namespaced(ONTOGRAPH_NAMESPACE, EXPLAIN_MODULE_TOOL_NAME),
        model: "gpt-test".to_string(),
        truncation_policy: TruncationPolicy::Bytes(1024),
        conversation_history: ConversationHistory::default(),
        turn_item_emitter: Arc::new(NoopTurnItemEmitter),
        payload: ToolPayload::Function {
            arguments: arguments.to_string(),
        },
    })
    .await
}

fn expect_tool_error(
    result: Result<
        Box<dyn ontocode_extension_api::ToolOutput>,
        ontocode_extension_api::FunctionCallError,
    >,
    message: &str,
) -> ontocode_extension_api::FunctionCallError {
    match result {
        Ok(_) => panic!("{message}"),
        Err(err) => err,
    }
}

async fn install_and_find_tool(
    cwd: &Path,
) -> Arc<dyn ontocode_extension_api::ToolExecutor<ontocode_extension_api::ToolCall>> {
    install_and_find_named_tool(cwd, EXPLAIN_MODULE_TOOL_NAME).await
}

async fn install_and_find_named_tool(
    cwd: &Path,
    tool_name: &str,
) -> Arc<dyn ontocode_extension_api::ToolExecutor<ontocode_extension_api::ToolCall>> {
    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");
    registry.turn_input_contributors()[0]
        .contribute(
            TurnInputContext {
                turn_id: "turn-1".to_string(),
                user_input: Vec::new(),
                environments: vec![TurnInputEnvironment {
                    environment_id: "local".to_string(),
                    cwd: cwd.to_path_buf(),
                    is_primary: true,
                }],
            },
            &session_store,
            &thread_store,
            &ExtensionData::new("turn"),
        )
        .await;

    registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .find(|tool| tool.tool_name() == ToolName::namespaced(ONTOGRAPH_NAMESPACE, tool_name))
        .expect("ontograph tool")
}
