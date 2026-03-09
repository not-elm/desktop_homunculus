//! MCP (Model Context Protocol) server integration for Desktop Homunculus.
//!
//! This crate provides an embedded MCP server that exposes character control
//! capabilities to AI agents via the streamable HTTP transport, mounted on the
//! engine's existing Axum router.

mod handler;
mod service;
