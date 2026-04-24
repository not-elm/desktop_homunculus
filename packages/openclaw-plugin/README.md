# @hmcs/openclaw-plugin

OpenClaw plugin that bridges agent output to [Desktop Homunculus](https://github.com/not-elm/desktop-homunculus) characters. Renders agent replies as speech on VRM mascots and keeps persona state synchronized with OpenClaw sessions.

## Installation

Install into your OpenClaw runtime:

```bash
openclaw plugins install @hmcs/openclaw-plugin
```

Desktop Homunculus must be running locally (default `http://127.0.0.1:3100`) for the plugin to dispatch replies.

## Configuration

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `hmcsBaseUrl` | `string` | `http://127.0.0.1:3100` | Base URL of the Desktop Homunculus HTTP server. |
| `soulMaxChars` | `integer` | `10000` | Maximum character length carried per persona's soul/context buffer. |

Set via the OpenClaw plugin configuration UI or `openclaw.plugin.json`.

## Related

- [Desktop Homunculus](https://github.com/not-elm/desktop-homunculus) — the host application
- [OpenClaw](https://docs.openclaw.ai) — the agent runtime this plugin extends

## License

MIT
