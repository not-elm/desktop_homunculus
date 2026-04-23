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

If you'd like to help build template MODs or improve MCP tools, see the [Contributing guide](/contributing).

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

**Cause:** Desktop Homunculus is not running, or the client is configured with the wrong URL.

**Solution:**
1. Ensure Desktop Homunculus is running
2. Verify the MCP URL is `http://localhost:3100/mcp` (or your custom port)
3. Test the endpoint: `curl http://localhost:3100/mcp` should return a response (not connection refused)

### Tools Return Unexpected Errors

**Symptom:** Tools return error responses.

**Cause:** Desktop Homunculus HTTP API is unreachable or internal error.

**Solution:** Verify the app is running on the expected port (default: 3100). Check the logs at `~/.homunculus/Logs/log.txt` for details.

### Agent MOD Won't Start

**Symptom:** Agent session fails to start or immediately disconnects.

**Cause:** Missing or invalid API key, or runtime not configured.

**Solution:**
1. Right-click the character → "Agent" to open settings
2. Verify the correct runtime is selected (Claude SDK or Codex AppServer)
3. For Claude runtime: ensure a valid Anthropic API key is entered
4. Check the logs at `~/.homunculus/Logs/log.txt` for detailed errors

### STT Not Recognizing Speech

**Symptom:** Push-to-Talk records but returns empty or incorrect text.

**Cause:** No Whisper model downloaded, microphone not accessible, or model too small.

**Solution:**
1. Open **system tray** → **"Speech to Text"** and verify a model is downloaded
2. Check that your OS has granted microphone permission to Desktop Homunculus
3. Try a larger model size (e.g., switch from `tiny` to `base` or `small`) for better accuracy

### PTT Key Not Responding

**Symptom:** Pressing the Push-to-Talk key does nothing.

**Cause:** Key mapping conflict or agent session not active.

**Solution:**
1. Verify the PTT key is configured in Agent settings (right-click → "Agent" → settings)
2. Check for OS-level keyboard shortcut conflicts (macOS System Settings → Keyboard → Shortcuts)
3. Ensure an agent session is active — PTT only works during an active session

## Getting Help

If you encounter an issue not listed here, please [open an issue on GitHub](https://github.com/not-elm/desktop-homunculus/issues).
