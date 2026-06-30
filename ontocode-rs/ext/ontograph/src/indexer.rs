use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use ontocode_utils_path::write_atomically;
use serde::Deserialize;
use serde::Serialize;

use crate::manifest::ArtifactGeneration;
use crate::manifest::ArtifactPublishError;
use crate::manifest::ArtifactVersion;
use crate::manifest::GenerationMetadata;
use crate::manifest::GraphCountSummary;
use crate::manifest::OntographArtifactRoot;
use crate::manifest::OntographManifest;
use crate::manifest::RepositoryIdentity;
use crate::manifest::SOURCE_INDEX_FILE;
use crate::query::BoundedItems;
use crate::query::ImpactItem;
use crate::query::ModuleContextItem;
use crate::query::ModuleContextRequest;
use crate::query::ModuleContextResult;
use crate::query::OntographReadQuery;
use crate::query::QueryError;
use crate::query::QueryResult;
use crate::query::RepoMapFileItem;
use crate::query::RepoMapRequest;
use crate::query::RepoMapResult;
use crate::query::SymbolImpactRequest;
use crate::query::SymbolImpactResult;
use crate::query::SymbolSearchItem;
use crate::query::SymbolSearchRequest;
use crate::query::SymbolSearchResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RustIndexRequest {
    pub repo_root: PathBuf,
    pub generation_id: String,
    pub target_head: String,
    pub implementation_version: String,
    pub timestamp_unix_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct SourceIndexArtifact {
    pub schema_version: u32,
    pub files: Vec<IndexedFile>,
    pub relationships: Vec<IndexedRelationship>,
    pub processes: Vec<IndexedProcess>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct IndexedFile {
    pub path: String,
    pub language: String,
    pub symbols: Vec<IndexedSymbol>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct IndexedSymbol {
    pub name: String,
    pub kind: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct IndexedRelationship {
    pub from_symbol: String,
    pub to_symbol: String,
    pub kind: String,
    pub path: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct IndexedProcess {
    pub name: String,
    pub entry_symbol: String,
    pub path: String,
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RustIndexError {
    Walk,
    ReadFile,
    Serialize,
    WriteArtifact,
    PublishManifest(ArtifactPublishError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceIndexStore {
    artifact: SourceIndexArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceIndexLoadError {
    MissingCurrentPointer,
    InvalidCurrentPointer,
    MissingSourceIndex,
    ReadSourceIndex,
    InvalidSourceIndex,
}

pub(crate) fn index_rust_workspace(
    request: RustIndexRequest,
) -> Result<SourceIndexArtifact, RustIndexError> {
    let files = collect_rust_files(&request.repo_root)?;
    let mut working_files = Vec::with_capacity(files.len());
    for path in files {
        let content = fs::read_to_string(&path).map_err(|_| RustIndexError::ReadFile)?;
        let relative_path = relative_slash_path(&request.repo_root, &path)?;
        let symbols = collect_rust_symbols(&content);
        working_files.push(WorkingFile {
            indexed: IndexedFile {
                path: relative_path,
                language: "rust".to_string(),
                symbols,
            },
            content,
        });
    }

    let relationships = collect_relationships(&working_files);
    let processes = collect_processes(&relationships);
    let indexed_files = working_files
        .into_iter()
        .map(|file| file.indexed)
        .collect::<Vec<_>>();
    let artifact = SourceIndexArtifact {
        schema_version: 1,
        files: indexed_files,
        relationships,
        processes,
    };
    publish_index_artifact(&request, &artifact)?;
    Ok(artifact)
}

impl SourceIndexStore {
    pub(crate) fn load_active_generation(
        root: &OntographArtifactRoot,
    ) -> Result<Self, SourceIndexLoadError> {
        let generation_id =
            fs::read_to_string(root.current_pointer_path()).map_err(|err| match err.kind() {
                io::ErrorKind::NotFound => SourceIndexLoadError::MissingCurrentPointer,
                _ => SourceIndexLoadError::InvalidCurrentPointer,
            })?;
        let source_index_path = root
            .generation_file_path(generation_id.trim(), SOURCE_INDEX_FILE)
            .map_err(|_| SourceIndexLoadError::InvalidCurrentPointer)?;
        let contents = fs::read_to_string(source_index_path).map_err(|err| match err.kind() {
            io::ErrorKind::NotFound => SourceIndexLoadError::MissingSourceIndex,
            _ => SourceIndexLoadError::ReadSourceIndex,
        })?;
        let artifact = serde_json::from_str::<SourceIndexArtifact>(&contents)
            .map_err(|_| SourceIndexLoadError::InvalidSourceIndex)?;
        Ok(Self { artifact })
    }
}

impl OntographReadQuery for SourceIndexStore {
    fn search_symbols(&self, request: SymbolSearchRequest) -> QueryResult<SymbolSearchResult> {
        let query = request.query.to_lowercase();
        let matches = self
            .artifact
            .files
            .iter()
            .flat_map(|file| {
                file.symbols.iter().filter_map(|symbol| {
                    if symbol.name.to_lowercase().contains(&query) {
                        Some(SymbolSearchItem {
                            symbol: symbol.name.clone(),
                            kind: symbol.kind.clone(),
                            path: file.path.clone(),
                        })
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();
        Ok(SymbolSearchResult {
            matches: BoundedItems::new(matches, request.budget),
        })
    }

    fn search_repomap(&self, request: RepoMapRequest) -> QueryResult<RepoMapResult> {
        let query = request.query.to_lowercase();
        let files = self
            .artifact
            .files
            .iter()
            .filter_map(|file| {
                let path_matches = file.path.to_lowercase().contains(&query);
                let symbols = file
                    .symbols
                    .iter()
                    .filter(|symbol| path_matches || symbol.name.to_lowercase().contains(&query))
                    .take(request.budget.max_items)
                    .map(|symbol| SymbolSearchItem {
                        symbol: symbol.name.clone(),
                        kind: symbol.kind.clone(),
                        path: file.path.clone(),
                    })
                    .collect::<Vec<_>>();
                if !path_matches && symbols.is_empty() {
                    None
                } else {
                    Some(RepoMapFileItem {
                        path: file.path.clone(),
                        language: file.language.clone(),
                        symbols,
                    })
                }
            })
            .collect::<Vec<_>>();
        Ok(RepoMapResult {
            files: BoundedItems::new(files, request.budget),
        })
    }

    fn inspect_module(&self, request: ModuleContextRequest) -> QueryResult<ModuleContextResult> {
        let file = self
            .artifact
            .files
            .iter()
            .find(|file| file.path == request.path)
            .ok_or(QueryError::MissingArtifact)?;
        let symbols = file
            .symbols
            .iter()
            .map(|symbol| SymbolSearchItem {
                symbol: symbol.name.clone(),
                kind: symbol.kind.clone(),
                path: file.path.clone(),
            })
            .collect::<Vec<_>>();
        Ok(ModuleContextResult {
            module: ModuleContextItem {
                path: file.path.clone(),
                language: Some(file.language.clone()),
                symbols: BoundedItems::new(symbols, request.budget),
            },
        })
    }

    fn impact_symbol(&self, request: SymbolImpactRequest) -> QueryResult<SymbolImpactResult> {
        let upstream = self
            .artifact
            .relationships
            .iter()
            .filter(|relationship| relationship.to_symbol == request.symbol)
            .map(|relationship| ImpactItem {
                symbol: relationship.from_symbol.clone(),
                path: relationship.path.clone(),
                relationship: relationship.kind.clone(),
            })
            .collect::<Vec<_>>();
        let downstream = self
            .artifact
            .relationships
            .iter()
            .filter(|relationship| relationship.from_symbol == request.symbol)
            .map(|relationship| ImpactItem {
                symbol: relationship.to_symbol.clone(),
                path: relationship.path.clone(),
                relationship: relationship.kind.clone(),
            })
            .collect::<Vec<_>>();
        Ok(SymbolImpactResult {
            upstream: BoundedItems::new(upstream, request.budget.clone()),
            downstream: BoundedItems::new(downstream, request.budget),
        })
    }
}

struct WorkingFile {
    indexed: IndexedFile,
    content: String,
}

fn collect_relationships(files: &[WorkingFile]) -> Vec<IndexedRelationship> {
    let mut relationships = Vec::new();
    for file in files {
        let function_names = file
            .indexed
            .symbols
            .iter()
            .filter(|symbol| symbol.kind == "fn")
            .map(|symbol| symbol.name.as_str())
            .collect::<Vec<_>>();
        let mut current_function = None;
        for (index, line) in file.content.lines().enumerate() {
            if let Some((kind, name)) = parse_top_level_rust_symbol(line) {
                current_function = (kind == "fn").then_some(name);
            }
            let Some(caller) = current_function.as_deref() else {
                continue;
            };
            for callee in &function_names {
                if caller != *callee && line.contains(&format!("{callee}(")) {
                    relationships.push(IndexedRelationship {
                        from_symbol: caller.to_string(),
                        to_symbol: (*callee).to_string(),
                        kind: "CALLS".to_string(),
                        path: file.indexed.path.clone(),
                        line: index + 1,
                    });
                }
            }
        }
    }
    relationships
}

fn collect_processes(relationships: &[IndexedRelationship]) -> Vec<IndexedProcess> {
    relationships
        .iter()
        .filter(|relationship| relationship.kind == "CALLS")
        .map(|relationship| IndexedProcess {
            name: format!("{} -> {}", relationship.from_symbol, relationship.to_symbol),
            entry_symbol: relationship.from_symbol.clone(),
            path: relationship.path.clone(),
            steps: vec![
                relationship.from_symbol.clone(),
                relationship.to_symbol.clone(),
            ],
        })
        .collect()
}

fn publish_index_artifact(
    request: &RustIndexRequest,
    artifact: &SourceIndexArtifact,
) -> Result<(), RustIndexError> {
    let root = OntographArtifactRoot::new(&request.repo_root);
    let artifact_path = root
        .generation_file_path(&request.generation_id, SOURCE_INDEX_FILE)
        .map_err(RustIndexError::PublishManifest)?;
    let contents = serde_json::to_string_pretty(artifact).map_err(|_| RustIndexError::Serialize)?;
    write_atomically(&artifact_path, &format!("{contents}\n"))
        .map_err(|_| RustIndexError::WriteArtifact)?;
    root.publish_manifest(&manifest_for_request(request, artifact))
        .map_err(RustIndexError::PublishManifest)
}

fn manifest_for_request(
    request: &RustIndexRequest,
    artifact: &SourceIndexArtifact,
) -> OntographManifest {
    let symbol_count = artifact
        .files
        .iter()
        .map(|file| file.symbols.len() as u64)
        .sum::<u64>();
    let relationship_count = artifact.relationships.len() as u64;
    OntographManifest {
        repository: RepositoryIdentity {
            root: request.repo_root.display().to_string(),
            vcs: "git".to_string(),
            remote_url: None,
        },
        version: ArtifactVersion {
            schema_version: artifact.schema_version,
            implementation_version: request.implementation_version.clone(),
        },
        active_generation_id: request.generation_id.clone(),
        completed_generations: vec![ArtifactGeneration {
            metadata: GenerationMetadata {
                generation_id: request.generation_id.clone(),
                target_head: request.target_head.clone(),
                created_at_unix_seconds: request.timestamp_unix_seconds,
                completed_at_unix_seconds: request.timestamp_unix_seconds,
                manifest_hash: format!(
                    "source-index-v{}-files-{}-symbols-{}",
                    artifact.schema_version,
                    artifact.files.len(),
                    symbol_count
                ),
            },
            counts: GraphCountSummary {
                files: artifact.files.len() as u64,
                symbols: symbol_count,
                relationships: relationship_count,
                nodes: artifact.files.len() as u64 + symbol_count,
                edges: relationship_count,
            },
        }],
    }
}

fn collect_rust_files(repo_root: &Path) -> Result<Vec<PathBuf>, RustIndexError> {
    let mut files = Vec::new();
    collect_rust_files_from(repo_root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_rust_files_from(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), RustIndexError> {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Err(RustIndexError::Walk),
        Err(_) => return Err(RustIndexError::Walk),
    };
    for entry in entries {
        let entry = entry.map_err(|_| RustIndexError::Walk)?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|_| RustIndexError::Walk)?;
        if file_type.is_dir() {
            if !is_ignored_dir(&path) {
                collect_rust_files_from(&path, files)?;
            }
        } else if file_type.is_file() && path.extension().is_some_and(|extension| extension == "rs")
        {
            files.push(path);
        }
    }
    Ok(())
}

fn is_ignored_dir(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(".git" | ".ontograph" | "target")
    )
}

fn relative_slash_path(repo_root: &Path, path: &Path) -> Result<String, RustIndexError> {
    let relative = path
        .strip_prefix(repo_root)
        .map_err(|_| RustIndexError::Walk)?;
    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/"))
}

fn collect_rust_symbols(content: &str) -> Vec<IndexedSymbol> {
    content
        .lines()
        .enumerate()
        .filter_map(|(index, line)| parse_top_level_rust_symbol(line).map(|symbol| (index, symbol)))
        .map(|(index, (kind, name))| IndexedSymbol {
            name,
            kind,
            line: index + 1,
        })
        .collect()
}

fn parse_top_level_rust_symbol(line: &str) -> Option<(String, String)> {
    if line != line.trim_start() {
        return None;
    }
    let mut tokens = line.split(|character: char| {
        character.is_whitespace() || matches!(character, '(' | '<' | '{' | ';' | '=')
    });
    let first = tokens.next()?;
    let kind = if first == "pub" {
        tokens.find(|token| is_rust_symbol_kind(token))?
    } else {
        first
    };
    if !is_rust_symbol_kind(kind) {
        return None;
    }
    let name = tokens.find(|token| !token.is_empty())?;
    Some((kind.to_string(), name.trim_end_matches(':').to_string()))
}

fn is_rust_symbol_kind(token: &str) -> bool {
    matches!(
        token,
        "struct" | "enum" | "trait" | "fn" | "mod" | "type" | "const"
    )
}

#[cfg(test)]
#[path = "indexer_tests.rs"]
mod tests;
