---
title: "hmcs mod"
sidebar_position: 4
---

# hmcs mod

List, install, and uninstall MOD packages.

## Quick Examples

```shell
hmcs mod list
hmcs mod install @hmcs/assets @hmcs/elmer
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
 NAME           VERSION  DESCRIPTION
 @hmcs/elmer    1.0.0    Default character model
 @hmcs/menu     1.0.0    Context menu
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
| `package` | Yes | One or more package specifiers (for example `@hmcs/elmer` or `pkg@version`). |

### Examples

Success:

```shell
hmcs mod install @hmcs/assets @hmcs/elmer
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
hmcs mod uninstall @hmcs/assets @hmcs/elmer
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
