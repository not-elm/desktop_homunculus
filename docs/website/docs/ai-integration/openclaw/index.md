---
title: "OpenClaw Integration"
sidebar_position: 1
---

# OpenClaw Integration

`@hmcs/openclaw-plugin` bridges [OpenClaw](https://docs.openclaw.ai) agents with Desktop Homunculus characters. Install it into your OpenClaw setup and your Homunculus personas become OpenClaw workspaces; agent replies render on the matching character — speech bubbles, and spoken aloud if the VoiceVox MOD is installed.

This page covers the end-to-end setup. For a broader comparison with external MCP clients (Claude Desktop, Claude Code, Codex), see the [AI Integration overview](/ai-integration).

## Prerequisites

- **Desktop Homunculus** running. The plugin reaches it over HTTP at `http://127.0.0.1:3100` by default.
- **OpenClaw** installed. See the [OpenClaw docs](https://docs.openclaw.ai) for installation.
- **Node.js ≥ 22** (required by OpenClaw).
- **[`@hmcs/persona` MOD](/mods/persona)** with at least one persona created. The plugin syncs Homunculus personas to OpenClaw workspaces; it has nothing to do until a persona exists.
- *(Optional)* **[`@hmcs/voicevox` MOD](/mods/voicevox)** to hear replies read aloud.
- *(Optional)* **[`@hmcs/stt` MOD](/mods/stt)** to talk to agents by voice.

## Install the plugin

From any terminal:

```bash
openclaw plugins install @hmcs/openclaw-plugin
```

OpenClaw fetches the package and registers it as an extension. Restart OpenClaw if it was already running.

## Register an OpenClaw agent for each persona

The plugin does not create OpenClaw agents automatically. For every Homunculus persona you want to drive, register a matching agent whose ID equals the persona ID:

```bash
openclaw agents add <persona-id>
```

You can list your Homunculus personas from the persona MOD UI or via the HTTP API:

```bash
curl http://127.0.0.1:3100/personas | jq '.[].id'
```

Skipping this step is harmless — the plugin just logs a warning for each unmatched persona and waits.

## Configure (optional)

The plugin reads configuration from OpenClaw's plugin config. Defaults work for the standard Homunculus setup:

| Key | Default | When to change |
|---|---|---|
| `hmcsBaseUrl` | `http://127.0.0.1:3100` | Override if you changed `port` in `~/.homunculus/config.toml`. |
| `soulMaxChars` | `10000` | Override to cap the per-persona soul prompt length written into each agent workspace. |

Refer to the [OpenClaw plugin configuration docs](https://docs.openclaw.ai) for where to set these values.

## Verify it works

1. Start Desktop Homunculus and summon a character.
2. Create a persona in the persona MOD UI (or confirm one already exists).
3. Run `openclaw agents add <persona-id>` for that persona.
4. Start OpenClaw and open the workspace for that agent.
5. Send a message. The matching Homunculus character should react — the reply appears as a speech bubble (and is spoken if VoiceVox is installed).

## Next steps

- [Persona MOD](/mods/persona) — create and manage personas.
- [VoiceVox MOD](/mods/voicevox) — add text-to-speech to replies.
- [STT MOD](/mods/stt) — talk to agents by voice.
- [AI Integration overview](/ai-integration) — compare OpenClaw with MCP-based integrations.
