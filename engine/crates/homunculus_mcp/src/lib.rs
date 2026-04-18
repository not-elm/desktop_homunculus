//! MCP (Model Context Protocol) server integration for Desktop Homunculus.
//!
//! This crate provides an embedded MCP server that exposes character control
//! capabilities to AI agents via the streamable HTTP transport, mounted on the
//! engine's existing Axum router.

pub mod downstream;
pub mod handler;
mod service;
pub mod upstream_hub;

pub use service::create_mcp_service;
pub use upstream_hub::SharedUpstreamHub;
