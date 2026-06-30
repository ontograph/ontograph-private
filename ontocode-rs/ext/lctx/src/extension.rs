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

use crate::tool::LctxReadTool;
use crate::tool::LctxThreadState;

#[derive(Clone, Default)]
struct LctxExtension;

impl ToolContributor for LctxExtension {
    fn tools(
        &self,
        _session_store: &ExtensionData,
        thread_store: &ExtensionData,
    ) -> Vec<Arc<dyn ToolExecutor<ToolCall>>> {
        let thread_state = thread_store.get_or_init(LctxThreadState::default);
        vec![Arc::new(LctxReadTool::new(thread_state))]
    }
}

#[async_trait::async_trait]
impl TurnInputContributor for LctxExtension {
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
            thread_store
                .get_or_init(LctxThreadState::default)
                .clear_current_cwd();
            return Vec::new();
        };

        thread_store
            .get_or_init(LctxThreadState::default)
            .set_current_cwd(environment.cwd.clone());
        Vec::new()
    }
}

pub fn install(registry: &mut ExtensionRegistryBuilder<Config>) {
    let extension = Arc::new(LctxExtension);
    registry.tool_contributor(extension.clone());
    registry.turn_input_contributor(extension);
}
