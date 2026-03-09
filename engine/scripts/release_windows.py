"""Build Windows MSI installer for Desktop Homunculus."""

import json
import os
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from utils import command_exists, error, log, run

BIN_NAME = "desktop_homunculus"
INSTALLER_PROJECT = Path("build/windows/installer/Installer.wixproj")
BUNDLE_DIR = Path("target/bundle")
RUST_LICENSES_OUTPUT = Path("credits/licenses/RUST_THIRD_PARTY.md")
CEF_STAGING_DIR = Path("build/windows/installer/cef_runtime")
DIST_DIR = Path("target/dist")
CEF_EXTENSIONS = {".dll", ".pak", ".dat", ".bin"}
CEF_EXCLUDE_EXTENSIONS = {".exe", ".lib", ".pdb", ".d", ".exp"}


def get_versions() -> tuple[str, str]:
    """Extract versions from cargo metadata.

    Returns (full_version, msi_version) where msi_version is 3-part numeric only.
    """
    result = run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        capture_output=True,
        text=True,
    )
    metadata = json.loads(result.stdout)
    for pkg in metadata["packages"]:
        if pkg["name"] == BIN_NAME:
            full_version = pkg["version"]
            # Strip pre-release suffix: "0.1.0-alpha.4" -> "0.1.0"
            msi_version = full_version.split("-")[0]
            return full_version, msi_version
    error(f"Package '{BIN_NAME}' not found in cargo metadata")


def stage_cef_files() -> None:
    """Stage CEF runtime files from target/dist/ into the installer staging directory."""
    if CEF_STAGING_DIR.exists():
        shutil.rmtree(CEF_STAGING_DIR)
    CEF_STAGING_DIR.mkdir(parents=True)

    count = 0
    for item in DIST_DIR.iterdir():
        if item.is_file() and item.suffix in CEF_EXTENSIONS:
            shutil.copy2(item, CEF_STAGING_DIR / item.name)
            count += 1
        elif item.is_dir() and item.name == "locales":
            shutil.copytree(item, CEF_STAGING_DIR / "locales")
            count += 1

    log(f"Staged {count} CEF items to {CEF_STAGING_DIR}")


def cleanup_cef_staging() -> None:
    """Remove the CEF staging directory after the build."""
    if CEF_STAGING_DIR.exists():
        shutil.rmtree(CEF_STAGING_DIR)
        log(f"Cleaned up {CEF_STAGING_DIR}")


def release_windows() -> None:
    # 1. Validate prerequisites
    if not command_exists("dotnet"):
        error("dotnet CLI not found. Install .NET SDK first.")

    skip_credits = os.environ.get("SKIP_GEN_CREDITS", "")

    if not skip_credits and not command_exists("cargo-about"):
        error("cargo-about not found. Run 'make setup' first.")

    # 2. Build
    run(["cargo", "build", "--profile", "dist", "--locked"])

    # 3. Generate credits (skip in CI where committed credits are used)
    if skip_credits:
        log("SKIP_GEN_CREDITS set, using committed credits file.")
    else:
        RUST_LICENSES_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
        run([
            "cargo", "about", "generate",
            "--workspace", "--locked",
            "--config", "about.toml",
            "--output-file", str(RUST_LICENSES_OUTPUT),
            "about.hbs",
        ])

    # 4. Stage CEF runtime files for installer
    stage_cef_files()

    # 5. Get versions
    full_version, msi_version = get_versions()
    log(f"Full version: {full_version}, MSI version: {msi_version}")

    # 6. Build MSI (pass 3-part numeric version via MSBuild property)
    run([
        "dotnet",
        "build",
        str(INSTALLER_PROJECT),
        "-c",
        "Release",
        f"-p:Version={msi_version}",
    ])

    # 7. Copy MSI to target/bundle/ with unified naming
    BUNDLE_DIR.mkdir(parents=True, exist_ok=True)
    msi_source = INSTALLER_PROJECT.parent / "bin" / "Release" / "en-US" / "installer.msi"
    msi_dest = BUNDLE_DIR / f"desktop-homunculus-{full_version}-x64.msi"
    shutil.copy2(msi_source, msi_dest)
    log(f"Done: {msi_dest}")

    # 8. Clean up CEF staging directory
    cleanup_cef_staging()


if __name__ == "__main__":
    release_windows()
