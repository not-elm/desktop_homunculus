# Release Workflow Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create a unified GitHub Actions release workflow that publishes npm packages, builds macOS engine DMGs, and creates GitHub Releases with auto-generated notes.

**Architecture:** Single workflow file (`.github/workflows/release.yml`) with 5 jobs: validate → build-cli + build-engine (parallel) → publish-npm → create-release. Replaces the existing `publish-cli.yml`.

**Tech Stack:** GitHub Actions, pnpm, npm provenance (OIDC), Cargo (Rust), Makefile (macOS bundling), `gh` CLI

---

### Task 1: Create the workflow skeleton with validate job

**Files:**
- Create: `.github/workflows/release.yml`

**Step 1: Create the workflow file with trigger, concurrency, and validate job**

```yaml
name: Release

on:
  push:
    tags:
      - "v*"

concurrency:
  group: release-${{ github.ref_name }}
  cancel-in-progress: false

jobs:
  validate:
    runs-on: ubuntu-24.04
    permissions:
      contents: read
    outputs:
      version: ${{ steps.check.outputs.version }}
      is_prerelease: ${{ steps.check.outputs.is_prerelease }}
      sdk_version: ${{ steps.check.outputs.sdk_version }}
      mcp_version: ${{ steps.check.outputs.mcp_version }}
      cli_tag: ${{ steps.check.outputs.cli_tag }}
      sdk_tag: ${{ steps.check.outputs.sdk_tag }}
      mcp_tag: ${{ steps.check.outputs.mcp_tag }}
    steps:
      - uses: actions/checkout@v4

      - name: Validate tag and extract versions
        id: check
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          echo "version=$VERSION" >> "$GITHUB_OUTPUT"

          # Validate semver format (reject non-semver tags)
          if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$'; then
            echo "::error::Tag '$GITHUB_REF_NAME' is not a valid semver version"
            exit 1
          fi

          # Validate tag version matches engine/Cargo.toml
          CARGO_VERSION=$(sed -n '/\[workspace\.package\]/,/\[/{ s/^version = "\(.*\)"/\1/p; }' engine/Cargo.toml)
          if [ "$VERSION" != "$CARGO_VERSION" ]; then
            echo "::error::Tag version ($VERSION) does not match Cargo.toml version ($CARGO_VERSION)"
            exit 1
          fi

          # Read SDK and MCP server versions from package.json
          SDK_VERSION=$(node -p "require('./packages/sdk/package.json').version")
          MCP_VERSION=$(node -p "require('./packages/mcp-server/package.json').version")
          echo "sdk_version=$SDK_VERSION" >> "$GITHUB_OUTPUT"
          echo "mcp_version=$MCP_VERSION" >> "$GITHUB_OUTPUT"

          # Prerelease detection
          if echo "$VERSION" | grep -qE '-(alpha|beta|rc)'; then
            echo "is_prerelease=true" >> "$GITHUB_OUTPUT"
          else
            echo "is_prerelease=false" >> "$GITHUB_OUTPUT"
          fi

          # Per-package dist-tag computation
          compute_tag() {
            local ver="$1"
            if echo "$ver" | grep -qE '-(alpha|beta|rc)'; then
              # Extract prerelease label (e.g., "alpha" from "0.1.0-alpha.4")
              echo "$ver" | sed -E 's/.*-(alpha|beta|rc).*/\1/'
            else
              echo "latest"
            fi
          }

          echo "cli_tag=$(compute_tag "$VERSION")" >> "$GITHUB_OUTPUT"
          echo "sdk_tag=$(compute_tag "$SDK_VERSION")" >> "$GITHUB_OUTPUT"
          echo "mcp_tag=$(compute_tag "$MCP_VERSION")" >> "$GITHUB_OUTPUT"
```

**Step 2: Verify the YAML is valid**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "feat: add release workflow skeleton with validate job"
```

---

### Task 2: Add the build-cli job

**Files:**
- Modify: `.github/workflows/release.yml`

**Step 1: Add the build-cli job after validate**

Append this job to the `jobs:` section. This is migrated from `publish-cli.yml:13-66` with the same matrix and steps.

```yaml
  build-cli:
    needs: validate
    timeout-minutes: 45
    permissions:
      contents: read
    strategy:
      matrix:
        include:
          - os: macos-14
            target: aarch64-apple-darwin
            platform-name: darwin-arm64
            exe-suffix: ""
          - os: macos-13
            target: x86_64-apple-darwin
            platform-name: darwin-x64
            exe-suffix: ""
          - os: windows-2022
            target: x86_64-pc-windows-msvc
            platform-name: win32-x64
            exe-suffix: ".exe"
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: "engine -> target"
          shared-key: ${{ matrix.platform-name }}-dist

      - name: Build hmcs binary
        working-directory: engine
        run: cargo build -p homunculus_cli --profile dist --target ${{ matrix.target }} --locked

      - name: Verify binary (non-Windows)
        if: matrix.exe-suffix == ''
        working-directory: engine
        run: |
          chmod +x target/${{ matrix.target }}/dist/hmcs
          target/${{ matrix.target }}/dist/hmcs --version

      - name: Verify binary (Windows)
        if: matrix.exe-suffix == '.exe'
        working-directory: engine
        run: target/${{ matrix.target }}/dist/hmcs.exe --version

      - name: Upload binary artifact
        uses: actions/upload-artifact@v4
        with:
          name: hmcs-${{ matrix.platform-name }}
          path: engine/target/${{ matrix.target }}/dist/hmcs${{ matrix.exe-suffix }}
```

**Step 2: Verify YAML validity**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "feat: add build-cli job to release workflow"
```

---

### Task 3: Add the build-engine job

**Files:**
- Modify: `.github/workflows/release.yml`

**Step 1: Add the build-engine job**

This job builds macOS .app bundles and DMGs using the existing Makefile targets. Key points:
- Uses `make setup` to install cargo tools (`bevy_cef_bundle_app`, `cargo-about`, etc.)
- Uses the correct `make release-macos-arm` or `make release-macos-x86` target per arch
- Renames DMG output (`target/bundle/desktop_homunculus.dmg`) with arch suffix to avoid collisions

```yaml
  build-engine:
    needs: validate
    timeout-minutes: 60
    permissions:
      contents: read
    strategy:
      matrix:
        include:
          - os: macos-14
            arch: arm64
            make-target: release-macos-arm
          - os: macos-13
            arch: x86
            make-target: release-macos-x86
    runs-on: ${{ matrix.os }}
    defaults:
      run:
        working-directory: engine
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: "engine -> target"
          shared-key: ${{ matrix.arch }}-engine-dist

      - uses: pnpm/action-setup@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm

      - name: Cache CEF framework
        uses: actions/cache@v4
        with:
          path: ~/.local/share/Chromium Embedded Framework.framework
          key: cef-${{ matrix.arch }}-144.4.0

      - name: Install dependencies
        run: |
          cd ..
          pnpm install --frozen-lockfile
          pnpm build

      - name: Setup engine tools and CEF
        run: make setup

      - name: Build, bundle, and create DMG
        run: make ${{ matrix.make-target }}

      - name: Rename DMG with arch suffix
        run: |
          VERSION="${{ needs.validate.outputs.version }}"
          mv target/bundle/desktop_homunculus.dmg \
             target/bundle/DesktopHomunculus-${VERSION}-${{ matrix.arch }}.dmg

      - name: Upload DMG artifact
        uses: actions/upload-artifact@v4
        with:
          name: engine-macos-${{ matrix.arch }}
          path: engine/target/bundle/DesktopHomunculus-${{ needs.validate.outputs.version }}-${{ matrix.arch }}.dmg
```

**Step 2: Verify YAML validity**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "feat: add build-engine job to release workflow"
```

---

### Task 4: Add the publish-npm job

**Files:**
- Modify: `.github/workflows/release.yml`

**Step 1: Add the publish-npm job**

This job publishes all npm packages sequentially (SDK → MCP → CLI platforms → CLI main) under a single `npm-publish` environment approval. Key points:
- Gates on both build jobs (`build-cli` + `build-engine`) to prevent partial releases
- Uses per-package dist-tags from validate outputs
- Uses `npm view` pre-check for idempotency (matching existing `publish-cli.yml` pattern)
- `pnpm --filter` publish auto-resolves `workspace:*` references
- Uses staging directories for CLI platform packages (from template)
- CLI main package gets version/optionalDependencies updated from git tag

```yaml
  publish-npm:
    needs: [validate, build-cli, build-engine]
    runs-on: ubuntu-24.04
    timeout-minutes: 15
    environment: npm-publish
    permissions:
      contents: read
      id-token: write
    steps:
      - uses: actions/checkout@v4

      - uses: pnpm/action-setup@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          registry-url: https://registry.npmjs.org

      - name: Install dependencies
        run: pnpm install --frozen-lockfile

      - name: Download CLI artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts
          pattern: hmcs-*

      # --- Publish @hmcs/sdk ---
      - name: Build @hmcs/sdk
        run: pnpm --filter @hmcs/sdk build

      - name: Publish @hmcs/sdk
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
        run: |
          VERSION="${{ needs.validate.outputs.sdk_version }}"
          TAG="${{ needs.validate.outputs.sdk_tag }}"

          if npm view "@hmcs/sdk@${VERSION}" version 2>/dev/null; then
            echo "@hmcs/sdk@${VERSION} already published, skipping."
          else
            pnpm --filter @hmcs/sdk publish --provenance --access public --tag "${TAG}" --no-git-checks
          fi

      # --- Publish @hmcs/mcp-server ---
      - name: Build @hmcs/mcp-server
        run: pnpm --filter @hmcs/mcp-server build

      - name: Publish @hmcs/mcp-server
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
        run: |
          VERSION="${{ needs.validate.outputs.mcp_version }}"
          TAG="${{ needs.validate.outputs.mcp_tag }}"

          if npm view "@hmcs/mcp-server@${VERSION}" version 2>/dev/null; then
            echo "@hmcs/mcp-server@${VERSION} already published, skipping."
          else
            pnpm --filter @hmcs/mcp-server publish --provenance --access public --tag "${TAG}" --no-git-checks
          fi

      # --- Publish CLI platform packages ---
      - name: Publish CLI platform packages
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
        run: |
          VERSION="${{ needs.validate.outputs.version }}"
          TAG="${{ needs.validate.outputs.cli_tag }}"

          declare -A PLATFORMS=(
            ["darwin-arm64"]="darwin arm64 "
            ["darwin-x64"]="darwin x64 "
            ["win32-x64"]="win32 x64 .exe"
          )

          for PLATFORM_NAME in "${!PLATFORMS[@]}"; do
            read -r OS CPU EXE_SUFFIX <<< "${PLATFORMS[$PLATFORM_NAME]}"
            PKG_NAME="@hmcs/cli-${PLATFORM_NAME}"

            echo "--- Publishing ${PKG_NAME}@${VERSION} ---"

            if npm view "${PKG_NAME}@${VERSION}" version 2>/dev/null; then
              echo "Already published, skipping."
              continue
            fi

            STAGE_DIR="$(mktemp -d)"

            # Generate package.json from template
            sed -e "s/{{PLATFORM_NAME}}/${PLATFORM_NAME}/g" \
                -e "s/{{VERSION}}/${VERSION}/g" \
                -e "s/{{OS}}/${OS}/g" \
                -e "s/{{CPU}}/${CPU}/g" \
                -e "s/{{EXE_SUFFIX}}/${EXE_SUFFIX}/g" \
                packages/cli-platform/package.json.tmpl > "${STAGE_DIR}/package.json"

            # Copy binary from artifact
            cp "artifacts/hmcs-${PLATFORM_NAME}/hmcs${EXE_SUFFIX}" "${STAGE_DIR}/"
            chmod +x "${STAGE_DIR}/hmcs${EXE_SUFFIX}" 2>/dev/null || true

            cp packages/cli-platform/README.md "${STAGE_DIR}/"

            echo "Contents of package:"
            (cd "${STAGE_DIR}" && npm pack --dry-run)

            (cd "${STAGE_DIR}" && npm publish --provenance --access public --tag "${TAG}")
          done

      # --- Publish @hmcs/cli main package ---
      - name: Publish @hmcs/cli
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
        run: |
          VERSION="${{ needs.validate.outputs.version }}"
          TAG="${{ needs.validate.outputs.cli_tag }}"
          PKG_NAME="@hmcs/cli"

          if npm view "${PKG_NAME}@${VERSION}" version 2>/dev/null; then
            echo "Already published, skipping."
            exit 0
          fi

          STAGE_DIR="$(mktemp -d)"
          cp packages/cli/package.json "${STAGE_DIR}/"
          cp -r packages/cli/bin "${STAGE_DIR}/"
          cp packages/cli/README.md "${STAGE_DIR}/"
          cp packages/cli/LICENSE "${STAGE_DIR}/"

          cd "${STAGE_DIR}"
          npm pkg set version="${VERSION}"
          npm pkg set "optionalDependencies.@hmcs/cli-darwin-arm64"="${VERSION}"
          npm pkg set "optionalDependencies.@hmcs/cli-darwin-x64"="${VERSION}"
          npm pkg set "optionalDependencies.@hmcs/cli-win32-x64"="${VERSION}"

          echo "Contents of package:"
          npm pack --dry-run

          npm publish --provenance --access public --tag "${TAG}"
```

**Step 2: Verify YAML validity**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "feat: add publish-npm job to release workflow"
```

---

### Task 5: Add the create-release job

**Files:**
- Modify: `.github/workflows/release.yml`

**Step 1: Add the create-release job**

This job downloads DMG artifacts and creates (or updates) a GitHub Release with auto-generated notes from `.github/release.yml` categories. Handles reruns by detecting existing releases and using `--clobber` for asset uploads.

```yaml
  create-release:
    needs: [validate, publish-npm]
    runs-on: ubuntu-latest
    timeout-minutes: 10
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4

      - name: Download DMG artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts
          pattern: engine-macos-*

      - name: Create GitHub Release
        env:
          GH_TOKEN: ${{ github.token }}
        run: |
          TAG="${GITHUB_REF_NAME}"
          VERSION="${{ needs.validate.outputs.version }}"
          IS_PRERELEASE="${{ needs.validate.outputs.is_prerelease }}"

          # Collect DMG files
          ARM64_DMG="artifacts/engine-macos-arm64/DesktopHomunculus-${VERSION}-arm64.dmg"
          X86_DMG="artifacts/engine-macos-x86/DesktopHomunculus-${VERSION}-x86.dmg"

          # Build gh release create args
          ARGS=(
            "$TAG"
            --generate-notes
            --title "Desktop Homunculus ${VERSION}"
          )

          if [ "$IS_PRERELEASE" = "true" ]; then
            ARGS+=(--prerelease)
          fi

          # Add DMG files if they exist
          for DMG in "$ARM64_DMG" "$X86_DMG"; do
            if [ -f "$DMG" ]; then
              ARGS+=("$DMG")
            fi
          done

          # Try creating the release; if it already exists, upload assets instead
          if gh release view "$TAG" &>/dev/null; then
            echo "Release $TAG already exists. Uploading assets..."
            for DMG in "$ARM64_DMG" "$X86_DMG"; do
              if [ -f "$DMG" ]; then
                gh release upload "$TAG" "$DMG" --clobber
              fi
            done
          else
            gh release create "${ARGS[@]}"
          fi
```

**Step 2: Verify YAML validity**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "feat: add create-release job to release workflow"
```

---

### Task 6: Delete the old publish-cli.yml

**Files:**
- Delete: `.github/workflows/publish-cli.yml`

**Step 1: Remove the old workflow**

The new `release.yml` completely replaces `publish-cli.yml`. All the CLI build and publish logic has been migrated.

```bash
git rm .github/workflows/publish-cli.yml
```

**Step 2: Commit**

```bash
git commit -m "chore: remove publish-cli.yml (replaced by release.yml)"
```

---

### Task 7: Final review — validate the complete workflow

**Files:**
- Read: `.github/workflows/release.yml`

**Step 1: Verify the complete YAML is valid**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
Expected: No output (valid YAML)

**Step 2: Verify job dependency chain is correct**

Check that:
- `build-cli.needs` = `[validate]`
- `build-engine.needs` = `[validate]`
- `publish-npm.needs` = `[validate, build-cli, build-engine]`
- `create-release.needs` = `[validate, publish-npm]`

**Step 3: Verify permissions are job-level (not workflow-level)**

Check that the workflow does NOT have top-level `permissions:` block. Each job should have its own.

**Step 4: Read through the complete file for any issues**

Review for:
- Missing `${{ }}` interpolations
- Correct artifact names between upload/download
- Correct output variable references between jobs
- No hardcoded versions

**Step 5: Commit any fixes if needed**
