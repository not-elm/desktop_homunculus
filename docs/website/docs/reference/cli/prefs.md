---
title: "hmcs prefs"
sidebar_position: 2
---

# hmcs prefs

Manage key-value preferences stored in SQLite.

## Quick Examples

```shell
hmcs prefs list
hmcs prefs get theme
hmcs prefs set theme dark
hmcs prefs set shadow_panel::alpha 0.5
hmcs prefs delete theme
```

## list

### Syntax

```shell
hmcs prefs list
```

### Arguments

This subcommand takes no arguments.

### Examples

Success:

```text
theme
shadow_panel::alpha
persona::elmer:vrm
```

No stored preferences:

```text
No preferences found.
```

Failure example:

```text
[stderr]
...database error...
```

### Behavior

- Prints keys only, one per line.
- Reads from `~/.homunculus/prefs.db`.

### Related

- [`hmcs prefs get`](#get)

## get

### Syntax

```shell
hmcs prefs get <key>
```

### Arguments

| Name | Required | Description |
|---|---|---|
| `key` | Yes | Preference key to load. |

### Examples

Success:

```shell
hmcs prefs get theme
```

```text
dark (string)
```

Success with JSON value:

```shell
hmcs prefs get profile
```

```text
{
  "voice": "ja",
  "speed": 1.1
} (json)
```

Failure example:

```shell
hmcs prefs get missing_key
```

```text
[stderr]
key not found: missing_key
```

### Behavior

- Prints `value (type)` where `type` is one of `null`, `bool`, `number`, `string`, `json`.
- Pretty-prints JSON values.
- Exits non-zero when the key does not exist.

### Related

- [`hmcs prefs set`](#set)
- [`hmcs prefs delete`](#delete)

## set

### Syntax

```shell
hmcs prefs set <key> <value>
```

### Arguments

| Name | Required | Description |
|---|---|---|
| `key` | Yes | Preference key to write. |
| `value` | Yes | Value string parsed by type inference. |

### Examples

Success:

```shell
hmcs prefs set theme dark
hmcs prefs set ui_scale 1.25
hmcs prefs set enabled true
hmcs prefs set profile '{"voice":"ja","speed":1.1}'
```

Failure example:

```text
[stderr]
...database error...
```

### Behavior

- Writes to `~/.homunculus/prefs.db`.
- Infers type in this order:
  1. `null`
  2. `bool`
  3. `number`
  4. JSON object or array
  5. `string`
- Prints no output on success.

### Related

- [`hmcs prefs get`](#get)
- [`hmcs prefs list`](#list)

## delete

### Syntax

```shell
hmcs prefs delete <key>
```

### Arguments

| Name | Required | Description |
|---|---|---|
| `key` | Yes | Preference key to remove. |

### Examples

Success:

```shell
hmcs prefs delete theme
```

Missing key is also treated as success:

```shell
hmcs prefs delete missing_key
```

Failure example:

```text
[stderr]
...database error...
```

### Behavior

- Deletes the key when present.
- Returns success even when the key does not exist.
- Prints no output on success.

### Related

- [`hmcs prefs list`](#list)
- [`hmcs config`](./config)
