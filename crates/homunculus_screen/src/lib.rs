//! # Homunculus Screen
//!
//! This crate provides screen and display management functionality for the
//! Desktop Homunculus application, enabling multi-monitor support and window
//! detection across different operating systems.
//!
//! ## Overview
//!
//! `homunculus_screen` handles interaction with the operating system's display
//! and window management APIs, providing cross-platform access to:
//! - Display information and monitor enumeration
//! - Global window detection and tracking
//! - Multi-monitor coordinate system management
//! - Window geometry and positioning data
//!
//! ## Key Features
//!
//! - **Multi-Monitor Support**: Detection and management of multiple displays
//! - **Cross-Platform**: Supports Windows, macOS, and Linux
//! - **Global Window Detection**: Find and track windows from other applications
//! - **Display Geometry**: Access to display bounds, resolution, and scaling
//! - **Window Sitting Detection**: Determine when mascots can "sit" on other windows
//!
//! ## Platform Support
//!
//! The crate includes platform-specific implementations for:
//! - **Windows**: Win32 API integration for display and window management
//! - **macOS**: Core Graphics and Accessibility APIs
//! - **Linux**: X11/Wayland display server integration
//!
//! ## Use Cases
//!
//! - Positioning mascots on specific monitors
//! - Detecting when mascots are dropped on application windows
//! - Managing mascot behavior across multiple displays
//! - Implementing desktop "sitting" functionality

mod displays;
mod global_windows;

use crate::global_windows::HomunculusGlobalWindowsPlugin;
use bevy::prelude::{App, Plugin};

pub mod prelude {
    pub use crate::{displays::*, global_windows::prelude::*};
}

/// Plugin that provides screen and display management functionality.
///
/// This plugin enables multi-monitor support and global window detection,
/// allowing mascots to interact with the desktop environment and other
/// applications across multiple displays.
///
/// # Functionality
///
/// - **Display Detection**: Enumerate and track multiple monitors
/// - **Window Detection**: Find and track windows from other applications
/// - **Cross-Platform Support**: Works on Windows, macOS, and Linux
/// - **Coordinate Systems**: Handle coordinate transformations between displays
///
/// # Included Plugins
///
/// - `HomunculusGlobalWindowsPlugin`: Handles global window detection and tracking
///
/// # Platform Integration
///
/// The plugin automatically selects the appropriate platform-specific
/// implementation based on the target operating system, providing a
/// unified interface for display and window management.
pub struct HomunculusScreenPlugin;

impl Plugin for HomunculusScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HomunculusGlobalWindowsPlugin,));
    }
}
