---
title: "What is Desktop Homunculus?"
sidebar_position: 1
---

# What is Desktop Homunculus?

Desktop Homunculus is a cross-platform desktop mascot application built with the [Bevy](https://bevyengine.org/) game engine. It features VRM-based character creation, WebView-based UI overlays, and high extensibility through a MOD system.

## Key Features

- **VRM 3D character rendering** — Characters render on your desktop, blending seamlessly with your workspace
- **MOD system** — Install and create MODs to add characters, animations, sound effects, UI panels, and custom behaviors. MODs are npm packages managed with pnpm
- **AI integration via MCP** — Connect AI assistants (such as Claude) to control your character through the Model Context Protocol (MCP) server
- **WebView-based UI overlays** — Settings panels, context menus, and custom UIs are React apps rendered inside the engine via Chromium Embedded Framework (CEF)
- **Extensible TypeScript SDK** — The `@hmcs/sdk` package gives MOD developers control over characters, WebView UI, audio, and settings

## What You Need

| Requirement | Version |
|---|---|
| **OS** | macOS 12+ / Windows 10+ |
| **Node.js** | 22 or later |
| **pnpm** | 10.x |

:::info[Alpha Status]
Desktop Homunculus is currently in **alpha**. APIs and MOD specifications may change between releases. We welcome feedback and contributions.
:::

## Next Steps

- **[Installation](/getting-started/installation)** — Download the app and set up your MOD environment
- **[Quick Start](/getting-started/quick-start)** — Get up and running in minutes: configure settings, interact with your character, and explore [official MODs](/mods/)
- **[MOD Development](/mod-development)** — Build your own MODs with the TypeScript SDK
- **[AI Integration](/ai-integration)** — Connect AI assistants to your character via MCP
