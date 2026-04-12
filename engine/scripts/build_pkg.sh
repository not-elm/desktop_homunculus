#!/bin/bash
# build_pkg.sh — Build a macOS .pkg installer from the .app bundle.
#
# Usage: ./scripts/build_pkg.sh <version> [signing-identity]
#
# Produces: target/bundle/desktop_homunculus-<version>-<arch>.pkg
set -euo pipefail

VERSION="${1:?Usage: build_pkg.sh <version> [signing-identity]}"
SIGNING_IDENTITY="${2:-}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ENGINE_DIR="$(dirname "$SCRIPT_DIR")"
BUNDLE_DIR="$ENGINE_DIR/target/bundle"
APP_BUNDLE="$BUNDLE_DIR/DesktopHomunculus.app"
MACOS_BUILD_DIR="$ENGINE_DIR/build/macos"
BUNDLE_ID="not.elm.homunculus.desktop"

# Detect architecture
ARCH="$(uname -m)"
case "$ARCH" in
    arm64) ARCH_SUFFIX="arm64" ;;
    x86_64) ARCH_SUFFIX="x86" ;;
    *) ARCH_SUFFIX="$ARCH" ;;
esac

COMPONENT_PKG="$BUNDLE_DIR/DesktopHomunculus.pkg"
PRODUCT_PKG="$BUNDLE_DIR/desktop_homunculus-${VERSION}-${ARCH_SUFFIX}.pkg"

echo "==> Building pkg installer (version=$VERSION, arch=$ARCH_SUFFIX)"

# ── 1. Code sign the bundled node binary ─────────────────────────────────
NODE_BIN="$APP_BUNDLE/Contents/Resources/runtime/node/bin/node"
if [ -f "$NODE_BIN" ] && [ -n "$SIGNING_IDENTITY" ]; then
    echo "==> Signing bundled Node.js binary..."
    codesign --force --sign "$SIGNING_IDENTITY" \
        --options runtime \
        "$NODE_BIN"
fi

# ── 2. Sign the .app bundle ─────────────────────────────────────────────
if [ -n "$SIGNING_IDENTITY" ]; then
    echo "==> Signing .app bundle..."
    codesign --deep --force --sign "$SIGNING_IDENTITY" \
        --options runtime \
        --entitlements "$MACOS_BUILD_DIR/Entitlements.plist" \
        "$APP_BUNDLE" 2>/dev/null || \
    codesign --deep --force --sign "$SIGNING_IDENTITY" \
        --options runtime \
        "$APP_BUNDLE"
fi

# ── 3. Build component package ───────────────────────────────────────────
echo "==> Building component package..."

# Create a temporary root for pkgbuild
PKG_ROOT="$BUNDLE_DIR/pkg-root"
rm -rf "$PKG_ROOT"
mkdir -p "$PKG_ROOT/Applications"
cp -R "$APP_BUNDLE" "$PKG_ROOT/Applications/"

pkgbuild \
    --root "$PKG_ROOT" \
    --scripts "$MACOS_BUILD_DIR/scripts" \
    --identifier "$BUNDLE_ID" \
    --version "$VERSION" \
    --install-location "/" \
    "$COMPONENT_PKG"

rm -rf "$PKG_ROOT"

# ── 4. Build product package with Distribution.xml ───────────────────────
echo "==> Building product package..."

# Substitute version in Distribution.xml
DIST_XML="$BUNDLE_DIR/Distribution.xml"
sed "s/__VERSION__/$VERSION/g" "$MACOS_BUILD_DIR/Distribution.xml" > "$DIST_XML"

if [ -n "$SIGNING_IDENTITY" ]; then
    productbuild \
        --distribution "$DIST_XML" \
        --package-path "$BUNDLE_DIR" \
        --sign "$SIGNING_IDENTITY" \
        "$PRODUCT_PKG"
else
    productbuild \
        --distribution "$DIST_XML" \
        --package-path "$BUNDLE_DIR" \
        "$PRODUCT_PKG"
fi

rm -f "$COMPONENT_PKG" "$DIST_XML"

# ── 5. Notarize (if identity provided) ───────────────────────────────────
if [ -n "$SIGNING_IDENTITY" ] && command -v xcrun &>/dev/null; then
    APPLE_ID="${APPLE_ID:-}"
    TEAM_ID="${TEAM_ID:-}"
    APP_PASSWORD="${APP_PASSWORD:-}"
    if [ -n "$APPLE_ID" ] && [ -n "$TEAM_ID" ] && [ -n "$APP_PASSWORD" ]; then
        echo "==> Submitting for notarization..."
        xcrun notarytool submit "$PRODUCT_PKG" \
            --apple-id "$APPLE_ID" \
            --team-id "$TEAM_ID" \
            --password "$APP_PASSWORD" \
            --wait
        xcrun stapler staple "$PRODUCT_PKG"
    else
        echo "==> Skipping notarization (APPLE_ID, TEAM_ID, or APP_PASSWORD not set)"
    fi
fi

echo "==> Done: $PRODUCT_PKG"
