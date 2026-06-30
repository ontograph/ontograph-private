use pretty_assertions::assert_eq;

use crate::manifest::OntographArtifactRoot;
use crate::manifest::OntographManifest;
use crate::manifest::SOURCE_INDEX_FILE;
use crate::query::BoundedItems;
use crate::query::ImpactItem;
use crate::query::ModuleContextItem;
use crate::query::ModuleContextRequest;
use crate::query::ModuleContextResult;
use crate::query::OntographReadQuery;
use crate::query::QueryBudget;
use crate::query::RepoMapFileItem;
use crate::query::RepoMapRequest;
use crate::query::RepoMapResult;
use crate::query::SymbolImpactRequest;
use crate::query::SymbolImpactResult;
use crate::query::SymbolSearchItem;
use crate::query::SymbolSearchRequest;
use crate::query::SymbolSearchResult;

use super::*;

#[test]
fn indexes_rust_files_deterministically_and_ignores_private_dirs() {
    let dir = tempfile::tempdir().expect("temp dir");
    write_file(
        dir.path().join("src/z.rs"),
        "pub struct Zebra;\n    fn nested() {}\nconst LIMIT: usize = 1;\n",
    );
    write_file(
        dir.path().join("src/a.rs"),
        "pub(crate) enum Mode { A }\ntrait Local {}\nfn helper() {}\n",
    );
    write_file(dir.path().join("README.md"), "# ignored\n");
    write_file(dir.path().join(".git/ignored.rs"), "pub struct Ignored;\n");
    write_file(
        dir.path().join(".ontograph/ignored.rs"),
        "pub struct Ignored;\n",
    );
    write_file(
        dir.path().join("target/ignored.rs"),
        "pub struct Ignored;\n",
    );

    let artifact = index_rust_workspace(sample_request(dir.path())).expect("index workspace");

    assert_eq!(
        artifact,
        SourceIndexArtifact {
            schema_version: 1,
            files: vec![
                IndexedFile {
                    path: "src/a.rs".to_string(),
                    language: "rust".to_string(),
                    symbols: vec![
                        IndexedSymbol {
                            name: "Mode".to_string(),
                            kind: "enum".to_string(),
                            line: 1,
                        },
                        IndexedSymbol {
                            name: "Local".to_string(),
                            kind: "trait".to_string(),
                            line: 2,
                        },
                        IndexedSymbol {
                            name: "helper".to_string(),
                            kind: "fn".to_string(),
                            line: 3,
                        },
                    ],
                },
                IndexedFile {
                    path: "src/z.rs".to_string(),
                    language: "rust".to_string(),
                    symbols: vec![
                        IndexedSymbol {
                            name: "Zebra".to_string(),
                            kind: "struct".to_string(),
                            line: 1,
                        },
                        IndexedSymbol {
                            name: "LIMIT".to_string(),
                            kind: "const".to_string(),
                            line: 3,
                        },
                    ],
                },
            ],
            relationships: Vec::new(),
            processes: Vec::new(),
        }
    );
}

#[test]
fn writes_source_index_artifact_and_manifest_counts() {
    let dir = tempfile::tempdir().expect("temp dir");
    write_file(
        dir.path().join("lib.rs"),
        "pub mod graph;\ntype NodeId = u64;\n",
    );

    let artifact = index_rust_workspace(sample_request(dir.path())).expect("index workspace");
    let root = OntographArtifactRoot::new(dir.path());
    let source_index_path = root
        .generation_file_path("gen-1", SOURCE_INDEX_FILE)
        .expect("source index path");
    let written_artifact =
        std::fs::read_to_string(source_index_path).expect("source index artifact");
    assert_eq!(
        serde_json::from_str::<SourceIndexArtifact>(&written_artifact).expect("artifact json"),
        artifact
    );

    let manifest_path = root
        .generation_manifest_path("gen-1")
        .expect("manifest path");
    let manifest = serde_json::from_str::<OntographManifest>(
        &std::fs::read_to_string(manifest_path).expect("manifest"),
    )
    .expect("manifest json");
    assert_eq!(manifest.active_generation_id, "gen-1");
    assert_eq!(manifest.completed_generations[0].counts.files, 1);
    assert_eq!(manifest.completed_generations[0].counts.symbols, 2);
    assert_eq!(manifest.completed_generations[0].counts.relationships, 0);
}

#[test]
fn indexes_same_file_function_call_relationships() {
    let dir = tempfile::tempdir().expect("temp dir");
    write_file(
        dir.path().join("lib.rs"),
        "mod graph;\nfn caller() {\n    callee();\n}\nfn callee() {}\n",
    );

    let artifact = index_rust_workspace(sample_request(dir.path())).expect("index workspace");

    assert_eq!(
        artifact.relationships,
        vec![IndexedRelationship {
            from_symbol: "caller".to_string(),
            to_symbol: "callee".to_string(),
            kind: "CALLS".to_string(),
            path: "lib.rs".to_string(),
            line: 3,
        }]
    );

    let root = OntographArtifactRoot::new(dir.path());
    let manifest_path = root
        .generation_manifest_path("gen-1")
        .expect("manifest path");
    let manifest = serde_json::from_str::<OntographManifest>(
        &std::fs::read_to_string(manifest_path).expect("manifest"),
    )
    .expect("manifest json");
    assert_eq!(manifest.completed_generations[0].counts.relationships, 1);
    assert_eq!(manifest.completed_generations[0].counts.edges, 1);
}

#[test]
fn indexes_same_file_call_relationships_as_processes() {
    let dir = tempfile::tempdir().expect("temp dir");
    write_file(
        dir.path().join("lib.rs"),
        "fn caller() {\n    middle();\n}\nfn middle() {\n    callee();\n}\nfn callee() {}\n",
    );

    let artifact = index_rust_workspace(sample_request(dir.path())).expect("index workspace");

    assert_eq!(
        artifact.processes,
        vec![
            IndexedProcess {
                name: "caller -> middle".to_string(),
                entry_symbol: "caller".to_string(),
                path: "lib.rs".to_string(),
                steps: vec!["caller".to_string(), "middle".to_string()],
            },
            IndexedProcess {
                name: "middle -> callee".to_string(),
                entry_symbol: "middle".to_string(),
                path: "lib.rs".to_string(),
                steps: vec!["middle".to_string(), "callee".to_string()],
            },
        ]
    );
}

#[test]
fn active_source_index_store_supports_bounded_search_module_context_and_impact() {
    let dir = tempfile::tempdir().expect("temp dir");
    write_file(
        dir.path().join("lib.rs"),
        "fn caller() {\n    callee();\n}\nfn callee() {}\nfn called_other() {}\n",
    );
    index_rust_workspace(sample_request(dir.path())).expect("index workspace");

    let store = SourceIndexStore::load_active_generation(&OntographArtifactRoot::new(dir.path()))
        .expect("load active source index");
    let budget = QueryBudget::bounded(100, 1, 512);

    assert_eq!(
        store.search_symbols(SymbolSearchRequest {
            query: "call".to_string(),
            budget: budget.clone(),
        }),
        Ok(SymbolSearchResult {
            matches: BoundedItems {
                items: vec![SymbolSearchItem {
                    symbol: "caller".to_string(),
                    kind: "fn".to_string(),
                    path: "lib.rs".to_string(),
                }],
                truncated: true,
                budget: budget.clone(),
            },
        })
    );
    assert_eq!(
        store.inspect_module(ModuleContextRequest {
            path: "lib.rs".to_string(),
            budget: budget.clone(),
        }),
        Ok(ModuleContextResult {
            module: ModuleContextItem {
                path: "lib.rs".to_string(),
                language: Some("rust".to_string()),
                symbols: BoundedItems {
                    items: vec![SymbolSearchItem {
                        symbol: "caller".to_string(),
                        kind: "fn".to_string(),
                        path: "lib.rs".to_string(),
                    }],
                    truncated: true,
                    budget: budget.clone(),
                },
            },
        })
    );
    assert_eq!(
        store.search_repomap(RepoMapRequest {
            query: "lib".to_string(),
            budget: budget.clone(),
        }),
        Ok(RepoMapResult {
            files: BoundedItems {
                items: vec![RepoMapFileItem {
                    path: "lib.rs".to_string(),
                    language: "rust".to_string(),
                    symbols: vec![SymbolSearchItem {
                        symbol: "caller".to_string(),
                        kind: "fn".to_string(),
                        path: "lib.rs".to_string(),
                    }],
                }],
                truncated: false,
                budget: budget.clone(),
            },
        })
    );
    assert_eq!(
        store.impact_symbol(SymbolImpactRequest {
            symbol: "callee".to_string(),
            budget: budget.clone(),
        }),
        Ok(SymbolImpactResult {
            upstream: BoundedItems {
                items: vec![ImpactItem {
                    symbol: "caller".to_string(),
                    path: "lib.rs".to_string(),
                    relationship: "CALLS".to_string(),
                }],
                truncated: false,
                budget: budget.clone(),
            },
            downstream: BoundedItems {
                items: Vec::new(),
                truncated: false,
                budget,
            },
        })
    );
}

#[test]
fn active_source_index_store_fails_closed_without_current_pointer() {
    let dir = tempfile::tempdir().expect("temp dir");

    assert_eq!(
        SourceIndexStore::load_active_generation(&OntographArtifactRoot::new(dir.path()))
            .unwrap_err(),
        SourceIndexLoadError::MissingCurrentPointer
    );
}

fn sample_request(repo_root: &std::path::Path) -> RustIndexRequest {
    RustIndexRequest {
        repo_root: repo_root.to_path_buf(),
        generation_id: "gen-1".to_string(),
        target_head: "abc123".to_string(),
        implementation_version: "0.0.0-test".to_string(),
        timestamp_unix_seconds: 1_725_000_000,
    }
}

fn write_file(path: std::path::PathBuf, contents: &str) {
    std::fs::create_dir_all(path.parent().expect("parent")).expect("create parent");
    std::fs::write(path, contents).expect("write file");
}
