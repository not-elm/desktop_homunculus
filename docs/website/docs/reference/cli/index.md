---
title: "CLI Reference"
sidebar_position: 1
---

# CLI Reference

`hmcs` is the Desktop Homunculus command-line interface.

## Quick Start

```shell
hmcs --help
hmcs prefs --help
hmcs config --help
hmcs mod --help
```

## Command Map

| Command           | Purpose                                               |
| ----------------- | ----------------------------------------------------- |
| `hmcs prefs ...`  | Read and write preference values in `preferences.db`. |
| `hmcs config ...` | Read and write app config values in `config.toml`.    |
| `hmcs mod ...`    | List, install, and uninstall MOD packages.            |

## Output and Exit Codes

- Successful commands exit with `0`.
- Failing commands exit with non-zero.
- Command output is written to stdout.
- Errors are written to stderr.

## Data Paths

| Data           | Path                           |
| -------------- | ------------------------------ |
| App config     | `~/.homunculus/config.toml`    |
| Preferences DB | `~/.homunculus/preferences.db` |

## Subcommands

- [hmcs prefs](./prefs)
- [hmcs config](./config)
- [hmcs mod](./mod)
