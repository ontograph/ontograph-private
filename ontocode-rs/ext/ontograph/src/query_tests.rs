use pretty_assertions::assert_eq;

use super::*;

#[test]
fn query_budget_clamps_to_accepted_adr_limits() {
    assert_eq!(
        QueryBudget::bounded(30_000, 50_000, 1024 * 1024),
        QueryBudget {
            timeout_ms: MAX_QUERY_TIMEOUT_MS,
            max_items: MAX_QUERY_ITEMS,
            max_serialized_bytes: MAX_SERIALIZED_BYTES,
        }
    );
    assert_eq!(
        QueryBudget::bounded(0, 0, 0),
        QueryBudget {
            timeout_ms: 1,
            max_items: 1,
            max_serialized_bytes: 1,
        }
    );
}

#[test]
fn bounded_items_truncates_deterministic_results() {
    let items = vec![
        SymbolSearchItem {
            symbol: "A".to_string(),
            kind: "struct".to_string(),
            path: "src/a.rs".to_string(),
        },
        SymbolSearchItem {
            symbol: "B".to_string(),
            kind: "enum".to_string(),
            path: "src/b.rs".to_string(),
        },
    ];
    let budget = QueryBudget::bounded(100, 1, 256);

    assert_eq!(
        BoundedItems::new(items, budget.clone()),
        BoundedItems {
            items: vec![SymbolSearchItem {
                symbol: "A".to_string(),
                kind: "struct".to_string(),
                path: "src/a.rs".to_string(),
            }],
            truncated: true,
            budget,
        }
    );
}

#[test]
fn read_query_trait_returns_typed_search_inspect_and_impact_results() {
    let store = FakeReadStore;
    let budget = QueryBudget::bounded(100, 1, 512);

    assert_eq!(
        store.search_symbols(SymbolSearchRequest {
            query: "Demo".to_string(),
            budget: budget.clone(),
        }),
        Ok(SymbolSearchResult {
            matches: BoundedItems {
                items: vec![SymbolSearchItem {
                    symbol: "Demo".to_string(),
                    kind: "struct".to_string(),
                    path: "src/demo.rs".to_string(),
                }],
                truncated: false,
                budget: budget.clone(),
            },
        })
    );
    assert_eq!(
        store.search_repomap(RepoMapRequest {
            query: "Demo".to_string(),
            budget: budget.clone(),
        }),
        Ok(RepoMapResult {
            files: BoundedItems {
                items: vec![RepoMapFileItem {
                    path: "src/demo.rs".to_string(),
                    language: "rust".to_string(),
                    symbols: vec![SymbolSearchItem {
                        symbol: "Demo".to_string(),
                        kind: "struct".to_string(),
                        path: "src/demo.rs".to_string(),
                    }],
                }],
                truncated: false,
                budget: budget.clone(),
            },
        })
    );
    assert_eq!(
        store.inspect_module(ModuleContextRequest {
            path: "src/demo.rs".to_string(),
            budget: budget.clone(),
        }),
        Ok(ModuleContextResult {
            module: ModuleContextItem {
                path: "src/demo.rs".to_string(),
                language: Some("rust".to_string()),
                symbols: BoundedItems {
                    items: vec![SymbolSearchItem {
                        symbol: "Demo".to_string(),
                        kind: "struct".to_string(),
                        path: "src/demo.rs".to_string(),
                    }],
                    truncated: false,
                    budget: budget.clone(),
                },
            },
        })
    );
    assert_eq!(
        store.impact_symbol(SymbolImpactRequest {
            symbol: "Demo".to_string(),
            budget: budget.clone(),
        }),
        Ok(SymbolImpactResult {
            upstream: BoundedItems {
                items: vec![ImpactItem {
                    symbol: "caller".to_string(),
                    path: "src/caller.rs".to_string(),
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

struct FakeReadStore;

impl OntographReadQuery for FakeReadStore {
    fn search_symbols(&self, request: SymbolSearchRequest) -> QueryResult<SymbolSearchResult> {
        Ok(SymbolSearchResult {
            matches: BoundedItems::new(
                vec![SymbolSearchItem {
                    symbol: request.query,
                    kind: "struct".to_string(),
                    path: "src/demo.rs".to_string(),
                }],
                request.budget,
            ),
        })
    }

    fn search_repomap(&self, request: RepoMapRequest) -> QueryResult<RepoMapResult> {
        Ok(RepoMapResult {
            files: BoundedItems::new(
                vec![RepoMapFileItem {
                    path: "src/demo.rs".to_string(),
                    language: "rust".to_string(),
                    symbols: vec![SymbolSearchItem {
                        symbol: request.query,
                        kind: "struct".to_string(),
                        path: "src/demo.rs".to_string(),
                    }],
                }],
                request.budget,
            ),
        })
    }

    fn inspect_module(&self, request: ModuleContextRequest) -> QueryResult<ModuleContextResult> {
        Ok(ModuleContextResult {
            module: ModuleContextItem {
                path: request.path.clone(),
                language: Some("rust".to_string()),
                symbols: BoundedItems::new(
                    vec![SymbolSearchItem {
                        symbol: "Demo".to_string(),
                        kind: "struct".to_string(),
                        path: request.path,
                    }],
                    request.budget,
                ),
            },
        })
    }

    fn impact_symbol(&self, request: SymbolImpactRequest) -> QueryResult<SymbolImpactResult> {
        Ok(SymbolImpactResult {
            upstream: BoundedItems::new(
                vec![ImpactItem {
                    symbol: "caller".to_string(),
                    path: "src/caller.rs".to_string(),
                    relationship: "CALLS".to_string(),
                }],
                request.budget.clone(),
            ),
            downstream: BoundedItems::new(Vec::new(), request.budget),
        })
    }
}
