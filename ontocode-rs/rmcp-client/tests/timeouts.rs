use std::io;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use futures::FutureExt as _;
use ontocode_rmcp_client::InProcessTransportFactory;
use ontocode_rmcp_client::RmcpClient;
use pretty_assertions::assert_eq;
use rmcp::ServiceExt;
use rmcp::handler::server::ServerHandler;
use rmcp::model::CallToolRequestParams;
use rmcp::model::CallToolResult;
use rmcp::model::ClientCapabilities;
use rmcp::model::Implementation;
use rmcp::model::InitializeRequestParams;
use rmcp::model::JsonObject;
use rmcp::model::ListToolsResult;
use rmcp::model::PaginatedRequestParams;
use rmcp::model::ProtocolVersion;
use rmcp::model::ServerCapabilities;
use rmcp::model::ServerInfo;
use rmcp::model::Tool;
use serde_json::json;
use tokio::io::duplex;

#[derive(Clone)]
struct SlowServer {
    list_tools_delay: Duration,
    call_tool_delay: Duration,
}

impl SlowServer {
    fn new(list_tools_delay: Duration, call_tool_delay: Duration) -> Self {
        Self {
            list_tools_delay,
            call_tool_delay,
        }
    }

    fn echo_tool() -> Tool {
        Tool::new(
            "echo".to_string(),
            "Slow echo tool".to_string(),
            Arc::new(JsonObject::default()),
        )
    }
}

impl ServerHandler for SlowServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
    }

    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, rmcp::ErrorData>> + Send + '_
    {
        let delay = self.list_tools_delay;
        async move {
            tokio::time::sleep(delay).await;
            Ok(ListToolsResult {
                tools: vec![Self::echo_tool()],
                next_cursor: None,
                meta: None,
            })
        }
    }

    async fn call_tool(
        &self,
        _request: CallToolRequestParams,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        tokio::time::sleep(self.call_tool_delay).await;
        Ok(CallToolResult::success(vec![]))
    }
}

#[derive(Clone)]
struct SlowInProcessFactory {
    server: Arc<SlowServer>,
}

impl InProcessTransportFactory for SlowInProcessFactory {
    fn open(&self) -> futures::future::BoxFuture<'static, io::Result<tokio::io::DuplexStream>> {
        let server = Arc::clone(&self.server);
        async move {
            let (client_side, server_side) = duplex(1024);
            tokio::spawn(async move {
                let server = (*server).clone();
                if let Ok(running) = server.serve(server_side).await {
                    let _ = running.waiting().await;
                }
            });
            Ok(client_side)
        }
        .boxed()
    }
}

fn init_params() -> InitializeRequestParams {
    InitializeRequestParams::new(
        ClientCapabilities::default(),
        Implementation::new("codex-test", "0.0.0-test").with_title("Codex rmcp timeout test"),
    )
    .with_protocol_version(ProtocolVersion::V_2025_06_18)
}

async fn initialized_client(
    list_tools_delay: Duration,
    call_tool_delay: Duration,
) -> Result<RmcpClient> {
    let client = RmcpClient::new_in_process_client(Arc::new(SlowInProcessFactory {
        server: Arc::new(SlowServer::new(list_tools_delay, call_tool_delay)),
    }))
    .await?;

    client
        .initialize(
            init_params(),
            Some(Duration::from_secs(5)),
            Box::new(|_, _| {
                async {
                    Ok(ontocode_rmcp_client::ElicitationResponse {
                        action: ontocode_rmcp_client::ElicitationAction::Accept,
                        content: Some(json!({})),
                        meta: None,
                    })
                }
                .boxed()
            }),
        )
        .await?;

    Ok(client)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn list_tools_timeout_is_reported() -> Result<()> {
    let client = initialized_client(Duration::from_millis(250), Duration::from_millis(0)).await?;

    let err = client
        .list_tools(Some(Duration::from_millis(25)))
        .await
        .expect_err("expected tools/list timeout");
    assert_eq!(err.to_string(), "timed out awaiting tools/list after 25ms");
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn call_tool_timeout_is_reported() -> Result<()> {
    let client = initialized_client(Duration::from_millis(0), Duration::from_millis(250)).await?;

    let err = client
        .call_tool(
            "echo".to_string(),
            Some(json!({ "message": "hello" })),
            /*meta*/ None,
            Some(Duration::from_millis(25)),
        )
        .await
        .expect_err("expected tools/call timeout");
    assert_eq!(err.to_string(), "timed out awaiting tools/call after 25ms");
    Ok(())
}
