use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use ontocode_extension_api::FunctionCallError;
use ontocode_extension_api::JsonToolOutput;
use ontocode_extension_api::ToolCall;
use ontocode_extension_api::ToolExecutor;
use ontocode_extension_api::ToolName;
use ontocode_extension_api::ToolOutput;
use ontocode_extension_api::ToolSpec;
use ontocode_extension_api::parse_tool_input_schema;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_value;

use crate::powerquery_extract::ExtractPowerQueryQueriesResult;
use crate::powerquery_extract::PowerQueryLexicalReference;
use crate::powerquery_extract::PowerQueryLintFinding;
use crate::powerquery_extract::extract_powerquery_queries_from_workbook;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;

pub(crate) const GENERATE_POWERQUERY_REVIEW_BUNDLE_TOOL_NAME: &str =
    "generate_powerquery_review_bundle";

const GENERATE_POWERQUERY_REVIEW_BUNDLE_DESCRIPTION: &str = "Generate a deterministic offline Power Query review bundle with extracted queries, lint summary, lineage summary, and a manifest.";

#[derive(Clone, Default)]
pub(crate) struct ExcelGeneratePowerQueryReviewBundleTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct GeneratePowerQueryReviewBundleArgs {
    pub path: String,
    pub output_bundle_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PowerQueryBundleNormalizationStatus {
    CopyArtifacts,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PowerQueryReviewBundleQuerySummary {
    pub name: String,
    pub source_path: String,
    pub normalized_source_path: Option<String>,
    pub lint_finding_count: usize,
    pub lineage_reference_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PowerQueryReviewBundleManifest {
    pub bundle_name: String,
    pub query_count: usize,
    pub queries: Vec<PowerQueryReviewBundleQuerySummary>,
    pub lint_summary_path: String,
    pub lineage_summary_path: String,
    pub normalization_status: PowerQueryBundleNormalizationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeneratePowerQueryReviewBundleResult {
    pub path: String,
    pub bundle_path: String,
    pub manifest_path: String,
    pub manifest: PowerQueryReviewBundleManifest,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PowerQueryReviewBundleLintEntry {
    name: String,
    findings: Vec<PowerQueryLintFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PowerQueryReviewBundleLineageEntry {
    name: String,
    references: Vec<PowerQueryLexicalReference>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelGeneratePowerQueryReviewBundleTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, GENERATE_POWERQUERY_REVIEW_BUNDLE_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(GeneratePowerQueryReviewBundleArgs))
                .unwrap_or_else(|err| {
                    panic!("generate_powerquery_review_bundle args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(GeneratePowerQueryReviewBundleResult))
                .unwrap_or_else(|err| {
                    panic!(
                        "generate_powerquery_review_bundle result schema should serialize: {err}"
                    )
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: GENERATE_POWERQUERY_REVIEW_BUNDLE_TOOL_NAME.to_string(),
                    description: GENERATE_POWERQUERY_REVIEW_BUNDLE_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("generate_powerquery_review_bundle args schema should parse: {err}")
                    }),
                    output_schema: Some(output_schema),
                },
            )],
        })
    }

    fn exposure(&self) -> ontocode_tools::ToolExposure {
        ontocode_tools::ToolExposure::DirectModelOnly
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    async fn handle(&self, call: ToolCall) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args = parse_tool_args::<GeneratePowerQueryReviewBundleArgs>(
            &call,
            "excel.generate_powerquery_review_bundle",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.generate_powerquery_review_bundle workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let output_bundle_path = bundle_output_path_from_model_arg(&args.output_bundle_path, &cwd)?;
        let result = generate_powerquery_review_bundle_with_display_path(
            &workbook_path,
            Path::new(args.path.trim()),
            &output_bundle_path,
            Path::new(args.output_bundle_path.trim()),
        )
        .map_err(FunctionCallError::RespondToModel)?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize Power Query review bundle result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelGeneratePowerQueryReviewBundleTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn generate_powerquery_review_bundle_with_display_path(
    path: &Path,
    display_path: &Path,
    output_bundle_path: &Path,
    output_bundle_display_path: &Path,
) -> Result<GeneratePowerQueryReviewBundleResult, String> {
    let extraction = extract_powerquery_queries_from_workbook(path, display_path);
    let manifest = write_powerquery_review_bundle(
        output_bundle_path,
        &extraction,
        output_bundle_display_path,
    )?;
    Ok(GeneratePowerQueryReviewBundleResult {
        path: display_path.display().to_string(),
        bundle_path: output_bundle_display_path.display().to_string(),
        manifest_path: output_bundle_display_path
            .join("manifest.json")
            .display()
            .to_string(),
        manifest,
        warnings: extraction.warnings,
    })
}

fn write_powerquery_review_bundle(
    output_bundle_path: &Path,
    extraction: &ExtractPowerQueryQueriesResult,
    output_bundle_display_path: &Path,
) -> Result<PowerQueryReviewBundleManifest, String> {
    std::fs::create_dir_all(output_bundle_path).map_err(|err| {
        format!(
            "failed to create Power Query review bundle directory {}: {err}",
            output_bundle_path.display()
        )
    })?;
    let queries_dir = output_bundle_path.join("queries");
    let normalized_dir = output_bundle_path.join("normalized_m");
    let reports_dir = output_bundle_path.join("reports");
    std::fs::create_dir_all(&queries_dir).map_err(|err| {
        format!(
            "failed to create Power Query review bundle queries directory {}: {err}",
            queries_dir.display()
        )
    })?;
    std::fs::create_dir_all(&normalized_dir).map_err(|err| {
        format!(
            "failed to create Power Query review bundle normalized queries directory {}: {err}",
            normalized_dir.display()
        )
    })?;
    std::fs::create_dir_all(&reports_dir).map_err(|err| {
        format!(
            "failed to create Power Query review bundle reports directory {}: {err}",
            reports_dir.display()
        )
    })?;

    let mut query_summaries = Vec::new();
    let mut lint_entries = Vec::new();
    let mut lineage_entries = Vec::new();
    for (index, query) in extraction.queries.iter().enumerate() {
        let query_slug = format!("{:02}_{}", index + 1, slugify(&query.name));
        let query_path = queries_dir.join(format!("{query_slug}.m"));
        let normalized_query_path = normalized_dir.join(format!("{query_slug}.m"));
        std::fs::write(&query_path, &query.source).map_err(|err| {
            format!(
                "failed to write Power Query source file {}: {err}",
                query_path.display()
            )
        })?;
        let normalized_source = normalize_powerquery_source(&query.source);
        std::fs::write(&normalized_query_path, normalized_source).map_err(|err| {
            format!(
                "failed to write normalized Power Query source file {}: {err}",
                normalized_query_path.display()
            )
        })?;
        query_summaries.push(PowerQueryReviewBundleQuerySummary {
            name: query.name.clone(),
            source_path: relative_path(output_bundle_path, &query_path),
            normalized_source_path: Some(relative_path(output_bundle_path, &normalized_query_path)),
            lint_finding_count: query.lint_findings.len(),
            lineage_reference_count: query.lexical_references.len(),
        });
        lint_entries.push(PowerQueryReviewBundleLintEntry {
            name: query.name.clone(),
            findings: query.lint_findings.clone(),
        });
        lineage_entries.push(PowerQueryReviewBundleLineageEntry {
            name: query.name.clone(),
            references: query.lexical_references.clone(),
        });
    }

    let lint_summary_path = reports_dir.join("lint-summary.json");
    std::fs::write(
        &lint_summary_path,
        serde_json::to_string_pretty(&lint_entries)
            .map_err(|err| format!("failed to serialize Power Query lint summary: {err}"))?,
    )
    .map_err(|err| {
        format!(
            "failed to write Power Query lint summary {}: {err}",
            lint_summary_path.display()
        )
    })?;

    let lineage_summary_path = reports_dir.join("lineage-summary.json");
    std::fs::write(
        &lineage_summary_path,
        serde_json::to_string_pretty(&lineage_entries)
            .map_err(|err| format!("failed to serialize Power Query lineage summary: {err}"))?,
    )
    .map_err(|err| {
        format!(
            "failed to write Power Query lineage summary {}: {err}",
            lineage_summary_path.display()
        )
    })?;

    let bundle_name = format!(
        "{}_powerquery_review_bundle",
        slugify(
            output_bundle_display_path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("powerquery")
        )
    );
    let manifest = PowerQueryReviewBundleManifest {
        bundle_name,
        query_count: extraction.query_count,
        queries: query_summaries,
        lint_summary_path: relative_path(output_bundle_path, &lint_summary_path),
        lineage_summary_path: relative_path(output_bundle_path, &lineage_summary_path),
        normalization_status: PowerQueryBundleNormalizationStatus::CopyArtifacts,
    };
    let manifest_path = output_bundle_path.join("manifest.json");
    std::fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest)
            .map_err(|err| format!("failed to serialize Power Query manifest: {err}"))?,
    )
    .map_err(|err| {
        format!(
            "failed to write Power Query manifest {}: {err}",
            manifest_path.display()
        )
    })?;

    Ok(manifest)
}

fn normalize_powerquery_source(source: &str) -> String {
    let normalized_line_endings = source.replace("\r\n", "\n").replace('\r', "\n");
    let lines = normalized_line_endings.split('\n').collect::<Vec<_>>();
    let mut last_content_line = lines.len();
    while last_content_line > 0 && lines[last_content_line - 1].trim().is_empty() {
        last_content_line -= 1;
    }

    if last_content_line == 0 {
        return "\n".to_string();
    }

    let mut normalized = String::new();
    for line in lines.iter().take(last_content_line) {
        normalized.push_str(line.trim_end());
        normalized.push('\n');
    }
    normalized
}

fn relative_path(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf())
        .display()
        .to_string()
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_separator = false;
    for ch in value.chars() {
        let mapped = if ch.is_ascii_alphanumeric() {
            last_was_separator = false;
            Some(ch.to_ascii_lowercase())
        } else if last_was_separator {
            None
        } else {
            last_was_separator = true;
            Some('_')
        };
        if let Some(ch) = mapped {
            slug.push(ch);
        }
    }
    let slug = slug.trim_matches('_');
    if slug.is_empty() {
        "query".to_string()
    } else {
        slug.to_string()
    }
}

fn workbook_path_from_model_arg(path: &str, cwd: &Path) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_powerquery_review_bundle path must not be empty".to_string(),
        ));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_powerquery_review_bundle path must be a local workbook path"
                .to_string(),
        ));
    }

    let path = Path::new(trimmed);
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_powerquery_review_bundle path must be relative and stay within the current working directory"
                .to_string(),
        ));
    }

    let Some(extension) = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
    else {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_powerquery_review_bundle path must end in .xlsx, .xlsm, or .xlsb"
                .to_string(),
        ));
    };
    if !matches!(extension.as_str(), "xlsx" | "xlsm" | "xlsb") {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_powerquery_review_bundle path must end in .xlsx, .xlsm, or .xlsb"
                .to_string(),
        ));
    }

    let resolved_path = cwd.join(path);
    let mut scoped_path = cwd.to_path_buf();
    for component in path.components() {
        let Component::Normal(segment) = component else {
            continue;
        };
        scoped_path.push(segment);
        let Ok(metadata) = std::fs::symlink_metadata(&scoped_path) else {
            break;
        };
        if metadata.file_type().is_symlink() {
            return Err(FunctionCallError::RespondToModel(
                "excel.generate_powerquery_review_bundle path must not traverse symlinks"
                    .to_string(),
            ));
        }
    }

    Ok(resolved_path)
}

fn bundle_output_path_from_model_arg(path: &str, cwd: &Path) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_powerquery_review_bundle output_bundle_path must not be empty"
                .to_string(),
        ));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_powerquery_review_bundle output_bundle_path must be a local path"
                .to_string(),
        ));
    }

    let path = Path::new(trimmed);
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_powerquery_review_bundle output_bundle_path must be relative and stay within the current working directory"
                .to_string(),
        ));
    }

    let resolved_path = cwd.join(path);
    let mut scoped_path = cwd.to_path_buf();
    for component in path.components() {
        let Component::Normal(segment) = component else {
            continue;
        };
        scoped_path.push(segment);
        let Ok(metadata) = std::fs::symlink_metadata(&scoped_path) else {
            break;
        };
        if metadata.file_type().is_symlink() {
            return Err(FunctionCallError::RespondToModel(
                "excel.generate_powerquery_review_bundle output_bundle_path must not traverse symlinks"
                    .to_string(),
            ));
        }
    }

    Ok(resolved_path)
}

fn parse_tool_args<T: serde::de::DeserializeOwned>(
    call: &ToolCall,
    tool_name: &str,
) -> Result<T, FunctionCallError> {
    let arguments = call.function_arguments()?;
    serde_json::from_str(arguments).map_err(|err| {
        FunctionCallError::RespondToModel(format!("invalid {tool_name} arguments: {err}"))
    })
}
