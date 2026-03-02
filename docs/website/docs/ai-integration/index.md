---
title: "AI Integration"
sidebar_position: 1
---

# AI Integration

Your desktop character can respond to your AI conversations — reacting to events, narrating research findings, debating ideas with other characters, and displaying information in floating panels. AI Integration connects your favorite AI tools to Desktop Homunculus.

## What You Need

- **Desktop Homunculus** installed and running
- **An AI client** — any of the following:
  - [Claude Desktop](./setup/claude-desktop) (free)
  - [Claude Code](./setup/claude-code)
  - [Codex](./setup/codex)
  - [Other MCP-compatible clients](./setup/other-clients)
- **(Optional)** VoiceVox MOD installed for speech and narration

## What Your Character Can Do

| Capability | Description |
|---|---|
| **React & emote** | Facial expressions, animations, and preset reactions (happy, thinking, surprised, confused, and more) |
| **Speak** | Text-to-speech narration powered by VoiceVox |
| **Move** | Teleport or smooth animation to any screen position |
| **Show information** | Floating Webview panels anchored near the character — HTML content, dashboards, presentations |
| **Run MOD commands** | Trigger any installed MOD's functionality |
| **Have a personality** | Personas give each character distinct viewpoints for debates and reviews |

## How It Works

:::info For Developers
This section explains the technical architecture. If you just want to get started, skip to [Next Steps](#next-steps).
:::

Desktop Homunculus exposes character control through the **Model Context Protocol (MCP)**. An MCP server (`@hmcs/mcp-server`) acts as a bridge between AI agents and the application:

```
[AI Agent] ←— MCP (stdio) —→ [@hmcs/mcp-server] ←— HTTP —→ [Desktop Homunculus :3100]
```

- The MCP server is a Node.js subprocess spawned by the AI client
- It communicates with Desktop Homunculus via HTTP on `localhost:3100` (local only)
- Desktop Homunculus must be running for tools to work
- Set `HOMUNCULUS_HOST` environment variable for a non-default host or port

### MCP Primitives

| Primitive | Count | Purpose |
|---|---|---|
| **Tools** | 20 | Atomic actions — move character, speak, open webview, set expression, etc. |
| **Resources** | 4 | Read-only state — `homunculus://characters`, `homunculus://mods`, `homunculus://assets`, `homunculus://info` |
| **Prompts** | 3 | Pre-built interaction patterns (see below) |

### Prompts

Prompts are the easiest entry point for common workflows:

- **`developer-assistant`** — Auto-react to development events (build-success, test-fail, deploy, etc.). Character plays matching expressions, animations, and sounds.
- **`character-interaction`** — Natural conversation with your character. Supports mood parameters (happy, playful, serious, encouraging).
- **`mod-command-helper`** — Discover and explain available MOD commands. Reads the `homunculus://mods` resource to find what's installed.

### Self-Discovery Pattern

AI agents should read `homunculus://characters` and `homunculus://mods` resources before calling tools. This lets the agent understand what characters are loaded and what MOD commands are available. The `mod-command-helper` prompt models this pattern.

### Stateful Sessions

The MCP server tracks the active character within a session. Calling `select_character` affects all subsequent tool calls. Personas set via `set_persona` also persist within the session.

For the full tool reference, see [MCP Reference](/docs/reference/mcp-tools).

## Use Cases

- **Development event reactions** — Use the `developer-assistant` prompt. Your character automatically reacts with expressions, animations, and sounds when builds succeed or fail, tests pass or fail, or code is deployed.
- **Research & presentation** — Combine speech narration with Webview panels. Your character explains research findings while displaying supporting materials in a floating panel near the character.
- **Multi-character debates** — Spawn multiple characters with distinct personas. Each character contributes a different viewpoint to discussions. Works well with AI agent teams.
- **Code review companion** — Characters react to code changes and show review feedback in Webview panels. Different personas can focus on different aspects — security, performance, readability.

## Known Limitations

AI Integration is functional today, but some workflows have performance constraints:

- **Webview generation latency** — When the AI generates full HTML inline during inference, content appears slowly. This is inherent to real-time content generation.
- **TTS speech latency** — Speech generation through VoiceVox has noticeable delay for longer text.

### Path Forward: Template MODs

The solution is dedicated **template MODs** — MODs that ship pre-built Webview assets (presentation templates, dashboards, review UIs). Instead of the AI generating HTML from scratch, it fills in data via MOD commands. This leverages the existing MOD asset system where `open_webview` loads pre-built local assets.

If you'd like to help build template MODs or improve MCP tools, see the [Contributing guide](/docs/contributing).

## Next Steps {#next-steps}

- **[Set up your AI client](./setup/claude-desktop)** — Get connected in minutes
- **[Explore MCP capabilities](/docs/reference/mcp-tools)** — Full reference for all 20 tools, 4 resources, and 3 prompts
- **[Build a MOD](/docs/mod-development/quick-start)** — Create template MODs for richer AI workflows
