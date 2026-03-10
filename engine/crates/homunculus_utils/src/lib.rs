//! # Homunculus Utils
//!
//! Bevy-independent utilities shared across Homunculus tools (engine, CLI, MCP server).
//! Provides path resolution, constants, and schema types.

pub mod config;
pub mod consts;
pub mod error;
pub mod mods;
pub mod path;
pub mod process;
pub mod schema;

pub mod prelude {
    pub use crate::{config::*, consts::*, error::*, path::*, schema::prelude::*};
}
