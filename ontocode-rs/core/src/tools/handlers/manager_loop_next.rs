use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::handlers::manager_loop_next_spec::MANAGER_LOOP_NEXT_TOOL_NAME;
use crate::tools::handlers::manager_loop_next_spec::create_manager_loop_next_tool;
use crate::tools::handlers::parse_arguments;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use ontocode_tools::JsonToolOutput;
use ontocode_tools::ToolName;
use ontocode_tools::ToolSpec;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Component;
use std::path::Path;

const MAX_TRACKING_FILE_BYTES: u64 = 128 * 1024;
const MAX_OUTPUT_STRING_CHARS: usize = 4096;
const MAX_REQUIRED_ROLES: usize = 16;
const MAX_REQUIRED_ROLE_NAME_CHARS: usize = 128;

pub struct ManagerLoopNextHandler;

#[derive(Debug, Deserialize)]
struct ManagerLoopNextArgs {
    tracking_path: String,
    mode: ManagerLoopMode,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum ManagerLoopMode {
    Strict,
}

#[derive(Debug)]
struct ManagerLoopBlock {
    authority: bool,
    active_next_task: Option<String>,
    last_decision: Option<LastDecision>,
    reopen_gate: Option<String>,
    required_roles: Vec<RequiredRoleOutput>,
    tasks: Vec<ManagerLoopTask>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum LastDecision {
    Dispatch,
    NoDispatch,
}

#[derive(Debug)]
struct ManagerLoopTask {
    id: String,
    status: TaskStatus,
    classification: TaskClassification,
    depends_on: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum TaskStatus {
    Open,
    Closed,
    Blocked,
}

#[derive(Debug, PartialEq, Eq)]
enum TaskClassification {
    ImplementationReady,
    DocsDesignOnly,
    ProofOnly,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ManagerLoopDecision {
    ExecuteActiveNextTask,
    PromoteNextOpen,
    NoDispatch,
    Complete,
    InvalidTracking,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct ManagerLoopNextResult {
    decision: ManagerLoopDecision,
    task_id: Option<String>,
    reason: String,
    required_roles: Vec<RequiredRoleOutput>,
    reopen_gate: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct RequiredRoleOutput {
    role: String,
    required: bool,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolInvocation> for ManagerLoopNextHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(MANAGER_LOOP_NEXT_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        create_manager_loop_next_tool()
    }

    async fn handle(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ontocode_tools::ToolOutput>, FunctionCallError> {
        let ToolInvocation { turn, payload, .. } = invocation;
        let arguments = match payload {
            ontocode_tools::ToolPayload::Function { arguments } => arguments,
            _ => {
                return Err(FunctionCallError::RespondToModel(
                    "manager_loop_next handler received unsupported payload".to_string(),
                ));
            }
        };

        let args: ManagerLoopNextArgs = parse_arguments(&arguments)?;
        let result = run_manager_loop_next(turn.as_ref(), args).await;

        Ok(Box::new(JsonToolOutput::new(
            serde_json::to_value(result).unwrap_or_else(|err| {
                serde_json::Value::String(format!(
                    "failed to serialize manager_loop_next result: {err}"
                ))
            }),
        )))
    }
}

impl CoreToolRuntime for ManagerLoopNextHandler {}

async fn run_manager_loop_next(
    turn: &crate::session::turn_context::TurnContext,
    args: ManagerLoopNextArgs,
) -> ManagerLoopNextResult {
    if args.mode != ManagerLoopMode::Strict {
        return invalid_tracking("strict mode is required".to_string());
    }

    let Some(cwd) = turn
        .environments
        .primary()
        .map(|environment| environment.cwd.as_path())
    else {
        return invalid_tracking(
            "manager_loop_next requires a selected turn environment".to_string(),
        );
    };

    let tracking_path = match resolve_tracking_path(cwd, &args.tracking_path) {
        Ok(path) => path,
        Err(reason) => return invalid_tracking(reason),
    };
    match tokio::fs::metadata(&tracking_path).await {
        Ok(metadata) if metadata.len() > MAX_TRACKING_FILE_BYTES => {
            return invalid_tracking("tracking file exceeds maximum size".to_string());
        }
        Ok(_) => {}
        Err(err) => {
            return invalid_tracking(format!(
                "failed to read tracking file `{}`: {err}",
                args.tracking_path
            ));
        }
    }
    let contents = match tokio::fs::read_to_string(&tracking_path).await {
        Ok(contents) => contents,
        Err(err) => {
            return invalid_tracking(format!(
                "failed to read tracking file `{}`: {err}",
                args.tracking_path
            ));
        }
    };
    if contents.len() as u64 > MAX_TRACKING_FILE_BYTES {
        return invalid_tracking("tracking file exceeds maximum size".to_string());
    }
    let manager_loop = match parse_manager_loop_block(&contents) {
        Ok(manager_loop) => manager_loop,
        Err(reason) => return invalid_tracking(reason),
    };

    evaluate_manager_loop(manager_loop)
}

fn resolve_tracking_path(cwd: &Path, tracking_path: &str) -> Result<std::path::PathBuf, String> {
    let path = Path::new(tracking_path);
    if path.is_absolute() {
        return Err("tracking_path must be workspace-relative".to_string());
    }
    if path.extension() != Some(OsStr::new("md")) {
        return Err("tracking_path must point to a Markdown file".to_string());
    }

    let mut components = path.components();
    let Some(Component::Normal(first)) = components.next() else {
        return Err("tracking_path must stay under `.memory-bank/`".to_string());
    };
    if first != OsStr::new(".memory-bank") {
        return Err("tracking_path must stay under `.memory-bank/`".to_string());
    }

    if path
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err("tracking_path must not contain traversal segments".to_string());
    }
    if path.file_name().is_none() {
        return Err("tracking_path must point to a file".to_string());
    }

    Ok(cwd.join(path))
}

fn parse_manager_loop_block(contents: &str) -> Result<ManagerLoopBlock, String> {
    let block = extract_manager_loop_yaml_block(contents)?;
    parse_manager_loop_yaml(&block)
}

fn evaluate_manager_loop(manager_loop: ManagerLoopBlock) -> ManagerLoopNextResult {
    if !manager_loop.authority {
        return invalid_tracking("manager_loop block must set `authority: true`".to_string());
    }

    let required_roles = manager_loop.required_roles;
    let tasks_by_id = match task_index(&manager_loop.tasks) {
        Ok(tasks_by_id) => tasks_by_id,
        Err(reason) => return invalid_tracking(reason),
    };

    if let Some(active_next_task) = manager_loop.active_next_task.as_deref() {
        let Some(task) = tasks_by_id.get(active_next_task) else {
            return invalid_tracking(format!(
                "active_next_task `{active_next_task}` does not match any task"
            ));
        };
        if !is_open(task) {
            return invalid_tracking(format!(
                "active_next_task `{active_next_task}` must reference an OPEN task"
            ));
        }
        if task.classification != TaskClassification::ImplementationReady {
            return invalid_tracking(format!(
                "active_next_task `{active_next_task}` must be implementation-ready"
            ));
        }
        if let Err(reason) = ensure_dependencies_ready(task, &tasks_by_id) {
            return invalid_tracking(reason);
        }

        return ManagerLoopNextResult {
            decision: ManagerLoopDecision::ExecuteActiveNextTask,
            task_id: Some(active_next_task.to_string()),
            reason: "active_next_task is set and classified implementation-ready".to_string(),
            required_roles,
            reopen_gate: None,
        };
    }

    if manager_loop.last_decision == Some(LastDecision::NoDispatch) {
        let Some(reopen_gate) = manager_loop
            .reopen_gate
            .filter(|gate| !gate.trim().is_empty())
        else {
            return invalid_tracking(
                "last_decision `no-dispatch` requires an exact reopen_gate".to_string(),
            );
        };
        if reopen_gate.chars().count() > MAX_OUTPUT_STRING_CHARS {
            return invalid_tracking("reopen_gate exceeds maximum length".to_string());
        }

        return ManagerLoopNextResult {
            decision: ManagerLoopDecision::NoDispatch,
            task_id: None,
            reason: "last_decision is no-dispatch".to_string(),
            required_roles,
            reopen_gate: Some(reopen_gate),
        };
    }

    for task in &manager_loop.tasks {
        if !is_open(task) || task.classification != TaskClassification::ImplementationReady {
            continue;
        }
        if let Err(reason) = ensure_dependencies_ready(task, &tasks_by_id) {
            if reason.contains("does not match any task") {
                return invalid_tracking(reason);
            }
            continue;
        }

        return ManagerLoopNextResult {
            decision: ManagerLoopDecision::PromoteNextOpen,
            task_id: Some(task.id.clone()),
            reason: "first dependency-ready OPEN task with classification implementation-ready"
                .to_string(),
            required_roles,
            reopen_gate: None,
        };
    }

    ManagerLoopNextResult {
        decision: ManagerLoopDecision::Complete,
        task_id: None,
        reason: "nothing left in scope".to_string(),
        required_roles,
        reopen_gate: None,
    }
}

fn task_index(tasks: &[ManagerLoopTask]) -> Result<BTreeMap<&str, &ManagerLoopTask>, String> {
    let mut by_id = BTreeMap::new();
    for task in tasks {
        if task.id.trim().is_empty() {
            return Err("manager_loop tasks require non-empty ids".to_string());
        }
        if by_id.insert(task.id.as_str(), task).is_some() {
            return Err(format!("duplicate manager_loop task id `{}`", task.id));
        }
    }
    Ok(by_id)
}

fn ensure_dependencies_ready(
    task: &ManagerLoopTask,
    tasks_by_id: &BTreeMap<&str, &ManagerLoopTask>,
) -> Result<(), String> {
    for dependency in &task.depends_on {
        let Some(required_task) = tasks_by_id.get(dependency.as_str()) else {
            return Err(format!(
                "task `{}` depends on `{dependency}`, which does not match any task",
                task.id
            ));
        };
        if required_task.status != TaskStatus::Closed {
            return Err(format!(
                "task `{}` is not dependency-ready because `{dependency}` is `{}`",
                task.id,
                task_status_label(&required_task.status)
            ));
        }
    }
    Ok(())
}

fn is_open(task: &ManagerLoopTask) -> bool {
    task.status == TaskStatus::Open
}

fn task_status_label(status: &TaskStatus) -> &'static str {
    match status {
        TaskStatus::Open => "OPEN",
        TaskStatus::Closed => "CLOSED",
        TaskStatus::Blocked => "BLOCKED",
    }
}

fn invalid_tracking(reason: String) -> ManagerLoopNextResult {
    ManagerLoopNextResult {
        decision: ManagerLoopDecision::InvalidTracking,
        task_id: None,
        reason: truncate_chars(reason, MAX_OUTPUT_STRING_CHARS),
        required_roles: Vec::new(),
        reopen_gate: None,
    }
}

fn extract_manager_loop_yaml_block(contents: &str) -> Result<String, String> {
    let mut inside_yaml_block = false;
    let mut block = String::new();

    for line in contents.lines() {
        let trimmed = line.trim();
        if !inside_yaml_block {
            if trimmed == "```yaml" || trimmed == "```yml" {
                inside_yaml_block = true;
                block.clear();
            }
            continue;
        }

        if trimmed == "```" {
            if block.contains("manager_loop:") {
                return Ok(block);
            }
            inside_yaml_block = false;
            block.clear();
            continue;
        }

        block.push_str(line);
        block.push('\n');
    }

    Err("missing strict fenced `manager_loop` YAML block".to_string())
}

fn parse_manager_loop_yaml(block: &str) -> Result<ManagerLoopBlock, String> {
    let lines = block.lines().collect::<Vec<_>>();
    let mut index = 0;
    skip_blank_lines(&lines, &mut index);
    let Some(line) = lines.get(index) else {
        return Err("manager_loop block is empty".to_string());
    };
    if line.trim() != "manager_loop:" {
        return Err("manager_loop block must start with `manager_loop:`".to_string());
    }
    index += 1;

    let mut manager_loop = ManagerLoopBlock {
        authority: false,
        active_next_task: None,
        last_decision: None,
        reopen_gate: None,
        required_roles: Vec::new(),
        tasks: Vec::new(),
    };

    while index < lines.len() {
        let line = lines[index];
        if line.trim().is_empty() {
            index += 1;
            continue;
        }

        let (indent, trimmed) = indent_and_trim(line);
        if indent != 2 {
            return Err(format!(
                "manager_loop top-level entries must use 2-space indentation: `{trimmed}`"
            ));
        }

        if let Some(rest) = trimmed.strip_prefix("authority:") {
            manager_loop.authority = parse_bool(rest.trim(), "authority")?;
            index += 1;
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("active_next_task:") {
            manager_loop.active_next_task = parse_nullable_string(rest.trim());
            index += 1;
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("last_decision:") {
            manager_loop.last_decision = Some(parse_last_decision(rest.trim())?);
            index += 1;
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("reopen_gate:") {
            manager_loop.reopen_gate = parse_nullable_string(rest.trim());
            index += 1;
            continue;
        }
        if trimmed == "required_roles:" {
            manager_loop.required_roles = parse_required_roles(&lines, &mut index)?;
            continue;
        }
        if trimmed == "required_roles: {}" {
            manager_loop.required_roles = Vec::new();
            index += 1;
            continue;
        }
        if trimmed == "tasks:" {
            manager_loop.tasks = parse_tasks(&lines, &mut index)?;
            continue;
        }
        if trimmed == "tasks: []" {
            manager_loop.tasks = Vec::new();
            index += 1;
            continue;
        }

        return Err(format!("unsupported manager_loop field `{trimmed}`"));
    }

    Ok(manager_loop)
}

fn parse_required_roles(
    lines: &[&str],
    index: &mut usize,
) -> Result<Vec<RequiredRoleOutput>, String> {
    *index += 1;
    skip_blank_lines(lines, index);
    if matches!(lines.get(*index), Some(line) if line.trim() == "{}") {
        *index += 1;
        return Ok(Vec::new());
    }

    let mut roles = Vec::new();
    while *index < lines.len() {
        let line = lines[*index];
        if line.trim().is_empty() {
            *index += 1;
            continue;
        }
        let (indent, trimmed) = indent_and_trim(line);
        if indent <= 2 {
            break;
        }
        if indent == 4 && trimmed.ends_with(':') {
            let role = parse_bounded_required_string(
                trimmed.trim_end_matches(':').trim(),
                "required role",
                MAX_REQUIRED_ROLE_NAME_CHARS,
            )?;
            *index += 1;
            while *index < lines.len() {
                let nested = lines[*index];
                if nested.trim().is_empty() {
                    *index += 1;
                    continue;
                }
                let (nested_indent, nested_trimmed) = indent_and_trim(nested);
                if nested_indent <= 4 {
                    break;
                }
                let _ = nested_trimmed;
                *index += 1;
            }

            roles.push(RequiredRoleOutput {
                role: role.to_string(),
                required: true,
            });
            continue;
        }
        return Err(format!(
            "required_roles entries must use 4-space indentation with `role:` keys: `{trimmed}`"
        ));
    }

    roles.sort_by(|left, right| left.role.cmp(&right.role));
    roles.dedup_by(|left, right| left.role == right.role);
    if roles.len() > MAX_REQUIRED_ROLES {
        return Err("required_roles exceeds maximum count".to_string());
    }
    Ok(roles)
}

fn parse_tasks(lines: &[&str], index: &mut usize) -> Result<Vec<ManagerLoopTask>, String> {
    *index += 1;
    skip_blank_lines(lines, index);
    if matches!(lines.get(*index), Some(line) if line.trim() == "[]") {
        *index += 1;
        return Ok(Vec::new());
    }

    let mut tasks = Vec::new();
    while *index < lines.len() {
        let line = lines[*index];
        if line.trim().is_empty() {
            *index += 1;
            continue;
        }
        let (indent, trimmed) = indent_and_trim(line);
        if indent <= 2 {
            break;
        }
        if indent != 4 || !trimmed.starts_with("- ") {
            return Err(format!(
                "tasks entries must start with 4-space indented `- id:` rows: `{trimmed}`"
            ));
        }

        let Some(id) = trimmed.strip_prefix("- id:") else {
            return Err("tasks entries must start with `- id:`".to_string());
        };
        let mut task = ManagerLoopTask {
            id: parse_required_string(id.trim(), "task id")?,
            status: TaskStatus::Open,
            classification: TaskClassification::ImplementationReady,
            depends_on: Vec::new(),
        };
        let mut saw_status = false;
        let mut saw_classification = false;
        *index += 1;

        while *index < lines.len() {
            let nested = lines[*index];
            if nested.trim().is_empty() {
                *index += 1;
                continue;
            }
            let (nested_indent, nested_trimmed) = indent_and_trim(nested);
            if nested_indent <= 4 {
                break;
            }
            if nested_indent == 6 {
                if let Some(rest) = nested_trimmed.strip_prefix("status:") {
                    task.status = parse_task_status(rest.trim())?;
                    saw_status = true;
                    *index += 1;
                    continue;
                }
                if let Some(rest) = nested_trimmed.strip_prefix("classification:") {
                    task.classification = parse_task_classification(rest.trim())?;
                    saw_classification = true;
                    *index += 1;
                    continue;
                }
                if let Some(rest) = nested_trimmed.strip_prefix("depends_on:") {
                    let parsed = parse_depends_on_inline(rest.trim())?;
                    *index += 1;
                    if parsed.is_empty() {
                        task.depends_on = parse_depends_on_list(lines, index)?;
                    } else {
                        task.depends_on = parsed;
                    }
                    continue;
                }
                if nested_trimmed.strip_prefix("owner:").is_some() {
                    *index += 1;
                    continue;
                }
                if nested_trimmed.strip_prefix("verification:").is_some() {
                    *index += 1;
                    skip_nested_block(lines, index, 6);
                    continue;
                }
                return Err(format!("unsupported task field `{nested_trimmed}`"));
            }
            return Err(format!("unexpected nested task field `{nested_trimmed}`"));
        }

        if !saw_status {
            return Err(format!("task `{}` is missing `status`", task.id));
        }
        if !saw_classification {
            return Err(format!("task `{}` is missing `classification`", task.id));
        }
        tasks.push(task);
    }

    Ok(tasks)
}

fn parse_depends_on_list(lines: &[&str], index: &mut usize) -> Result<Vec<String>, String> {
    let mut depends_on = Vec::new();
    while *index < lines.len() {
        let line = lines[*index];
        if line.trim().is_empty() {
            *index += 1;
            continue;
        }
        let (indent, trimmed) = indent_and_trim(line);
        if indent <= 6 {
            break;
        }
        let Some(value) = trimmed.strip_prefix("- ") else {
            return Err(format!(
                "depends_on list entries must use `- value` syntax: `{trimmed}`"
            ));
        };
        depends_on.push(parse_required_string(value.trim(), "depends_on value")?);
        *index += 1;
    }
    Ok(depends_on)
}

fn parse_depends_on_inline(value: &str) -> Result<Vec<String>, String> {
    if value.is_empty() || value == "[]" {
        return Ok(Vec::new());
    }
    if !(value.starts_with('[') && value.ends_with(']')) {
        return Err("depends_on must be `[]`, an inline list, or a YAML list".to_string());
    }
    let inner = &value[1..value.len() - 1];
    if inner.trim().is_empty() {
        return Ok(Vec::new());
    }
    inner
        .split(',')
        .map(|item| parse_required_string(item.trim(), "depends_on value"))
        .collect()
}

fn parse_last_decision(value: &str) -> Result<LastDecision, String> {
    match value {
        "dispatch" => Ok(LastDecision::Dispatch),
        "no-dispatch" => Ok(LastDecision::NoDispatch),
        other => Err(format!("unsupported last_decision `{other}`")),
    }
}

fn parse_task_status(value: &str) -> Result<TaskStatus, String> {
    match value {
        "OPEN" => Ok(TaskStatus::Open),
        "CLOSED" => Ok(TaskStatus::Closed),
        "BLOCKED" => Ok(TaskStatus::Blocked),
        other => Err(format!("unsupported task status `{other}`")),
    }
}

fn parse_task_classification(value: &str) -> Result<TaskClassification, String> {
    match value {
        "implementation-ready" => Ok(TaskClassification::ImplementationReady),
        "docs/design-only" => Ok(TaskClassification::DocsDesignOnly),
        "proof-only" => Ok(TaskClassification::ProofOnly),
        other => Err(format!("unsupported task classification `{other}`")),
    }
}

fn parse_bool(value: &str, field_name: &str) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!(
            "{field_name} must be `true` or `false`, got `{other}`"
        )),
    }
}

fn parse_nullable_string(value: &str) -> Option<String> {
    match value {
        "" | "null" => None,
        other => Some(unquote(other)),
    }
}

fn parse_required_string(value: &str, field_name: &str) -> Result<String, String> {
    parse_bounded_required_string(value, field_name, MAX_OUTPUT_STRING_CHARS)
}

fn parse_bounded_required_string(
    value: &str,
    field_name: &str,
    max_chars: usize,
) -> Result<String, String> {
    let value = unquote(value);
    if value.is_empty() {
        Err(format!("{field_name} must not be empty"))
    } else if value.chars().count() > max_chars {
        Err(format!("{field_name} exceeds maximum length"))
    } else {
        Ok(value)
    }
}

fn unquote(value: &str) -> String {
    value
        .trim_matches('"')
        .trim_matches('\'')
        .trim()
        .to_string()
}

fn indent_and_trim(line: &str) -> (usize, &str) {
    let indent = line.chars().take_while(|c| *c == ' ').count();
    (indent, line[indent..].trim_end())
}

fn skip_blank_lines(lines: &[&str], index: &mut usize) {
    while *index < lines.len() && lines[*index].trim().is_empty() {
        *index += 1;
    }
}

fn skip_nested_block(lines: &[&str], index: &mut usize, parent_indent: usize) {
    while *index < lines.len() {
        let line = lines[*index];
        if line.trim().is_empty() {
            *index += 1;
            continue;
        }
        let (indent, _) = indent_and_trim(line);
        if indent <= parent_indent {
            break;
        }
        *index += 1;
    }
}

fn truncate_chars(value: String, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value;
    }
    value.chars().take(max_chars).collect()
}

#[cfg(test)]
#[path = "manager_loop_next_tests.rs"]
mod tests;
