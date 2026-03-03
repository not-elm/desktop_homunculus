---
title: "Troubleshooting"
sidebar_position: 5
---

# Troubleshooting

## Known Limitations

AI Integration is functional today, but some workflows have performance constraints:

### Webview Generation Latency

When the AI generates full HTML content inline during inference, Webview panels appear slowly. This is inherent to real-time content generation — the AI must produce the complete HTML before it can be displayed.

### TTS Speech Latency

Speech generation through VoiceVox has noticeable delay, especially for longer text. Each utterance requires synthesis processing before playback begins.

### Path Forward: Template MODs

Both limitations can be addressed by creating dedicated **template MODs** that ship pre-built Webview assets. Instead of generating HTML from scratch, the AI fills in data via MOD commands — significantly reducing latency.

If you'd like to help build template MODs or improve MCP tools, see the [Contributing guide](/docs/contributing).

## Common Issues

### Connection Refused

**Symptom:** MCP tools return connection errors.

**Cause:** Desktop Homunculus is not running.

**Solution:** Start Desktop Homunculus before using AI Integration. The MCP server communicates with the app via HTTP on `localhost:3100`.

### VoiceVox Errors

**Symptom:** `speak_message` fails or returns an error.

**Cause:** The VoiceVox MOD is not installed, or the VoiceVox engine is not running.

**Solution:** Install the VoiceVox MOD and ensure the VoiceVox engine is running. See the VoiceVox MOD documentation for setup instructions.

### No Characters Found

**Symptom:** `homunculus://characters` returns an empty list.

**Cause:** No VRM model is loaded in Desktop Homunculus.

**Solution:** Load a VRM character in Desktop Homunculus before using character-related tools.

### MCP Server Won't Connect

**Symptom:** The AI client can't connect to the MCP server.

**Cause:** The MCP server failed to start or Node.js is missing/outdated.

**Solution:**
1. Verify that `npx -y @hmcs/mcp-server@latest` runs without errors
2. Ensure Node.js >= 22 is installed (`node --version`)

### Tools Return Unexpected Errors

**Symptom:** Tools return error responses.

**Cause:** Desktop Homunculus HTTP API is unreachable.

**Solution:** Verify the app is running on the expected port (default: 3100). If using a non-default port, set the `HOMUNCULUS_HOST` environment variable.

## Getting Help

If you encounter an issue not listed here, please [open an issue on GitHub](https://github.com/not-elm/desktop-homunculus/issues).
