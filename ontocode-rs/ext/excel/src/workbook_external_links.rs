use std::collections::BTreeMap;
use std::fs::File;
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
use quick_xml::Reader;
use quick_xml::events::Event;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_value;
use zip::ZipArchive;

use crate::backend::ExcelInspectionError;
use crate::backend::inspect_workbook_with_display_path;
use crate::preview::attr_value;
use crate::preview::bounded_text;
use crate::preview::read_xml_entry;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::WorkbookFormat;
use crate::vba_extract::parse_tool_args;
use crate::vba_extract::resolve_workbook_path_from_model_arg;

pub(crate) const INSPECT_WORKBOOK_EXTERNAL_LINKS_TOOL_NAME: &str =
    "inspect_workbook_external_links";

const INSPECT_WORKBOOK_EXTERNAL_LINKS_DESCRIPTION: &str = "Inspect bounded offline workbook external-link metadata, including package part counts, workbook relationship ids and targets, and explicit unsupported-kind warnings.";
const MAX_EXTERNAL_LINKS: usize = 64;
const MAX_WORKBOOK_XML_BYTES: usize = 2 * 1024 * 1024;
const MAX_RELATIONSHIPS_XML_BYTES: usize = 2 * 1024 * 1024;
const MAX_PATH_CHARS: usize = 512;
const MAX_KIND_CHARS: usize = 64;
const MAX_WARNINGS: usize = 32;

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectWorkbookExternalLinksTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectWorkbookExternalLinksArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct WorkbookExternalLinkSummary {
    pub part_path: String,
    pub workbook_relationship_id: Option<String>,
    pub workbook_relationship_target: Option<String>,
    pub detail_kind: String,
    pub detail_relationship_id: Option<String>,
    pub detail_relationship_target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectWorkbookExternalLinksResult {
    pub mode: String,
    pub path: String,
    pub external_link_count: usize,
    pub external_links: Vec<WorkbookExternalLinkSummary>,
    pub inventory_truncated: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct Relationship {
    id: String,
    target: String,
    target_mode: Option<String>,
    relationship_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExternalLinkDetail {
    kind: String,
    relationship_id: Option<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectWorkbookExternalLinksTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_EXTERNAL_LINKS_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(InspectWorkbookExternalLinksArgs))
                .unwrap_or_else(|err| {
                    panic!("inspect_workbook_external_links args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(InspectWorkbookExternalLinksResult))
                .unwrap_or_else(|err| {
                    panic!("inspect_workbook_external_links result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_WORKBOOK_EXTERNAL_LINKS_TOOL_NAME.to_string(),
                    description: INSPECT_WORKBOOK_EXTERNAL_LINKS_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_workbook_external_links args schema should parse: {err}")
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
        let args = parse_tool_args::<InspectWorkbookExternalLinksArgs>(
            &call,
            "excel.inspect_workbook_external_links",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_workbook_external_links workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = resolve_workbook_path_from_model_arg(
            "excel.inspect_workbook_external_links",
            &args.path,
            &cwd,
        )?;
        let result = inspect_workbook_external_links_from_workbook(
            &workbook_path,
            Path::new(args.path.trim()),
        )
        .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize workbook external links: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectWorkbookExternalLinksTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn inspect_workbook_external_links_from_workbook(
    path: &Path,
    display_path: &Path,
) -> Result<InspectWorkbookExternalLinksResult, ExcelInspectionError> {
    let workbook = inspect_workbook_with_display_path(path, display_path)?;
    match workbook.format {
        WorkbookFormat::Xlsx | WorkbookFormat::Xlsm => {
            inspect_openxml_workbook_external_links(path, display_path)
        }
        WorkbookFormat::Xlsb => Err(ExcelInspectionError::Message(
            "excel.inspect_workbook_external_links supports only .xlsx and .xlsm in this stage; .xlsb external-link inventory remains unsupported"
                .to_string(),
        )),
        WorkbookFormat::Unknown => Err(ExcelInspectionError::Message(
            "excel.inspect_workbook_external_links supports only .xlsx and .xlsm in this stage"
                .to_string(),
        )),
    }
}

fn inspect_openxml_workbook_external_links(
    path: &Path,
    display_path: &Path,
) -> Result<InspectWorkbookExternalLinksResult, ExcelInspectionError> {
    let file = File::open(path).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to open workbook {}: {err}", path.display()))
    })?;
    let mut archive = ZipArchive::new(file).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to read workbook archive {}: {err}",
            path.display()
        ))
    })?;

    let mut external_link_parts = Vec::new();
    for index in 0..archive.len() {
        let entry = archive.by_index(index).map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to read workbook entry {index} in {}: {err}",
                path.display()
            ))
        })?;
        let part_path = normalize_part_name(entry.name());
        if part_path.starts_with("xl/externalLinks/") && part_path.ends_with(".xml") {
            external_link_parts.push(part_path);
        }
    }
    external_link_parts.sort();

    let workbook_relationships = read_workbook_external_link_relationships(&mut archive)?;
    let workbook_relationships_by_target = workbook_relationships
        .into_iter()
        .map(|relationship| {
            (
                normalize_part_path("xl/workbook.xml", &relationship.target),
                relationship,
            )
        })
        .collect::<BTreeMap<_, _>>();

    let external_link_count = external_link_parts.len();
    let inventory_truncated = external_link_count > MAX_EXTERNAL_LINKS;
    let mut warnings = Vec::new();
    if inventory_truncated {
        push_warning(
            &mut warnings,
            format!(
                "external-link inventory truncated to {MAX_EXTERNAL_LINKS} of {external_link_count} parts"
            ),
        );
    }

    let external_links = external_link_parts
        .iter()
        .take(MAX_EXTERNAL_LINKS)
        .map(|part_path| {
            let detail = read_external_link_detail(&mut archive, part_path)?;
            let detail_relationship_target =
                read_external_link_relationship_target(&mut archive, part_path, &detail)?;
            let workbook_relationship = workbook_relationships_by_target.get(part_path);
            if workbook_relationship.is_none() {
                push_warning(
                    &mut warnings,
                    format!(
                        "external-link part `{part_path}` has no matching workbook relationship entry"
                    ),
                );
            }
            if detail.kind != "external_book" {
                push_warning(
                    &mut warnings,
                    format!(
                        "external-link part `{part_path}` uses unsupported detail kind `{}`; only bounded metadata is reported",
                        detail.kind
                    ),
                );
            }
            if detail.relationship_id.is_some() && detail_relationship_target.is_none() {
                push_warning(
                    &mut warnings,
                    format!(
                        "external-link part `{part_path}` references a nested relationship id but no target could be resolved"
                    ),
                );
            }
            Ok(WorkbookExternalLinkSummary {
                part_path: bounded_text(part_path, MAX_PATH_CHARS),
                workbook_relationship_id: workbook_relationship
                    .map(|relationship| bounded_text(&relationship.id, MAX_PATH_CHARS)),
                workbook_relationship_target: workbook_relationship.map(|relationship| {
                    bounded_text(
                        &normalize_relationship_target("xl/workbook.xml", relationship),
                        MAX_PATH_CHARS,
                    )
                }),
                detail_kind: bounded_text(&detail.kind, MAX_KIND_CHARS),
                detail_relationship_id: detail
                    .relationship_id
                    .as_deref()
                    .map(|value| bounded_text(value, MAX_PATH_CHARS)),
                detail_relationship_target: detail_relationship_target
                    .as_deref()
                    .map(|value| bounded_text(value, MAX_PATH_CHARS)),
            })
        })
        .collect::<Result<Vec<_>, ExcelInspectionError>>()?;

    Ok(InspectWorkbookExternalLinksResult {
        mode: "read_only_inspection".to_string(),
        path: display_path.display().to_string(),
        external_link_count,
        external_links,
        inventory_truncated,
        warnings,
    })
}

fn read_workbook_external_link_relationships(
    archive: &mut ZipArchive<File>,
) -> Result<Vec<Relationship>, ExcelInspectionError> {
    let workbook_rels = read_xml_entry(
        archive,
        "xl/_rels/workbook.xml.rels",
        MAX_RELATIONSHIPS_XML_BYTES,
    )?;
    parse_relationships(&workbook_rels).map(|relationships| {
        relationships
            .into_iter()
            .filter(|relationship| relationship.relationship_type.contains("/externalLink"))
            .collect()
    })
}

fn read_external_link_detail(
    archive: &mut ZipArchive<File>,
    part_path: &str,
) -> Result<ExternalLinkDetail, ExcelInspectionError> {
    let xml = read_xml_entry(archive, part_path, MAX_WORKBOOK_XML_BYTES)?;
    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) | Ok(Event::Empty(event)) => match event.name().as_ref() {
                b"externalBook" => {
                    return Ok(ExternalLinkDetail {
                        kind: "external_book".to_string(),
                        relationship_id: attr_value(&event, b"r:id")?
                            .or_else(|| attr_value(&event, b"id").ok().flatten()),
                    });
                }
                b"ddeLink" => {
                    return Ok(ExternalLinkDetail {
                        kind: "dde_link".to_string(),
                        relationship_id: None,
                    });
                }
                b"oleLink" => {
                    return Ok(ExternalLinkDetail {
                        kind: "ole_link".to_string(),
                        relationship_id: None,
                    });
                }
                _ => {}
            },
            Ok(Event::Eof) => {
                return Ok(ExternalLinkDetail {
                    kind: "unknown".to_string(),
                    relationship_id: None,
                });
            }
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse external-link part {part_path}: {err}"
                )));
            }
        }
        buf.clear();
    }
}

fn read_external_link_relationship_target(
    archive: &mut ZipArchive<File>,
    part_path: &str,
    detail: &ExternalLinkDetail,
) -> Result<Option<String>, ExcelInspectionError> {
    let Some(detail_relationship_id) = detail.relationship_id.as_deref() else {
        return Ok(None);
    };
    let rels_path = external_link_rels_path(part_path);
    let rels_xml = match read_xml_entry(archive, &rels_path, MAX_RELATIONSHIPS_XML_BYTES) {
        Ok(value) => value,
        Err(ExcelInspectionError::Message(_)) => return Ok(None),
    };
    let relationships = parse_relationships(&rels_xml)?;
    Ok(relationships
        .into_iter()
        .find(|relationship| relationship.id == detail_relationship_id)
        .map(|relationship| normalize_relationship_target(part_path, &relationship)))
}

fn parse_relationships(xml: &str) -> Result<Vec<Relationship>, ExcelInspectionError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut relationships = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"Relationship" =>
            {
                relationships.push(Relationship {
                    id: attr_value(&event, b"Id")?.unwrap_or_default(),
                    target: attr_value(&event, b"Target")?.unwrap_or_default(),
                    target_mode: attr_value(&event, b"TargetMode")?,
                    relationship_type: attr_value(&event, b"Type")?.unwrap_or_default(),
                });
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse workbook relationships: {err}"
                )));
            }
        }
        buf.clear();
    }
    Ok(relationships)
}

fn external_link_rels_path(part_path: &str) -> String {
    let mut rels_path = PathBuf::from(part_path);
    let file_name = rels_path
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_default();
    rels_path.pop();
    rels_path.push("_rels");
    rels_path.push(format!("{file_name}.rels"));
    normalize_path(rels_path)
}

fn normalize_relationship_target(base_part: &str, relationship: &Relationship) -> String {
    if relationship
        .target_mode
        .as_deref()
        .is_some_and(|value| value.eq_ignore_ascii_case("External"))
        || relationship.target.contains("://")
        || relationship.target.starts_with("file:")
    {
        return relationship.target.clone();
    }
    normalize_part_path(base_part, &relationship.target)
}

fn normalize_part_name(name: &str) -> String {
    name.trim_start_matches("./").replace('\\', "/")
}

fn normalize_part_path(base_part: &str, target: &str) -> String {
    let mut base = PathBuf::from(base_part);
    let _ = base.pop();
    let mut path = if target.starts_with('/') {
        PathBuf::new()
    } else {
        base
    };
    path.push(target);
    normalize_path(path)
}

fn normalize_path(path: PathBuf) -> String {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            std::path::Component::CurDir => {}
            std::path::Component::Normal(segment) => normalized.push(segment),
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {
                normalized.push(component.as_os_str())
            }
        }
    }
    normalized.to_string_lossy().replace('\\', "/")
}

fn push_warning(warnings: &mut Vec<String>, warning: String) {
    if warnings.len() < MAX_WARNINGS && !warnings.iter().any(|item| item == &warning) {
        warnings.push(warning);
    }
}
