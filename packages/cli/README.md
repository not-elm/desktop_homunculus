# @hmcs/cli

CLI for [Desktop Homunculus](https://github.com/not-elm/desktop_homunculus) — manage mods, preferences, and configuration.

## Install

```bash
npm install -g @hmcs/cli
```

## Usage

```bash
hmcs mod list              # List installed mods
hmcs mod install <pkg>     # Install a mod
hmcs mod uninstall <name>  # Uninstall a mod

hmcs prefs list            # List all preferences
hmcs prefs get <key>       # Get a preference value
hmcs prefs set <key> <val> # Set a preference
hmcs prefs delete <key>    # Delete a preference

hmcs config list           # List configuration
hmcs config get <key>      # Get a config value
hmcs config set <key> <val># Set a config value
```

## Supported Platforms

- macOS (Apple Silicon)
- macOS (Intel)
- Windows (x64)

## License

MIT
