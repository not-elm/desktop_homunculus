# @hmcs/openclaw

OpenClaw integration settings UI for Desktop Homunculus.

## Purpose

- Provide a right-click context menu entry ("OpenClaw Settings") that opens a WebView.
- Let users pick a TTS engine (or disable TTS) per persona. Selections are persisted in `persona.metadata.ttsModName`.
- Discovers TTS engines at runtime via `rpc.registrations({ category: "tts" })`.
