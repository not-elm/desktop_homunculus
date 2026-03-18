---
title: Development Setup
sidebar_position: 2
---

# Development Setup

This guide covers the tools you need to install before contributing to Desktop Homunculus.

## Prerequisites

### All Contributors

| Tool | Version | Link |
|------|---------|------|
| Git | latest | [git-scm.com](https://git-scm.com/) |
| Node.js | 22+ | [nodejs.org](https://nodejs.org/) (npm is bundled and used by setup scripts) |
| pnpm | 10.x | [pnpm.io](https://pnpm.io/) |

### Engine (Rust) Contributors

All of the above, plus:

| Tool | Version | Link |
|------|---------|------|
| Rust | latest stable | [rustup.rs](https://rustup.rs/) |
| Python | 3.x | [python.org](https://www.python.org/) (required by setup scripts) |
| Make | latest | Included with Xcode Command Line Tools (macOS) or build tools (Windows/Linux) |

### Platform-Specific Notes

- **macOS**: Install Xcode Command Line Tools — `xcode-select --install`
- **Windows**: Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the **C++ workload**

## Clone & Setup

```shell
git clone https://github.com/not-elm/desktop-homunculus.git
cd desktop-homunculus

# Install all dependencies (Node packages, Rust tooling, global npm tools, CEF framework)
make setup

# Start the app in debug mode (with hot-reload and DevTools)
make debug
```

`make setup` installs Node dependencies, Rust tooling, global npm tools (e.g. `@redocly/cli`), and downloads the CEF framework. See the root `Makefile` for all available commands.

## Next Steps

See [Contributing](/contributing) for how to contribute, PR guidelines, and wanted contribution areas.
