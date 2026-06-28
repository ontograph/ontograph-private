use std::sync::Arc;

use ontocode_core::config::Config;
use ontocode_extension_api::ContextualUserFragment;
use ontocode_extension_api::ExtensionData;
use ontocode_extension_api::ExtensionRegistryBuilder;
use ontocode_extension_api::ToolCall;
use ontocode_extension_api::ToolContributor;
use ontocode_extension_api::ToolExecutor;
use ontocode_extension_api::TurnInputContext;
use ontocode_extension_api::TurnInputContributor;

use crate::formula_cte_pipeline::ExcelInspectFormulaCtePipelineTool;
use crate::formula_sql_readiness::ExcelInspectFormulaSqlReadinessTool;
use crate::named_range_rewrite::ExcelNamedRangeRewriteDryRunTool;
use crate::pivot_report_metadata::ExcelInspectPivotReportMetadataTool;
use crate::powerquery_extract::ExcelExtractPowerQueryQueriesTool;
use crate::powerquery_review_bundle::ExcelGeneratePowerQueryReviewBundleTool;
use crate::powerquery_translate::ExcelTranslatePowerQueryToSqlPreviewTool;
use crate::sheet_layout_metadata::ExcelInspectSheetLayoutMetadataTool;
use crate::slider_query::ExcelGenerateSliderQueryPackageTool;
use crate::slider_query::ExcelScanSheetFormulasDependencyTool;
use crate::tool::ExcelExportSheetToCsvTool;
use crate::tool::ExcelInspectSheetFormulasTool;
use crate::tool::ExcelInspectionTool;
use crate::tool::ExcelReadSheetPreviewTool;
use crate::tool::ExcelThreadState;
use crate::vba_extract::ExcelExtractVbaModulesTool;
use crate::vba_onlyoffice_analyze::ExcelAnalyzeVbaOnlyofficeMigrationTool;
use crate::vba_onlyoffice_translate::ExcelTranslateVbaToOnlyofficeJsPreviewTool;
use crate::vba_onlyoffice_workbook_review::ExcelReviewVbaOnlyofficeWorkbookTool;
use crate::vba_project_metadata::ExcelInspectVbaProjectMetadataTool;
use crate::vba_translate::ExcelTranslateVbaToMPreviewTool;
use crate::workbook_connections::ExcelInspectWorkbookConnectionsTool;
use crate::workbook_defined_names::ExcelInspectWorkbookDefinedNamesTool;
use crate::workbook_external_links::ExcelInspectWorkbookExternalLinksTool;
use crate::workbook_graph::ExcelInspectWorkbookGraphTool;
use crate::workbook_migration_manifest::ExcelGenerateWorkbookMigrationManifestTool;
use crate::workbook_tables::ExcelInspectWorkbookTablesTool;
use crate::workbook_used_ranges::ExcelInspectWorkbookUsedRangesTool;

#[derive(Clone, Default)]
struct ExcelExtension;

impl ToolContributor for ExcelExtension {
    fn tools(
        &self,
        _session_store: &ExtensionData,
        thread_store: &ExtensionData,
    ) -> Vec<Arc<dyn ToolExecutor<ToolCall>>> {
        let thread_state = thread_store.get_or_init(ExcelThreadState::default);
        vec![
            Arc::new(ExcelInspectionTool::new(thread_state.clone())),
            Arc::new(ExcelReadSheetPreviewTool::new(thread_state.clone())),
            Arc::new(ExcelInspectSheetFormulasTool::new(thread_state.clone())),
            Arc::new(ExcelInspectWorkbookDefinedNamesTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelInspectWorkbookExternalLinksTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelInspectWorkbookUsedRangesTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelInspectFormulaSqlReadinessTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelInspectFormulaCtePipelineTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelScanSheetFormulasDependencyTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelGenerateSliderQueryPackageTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelInspectPivotReportMetadataTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelInspectWorkbookConnectionsTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelInspectWorkbookTablesTool::new(thread_state.clone())),
            Arc::new(ExcelInspectSheetLayoutMetadataTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelInspectWorkbookGraphTool::new(thread_state.clone())),
            Arc::new(ExcelNamedRangeRewriteDryRunTool::new(thread_state.clone())),
            Arc::new(ExcelExportSheetToCsvTool::new(thread_state.clone())),
            Arc::new(ExcelExtractPowerQueryQueriesTool::new(thread_state.clone())),
            Arc::new(ExcelGeneratePowerQueryReviewBundleTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelGenerateWorkbookMigrationManifestTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelExtractVbaModulesTool::new(thread_state.clone())),
            Arc::new(ExcelInspectVbaProjectMetadataTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelTranslatePowerQueryToSqlPreviewTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelAnalyzeVbaOnlyofficeMigrationTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelTranslateVbaToOnlyofficeJsPreviewTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelReviewVbaOnlyofficeWorkbookTool::new(
                thread_state.clone(),
            )),
            Arc::new(ExcelTranslateVbaToMPreviewTool::new(thread_state)),
        ]
    }
}

#[async_trait::async_trait]
impl TurnInputContributor for ExcelExtension {
    async fn contribute(
        &self,
        input: TurnInputContext,
        _session_store: &ExtensionData,
        thread_store: &ExtensionData,
        _turn_store: &ExtensionData,
    ) -> Vec<Box<dyn ContextualUserFragment + Send>> {
        let Some(environment) = input
            .environments
            .iter()
            .find(|environment| environment.is_primary)
            .or_else(|| input.environments.first())
        else {
            return Vec::new();
        };

        thread_store
            .get_or_init(ExcelThreadState::default)
            .set_current_cwd(environment.cwd.clone());
        Vec::new()
    }
}

pub fn install(registry: &mut ExtensionRegistryBuilder<Config>) {
    let extension = Arc::new(ExcelExtension);
    registry.tool_contributor(extension.clone());
    registry.turn_input_contributor(extension);
}
