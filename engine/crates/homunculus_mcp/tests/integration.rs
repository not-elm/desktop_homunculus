//! Integration tests for the MCP extension proxy aggregator.
//!
//! Spawns a mock downstream MCP server on an ephemeral port, registers it via
//! [`McpExtensionRegistry::add`], and verifies the full
//! register → list_tools → call_tool → deregister flow end-to-end.
//!
//! Note: Full upstream notifications/tools/list_changed propagation is covered by manual
//! verification (plan Task 26) rather than automated tests. Creating a standalone Peer<RoleServer>
//! observer requires running a complete rmcp server lifecycle, which is disproportionate for
//! Phase-1 coverage. The tests here verify the registry bookkeeping (which is what would trigger
//! UpstreamSessionHub broadcasts in production).

use std::sync::Arc;

use rmcp::{
    ServerHandler,
    model::{
        CallToolRequestParams, CallToolResult, Content, Implementation, ListToolsResult,
        PaginatedRequestParams, RawContent, ServerCapabilities, ServerInfo, Tool,
    },
    service::{RequestContext, RoleServer},
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService,
        session::local::LocalSessionManager,
    },
};

use homunculus_mcp::downstream::{McpExtensionRegistry, RegisterArgs};
use homunculus_mcp::upstream_hub::UpstreamSessionHub;

// ---------------------------------------------------------------------------
// Mock downstream server
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct MockDownstream;

impl ServerHandler for MockDownstream {
    fn get_info(&self) -> ServerInfo {
        let capabilities = ServerCapabilities::builder().enable_tools().build();
        ServerInfo::new(capabilities)
            .with_server_info(Implementation::new("mock-downstream", "0.0.0"))
    }

    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, rmcp::ErrorData>> + Send + '_
    {
        async move {
            let schema: Arc<serde_json::Map<String, serde_json::Value>> =
                Arc::new(serde_json::Map::new());
            let tool = Tool::new("echo", "Echo back the tool name", schema);
            Ok(ListToolsResult {
                tools: vec![tool],
                next_cursor: None,
                meta: None,
            })
        }
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, rmcp::ErrorData>> + Send + '_
    {
        let name = request.name.to_string();
        async move {
            Ok(CallToolResult::success(vec![Content::text(format!(
                "echoed: {}",
                name
            ))]))
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: spawn the mock server on an ephemeral port
// ---------------------------------------------------------------------------

async fn spawn_mock_server() -> (u16, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral port");
    let port = listener.local_addr().unwrap().port();

    let session_manager = Arc::new(LocalSessionManager {
        sessions: Default::default(),
        session_config: Default::default(),
    });
    let service = StreamableHttpService::new(
        || Ok(MockDownstream),
        session_manager,
        StreamableHttpServerConfig::default(),
    );

    let router = axum::Router::new().route_service("/mcp", service);

    let handle = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    // Give the server a moment to be ready before clients connect.
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    (port, handle)
}

// ---------------------------------------------------------------------------
// Integration test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn register_list_tools_call_tool_then_deregister() {
    let (mock_port, _mock_task) = spawn_mock_server().await;

    let hub = UpstreamSessionHub::new();
    let (registry, _deregister_tx) = McpExtensionRegistry::new(hub.clone());

    // Register the mock downstream server.
    registry
        .0
        .write()
        .await
        .add(RegisterArgs {
            mod_slug: "mockmod".into(),
            mod_name: "@test/mockmod".into(),
            mcp_url: format!("http://127.0.0.1:{}/mcp", mock_port),
        })
        .await
        .expect("add should succeed");

    // list_all_tools_prefixed should include mockmod__echo.
    let tools = registry.0.read().await.list_all_tools_prefixed().await;
    assert!(
        tools.iter().any(|t| t.name.as_ref() == "mockmod__echo"),
        "expected mockmod__echo in tool list, got: {:?}",
        tools.iter().map(|t| t.name.as_ref()).collect::<Vec<_>>()
    );

    // call_tool_by_parts should dispatch to the downstream and return the echoed text.
    let result = registry
        .0
        .read()
        .await
        .call_tool_by_parts("mockmod", "echo", Default::default())
        .await
        .expect("call_tool should succeed");

    let echoed_text = result.content.iter().find_map(|c| match &c.raw {
        RawContent::Text(t) => Some(t.text.clone()),
        _ => None,
    });
    assert!(
        echoed_text
            .as_deref()
            .is_some_and(|s| s.contains("echoed: echo")),
        "expected echoed text in result, got: {:?}",
        echoed_text
    );

    // Deregister and confirm the tool is no longer listed.
    registry.0.write().await.remove("mockmod").await;
    let tools_after = registry.0.read().await.list_all_tools_prefixed().await;
    assert!(
        !tools_after
            .iter()
            .any(|t| t.name.as_ref() == "mockmod__echo"),
        "expected mockmod__echo to be absent after deregister, got: {:?}",
        tools_after
            .iter()
            .map(|t| t.name.as_ref())
            .collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn re_registration_replaces_existing_downstream() {
    let (mock_port, _mock_task) = spawn_mock_server().await;

    let hub = UpstreamSessionHub::new();
    let (registry, _deregister_tx) = McpExtensionRegistry::new(hub.clone());

    let make_args = |slug: &str| RegisterArgs {
        mod_slug: slug.to_string(),
        mod_name: "@test/mockmod".into(),
        mcp_url: format!("http://127.0.0.1:{}/mcp", mock_port),
    };

    registry
        .0
        .write()
        .await
        .add(make_args("mockmod"))
        .await
        .expect("first add");
    registry
        .0
        .write()
        .await
        .add(make_args("mockmod"))
        .await
        .expect("re-add should succeed (upsert)");

    // Only one entry with slug "mockmod" should exist.
    let regs = registry.0.read().await.list_registrations().await;
    let mockmod_count = regs.iter().filter(|r| r.mod_slug == "mockmod").count();
    assert_eq!(
        mockmod_count, 1,
        "upsert should keep exactly one entry per slug"
    );
}

#[tokio::test]
async fn deregister_removes_tools_from_aggregated_list() {
    let (mock_port, _mock_task) = spawn_mock_server().await;

    let hub = UpstreamSessionHub::new();
    let (registry, _deregister_tx) = McpExtensionRegistry::new(hub.clone());

    registry
        .0
        .write()
        .await
        .add(RegisterArgs {
            mod_slug: "mockmod".into(),
            mod_name: "@test/mockmod".into(),
            mcp_url: format!("http://127.0.0.1:{}/mcp", mock_port),
        })
        .await
        .expect("add");

    // Before deregister — tool is visible.
    let tools_before = registry.0.read().await.list_all_tools_prefixed().await;
    assert!(
        tools_before.iter().any(|t| t.name.as_ref() == "mockmod__echo"),
        "expected mockmod__echo before deregister, got: {:?}",
        tools_before
            .iter()
            .map(|t| t.name.as_ref())
            .collect::<Vec<_>>()
    );

    // Deregister.
    registry.0.write().await.remove("mockmod").await;

    // After deregister — tool is gone.
    let tools_after = registry.0.read().await.list_all_tools_prefixed().await;
    assert!(
        tools_after.iter().all(|t| t.name.as_ref() != "mockmod__echo"),
        "expected mockmod__echo absent after deregister, got: {:?}",
        tools_after
            .iter()
            .map(|t| t.name.as_ref())
            .collect::<Vec<_>>()
    );

    // list_registrations is empty.
    let regs = registry.0.read().await.list_registrations().await;
    assert!(regs.is_empty(), "no registrations should remain after remove");
}

#[tokio::test]
async fn multiple_downstreams_aggregate_correctly() {
    let (port_a, _task_a) = spawn_mock_server().await;
    let (port_b, _task_b) = spawn_mock_server().await;

    let hub = UpstreamSessionHub::new();
    let (registry, _deregister_tx) = McpExtensionRegistry::new(hub.clone());

    registry
        .0
        .write()
        .await
        .add(RegisterArgs {
            mod_slug: "modone".into(),
            mod_name: "@test/modone".into(),
            mcp_url: format!("http://127.0.0.1:{}/mcp", port_a),
        })
        .await
        .expect("add modone");

    registry
        .0
        .write()
        .await
        .add(RegisterArgs {
            mod_slug: "modtwo".into(),
            mod_name: "@test/modtwo".into(),
            mcp_url: format!("http://127.0.0.1:{}/mcp", port_b),
        })
        .await
        .expect("add modtwo");

    let tools = registry.0.read().await.list_all_tools_prefixed().await;
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert!(
        names.contains(&"modone__echo"),
        "expected modone__echo in tool list, got: {:?}",
        names
    );
    assert!(
        names.contains(&"modtwo__echo"),
        "expected modtwo__echo in tool list, got: {:?}",
        names
    );
    assert_eq!(tools.len(), 2, "expected exactly 2 tools from 2 downstreams");
}
