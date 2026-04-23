---
title: "hmcs mod"
sidebar_position: 4
---

# hmcs mod

List, install, and uninstall MOD packages.

## Quick Examples

```shell
hmcs mod list
hmcs mod install @hmcs/assets @hmcs/persona
hmcs mod uninstall @hmcs/assets
```

## list

### Syntax

```shell
hmcs mod list
```

### Arguments

This subcommand takes no arguments.

### Examples

Success:

```text
 NAME            VERSION  DESCRIPTION
 @hmcs/persona   1.0.0    Persona management
 @hmcs/menu      1.0.0    Context menu
```

No installed MODs:

```text
(no output)
```

Failure example:

```text
[stderr]
...pnpm ls failed...
```

### Behavior

- Lists installed MOD metadata from the configured mods directory.
- Uses `pnpm -C <mods_dir> ls --parseable -P --depth 0` internally.

### Related

- [`hmcs mod install`](#install)
- [`hmcs mod uninstall`](#uninstall)

## install

### Syntax

```shell
hmcs mod install <package>...
```

### Arguments

| Name | Required | Description |
|---|---|---|
| `package` | Yes | One or more package specifiers (for example `@hmcs/persona` or `pkg@version`). |

### Examples

Success:

```shell
hmcs mod install @hmcs/assets @hmcs/persona
```

Failure example (invalid package name):

```shell
hmcs mod install 'foo;rm -rf /'
```

```text
[stderr]
invalid package name: contains forbidden characters: foo;rm -rf /
```

Failure example (pnpm add failed):

```text
[stderr]
pnpm add failed with status: ...
```

### Behavior

- Validates package names before calling `pnpm`.
- Installs into the configured `mods_dir`.
- Exits non-zero on validation or install failure.

### Related

- [`hmcs mod list`](#list)
- [`hmcs mod uninstall`](#uninstall)

## uninstall

### Syntax

```shell
hmcs mod uninstall <package>...
```

### Arguments

| Name | Required | Description |
|---|---|---|
| `package` | Yes | One or more installed package names. |

### Examples

Success:

```shell
hmcs mod uninstall @hmcs/assets @hmcs/persona
```

Failure example (invalid package name):

```shell
hmcs mod uninstall '../etc/passwd'
```

```text
[stderr]
invalid package name: contains path traversal: ../etc/passwd
```

Failure example (pnpm remove failed):

```text
[stderr]
pnpm remove failed with status: ...
```

### Behavior

- Validates package names before calling `pnpm`.
- Removes packages from the configured `mods_dir`.
- Exits non-zero on validation or uninstall failure.

### Related

- [`hmcs mod list`](#list)
- [`hmcs config`](./config)

## path

### Syntax

```shell
hmcs mod path [mods_dir_path]
```

### Arguments

| Name | Required | Description |
|---|---|---|
| `mods_dir_path` | No | New mods directory path. If omitted, displays the current path. |

### Examples

Display the current mods directory:

```shell
hmcs mod path
```

```text
/Users/alice/.homunculus/mods
```

Update the mods directory:

```shell
hmcs mod path ~/custom-mods
```

```text
mods_dir updated to: /Users/alice/custom-mods
```

Failure example (cannot create directory):

```text
[stderr]
failed to create directory "/readonly/path": Permission denied
```

### Behavior

- Without arguments, prints the current `mods_dir` from `~/.homunculus/config.toml`.
- With a path argument, resolves the path (expanding `~` and relative paths), creates the directory if it does not exist, and saves the updated path to `config.toml`.
- Exits non-zero if the directory cannot be created or the config cannot be saved.

### Related

- [`hmcs mod list`](#list)
- [`hmcs config`](./config)

## update

### Syntax

```shell
hmcs mod update [mod_patterns...] [--latest|-L]
```

### Arguments

| Name | Required | Description |
|---|---|---|
| `mod_patterns` | No | One or more mod name patterns to update. If omitted, all installed MODs are updated. |
| `--latest`, `-L` | No | Update MODs to their latest versions. |

### Examples

Update all installed MODs:

```shell
hmcs mod update
```

Update specific MODs:

```shell
hmcs mod update @hmcs/persona @hmcs/assets
```

Update all MODs to their latest versions:

```shell
hmcs mod update --latest
```

Update a specific MOD to the latest version:

```shell
hmcs mod update @hmcs/persona -L
```

Failure example (pnpm update failed):

```text
[stderr]
pnpm update failed with status: ...
```

### Behavior

- Uses `pnpm update` internally in the configured `mods_dir`.
- Without `mod_patterns`, updates all installed MODs.
- With `--latest` / `-L`, passes `--latest` to `pnpm update` to install the newest available versions.
- Exits non-zero on update failure.

### Related

- [`hmcs mod list`](#list)
- [`hmcs mod install`](#install)
