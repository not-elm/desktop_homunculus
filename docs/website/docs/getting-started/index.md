---
title: "What is Desktop Homunculus?"
sidebar_position: 1
---

# What is Desktop Homunculus?

Desktop Homunculus is a cross-platform desktop mascot application built with the [Bevy](https://bevyengine.org/) game engine. It renders VRM 3D characters in a transparent window directly on your desktop, with WebView-based UI overlays for settings, menus, and custom interfaces. The application is designed to be extended through a MOD system, letting you customize characters, behaviors, and integrations.

## Key Features

- **VRM 3D character rendering** — Characters render in a transparent window that sits on your desktop, blending seamlessly with your workspace
- **MOD system** — Install and create MODs to add characters, animations, sound effects, UI panels, and custom behaviors. MODs are npm packages managed with pnpm
- **AI integration via MCP** — Connect AI assistants (such as Claude) to control your character through the Model Context Protocol (MCP) server
- **WebView-based UI overlays** — Settings panels, context menus, and custom UIs are React apps rendered inside the engine via Chromium Embedded Framework (CEF)
- **Extensible TypeScript SDK** — The `@hmcs/sdk` package provides a typed API for MOD developers to interact with characters, animations, audio, preferences, and more

## How It Works

The engine, built on Bevy, renders a transparent desktop window containing your VRM character. MODs are npm packages that run as Node.js child processes alongside the engine, communicating through an HTTP API on `localhost:3100`. The TypeScript SDK (`@hmcs/sdk`) wraps this API with convenient, typed functions for spawning characters, playing animations, managing preferences, and opening WebViews. AI assistants can control the character through an MCP server that exposes the same capabilities over stdio.

## What You Need

| Requirement | Version |
|---|---|
| **OS** | macOS 12+ (Windows support is planned) |
| **Node.js** | 22 or later |
| **pnpm** | 10.x |

:::info[Alpha Status]
Desktop Homunculus is currently in **alpha** (v0.1.0-alpha.4). APIs and MOD specifications may change between releases. We welcome feedback and contributions.
:::

## Next Steps

- **[Installation](/docs/getting-started/installation)** — Download the app and set up your MOD environment
- **[Quick Start](/docs/getting-started/quick-start)** — Get up and running in minutes: configure settings, interact with your character, and explore official MODs
- **[MOD Development](/docs/mod-development)** — Build your own MODs with the TypeScript SDK
- **[AI Integration](/docs/ai-integration)** — Connect AI assistants to your character via MCP
