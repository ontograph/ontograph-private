use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;

use ontocode_extension_api::FunctionCallError;
use ontocode_extension_api::JsonToolOutput;
use ontocode_extension_api::ToolCall;
use ontocode_extension_api::ToolExecutor;
use ontocode_extension_api::ToolName;
use ontocode_extension_api::ToolOutput;
use ontocode_extension_api::ToolSpec;
use ontocode_extension_api::parse_tool_input_schema;
use ontocode_tools::ResponsesApiNamespace;
use ontocode_tools::ResponsesApiNamespaceTool;
use ontocode_tools::ResponsesApiTool;
use ontocode_tools::ToolExposure;
use ontocode_tools::default_namespace_description;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::indexer::SourceIndexLoadError;
use crate::indexer::SourceIndexStore;
use crate::manifest::OntographArtifactRoot;
use crate::manifest::OntographManifest;
use crate::query::MAX_QUERY_ITEMS;
use crate::query::MAX_QUERY_TIMEOUT_MS;
use crate::query::MAX_SERIALIZED_BYTES;
use crate::query::ModuleContextRequest;
use crate::query::OntographReadQuery;
use crate::query::QueryBudget;
use crate::query::QueryError;
use crate::query::RepoMapRequest;
use crate::query::SymbolImpactRequest;
use crate::query::SymbolSearchRequest;

pub(crate) const ONTOGRAPH_NAMESPACE: &str = "ontograph";
pub(crate) const DISCOVER_TOOL_NAME: &str = "discover";
pub(crate) const EXPLAIN_MODULE_TOOL_NAME: &str = "explain_module";
pub(crate) const IMPACT_TOOL_NAME: &str = "impact";
pub(crate) const INSPECT_TOOL_NAME: &str = "inspect";
pub(crate) const SEARCH_TOOL_NAME: &str = "search";

const DISCOVER_TOOL_DESCRIPTION: &str =
    "Report the live native Ontograph tool surface and checked-in parity metadata.";
const EXPLAIN_MODULE_DESCRIPTION: &str =
    "Inspect one local UTF-8 file and return bounded module facts without building an index.";
const IMPACT_DESCRIPTION: &str =
    "Read bounded symbol impact from the active native Ontograph source index.";
const INSPECT_DESCRIPTION: &str =
    "Read bounded context from the active native Ontograph source index.";
const SEARCH_DESCRIPTION: &str =
    "Read bounded symbol matches from the active native Ontograph source index.";
const PARITY_MATRIX_PATH: &str = ".memory-bank/knowledge-hub/ONTOGRAPH_DONOR_PARITY_MATRIX.md";
const DONOR_ACTIONS_TOTAL: usize = 62;
const DONOR_ACTIONS_IMPLEMENTED: usize = 6;
const DONOR_ACTIONS_INTENTIONAL_DIVERGENCE: usize = 1;
pub(crate) const MAX_FILE_BYTES: usize = 1024 * 1024;
const MAX_SYMBOLS: usize = 32;
const RUST_SCAN_LIMIT_NOTE: &str =
    "top_level_symbols are collected only from top-level Rust lines using a simple line-based scan";

#[derive(Clone, Default)]
pub(crate) struct DiscoverTool {
    thread_state: Arc<OntographThreadState>,
}

#[derive(Clone, Default)]
pub(crate) struct ExplainModuleTool {
    thread_state: Arc<OntographThreadState>,
}

#[derive(Clone, Default)]
pub(crate) struct ImpactTool {
    thread_state: Arc<OntographThreadState>,
}

#[derive(Clone, Default)]
pub(crate) struct InspectTool {
    thread_state: Arc<OntographThreadState>,
}

#[derive(Clone, Default)]
pub(crate) struct SearchTool {
    thread_state: Arc<OntographThreadState>,
}

#[derive(Debug, Default)]
pub(crate) struct OntographThreadState {
    current_cwd: Mutex<Option<PathBuf>>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ExplainModuleArgs {
    pub path: String,
    #[serde(default)]
    pub max_symbols: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct DiscoverArgs {
    pub action: DiscoverAction,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectArgs {
    pub action: InspectAction,
    pub path: String,
    #[serde(default)]
    pub max_items: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ImpactArgs {
    pub action: ImpactAction,
    pub symbol: String,
    #[serde(default)]
    pub max_items: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct SearchArgs {
    pub action: SearchAction,
    pub query: String,
    #[serde(default)]
    pub max_items: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DiscoverAction {
    Repos,
    Tools,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum InspectAction {
    Context,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ImpactAction {
    Symbol,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SearchAction {
    Semantic,
    Repomap,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ImplementedToolSummary {
    pub full_name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct DiscoverResult {
    pub namespace: String,
    pub tools: Vec<ImplementedToolSummary>,
    pub repos: Vec<DiscoveredRepoSummary>,
    pub parity_matrix_path: String,
    pub donor_actions_total: usize,
    pub donor_actions_implemented: usize,
    pub donor_actions_intentional_divergence: usize,
    pub donor_actions_missing: usize,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct DiscoveredRepoSummary {
    pub root: String,
    pub vcs: String,
    pub remote_url: Option<String>,
    pub active_generation_id: String,
    pub target_head: String,
    pub files: u64,
    pub symbols: u64,
    pub relationships: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ExplainModuleResult {
    pub path: String,
    pub language: Option<String>,
    pub line_count: usize,
    pub top_level_symbols: Vec<String>,
    pub limits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectContextResult {
    pub path: String,
    pub language: Option<String>,
    pub symbols: Vec<InspectSymbolSummary>,
    pub truncated: bool,
    pub limits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectSymbolSummary {
    pub symbol: String,
    pub kind: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ImpactSymbolResult {
    pub symbol: String,
    pub upstream: Vec<ImpactSymbolSummary>,
    pub downstream: Vec<ImpactSymbolSummary>,
    pub upstream_truncated: bool,
    pub downstream_truncated: bool,
    pub limits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ImpactSymbolSummary {
    pub symbol: String,
    pub path: String,
    pub relationship: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SearchResult {
    pub action: SearchAction,
    pub query: String,
    pub matches: Vec<SearchSymbolSummary>,
    pub files: Vec<SearchRepomapFileSummary>,
    pub truncated: bool,
    pub limits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SearchSymbolSummary {
    pub symbol: String,
    pub kind: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SearchRepomapFileSummary {
    pub path: String,
    pub language: String,
    pub symbols: Vec<SearchSymbolSummary>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for DiscoverTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(ONTOGRAPH_NAMESPACE, DISCOVER_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        namespace_function_tool::<DiscoverArgs, DiscoverResult>(
            DISCOVER_TOOL_NAME,
            DISCOVER_TOOL_DESCRIPTION,
        )
    }

    fn exposure(&self) -> ToolExposure {
        ToolExposure::DirectModelOnly
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    async fn handle(&self, call: ToolCall) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args = parse_tool_args::<DiscoverArgs>(&call, "ontograph.discover")?;
        let result = match args.action {
            DiscoverAction::Repos => {
                let cwd = self.thread_state.current_cwd().ok_or_else(|| {
                    FunctionCallError::RespondToModel(
                        "ontograph.discover workspace context is unavailable for this turn"
                            .to_string(),
                    )
                })?;
                DiscoverResult {
                    namespace: ONTOGRAPH_NAMESPACE.to_string(),
                    tools: Vec::new(),
                    repos: vec![discover_repo_from_manifest(&cwd)?],
                    parity_matrix_path: PARITY_MATRIX_PATH.to_string(),
                    donor_actions_total: DONOR_ACTIONS_TOTAL,
                    donor_actions_implemented: DONOR_ACTIONS_IMPLEMENTED,
                    donor_actions_intentional_divergence: DONOR_ACTIONS_INTENTIONAL_DIVERGENCE,
                    donor_actions_missing: donor_actions_missing(),
                    notes: vec![
                        "Only the current workspace active native Ontograph artifact is listed."
                            .to_string(),
                        "No donor repo registry, daemon, sync, or group model is used.".to_string(),
                    ],
                }
            }
            DiscoverAction::Tools => DiscoverResult {
                namespace: ONTOGRAPH_NAMESPACE.to_string(),
                tools: implemented_tool_summaries(),
                repos: Vec::new(),
                parity_matrix_path: PARITY_MATRIX_PATH.to_string(),
                donor_actions_total: DONOR_ACTIONS_TOTAL,
                donor_actions_implemented: DONOR_ACTIONS_IMPLEMENTED,
                donor_actions_intentional_divergence: DONOR_ACTIONS_INTENTIONAL_DIVERGENCE,
                donor_actions_missing: donor_actions_missing(),
                notes: vec![
                    "Only live native Ontograph tools are listed.".to_string(),
                    "Donor facade placeholders are intentionally excluded.".to_string(),
                ],
            },
        };
        let value = serde_json::to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize ontograph.discover result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl DiscoverTool {
    pub(crate) fn new(thread_state: Arc<OntographThreadState>) -> Self {
        Self { thread_state }
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExplainModuleTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(ONTOGRAPH_NAMESPACE, EXPLAIN_MODULE_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        namespace_function_tool::<ExplainModuleArgs, ExplainModuleResult>(
            EXPLAIN_MODULE_TOOL_NAME,
            EXPLAIN_MODULE_DESCRIPTION,
        )
    }

    fn exposure(&self) -> ToolExposure {
        ToolExposure::DirectModelOnly
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    async fn handle(&self, call: ToolCall) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args = parse_tool_args::<ExplainModuleArgs>(&call, "ontograph.explain_module")?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "ontograph.explain_module workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let normalized_path = normalize_model_path(&args.path, "ontograph.explain_module")?;
        let resolved_path = resolve_module_path(&normalized_path, &cwd)?;
        let content = read_bounded_utf8(&resolved_path)?;
        let language = detect_language(&normalized_path);
        let top_level_symbols = collect_top_level_symbols(
            &content,
            language.as_deref(),
            args.max_symbols
                .unwrap_or(MAX_SYMBOLS)
                .clamp(1, MAX_SYMBOLS),
        );

        let result = ExplainModuleResult {
            path: normalized_path,
            language,
            line_count: line_count(&content),
            top_level_symbols,
            limits: vec![
                format!("file reads are capped at {MAX_FILE_BYTES} bytes"),
                RUST_SCAN_LIMIT_NOTE.to_string(),
            ],
        };
        let value = serde_json::to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize ontograph.explain_module result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ImpactTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(ONTOGRAPH_NAMESPACE, IMPACT_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        namespace_function_tool::<ImpactArgs, ImpactSymbolResult>(
            IMPACT_TOOL_NAME,
            IMPACT_DESCRIPTION,
        )
    }

    fn exposure(&self) -> ToolExposure {
        ToolExposure::DirectModelOnly
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    async fn handle(&self, call: ToolCall) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args = parse_tool_args::<ImpactArgs>(&call, "ontograph.impact")?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "ontograph.impact workspace context is unavailable for this turn".to_string(),
            )
        })?;
        let symbol = args.symbol.trim();
        if symbol.is_empty() {
            return Err(FunctionCallError::RespondToModel(
                "ontograph.impact symbol must not be empty".to_string(),
            ));
        }
        let store = SourceIndexStore::load_active_generation(&OntographArtifactRoot::new(&cwd))
            .map_err(impact_load_error)?;
        let result = match args.action {
            ImpactAction::Symbol => store
                .impact_symbol(SymbolImpactRequest {
                    symbol: symbol.to_string(),
                    budget: QueryBudget::bounded(
                        MAX_QUERY_TIMEOUT_MS,
                        args.max_items.unwrap_or(MAX_SYMBOLS),
                        MAX_SERIALIZED_BYTES,
                    ),
                })
                .map_err(impact_query_error)?,
        };
        let upstream_truncated = result.upstream.truncated;
        let downstream_truncated = result.downstream.truncated;
        let response = ImpactSymbolResult {
            symbol: symbol.to_string(),
            upstream: result
                .upstream
                .items
                .into_iter()
                .map(|item| ImpactSymbolSummary {
                    symbol: item.symbol,
                    path: item.path,
                    relationship: item.relationship,
                })
                .collect(),
            downstream: result
                .downstream
                .items
                .into_iter()
                .map(|item| ImpactSymbolSummary {
                    symbol: item.symbol,
                    path: item.path,
                    relationship: item.relationship,
                })
                .collect(),
            upstream_truncated,
            downstream_truncated,
            limits: vec![
                format!("impact items are capped at {MAX_QUERY_ITEMS} per direction"),
                "impact.symbol reads only the active native source-index artifact".to_string(),
            ],
        };
        let value = serde_json::to_value(response).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize ontograph.impact result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for InspectTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(ONTOGRAPH_NAMESPACE, INSPECT_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        namespace_function_tool::<InspectArgs, InspectContextResult>(
            INSPECT_TOOL_NAME,
            INSPECT_DESCRIPTION,
        )
    }

    fn exposure(&self) -> ToolExposure {
        ToolExposure::DirectModelOnly
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    async fn handle(&self, call: ToolCall) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args = parse_tool_args::<InspectArgs>(&call, "ontograph.inspect")?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "ontograph.inspect workspace context is unavailable for this turn".to_string(),
            )
        })?;
        let normalized_path = normalize_model_path(&args.path, "ontograph.inspect")?;
        let store = SourceIndexStore::load_active_generation(&OntographArtifactRoot::new(&cwd))
            .map_err(inspect_load_error)?;
        let result = match args.action {
            InspectAction::Context => store
                .inspect_module(ModuleContextRequest {
                    path: normalized_path,
                    budget: QueryBudget::bounded(
                        MAX_QUERY_TIMEOUT_MS,
                        args.max_items.unwrap_or(MAX_SYMBOLS),
                        MAX_SERIALIZED_BYTES,
                    ),
                })
                .map_err(inspect_query_error)?,
        };
        let symbols = result
            .module
            .symbols
            .items
            .into_iter()
            .map(|symbol| InspectSymbolSummary {
                symbol: symbol.symbol,
                kind: symbol.kind,
                path: symbol.path,
            })
            .collect();
        let response = InspectContextResult {
            path: result.module.path,
            language: result.module.language,
            symbols,
            truncated: result.module.symbols.truncated,
            limits: vec![
                format!("context symbols are capped at {MAX_QUERY_ITEMS} items"),
                "inspect.context reads only the active native source-index artifact".to_string(),
            ],
        };
        let value = serde_json::to_value(response).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize ontograph.inspect result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for SearchTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(ONTOGRAPH_NAMESPACE, SEARCH_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        namespace_function_tool::<SearchArgs, SearchResult>(SEARCH_TOOL_NAME, SEARCH_DESCRIPTION)
    }

    fn exposure(&self) -> ToolExposure {
        ToolExposure::DirectModelOnly
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    async fn handle(&self, call: ToolCall) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args = parse_tool_args::<SearchArgs>(&call, "ontograph.search")?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "ontograph.search workspace context is unavailable for this turn".to_string(),
            )
        })?;
        let query = args.query.trim();
        if query.is_empty() {
            return Err(FunctionCallError::RespondToModel(
                "ontograph.search query must not be empty".to_string(),
            ));
        }
        let store = SourceIndexStore::load_active_generation(&OntographArtifactRoot::new(&cwd))
            .map_err(search_load_error)?;
        let budget = QueryBudget::bounded(
            MAX_QUERY_TIMEOUT_MS,
            args.max_items.unwrap_or(MAX_SYMBOLS),
            MAX_SERIALIZED_BYTES,
        );
        let response = match args.action {
            SearchAction::Semantic => {
                let result = store
                    .search_symbols(SymbolSearchRequest {
                        query: query.to_string(),
                        budget,
                    })
                    .map_err(search_query_error)?;
                SearchResult {
                    action: SearchAction::Semantic,
                    query: query.to_string(),
                    matches: result
                        .matches
                        .items
                        .into_iter()
                        .map(search_symbol_summary)
                        .collect(),
                    files: Vec::new(),
                    truncated: result.matches.truncated,
                    limits: vec![
                        format!("search matches are capped at {MAX_QUERY_ITEMS} items"),
                        "search.semantic currently uses native source-index symbol matching only"
                            .to_string(),
                    ],
                }
            }
            SearchAction::Repomap => {
                let result = store
                    .search_repomap(RepoMapRequest {
                        query: query.to_string(),
                        budget,
                    })
                    .map_err(search_query_error)?;
                SearchResult {
                    action: SearchAction::Repomap,
                    query: query.to_string(),
                    matches: Vec::new(),
                    files: result
                        .files
                        .items
                        .into_iter()
                        .map(|file| SearchRepomapFileSummary {
                            path: file.path,
                            language: file.language,
                            symbols: file
                                .symbols
                                .into_iter()
                                .map(search_symbol_summary)
                                .collect(),
                        })
                        .collect(),
                    truncated: result.files.truncated,
                    limits: vec![
                        format!("repomap files are capped at {MAX_QUERY_ITEMS} items"),
                        format!("repomap symbols per file are capped at {MAX_QUERY_ITEMS} items"),
                        "search.repomap currently uses native source-index file and symbol facts only"
                            .to_string(),
                    ],
                }
            }
        };
        let value = serde_json::to_value(response).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize ontograph.search result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

fn search_symbol_summary(item: crate::query::SymbolSearchItem) -> SearchSymbolSummary {
    SearchSymbolSummary {
        symbol: item.symbol,
        kind: item.kind,
        path: item.path,
    }
}

impl ExplainModuleTool {
    pub(crate) fn new(thread_state: Arc<OntographThreadState>) -> Self {
        Self { thread_state }
    }
}

impl ImpactTool {
    pub(crate) fn new(thread_state: Arc<OntographThreadState>) -> Self {
        Self { thread_state }
    }
}

impl InspectTool {
    pub(crate) fn new(thread_state: Arc<OntographThreadState>) -> Self {
        Self { thread_state }
    }
}

impl SearchTool {
    pub(crate) fn new(thread_state: Arc<OntographThreadState>) -> Self {
        Self { thread_state }
    }
}

impl OntographThreadState {
    pub(crate) fn set_current_cwd(&self, cwd: PathBuf) {
        *self
            .current_cwd
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = Some(cwd);
    }

    pub(crate) fn clear_current_cwd(&self) {
        *self
            .current_cwd
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = None;
    }

    pub(crate) fn current_cwd(&self) -> Option<PathBuf> {
        self.current_cwd
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

fn namespace_function_tool<I: JsonSchema, O: JsonSchema>(
    name: &str,
    description: &str,
) -> ToolSpec {
    let input_schema = serde_json::to_value(schemars::schema_for!(I))
        .unwrap_or_else(|err| panic!("{name} args schema should serialize: {err}"));
    let output_schema = serde_json::to_value(schemars::schema_for!(O))
        .unwrap_or_else(|err| panic!("{name} result schema should serialize: {err}"));
    ToolSpec::Namespace(ResponsesApiNamespace {
        name: ONTOGRAPH_NAMESPACE.to_string(),
        description: default_namespace_description(ONTOGRAPH_NAMESPACE),
        tools: vec![ResponsesApiNamespaceTool::Function(ResponsesApiTool {
            name: name.to_string(),
            description: description.to_string(),
            strict: false,
            defer_loading: None,
            parameters: parse_tool_input_schema(&input_schema)
                .unwrap_or_else(|err| panic!("{name} args schema should parse: {err}")),
            output_schema: Some(output_schema),
        })],
    })
}

fn parse_tool_args<T: for<'de> Deserialize<'de>>(
    call: &ToolCall,
    tool_name: &str,
) -> Result<T, FunctionCallError> {
    let arguments = call.function_arguments()?;
    serde_json::from_str(arguments).map_err(|err| {
        FunctionCallError::RespondToModel(format!("invalid {tool_name} arguments: {err}"))
    })
}

fn implemented_tool_summaries() -> Vec<ImplementedToolSummary> {
    vec![
        ImplementedToolSummary {
            full_name: format!("{ONTOGRAPH_NAMESPACE}.{DISCOVER_TOOL_NAME}"),
            description: DISCOVER_TOOL_DESCRIPTION.to_string(),
        },
        ImplementedToolSummary {
            full_name: format!("{ONTOGRAPH_NAMESPACE}.{EXPLAIN_MODULE_TOOL_NAME}"),
            description: EXPLAIN_MODULE_DESCRIPTION.to_string(),
        },
        ImplementedToolSummary {
            full_name: format!("{ONTOGRAPH_NAMESPACE}.{IMPACT_TOOL_NAME}"),
            description: IMPACT_DESCRIPTION.to_string(),
        },
        ImplementedToolSummary {
            full_name: format!("{ONTOGRAPH_NAMESPACE}.{INSPECT_TOOL_NAME}"),
            description: INSPECT_DESCRIPTION.to_string(),
        },
        ImplementedToolSummary {
            full_name: format!("{ONTOGRAPH_NAMESPACE}.{SEARCH_TOOL_NAME}"),
            description: SEARCH_DESCRIPTION.to_string(),
        },
    ]
}

fn donor_actions_missing() -> usize {
    DONOR_ACTIONS_TOTAL - DONOR_ACTIONS_IMPLEMENTED - DONOR_ACTIONS_INTENTIONAL_DIVERGENCE
}

fn discover_repo_from_manifest(cwd: &Path) -> Result<DiscoveredRepoSummary, FunctionCallError> {
    let root = OntographArtifactRoot::new(cwd);
    let generation_id = fs::read_to_string(root.current_pointer_path())
        .map_err(|_| {
            FunctionCallError::RespondToModel(
                "ontograph.discover active manifest pointer is missing or unreadable".to_string(),
            )
        })?
        .trim()
        .to_string();
    let manifest_path = root.generation_manifest_path(&generation_id).map_err(|_| {
        FunctionCallError::RespondToModel(
            "ontograph.discover active manifest pointer is invalid".to_string(),
        )
    })?;
    let manifest = serde_json::from_str::<OntographManifest>(
        &fs::read_to_string(manifest_path).map_err(|_| {
            FunctionCallError::RespondToModel(
                "ontograph.discover active manifest is missing or unreadable".to_string(),
            )
        })?,
    )
    .map_err(|_| {
        FunctionCallError::RespondToModel(
            "ontograph.discover active manifest is invalid".to_string(),
        )
    })?;
    let generation = manifest
        .completed_generations
        .iter()
        .find(|generation| generation.metadata.generation_id == manifest.active_generation_id)
        .ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "ontograph.discover active manifest has no completed active generation".to_string(),
            )
        })?;
    Ok(DiscoveredRepoSummary {
        root: manifest.repository.root,
        vcs: manifest.repository.vcs,
        remote_url: manifest.repository.remote_url,
        active_generation_id: manifest.active_generation_id,
        target_head: generation.metadata.target_head.clone(),
        files: generation.counts.files,
        symbols: generation.counts.symbols,
        relationships: generation.counts.relationships,
    })
}

fn normalize_model_path(path: &str, tool_name: &str) -> Result<String, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(format!(
            "{tool_name} path must not be empty"
        )));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(format!(
            "{tool_name} path must be a local file path"
        )));
    }

    let path = Path::new(trimmed);
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(segment) => normalized.push(segment),
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(FunctionCallError::RespondToModel(format!(
                    "{tool_name} path must be relative and stay within the current working directory"
                )));
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err(FunctionCallError::RespondToModel(format!(
            "{tool_name} path must point to a file"
        )));
    }

    Ok(normalized.display().to_string())
}

fn inspect_load_error(error: SourceIndexLoadError) -> FunctionCallError {
    let message = match error {
        SourceIndexLoadError::MissingCurrentPointer => {
            "ontograph.inspect active source index is missing"
        }
        SourceIndexLoadError::InvalidCurrentPointer => {
            "ontograph.inspect active source index pointer is invalid"
        }
        SourceIndexLoadError::MissingSourceIndex => {
            "ontograph.inspect source-index.json is missing"
        }
        SourceIndexLoadError::ReadSourceIndex => {
            "ontograph.inspect source-index.json is unreadable"
        }
        SourceIndexLoadError::InvalidSourceIndex => {
            "ontograph.inspect source-index.json is invalid"
        }
    };
    FunctionCallError::RespondToModel(message.to_string())
}

fn inspect_query_error(error: QueryError) -> FunctionCallError {
    let message = match error {
        QueryError::MissingArtifact => "ontograph.inspect path is not present in source-index.json",
        QueryError::StaleArtifact => "ontograph.inspect source index is stale",
        QueryError::CorruptArtifact => "ontograph.inspect source index is corrupt",
        QueryError::Timeout => "ontograph.inspect query timed out",
        QueryError::OverBudget => "ontograph.inspect query exceeded its budget",
    };
    FunctionCallError::RespondToModel(message.to_string())
}

fn impact_load_error(error: SourceIndexLoadError) -> FunctionCallError {
    let message = match error {
        SourceIndexLoadError::MissingCurrentPointer => {
            "ontograph.impact active source index is missing"
        }
        SourceIndexLoadError::InvalidCurrentPointer => {
            "ontograph.impact active source index pointer is invalid"
        }
        SourceIndexLoadError::MissingSourceIndex => "ontograph.impact source-index.json is missing",
        SourceIndexLoadError::ReadSourceIndex => "ontograph.impact source-index.json is unreadable",
        SourceIndexLoadError::InvalidSourceIndex => "ontograph.impact source-index.json is invalid",
    };
    FunctionCallError::RespondToModel(message.to_string())
}

fn impact_query_error(error: QueryError) -> FunctionCallError {
    let message = match error {
        QueryError::MissingArtifact => {
            "ontograph.impact symbol is not present in source-index.json"
        }
        QueryError::StaleArtifact => "ontograph.impact source index is stale",
        QueryError::CorruptArtifact => "ontograph.impact source index is corrupt",
        QueryError::Timeout => "ontograph.impact query timed out",
        QueryError::OverBudget => "ontograph.impact query exceeded its budget",
    };
    FunctionCallError::RespondToModel(message.to_string())
}

fn search_load_error(error: SourceIndexLoadError) -> FunctionCallError {
    let message = match error {
        SourceIndexLoadError::MissingCurrentPointer => {
            "ontograph.search active source index is missing"
        }
        SourceIndexLoadError::InvalidCurrentPointer => {
            "ontograph.search active source index pointer is invalid"
        }
        SourceIndexLoadError::MissingSourceIndex => "ontograph.search source-index.json is missing",
        SourceIndexLoadError::ReadSourceIndex => "ontograph.search source-index.json is unreadable",
        SourceIndexLoadError::InvalidSourceIndex => "ontograph.search source-index.json is invalid",
    };
    FunctionCallError::RespondToModel(message.to_string())
}

fn search_query_error(error: QueryError) -> FunctionCallError {
    let message = match error {
        QueryError::MissingArtifact => "ontograph.search query has no matches in source-index.json",
        QueryError::StaleArtifact => "ontograph.search source index is stale",
        QueryError::CorruptArtifact => "ontograph.search source index is corrupt",
        QueryError::Timeout => "ontograph.search query timed out",
        QueryError::OverBudget => "ontograph.search query exceeded its budget",
    };
    FunctionCallError::RespondToModel(message.to_string())
}

fn resolve_module_path(path: &str, cwd: &Path) -> Result<PathBuf, FunctionCallError> {
    let relative_path = Path::new(path);
    let resolved_path = cwd.join(relative_path);
    let mut scoped_path = cwd.to_path_buf();
    for component in relative_path.components() {
        let Component::Normal(segment) = component else {
            continue;
        };
        scoped_path.push(segment);
        let Ok(metadata) = fs::symlink_metadata(&scoped_path) else {
            break;
        };
        if metadata.file_type().is_symlink() {
            return Err(FunctionCallError::RespondToModel(
                "ontograph.explain_module path must not traverse symlinks".to_string(),
            ));
        }
    }

    let metadata = fs::metadata(&resolved_path).map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "ontograph.explain_module path does not exist: {err}"
        ))
    })?;
    if !metadata.is_file() {
        return Err(FunctionCallError::RespondToModel(
            "ontograph.explain_module path must point to a file".to_string(),
        ));
    }

    Ok(resolved_path)
}

fn read_bounded_utf8(path: &Path) -> Result<String, FunctionCallError> {
    let mut file = File::open(path).map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "ontograph.explain_module path could not be opened: {err}"
        ))
    })?;
    let mut bytes = Vec::with_capacity(MAX_FILE_BYTES.min(8192));
    file.by_ref()
        .take((MAX_FILE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "ontograph.explain_module path could not be read: {err}"
            ))
        })?;
    if bytes.len() > MAX_FILE_BYTES {
        return Err(FunctionCallError::RespondToModel(format!(
            "ontograph.explain_module path is too large; maximum supported size is {MAX_FILE_BYTES} bytes"
        )));
    }
    String::from_utf8(bytes).map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "ontograph.explain_module path could not be read as UTF-8 text: {err}"
        ))
    })
}

fn detect_language(path: &str) -> Option<String> {
    let extension = Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())?
        .to_ascii_lowercase();

    let language = match extension.as_str() {
        "rs" => "rust",
        "py" => "python",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "json" => "json",
        "md" => "markdown",
        "toml" => "toml",
        "yaml" | "yml" => "yaml",
        "sh" => "shell",
        _ => return None,
    };
    Some(language.to_string())
}

fn line_count(content: &str) -> usize {
    content.lines().count()
}

fn collect_top_level_symbols(
    content: &str,
    language: Option<&str>,
    max_symbols: usize,
) -> Vec<String> {
    if language != Some("rust") {
        return Vec::new();
    }

    let mut symbols = Vec::new();
    for line in content.lines() {
        if line.is_empty() || line.chars().next().is_some_and(char::is_whitespace) {
            continue;
        }
        if let Some(symbol) = top_level_rust_symbol(line) {
            symbols.push(symbol);
            if symbols.len() >= max_symbols {
                break;
            }
        }
    }
    symbols
}

fn top_level_rust_symbol(line: &str) -> Option<String> {
    let line = line.trim_end();
    let line = strip_visibility(line);
    let (keyword, rest) = line.split_once(' ')?;
    if !matches!(keyword, "fn" | "struct" | "enum" | "trait" | "mod") {
        return None;
    }

    let name: String = rest
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect();
    if name.is_empty() { None } else { Some(name) }
}

fn strip_visibility(line: &str) -> &str {
    if let Some(rest) = line.strip_prefix("pub ") {
        return rest;
    }
    if let Some(rest) = line.strip_prefix("pub(")
        && let Some((_, suffix)) = rest.split_once(") ")
    {
        return suffix;
    }
    line
}
