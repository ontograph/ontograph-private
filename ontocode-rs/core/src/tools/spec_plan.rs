use crate::session::turn_context::TurnContext;
use crate::tools::code_mode::execute_spec::create_code_mode_tool;
use crate::tools::handlers::CodeModeExecuteHandler;
use crate::tools::handlers::CodeModeWaitHandler;
use crate::tools::handlers::ToolSearchHandler;
use crate::tools::planning::CoreToolPlanContext;
use crate::tools::planning::PlannedTools;
use crate::tools::planning::dynamic;
use crate::tools::planning::extensions;
use crate::tools::planning::hosted;
use crate::tools::planning::mcp;
use crate::tools::planning::native;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExposure;
use crate::tools::registry::ToolRegistry;
use crate::tools::router::ToolRouter;
use crate::tools::router::ToolRouterParams;
use ontocode_protocol::openai_models::ToolMode;
use ontocode_tools::ResponsesApiNamespaceTool;
use ontocode_tools::ToolName;
use ontocode_tools::ToolSpec;
use ontocode_tools::collect_code_mode_exec_prompt_tool_definitions;
use ontocode_tools::default_namespace_description;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::sync::Arc;

pub(crate) fn build_tool_router(
    turn_context: &TurnContext,
    params: ToolRouterParams<'_>,
) -> ToolRouter {
    let (model_visible_specs, registry) = build_tool_specs_and_registry(turn_context, params);
    ToolRouter::from_parts(registry, model_visible_specs)
}

fn build_tool_specs_and_registry(
    turn_context: &TurnContext,
    params: ToolRouterParams<'_>,
) -> (Vec<ToolSpec>, ToolRegistry) {
    let ToolRouterParams {
        mcp_tools,
        deferred_mcp_tools,
        discoverable_tools,
        extension_tool_executors,
        dynamic_tools,
    } = params;
    let default_agent_type_description =
        crate::agent::role::spawn_tool_spec::build(&std::collections::BTreeMap::new());
    let context = CoreToolPlanContext {
        turn_context,
        mcp_tools: mcp_tools.as_deref(),
        deferred_mcp_tools: deferred_mcp_tools.as_deref(),
        discoverable_tools: discoverable_tools.as_deref(),
        extension_tool_executors: &extension_tool_executors,
        dynamic_tools,
        default_agent_type_description: &default_agent_type_description,
        wait_agent_timeouts: native::wait_agent_timeout_options(turn_context),
    };
    let mut planned_tools = PlannedTools::default();
    add_tool_sources(&context, &mut planned_tools);
    append_tool_search_executor(&context, &mut planned_tools);
    prepend_code_mode_executors(&context, &mut planned_tools);
    build_model_visible_specs_and_registry(turn_context, planned_tools)
}

fn build_model_visible_specs_and_registry(
    turn_context: &TurnContext,
    planned_tools: PlannedTools,
) -> (Vec<ToolSpec>, ToolRegistry) {
    let PlannedTools {
        runtimes,
        hosted_specs,
    } = planned_tools;
    let mut specs = Vec::new();
    let mut seen_tool_names = HashSet::new();
    for runtime in &runtimes {
        let tool_name = runtime.tool_name();
        if !seen_tool_names.insert(tool_name.clone()) {
            continue;
        }
        let exposure = runtime.exposure();
        if exposure.is_direct() && !is_hidden_by_code_mode_only(turn_context, &tool_name, exposure)
        {
            let spec = runtime.spec();
            specs.push(spec_for_model_request(turn_context, exposure, spec));
        }
    }
    specs.extend(hosted_specs);

    let registry = ToolRegistry::from_tools(runtimes);
    let model_visible_specs = merge_into_namespaces(specs)
        .into_iter()
        .filter(|spec| {
            namespace_tools_enabled(turn_context) || !matches!(spec, ToolSpec::Namespace(_))
        })
        .collect();

    (model_visible_specs, registry)
}

fn spec_for_model_request(
    turn_context: &TurnContext,
    exposure: ToolExposure,
    spec: ToolSpec,
) -> ToolSpec {
    if matches!(
        turn_context.tool_mode,
        ToolMode::CodeMode | ToolMode::CodeModeOnly
    ) && exposure != ToolExposure::DirectModelOnly
        && ontocode_code_mode::is_code_mode_nested_tool(spec.name())
    {
        ontocode_tools::augment_tool_spec_for_code_mode(spec)
    } else {
        spec
    }
}

pub(crate) fn search_tool_enabled(turn_context: &TurnContext) -> bool {
    turn_context.model_info.supports_search_tool
}

pub(crate) fn tool_suggest_enabled(turn_context: &TurnContext) -> bool {
    let features = turn_context.features.get();
    features.enabled(ontocode_features::Feature::ToolSuggest)
        && features.enabled(ontocode_features::Feature::Apps)
        && features.enabled(ontocode_features::Feature::Plugins)
}

fn namespace_tools_enabled(turn_context: &TurnContext) -> bool {
    turn_context.provider.capabilities().namespace_tools
}

fn is_hidden_by_code_mode_only(
    turn_context: &TurnContext,
    tool_name: &ToolName,
    exposure: ToolExposure,
) -> bool {
    turn_context.tool_mode == ToolMode::CodeModeOnly
        && exposure != ToolExposure::DirectModelOnly
        && ontocode_code_mode::is_code_mode_nested_tool(
            &ontocode_tools::code_mode_name_for_tool_name(tool_name),
        )
}

fn build_code_mode_executors(
    turn_context: &TurnContext,
    executors: &[Arc<dyn CoreToolRuntime>],
    deferred_tools_available: bool,
) -> Vec<Arc<dyn CoreToolRuntime>> {
    if !matches!(
        turn_context.tool_mode,
        ToolMode::CodeMode | ToolMode::CodeModeOnly
    ) {
        return vec![];
    }

    let mut code_mode_nested_tool_specs = Vec::new();
    let mut exec_prompt_tool_specs = Vec::new();
    for executor in executors {
        let exposure = executor.exposure();
        if exposure == ToolExposure::DirectModelOnly {
            continue;
        }

        if exposure == ToolExposure::Hidden {
            continue;
        }
        let spec = executor.spec();

        if exposure != ToolExposure::Deferred {
            exec_prompt_tool_specs.push(spec.clone());
        }
        code_mode_nested_tool_specs.push(spec);
    }

    let namespace_descriptions = code_mode_namespace_descriptions(&exec_prompt_tool_specs);
    let mut enabled_tools =
        collect_code_mode_exec_prompt_tool_definitions(exec_prompt_tool_specs.iter());
    enabled_tools
        .sort_by(|left, right| compare_code_mode_tools(left, right, &namespace_descriptions));

    vec![
        Arc::new(CodeModeExecuteHandler::new(
            create_code_mode_tool(
                &enabled_tools,
                &namespace_descriptions,
                turn_context.tool_mode == ToolMode::CodeModeOnly,
                deferred_tools_available,
            ),
            code_mode_nested_tool_specs,
        )),
        Arc::new(CodeModeWaitHandler),
    ]
}

fn merge_into_namespaces(specs: Vec<ToolSpec>) -> Vec<ToolSpec> {
    let mut merged_specs = Vec::with_capacity(specs.len());
    let mut namespace_indices = BTreeMap::<String, usize>::new();
    for spec in specs {
        match spec {
            ToolSpec::Namespace(mut namespace) => {
                if let Some(index) = namespace_indices.get(&namespace.name).copied() {
                    let ToolSpec::Namespace(existing_namespace) = &mut merged_specs[index] else {
                        unreachable!("namespace index must point to a namespace spec");
                    };
                    if existing_namespace.description.trim().is_empty()
                        && !namespace.description.trim().is_empty()
                    {
                        existing_namespace.description = namespace.description;
                    }
                    existing_namespace.tools.append(&mut namespace.tools);
                    continue;
                }

                namespace_indices.insert(namespace.name.clone(), merged_specs.len());
                merged_specs.push(ToolSpec::Namespace(namespace));
            }
            spec => merged_specs.push(spec),
        }
    }

    for spec in &mut merged_specs {
        let ToolSpec::Namespace(namespace) = spec else {
            continue;
        };

        namespace.tools.sort_by(|left, right| match (left, right) {
            (
                ResponsesApiNamespaceTool::Function(left),
                ResponsesApiNamespaceTool::Function(right),
            ) => left.name.cmp(&right.name),
        });

        if namespace.description.trim().is_empty() {
            namespace.description = default_namespace_description(&namespace.name);
        }
    }

    merged_specs
}

fn code_mode_namespace_descriptions(
    specs: &[ToolSpec],
) -> BTreeMap<String, ontocode_code_mode::ToolNamespaceDescription> {
    let mut namespace_descriptions = BTreeMap::new();
    for spec in specs {
        let ToolSpec::Namespace(namespace) = spec else {
            continue;
        };

        let entry = namespace_descriptions
            .entry(namespace.name.clone())
            .or_insert_with(|| ontocode_code_mode::ToolNamespaceDescription {
                name: namespace.name.clone(),
                description: namespace.description.clone(),
            });
        if entry.description.trim().is_empty() && !namespace.description.trim().is_empty() {
            entry.description = namespace.description.clone();
        }
    }
    namespace_descriptions
}

fn add_tool_sources(context: &CoreToolPlanContext<'_>, planned_tools: &mut PlannedTools) {
    native::add_native_tools(context, planned_tools);
    mcp::add_mcp_tools(context, planned_tools);
    dynamic::add_dynamic_tools(context, planned_tools);
    extensions::add_extension_tools(context, planned_tools);
    hosted::add_hosted_tools(context, planned_tools);
}

fn append_tool_search_executor(
    context: &CoreToolPlanContext<'_>,
    planned_tools: &mut PlannedTools,
) {
    let turn_context = context.turn_context;
    if !(search_tool_enabled(turn_context) && namespace_tools_enabled(turn_context)) {
        return;
    }

    let search_infos = planned_tools
        .runtimes()
        .iter()
        .filter(|executor| executor.exposure() == ToolExposure::Deferred)
        .filter_map(|executor| executor.search_info())
        .collect::<Vec<_>>();
    if search_infos.is_empty() {
        return;
    }

    planned_tools.add(ToolSearchHandler::new(search_infos));
}

fn prepend_code_mode_executors(
    context: &CoreToolPlanContext<'_>,
    planned_tools: &mut PlannedTools,
) {
    let turn_context = context.turn_context;
    let deferred_tools_available = search_tool_enabled(turn_context)
        && planned_tools
            .runtimes()
            .iter()
            .any(|executor| executor.exposure() == ToolExposure::Deferred);
    let code_mode_executors = build_code_mode_executors(
        turn_context,
        planned_tools.runtimes(),
        deferred_tools_available,
    );
    planned_tools.prepend_runtimes(code_mode_executors);
}

fn compare_code_mode_tools(
    left: &ontocode_code_mode::ToolDefinition,
    right: &ontocode_code_mode::ToolDefinition,
    namespace_descriptions: &BTreeMap<String, ontocode_code_mode::ToolNamespaceDescription>,
) -> std::cmp::Ordering {
    let left_namespace = code_mode_namespace_name(left, namespace_descriptions);
    let right_namespace = code_mode_namespace_name(right, namespace_descriptions);

    left_namespace
        .cmp(&right_namespace)
        .then_with(|| left.tool_name.name.cmp(&right.tool_name.name))
        .then_with(|| left.name.cmp(&right.name))
}

fn code_mode_namespace_name<'a>(
    tool: &ontocode_code_mode::ToolDefinition,
    namespace_descriptions: &'a BTreeMap<String, ontocode_code_mode::ToolNamespaceDescription>,
) -> Option<&'a str> {
    tool.tool_name
        .namespace
        .as_ref()
        .and_then(|namespace| namespace_descriptions.get(namespace))
        .map(|namespace_description| namespace_description.name.as_str())
}

#[cfg(test)]
#[path = "spec_plan_tests.rs"]
mod tests;
