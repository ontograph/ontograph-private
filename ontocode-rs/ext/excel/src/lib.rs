mod backend;
mod export;
mod extension;
mod formula_ast;
mod formula_cte_pipeline;
mod formula_inspect;
mod formula_sql;
mod formula_sql_readiness;
mod named_range_rewrite;
mod pivot_report_metadata;
mod powerquery_extract;
mod powerquery_review_bundle;
mod powerquery_translate;
mod preview;
mod sheet_layout_metadata;
mod slider_query;
mod tool;
mod vba_extract;
mod vba_onlyoffice_analyze;
mod vba_onlyoffice_translate;
mod vba_onlyoffice_workbook_review;
mod vba_project_metadata;
mod vba_translate;
mod workbook_connections;
mod workbook_defined_names;
mod workbook_external_links;
mod workbook_graph;
mod workbook_migration_manifest;
mod workbook_tables;
mod workbook_used_ranges;

pub use extension::install;

#[cfg(test)]
#[path = "formula_ast_tests.rs"]
mod formula_ast_tests;

#[cfg(test)]
#[path = "formula_sql_tests.rs"]
mod formula_sql_tests;

#[cfg(test)]
#[path = "formula_sql_readiness_tests.rs"]
mod formula_sql_readiness_tests;

#[cfg(test)]
#[path = "formula_cte_pipeline_tests.rs"]
mod formula_cte_pipeline_tests;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
#[path = "named_range_rewrite_tests.rs"]
mod named_range_rewrite_tests;

#[cfg(test)]
#[path = "workbook_graph_tests.rs"]
mod workbook_graph_tests;

#[cfg(test)]
#[path = "slider_query_tests.rs"]
mod slider_query_tests;

#[cfg(test)]
#[path = "workbook_migration_manifest_tests.rs"]
mod workbook_migration_manifest_tests;

#[cfg(test)]
#[path = "vba_project_metadata_tests.rs"]
mod vba_project_metadata_tests;

#[cfg(test)]
#[path = "workbook_tables_tests.rs"]
mod workbook_tables_tests;

#[cfg(test)]
#[path = "sheet_layout_metadata_tests.rs"]
mod sheet_layout_metadata_tests;
