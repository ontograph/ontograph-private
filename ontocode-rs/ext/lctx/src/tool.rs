use std::fs;
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

pub(crate) const LCTX_NAMESPACE: &str = "lctx";
pub(crate) const READ_TOOL_NAME: &str = "read";
const READ_TOOL_DESCRIPTION: &str = "Read one local file through the native lctx raw-read path without MCP or subprocess forwarding.";
const MAX_FILE_BYTES: usize = 256 * 1024;
pub(crate) const MAX_OUTPUT_BYTES: usize = 64 * 1024;
pub(crate) const RAW_ONLY_LIMIT_NOTE: &str = "only raw-read semantics are available in this stage";

#[derive(Clone, Default)]
pub(crate) struct LctxReadTool {
    thread_state: Arc<LctxThreadState>,
}

#[derive(Debug, Default)]
pub(crate) struct LctxThreadState {
    current_cwd: Mutex<Option<PathBuf>>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct LctxReadArgs {
    pub path: String,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub max_bytes: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct LctxReadResult {
    pub path: String,
    pub content: String,
    pub resolved_mode: String,
    pub limits: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for LctxReadTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(LCTX_NAMESPACE, READ_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema = serde_json::to_value(schemars::schema_for!(LctxReadArgs))
            .unwrap_or_else(|err| panic!("lctx.read args schema should serialize: {err}"));
        let output_schema = serde_json::to_value(schemars::schema_for!(LctxReadResult))
            .unwrap_or_else(|err| panic!("lctx.read result schema should serialize: {err}"));
        ToolSpec::Namespace(ResponsesApiNamespace {
            name: LCTX_NAMESPACE.to_string(),
            description: default_namespace_description(LCTX_NAMESPACE),
            tools: vec![ResponsesApiNamespaceTool::Function(ResponsesApiTool {
                name: READ_TOOL_NAME.to_string(),
                description: READ_TOOL_DESCRIPTION.to_string(),
                strict: false,
                defer_loading: None,
                parameters: parse_tool_input_schema(&input_schema)
                    .unwrap_or_else(|err| panic!("lctx.read args schema should parse: {err}")),
                output_schema: Some(output_schema),
            })],
        })
    }

    fn exposure(&self) -> ToolExposure {
        ToolExposure::DirectModelOnly
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    async fn handle(&self, call: ToolCall) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args = parse_tool_args::<LctxReadArgs>(&call)?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "lctx.read workspace context is unavailable for this turn".to_string(),
            )
        })?;
        validate_mode(args.mode.as_deref())?;
        let normalized_path = normalize_model_path(&args.path)?;
        let resolved_path = resolve_module_path(&normalized_path, &cwd)?;
        let max_bytes = args
            .max_bytes
            .unwrap_or(MAX_OUTPUT_BYTES)
            .clamp(1, MAX_OUTPUT_BYTES);
        let content = read_file_lossy(&resolved_path)
            .map_err(|err| FunctionCallError::RespondToModel(format!("lctx.read failed: {err}")))?;
        if content.len() > max_bytes {
            return Err(FunctionCallError::RespondToModel(format!(
                "lctx.read content exceeds requested max_bytes limit ({max_bytes} bytes)"
            )));
        }

        let result = LctxReadResult {
            path: normalized_path,
            content,
            resolved_mode: "raw".to_string(),
            limits: vec![
                format!("content output is capped at {MAX_OUTPUT_BYTES} bytes"),
                RAW_ONLY_LIMIT_NOTE.to_string(),
            ],
        };
        let value = serde_json::to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize lctx.read result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl LctxReadTool {
    pub(crate) fn new(thread_state: Arc<LctxThreadState>) -> Self {
        Self { thread_state }
    }
}

impl LctxThreadState {
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

fn parse_tool_args<T: for<'de> Deserialize<'de>>(call: &ToolCall) -> Result<T, FunctionCallError> {
    let arguments = call.function_arguments()?;
    serde_json::from_str(arguments).map_err(|err| {
        FunctionCallError::RespondToModel(format!("invalid lctx.read arguments: {err}"))
    })
}

fn validate_mode(mode: Option<&str>) -> Result<(), FunctionCallError> {
    let Some(mode) = mode.map(str::trim).filter(|mode| !mode.is_empty()) else {
        return Ok(());
    };
    if mode == "raw" {
        return Ok(());
    }
    Err(FunctionCallError::RespondToModel(
        "lctx.read supports only mode=\"raw\" in this stage".to_string(),
    ))
}

fn normalize_model_path(path: &str) -> Result<String, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "lctx.read path must not be empty".to_string(),
        ));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(
            "lctx.read path must be a local file path".to_string(),
        ));
    }

    let path = Path::new(trimmed);
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(segment) => normalized.push(segment),
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(FunctionCallError::RespondToModel(
                    "lctx.read path must be relative and stay within the current working directory"
                        .to_string(),
                ));
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "lctx.read path must point to a file".to_string(),
        ));
    }

    Ok(normalized.display().to_string())
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
                "lctx.read path must not traverse symlinks".to_string(),
            ));
        }
    }

    let metadata = fs::metadata(&resolved_path).map_err(|err| {
        FunctionCallError::RespondToModel(format!("lctx.read path does not exist: {err}"))
    })?;
    if !metadata.is_file() {
        return Err(FunctionCallError::RespondToModel(
            "lctx.read path must point to a file".to_string(),
        ));
    }

    Ok(resolved_path)
}

// Owner-local raw-read fallback after the direct lean-ctx crate dependency
// failed the workspace dependency gate.
fn read_file_lossy(path: &Path) -> Result<String, std::io::Error> {
    let file = open_with_retry(path)?;
    let metadata = file
        .metadata()
        .map_err(|err| std::io::Error::other(format!("cannot stat open file descriptor: {err}")))?;
    if metadata.len() > MAX_FILE_BYTES as u64 {
        return Err(std::io::Error::other(format!(
            "file too large ({} bytes, limit {} bytes)",
            metadata.len(),
            MAX_FILE_BYTES
        )));
    }

    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    std::io::BufReader::new(file).read_to_end(&mut bytes)?;
    if looks_binary(&bytes) {
        return Err(std::io::Error::other(
            "binary files are not supported in this stage",
        ));
    }
    match String::from_utf8(bytes) {
        Ok(content) => Ok(content),
        Err(err) => Ok(String::from_utf8_lossy(err.as_bytes()).into_owned()),
    }
}

fn looks_binary(bytes: &[u8]) -> bool {
    bytes.iter().take(1024).any(|byte| *byte == 0)
}

fn open_with_retry(path: &Path) -> Result<std::fs::File, std::io::Error> {
    match open_nofollow(path) {
        Ok(file) => Ok(file),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            std::thread::sleep(std::time::Duration::from_millis(50));
            open_nofollow(path).map_err(|retry_err| {
                if retry_err.kind() == std::io::ErrorKind::NotFound {
                    std::io::Error::other(format!("file not found: {}", path.display()))
                } else {
                    retry_err
                }
            })
        }
        Err(err) => Err(err),
    }
}

#[cfg(unix)]
fn open_nofollow(path: &Path) -> Result<std::fs::File, std::io::Error> {
    use std::os::unix::fs::OpenOptionsExt;

    if let (Some(parent), Some(filename)) = (path.parent(), path.file_name())
        && parent.exists()
    {
        let canonical_parent = fs::canonicalize(parent)?;
        let canonical_path = canonical_parent.join(filename);
        return std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(canonical_path);
    }

    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
}

#[cfg(not(unix))]
fn open_nofollow(path: &Path) -> Result<std::fs::File, std::io::Error> {
    std::fs::OpenOptions::new().read(true).open(path)
}
