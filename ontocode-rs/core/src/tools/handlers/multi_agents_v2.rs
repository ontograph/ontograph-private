//! Implements the MultiAgentV2 collaboration tool surface.

use crate::agent::AgentStatus;
use crate::agent::agent_resolver::resolve_agent_target;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::multi_agents_common::*;
use crate::tools::handlers::parse_arguments;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use ontocode_protocol::AgentPath;
use ontocode_protocol::models::ResponseInputItem;
use ontocode_protocol::openai_models::ReasoningEffort;
use ontocode_protocol::protocol::CollabAgentInteractionBeginEvent;
use ontocode_protocol::protocol::CollabAgentInteractionEndEvent;
use ontocode_protocol::protocol::CollabAgentSpawnBeginEvent;
use ontocode_protocol::protocol::CollabAgentSpawnEndEvent;
use ontocode_protocol::protocol::CollabCloseBeginEvent;
use ontocode_protocol::protocol::CollabCloseEndEvent;
use ontocode_protocol::protocol::CollabWaitingBeginEvent;
use ontocode_protocol::protocol::CollabWaitingEndEvent;
use ontocode_protocol::user_input::UserInput;
use ontocode_tools::ToolName;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JsonValue;

pub(crate) use close_agent::Handler as CloseAgentHandler;
pub(crate) use followup_task::Handler as FollowupTaskHandler;
pub(crate) use list_agents::Handler as ListAgentsHandler;
pub(crate) use send_message::Handler as SendMessageHandler;
pub(crate) use spawn::Handler as SpawnAgentHandler;
pub(crate) use wait::Handler as WaitAgentHandler;

mod close_agent;
mod followup_task;
mod list_agents;
mod message_tool;
mod send_message;
mod spawn;
pub(crate) mod wait;
