# Desktop Homunculus

> [!NOTE]
> This project is in **early alpha** (`v0.1.0-alpha.4`). APIs, features, and MOD interfaces may change without notice.

<img src="./docs/images/icon.png" width="200" alt="Desktop Homunculus">

**A cross-platform desktop mascot with AI-powered 3D VRM characters**

Bring your desktop to life with interactive 3D VRM characters. Desktop Homunculus renders transparent-window mascots that can sit on windows, be dragged around, and respond to your actions — all extensible through a MOD system.

## Documentation

[![Read the Docs](https://img.shields.io/badge/Read%20the%20Docs-Desktop%20Homunculus-0A7EA4?style=for-the-badge)](https://not-elm.github.io/desktop_homunculus/)

| I want to... | Go here |
| ------------ | ------- |
| Install and run quickly | [Getting Started](https://not-elm.github.io/desktop_homunculus/getting-started/) |
| Build MODs (SDK, WebView UI, assets) | [MOD Development](https://not-elm.github.io/desktop_homunculus/mod-development/) |
| Integrate AI agents (MCP, Claude Code, Codex) | [AI Integration](https://not-elm.github.io/desktop_homunculus/ai-integration/) |
| Look up CLI and MCP tools | [Reference](https://not-elm.github.io/desktop_homunculus/reference/) |

## Features

- **VRM 3D Characters** — Display multiple VRM models simultaneously with VRMA animations and multi-monitor support
- **Extensible MOD System** — Build custom extensions with the TypeScript SDK, HTTP API, and WebView-based UIs
- **AI Integration** — Control characters from AI agents via the built-in MCP server
- **Power Efficient** — Dynamic FPS limiting to conserve battery life

## Download

- [Github Releases](https://github.com/not-elm/desktop_homunculus/releases)
- [itch.io](https://notelm.itch.io/desktop-homunculus)
- [BOOTH](https://notelm.booth.pm/items/6904924)

> [!WARNING]
> **Windows with NVIDIA GPU:** You must configure the NVIDIA Control Panel **before first launch**, or the window will have a black background instead of being transparent. See the [Installation Guide](https://not-elm.github.io/desktop_homunculus/getting-started/installation) for step-by-step instructions.

## Platform Support

| Platform | Status                                             |
| -------- | -------------------------------------------------- |
| macOS    | Fully supported                                    |
| Windows  | Supported (NVIDIA GPU configuration required) |
| Linux    | Planned                                            |

## Contributing

See the [Contributing Guide](https://not-elm.github.io/desktop_homunculus/contributing) for guidelines.

## License

This project uses a three-lane licensing model:

- **Rust code** (engine, CLI): [MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)
- **TypeScript code** (SDK, UI, MCP server, mods): [MIT](./LICENSE-MIT)
- **Creative assets & documentation**: [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/)

See each package's `package.json` or `Cargo.toml` for its specific license.
