use std::sync::Arc;

use crate::session::turn_context::TurnContext;
use crate::tools::handlers::multi_agents_spec::WaitAgentTimeoutOptions;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExposure;
use ontocode_mcp::ToolInfo;
use ontocode_protocol::dynamic_tools::DynamicToolSpec;
use ontocode_tools::DiscoverableTool;
use ontocode_tools::ToolCall as ExtensionToolCall;
use ontocode_tools::ToolExecutor;
use ontocode_tools::ToolSpec;

pub(crate) mod dynamic;
pub(crate) mod extensions;
pub(crate) mod hosted;
pub(crate) mod mcp;
pub(crate) mod native;

type PlannedRuntime = Arc<dyn CoreToolRuntime>;

#[derive(Default)]
pub(crate) struct PlannedTools {
    pub(crate) runtimes: Vec<PlannedRuntime>,
    pub(crate) hosted_specs: Vec<ToolSpec>,
}

impl PlannedTools {
    pub(crate) fn add<T>(&mut self, handler: T)
    where
        T: CoreToolRuntime + 'static,
    {
        self.runtimes.push(Arc::new(handler));
    }

    pub(crate) fn add_arc(&mut self, handler: PlannedRuntime) {
        self.runtimes.push(handler);
    }

    pub(crate) fn add_with_exposure<T>(&mut self, handler: T, exposure: ToolExposure)
    where
        T: CoreToolRuntime + 'static,
    {
        self.runtimes
            .push(crate::tools::registry::override_tool_exposure(
                Arc::new(handler),
                exposure,
            ));
    }

    pub(crate) fn add_dispatch_only<T>(&mut self, handler: T)
    where
        T: CoreToolRuntime + 'static,
    {
        self.add_with_exposure(handler, ToolExposure::Hidden);
    }

    pub(crate) fn add_hosted_spec(&mut self, spec: ToolSpec) {
        self.hosted_specs.push(spec);
    }

    pub(crate) fn runtimes(&self) -> &[PlannedRuntime] {
        &self.runtimes
    }

    pub(crate) fn prepend_runtimes(&mut self, mut runtimes: Vec<PlannedRuntime>) {
        self.runtimes.splice(0..0, runtimes.drain(..));
    }
}

#[derive(Clone, Copy)]
pub(crate) struct CoreToolPlanContext<'a> {
    pub(crate) turn_context: &'a TurnContext,
    pub(crate) mcp_tools: Option<&'a [ToolInfo]>,
    pub(crate) deferred_mcp_tools: Option<&'a [ToolInfo]>,
    pub(crate) discoverable_tools: Option<&'a [DiscoverableTool]>,
    pub(crate) extension_tool_executors: &'a [Arc<dyn ToolExecutor<ExtensionToolCall>>],
    pub(crate) dynamic_tools: &'a [DynamicToolSpec],
    pub(crate) default_agent_type_description: &'a str,
    pub(crate) wait_agent_timeouts: WaitAgentTimeoutOptions,
}
