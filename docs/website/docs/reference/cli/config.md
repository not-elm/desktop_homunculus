---
title: "hmcs config"
sidebar_position: 3
---

# hmcs config

Manage Desktop Homunculus app configuration.

## Quick Examples

```shell
hmcs config list
hmcs config get port
hmcs config set port 3200
hmcs config set mods_dir /Users/me/.homunculus/mods
hmcs config reset port
hmcs config reset --all
```

## list

### Syntax

```shell
hmcs config list
```

### Arguments

This subcommand takes no arguments.

### Examples

Success:

```text
KEY      VALUE
mods_dir /Users/me/.homunculus/mods
port     3100
```

Failure example:

```text
[stderr]
...failed to parse ~/.homunculus/config.toml...
```

### Behavior

- Loads config from `~/.homunculus/config.toml`.
- Prints a table with KEY and VALUE columns sorted by key.
- If no config file exists, defaults are used.

### Related

- [`hmcs config get`](#get)
- [`hmcs config set`](#set)
- [`hmcs config reset`](#reset)

## get

### Syntax

```shell
hmcs config get <key>
```

### Arguments

| Name | Required | Description |
|---|---|---|
| `key` | Yes | Config key to read. |

### Examples

Success:

```shell
hmcs config get port
```

```text
3100
```

Failure example:

```shell
hmcs config get foo
```

```text
[stderr]
error: unknown config key 'foo'. ...valid keys: mods_dir, port
```

### Behavior

- Current keys are `mods_dir` and `port`.
- Exits non-zero for unknown keys.

### Related

- [`hmcs config list`](#list)
- [`hmcs config set`](#set)
- [`hmcs config reset`](#reset)

## set

### Syntax

```shell
hmcs config set <key> <value>
```

### Arguments

| Name | Required | Description |
|---|---|---|
| `key` | Yes | Config key to write (`mods_dir` or `port`). |
| `value` | Yes | New value. Parsed as TOML literal when possible. |

### Examples

Success:

```shell
hmcs config set port 3200
hmcs config set mods_dir /Users/me/.homunculus/mods
```

Failure example (unknown key):

```shell
hmcs config set foo bar
```

```text
[stderr]
error: unknown config key 'foo'. ...valid keys: mods_dir, port
```

Failure example (invalid type):

```shell
hmcs config set port not_a_number
```

```text
[stderr]
error: invalid value for 'port': ...
```

### Behavior

- Reads current config, applies one key change, then writes back.
- Value parsing order:
  1. Parse as TOML literal (for numbers, booleans, quoted strings).
  2. If parsing fails, treat the value as a plain string.
- Exits non-zero when validation fails.

### Related

- [`hmcs config get`](#get)
- [`hmcs config reset`](#reset)
- [`hmcs mod`](./mod)

## reset

### Syntax

```shell
hmcs config reset <key>
hmcs config reset --all
```

### Arguments

| Name | Required | Description |
|---|---|---|
| `key` | No | Config key to reset to its default value. |
| `--all` | No | Reset all keys to their default values. |

Either `key` or `--all` must be specified.

### Examples

Reset a single key:

```shell
hmcs config reset port
```

```text
port = 3100
```

Reset all keys:

```shell
hmcs config reset --all
```

```text
all config reset to defaults
```

Failure example (no argument):

```shell
hmcs config reset
```

```text
[stderr]
error: specify a key to reset, or use --all to reset all config
```

### Behavior

- With a key: replaces that key's value with its default and saves. Prints `{key} = {default_value}`.
- With `--all`: saves default config (`port = 3100`, `mods_dir = ~/.homunculus/mods/`).
- Exits non-zero for unknown keys.

### Related

- [`hmcs config get`](#get)
- [`hmcs config set`](#set)
