---
title: Version Management
sidebar_position: 3
---

# Version Management

This guide explains how versions are managed across the Desktop Homunculus monorepo.

## Overview

All package versions are managed from a single source: **`version.toml`** at the repository root.

```toml
version = "0.1.0-alpha.5-dev"

targets = [
    "engine/Cargo.toml",
    "packages/*/package.json",
    "packages/cli-platform/package.json.tmpl",
    "mods/*/package.json",
]

excludes = [
    "sandbox/package.json",
    "docs/website/package.json",
]
```

| Field | Description |
|-------|-------------|
| `version` | The current version string |
| `targets` | Glob patterns for files whose version is managed |
| `excludes` | Glob patterns for files explicitly excluded from version management |

## Bumping & Syncing Versions

### Bump to a new version

```shell
make bump-version VERSION=0.2.0
```

This updates `version.toml` to the new version, then propagates it to all target files.

### Re-sync existing version

```shell
make bump-version
```

When called without `VERSION`, the script reads the current version from `version.toml` and propagates it to all targets. This is useful when a new target file has been added or when a target is out of sync.

Both commands automatically run `cargo update --workspace` after propagation to keep `Cargo.lock` in sync.

### Version format

Versions must match the pattern `X.Y.Z` or `X.Y.Z-<prerelease>`:

- `0.2.0` — stable release
- `0.2.0-alpha.1` — prerelease
- `0.1.0-alpha.5-dev` — development version (not releasable; see [Release & CI](#release--ci))

## Checking Version Consistency

```shell
make check-version
```

This verifies that all managed files are in sync with `version.toml`.

**Errors** (exit code 1):

- A target file's version does not match `version.toml`

**Warnings** (does not fail):

- A package in `pnpm-workspace.yaml` is not covered by `targets` or `excludes` — indicates a newly added package may need to be tracked
- A Rust crate in `engine/crates/` does not use `version.workspace = true` — indicates the crate may have a hardcoded version

`docs/website` and `sandbox` are intentionally listed in `excludes` because they are private packages that are not published.

:::note Windows users
`check-version` outputs Unicode symbols (e.g. `✓`, `⚠`, `✗`). On Windows terminals using CP932 encoding, set `PYTHONUTF8=1` or use a UTF-8 terminal to avoid encoding errors.
:::

## How Version Propagation Works

The bump script handles each file type differently:

### Rust

Only `engine/Cargo.toml`'s `[workspace.package].version` field is directly updated. Individual crates inherit the version via `version.workspace = true` in their own `Cargo.toml` — the script does not modify crate-level files.

### TypeScript

`packages/*/package.json` and `mods/*/package.json` have their `"version"` field directly updated.

### Template

`packages/cli-platform/package.json.tmpl` is a template file used to generate platform-specific packages during release. The script replaces either a `{{VERSION}}` placeholder or an existing version string. `check-version` does not flag `{{...}}` placeholders as mismatches.

### Cargo.lock

After all files are updated, the script runs `cargo update --workspace` to refresh `Cargo.lock`.

## Adding a New Crate or Package

### Rust crate

Add `version.workspace = true` to the crate's `Cargo.toml`. No changes to `version.toml` are needed — the workspace inheritance handles it automatically. `make check-version` will warn if a crate does not use workspace version inheritance.

### TypeScript package

If the package matches an existing glob pattern in `targets` (e.g., `packages/*/package.json`), no changes are needed. Otherwise, add a new glob pattern to `targets` in `version.toml`.

To exclude a package from version management (e.g., private internal tools), add it to `excludes` instead.

## Release & CI

Releases are triggered by pushing a `v*` tag (e.g., `v0.2.0`).

The release workflow (`.github/workflows/release.yml`) validates:

1. The tag is a valid semver version
2. The tag version (with `v` prefix stripped) matches `version.toml`

:::warning
CI only validates the tag against `version.toml`. It does **not** check that all target files are in sync. Run `make check-version` locally before tagging a release.
:::

### npm dist-tag

The npm dist-tag is computed from the version string (`.github/scripts/compute-tag.sh`):

| Version pattern | dist-tag |
|----------------|----------|
| Contains `-alpha` | `alpha` |
| Contains `-beta` | `beta` |
| Contains `-rc` | `rc` |
| Everything else | `latest` |

:::caution
Versions with `-dev` suffix (e.g., `0.1.0-alpha.5-dev`) will fail CI's semver validation because the release workflow does not allow hyphens within the prerelease segment. The `-dev` suffix is intended for local development only — do not use it for release versions.
:::
