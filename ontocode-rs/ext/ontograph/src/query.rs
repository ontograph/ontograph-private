use serde::Deserialize;
use serde::Serialize;

pub(crate) const MAX_QUERY_TIMEOUT_MS: u64 = 5_000;
pub(crate) const MAX_QUERY_ITEMS: usize = 200;
pub(crate) const MAX_SERIALIZED_BYTES: usize = 64 * 1024;

/// Read-only Ontograph query boundary used by future native tool handlers.
///
/// Implementations must read only completed artifact generations and must
/// enforce the budget carried by each typed request.
pub(crate) trait OntographReadQuery {
    fn search_symbols(&self, request: SymbolSearchRequest) -> QueryResult<SymbolSearchResult>;

    fn search_repomap(&self, request: RepoMapRequest) -> QueryResult<RepoMapResult>;

    fn inspect_module(&self, request: ModuleContextRequest) -> QueryResult<ModuleContextResult>;

    fn impact_symbol(&self, request: SymbolImpactRequest) -> QueryResult<SymbolImpactResult>;
}

pub(crate) type QueryResult<T> = Result<T, QueryError>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct QueryBudget {
    pub timeout_ms: u64,
    pub max_items: usize,
    pub max_serialized_bytes: usize,
}

impl QueryBudget {
    pub(crate) fn bounded(timeout_ms: u64, max_items: usize, max_serialized_bytes: usize) -> Self {
        Self {
            timeout_ms: timeout_ms.clamp(1, MAX_QUERY_TIMEOUT_MS),
            max_items: max_items.clamp(1, MAX_QUERY_ITEMS),
            max_serialized_bytes: max_serialized_bytes.clamp(1, MAX_SERIALIZED_BYTES),
        }
    }
}

impl Default for QueryBudget {
    fn default() -> Self {
        Self {
            timeout_ms: MAX_QUERY_TIMEOUT_MS,
            max_items: MAX_QUERY_ITEMS,
            max_serialized_bytes: MAX_SERIALIZED_BYTES,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct SymbolSearchRequest {
    pub query: String,
    pub budget: QueryBudget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RepoMapRequest {
    pub query: String,
    pub budget: QueryBudget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ModuleContextRequest {
    pub path: String,
    pub budget: QueryBudget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct SymbolImpactRequest {
    pub symbol: String,
    pub budget: QueryBudget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct BoundedItems<T> {
    pub items: Vec<T>,
    pub truncated: bool,
    pub budget: QueryBudget,
}

impl<T> BoundedItems<T> {
    pub(crate) fn new(mut items: Vec<T>, budget: QueryBudget) -> Self {
        let truncated = items.len() > budget.max_items;
        items.truncate(budget.max_items);
        Self {
            items,
            truncated,
            budget,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct SymbolSearchResult {
    pub matches: BoundedItems<SymbolSearchItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RepoMapResult {
    pub files: BoundedItems<RepoMapFileItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RepoMapFileItem {
    pub path: String,
    pub language: String,
    pub symbols: Vec<SymbolSearchItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct SymbolSearchItem {
    pub symbol: String,
    pub kind: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ModuleContextResult {
    pub module: ModuleContextItem,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ModuleContextItem {
    pub path: String,
    pub language: Option<String>,
    pub symbols: BoundedItems<SymbolSearchItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct SymbolImpactResult {
    pub upstream: BoundedItems<ImpactItem>,
    pub downstream: BoundedItems<ImpactItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ImpactItem {
    pub symbol: String,
    pub path: String,
    pub relationship: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum QueryError {
    MissingArtifact,
    StaleArtifact,
    CorruptArtifact,
    Timeout,
    OverBudget,
}

#[cfg(test)]
#[path = "query_tests.rs"]
mod tests;
