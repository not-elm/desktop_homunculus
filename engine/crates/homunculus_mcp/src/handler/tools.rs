//! Tool router aggregation for the MCP handler.
//!
//! Each domain submodule defines its own `#[tool_router]` and this module
//! combines them into a single [`ToolRouter`] used by the handler.

mod animation;
mod audio;
mod reaction;
mod system;
mod transform;
mod vrm;
mod webview;

use rmcp::handler::server::router::tool::ToolRouter;

use super::HomunculusMcpHandler;

/// Returns the combined tool router from all domain submodules.
pub(super) fn tool_router() -> ToolRouter<HomunculusMcpHandler> {
    HomunculusMcpHandler::webview_tool_router()
        + HomunculusMcpHandler::vrm_tool_router()
        + HomunculusMcpHandler::animation_tool_router()
        + HomunculusMcpHandler::audio_tool_router()
        + HomunculusMcpHandler::transform_tool_router()
        + HomunculusMcpHandler::reaction_tool_router()
        + HomunculusMcpHandler::system_tool_router()
}
