"""Build Windows MSI installer for Desktop Homunculus."""

import json
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from utils import command_exists, error, log, run

BIN_NAME = "desktop_homunculus"
INSTALLER_PROJECT = Path("build/windows/installer/Installer.wixproj")
BUNDLE_DIR = Path("target/bundle")
RUST_LICENSES_OUTPUT = Path("credits/licenses/RUST_THIRD_PARTY.md")


def get_version() -> str:
    """Extract version from cargo metadata, strip pre-release for MSI (3-part numeric only)."""
    result = run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        capture_output=True,
        text=True,
    )
    metadata = json.loads(result.stdout)
    for pkg in metadata["packages"]:
        if pkg["name"] == BIN_NAME:
            version = pkg["version"]
            # Strip pre-release suffix: "0.1.0-alpha.4" -> "0.1.0"
            return version.split("-")[0]
    error(f"Package '{BIN_NAME}' not found in cargo metadata")


def release_windows() -> None:
    # 1. Validate prerequisites
    if not command_exists("dotnet"):
        error("dotnet CLI not found. Install .NET SDK first.")
    if not command_exists("cargo-about"):
        error("cargo-about not found. Run 'make setup' first.")

    # 2. Build
    run(["cargo", "build", "--profile", "dist", "--locked"])

    # 3. Generate credits
    RUST_LICENSES_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    run([
        "cargo", "about", "generate",
        "--workspace", "--locked",
        "--config", "about.toml",
        "--output-file", str(RUST_LICENSES_OUTPUT),
        "about.hbs",
    ])

    # 4. Get version (3-part numeric for MSI)
    version = get_version()
    log(f"Version: {version}")

    # 5. Build MSI (pass version via MSBuild property)
    run([
        "dotnet",
        "build",
        str(INSTALLER_PROJECT),
        "-c",
        "Release",
        f"-p:Version={version}",
    ])

    # 6. Copy MSI to target/bundle/
    BUNDLE_DIR.mkdir(parents=True, exist_ok=True)
    msi_source = INSTALLER_PROJECT.parent / "bin" / "Release" / "installer.msi"
    msi_dest = BUNDLE_DIR / f"{BIN_NAME}-{version}.msi"
    shutil.copy2(msi_source, msi_dest)
    log(f"Done: {msi_dest}")


if __name__ == "__main__":
    release_windows()
