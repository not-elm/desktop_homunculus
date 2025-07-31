# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Core Development

- `make dev` - Run application in development mode with features enabled
- `cargo run --features develop` - Alternative development run command
- `make setup` - Install development dependencies (mdbook, pnpm packages)
- `make fix` - Auto-format and fix linting issues across workspace

### Building

- `pnpm build` - Build TypeScript frontend components using Turbo
- `cargo build` - Build Rust components
- `make doc-build` - Generate API documentation from OpenAPI spec

### Frontend Development

Individual UI components can be developed independently:

- `pnpm dev` (in ui/settings/, ui/chat/, ui/menu/) - Watch mode builds
- `pnpm check-types` - TypeScript type checking
- `pnpm lint` - ESLint checking

## Architecture Overview

### Core Structure

Desktop Homunculus is a cross-platform desktop mascot application built with Bevy game engine. The architecture follows
a modular plugin system with clear separation of concerns:

**Main Application (`src/main.rs`):**

- Transparent window desktop application
- Plugin-based architecture using Bevy ECS
- VRM/VRMA model support for 3D mascots
- WebView integration for UI components

**Core Modules (`crates/`):**

- `homunculus_core` - Core data structures, events, resources
- `homunculus_api` - External API interfaces (GPT, VoiceVox, etc.)
- `homunculus_http_server` - REST API server for external control
- `bevy_vrm1` - VRM model loading and animation system
- `homunculus_*` modules - Specialized functionality (drag, effects, speech, etc.)

**Frontend (`ui/`):**

- React + TypeScript components built with Vite
- Shared component library in `ui/core/`
- Individual apps: settings, chat, menu
- Communication with Rust backend via WebView IPC

**SDK (`sdk/typescript/`):**

- TypeScript API client for external integrations
- Provides typed interfaces for all backend functionality

### Key Architectural Patterns

**Plugin System:**
Each major feature is implemented as a Bevy plugin, allowing modular development and easy feature toggling. Plugins
communicate through Bevy's ECS events and resources.

**WebView Integration:**
UI components run as embedded web applications using `bevy_webview_wry`. This enables modern web development while
maintaining native performance for 3D rendering.

**Multi-Monitor Support:**
The application can spawn multiple VRM models across different monitors with independent behavior and interactions.

**Asset Management:**
VRM models and VRMA animations are dynamically loaded. Mods system allows for custom JavaScript/HTML extensions.

**Mod System:**
The application supports a mod system where users can create custom JavaScript/HTML components that interact with the
Rust backend.
This allows for extensibility and user-generated content.
To add a mod, create a directory in `./assets/mods/` for each mod.
Inside the directory, place HTML for UI, JavaScript for scripts, and other assets. Write the configuration in
`mod.json`, which is read at application startup.

### Development Notes

**Platform Considerations:**

- Primary support: macOS
- Partial support: Windows (OpenGL backend required due to transparency limitations)
- The application uses OpenGL on Windows to work around transparency issues with Vulkan/DX12

**Build System:**

- Rust workspace with multiple crates
- Turbo.js for frontend build orchestration
- PNPM for package management
- Mixed development environment supporting both Rust and TypeScript

**Testing:**

- If you change the Rust code, run `cargo test --workspace` at the end of the task to test.
- If you change the TypeScript code, verify that you can build it with pnpm build at the end of the task.

**Documentation:**

- If you change `./crates/homunculus_http_server/src/**`, update the OpenAPI spec in
  `./sdk/typescript/src/api/openapi.json`.

**Hot Reload:**

- WebView components support hot reload during development
- Rust components require restart for changes

