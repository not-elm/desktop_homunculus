# Release Workflow Design

## Overview

A unified GitHub Actions release workflow that replaces the existing `publish-cli.yml`. Triggered by `v*` tag push, it publishes npm packages, builds macOS engine bundles (DMG), auto-generates release notes via `.github/release.yml`, and attaches artifacts to the GitHub Release.

## Trigger

- Git tag push matching `v*` (e.g., `v0.1.0-alpha.5`)
- Replaces: `.github/workflows/publish-cli.yml`

## Job Dependency Graph

```
v* tag push
  │
  ├─ validate
  │   ├─ build-cli (3 platforms matrix)
  │   ├─ build-engine (macOS ARM64 + x86 matrix)
  │   │
  │   └─ publish-npm ◄── build-cli, build-engine
  │       │
  │       └─ create-release ◄── publish-npm
```

All builds must succeed before any npm publishing begins. This prevents partial release states where packages are published but no GitHub Release exists.

## Version Strategy

| Package | Version Source | npm dist-tag |
|---------|---------------|-------------|
| `@hmcs/cli` + platform packages | Git tag (`v0.1.0-alpha.4` → `0.1.0-alpha.4`) | Derived from tag version (alpha/beta/rc/latest) |
| `@hmcs/sdk` | `packages/sdk/package.json` (independent, e.g., `1.0.0`) | Derived from SDK's own version |
| `@hmcs/mcp-server` | `packages/mcp-server/package.json` (independent, e.g., `0.1.0`) | Derived from MCP's own version |
| Engine (Rust) | `engine/Cargo.toml` workspace version (must match git tag) | N/A |

## Job Details

### 1. validate

- **Runner:** `ubuntu-24.04`
- **Permissions:** `contents: read`
- **Outputs:** `version`, `is_prerelease`, `sdk_version`, `mcp_version`, `cli_tag`, `sdk_tag`, `mcp_tag`
- **Steps:**
  1. Extract version from git tag: `VERSION="${GITHUB_REF_NAME#v}"`
  2. Validate semver format (reject non-semver tags like `v-next`)
  3. Validate tag version matches `engine/Cargo.toml` `[workspace.package] version`
  4. Read `sdk_version` from `packages/sdk/package.json`
  5. Read `mcp_version` from `packages/mcp-server/package.json`
  6. Determine `is_prerelease` (tag contains `-alpha`, `-beta`, `-rc`)
  7. Compute per-package npm dist-tags from each package's version

### 2. build-cli

- **Needs:** `validate`
- **Runner:** Matrix build
  - `macos-14` → `aarch64-apple-darwin` (darwin-arm64)
  - `macos-13` → `x86_64-apple-darwin` (darwin-x64)
  - `windows-2022` → `x86_64-pc-windows-msvc` (win32-x64)
- **Permissions:** `contents: read`
- **Timeout:** 45 minutes
- **Steps:**
  1. Checkout
  2. Setup Rust toolchain + target
  3. Cache cargo registry and build artifacts
  4. `cargo build -p homunculus_cli --profile dist --target ${{ matrix.target }}`
  5. Upload binary as artifact (`cli-${{ matrix.platform }}`)

### 3. build-engine

- **Needs:** `validate`
- **Runner:** Matrix build
  - `macos-14` → `aarch64-apple-darwin` (arm64)
  - `macos-13` → `x86_64-apple-darwin` (x86)
- **Permissions:** `contents: read`
- **Timeout:** 60 minutes
- **Steps:**
  1. Checkout
  2. Setup Rust toolchain + target
  3. Setup Node.js 22 + corepack + pnpm
  4. Cache cargo, pnpm store, CEF framework
  5. `pnpm install --frozen-lockfile && pnpm build` (all TS packages for mod UIs)
  6. `make setup-cef` (download CEF framework, cached)
  7. Use appropriate `make release-macos-arm` or `make release-macos-x86` target (handles cargo build + app bundle + CEF bundle + DMG)
  8. Rename DMG with arch suffix: `DesktopHomunculus-$VERSION-$ARCH.dmg`
  9. Upload DMG as artifact (`engine-macos-${{ matrix.arch }}`)

### 4. publish-npm

- **Needs:** `validate`, `build-cli`, `build-engine`
- **Runner:** `ubuntu-24.04`
- **Environment:** `npm-publish` (single approval gate)
- **Permissions:** `contents: read`, `id-token: write` (OIDC for npm provenance)
- **Steps:**
  1. Checkout
  2. Setup Node.js 22 with registry URL
  3. `corepack enable && pnpm install --frozen-lockfile`
  4. Download all `cli-*` artifacts
  5. Build publishable packages: `pnpm --filter @hmcs/sdk build && pnpm --filter @hmcs/mcp-server build`
  6. **Publish `@hmcs/sdk`**
     - Idempotency check: `npm view @hmcs/sdk@$SDK_VERSION` (skip if exists)
     - `pnpm --filter @hmcs/sdk publish --provenance --access public --tag $SDK_TAG`
  7. **Publish `@hmcs/mcp-server`**
     - pnpm auto-resolves `workspace:*` → real SDK version on publish
     - Idempotency check: `npm view @hmcs/mcp-server@$MCP_VERSION`
     - `pnpm --filter @hmcs/mcp-server publish --provenance --access public --tag $MCP_TAG`
  8. **Publish CLI platform packages** (for each of 3 platforms)
     - Stage from `packages/cli-platform/package.json.tmpl` with variable substitution
     - Copy binary from downloaded artifact
     - Idempotency check
     - `npm publish --provenance --access public --tag $CLI_TAG`
  9. **Publish `@hmcs/cli`** main package
     - Idempotency check
     - `npm publish --provenance --access public --tag $CLI_TAG`

### 5. create-release

- **Needs:** `validate`, `publish-npm`
- **Runner:** `ubuntu-latest`
- **Permissions:** `contents: write`
- **Steps:**
  1. Download DMG artifacts (`engine-macos-arm64`, `engine-macos-x86`)
  2. Attempt `gh release create`:
     ```
     gh release create $TAG \
       --generate-notes \
       --prerelease=$IS_PRERELEASE \
       --title "Desktop Homunculus $VERSION" \
       ./DesktopHomunculus-$VERSION-arm64.dmg \
       ./DesktopHomunculus-$VERSION-x86_64.dmg
     ```
  3. If release already exists (rerun scenario):
     - `gh release upload $TAG --clobber` to upload/overwrite assets

## Cross-Cutting Concerns

### Concurrency

```yaml
concurrency:
  group: release-${{ github.ref_name }}
  cancel-in-progress: false
```

Prevents overlapping runs for the same tag without canceling in-progress releases.

### Caching

- **Cargo:** `~/.cargo/registry` + `engine/target` (keyed by `Cargo.lock` hash)
- **pnpm:** pnpm store (keyed by `pnpm-lock.yaml` hash)
- **CEF:** Cached by CEF version to avoid ~300MB download on every run

### Action Pinning

All GitHub Actions should be pinned to SHA for supply-chain security (e.g., `actions/checkout@v4` → `actions/checkout@<sha>`).

### Idempotency

- npm publish: Pre-check with `npm view`, skip if version exists
- Dist-tag enforcement: `npm dist-tag add` step ensures correct tags even on rerun
- GitHub Release: Create-or-upload pattern with `--clobber`

### Prerelease Detection

Tags containing `-alpha`, `-beta`, or `-rc` trigger:
- `is_prerelease: true` → GitHub Release marked as prerelease
- Per-package dist-tags adjusted accordingly

## Future Enhancements

- macOS code signing and notarization (Apple Developer certificate)
- Windows installer (.msi) build
- Linux AppImage/deb/rpm builds
- Checksums file (SHA256) for DMG downloads
- Slack/Discord notifications on release success/failure
- Universal binary DMG (requires post-build `lipo` step)

## Files Changed

| File | Action |
|------|--------|
| `.github/workflows/release.yml` | Create (new unified workflow) |
| `.github/workflows/publish-cli.yml` | Delete (replaced by release.yml) |
