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
RUNTIME_STAGING_DIR = Path("build/windows/installer/runtime")
BIN_STAGING_DIR = Path("build/windows/installer/bin")
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

    # Include the dedicated CEF render process binary.
    # Without it, CEF re-launches the main executable (~104 MB) for every
    # renderer / GPU / utility subprocess, which is slow and fragile.
    render_process = DIST_DIR / "bevy_cef_render_process.exe"
    if render_process.exists():
        shutil.copy2(render_process, CEF_STAGING_DIR / render_process.name)
        count += 1
        log(f"Included CEF render process: {render_process.name}")
    else:
        log("WARNING: bevy_cef_render_process.exe not found in target/dist/")

    log(f"Staged {count} CEF items to {CEF_STAGING_DIR}")


CEF_REQUIRED_FILES = ["libcef.dll", "v8_context_snapshot.bin", "icudtl.dat"]


def verify_cef_staging() -> None:
    """Verify all required CEF runtime files are present in staging."""
    missing = [name for name in CEF_REQUIRED_FILES if not (CEF_STAGING_DIR / name).exists()]
    if missing:
        error(f"Required CEF files missing from staging: {', '.join(missing)}")

    render_process = CEF_STAGING_DIR / "bevy_cef_render_process.exe"
    if not render_process.exists():
        error("bevy_cef_render_process.exe missing from CEF staging. "
              "CEF subprocesses will fall back to the main executable, causing console windows.")

    log(f"CEF staging verification passed ({len(CEF_REQUIRED_FILES) + 1} required files present)")


def cleanup_cef_staging() -> None:
    """Remove the CEF staging directory after the build."""
    if CEF_STAGING_DIR.exists():
        shutil.rmtree(CEF_STAGING_DIR)
        log(f"Cleaned up {CEF_STAGING_DIR}")


ICON_PNG = Path("assets/icons/icon.png")
ICON_ICO = Path("build/windows/icon.ico")


def generate_icon_for_installer() -> None:
    """Generate ICO from source PNG for the WiX installer.

    The WiX Package.wxs references build/windows/icon.ico.
    This generates it from assets/icons/icon.png using Pillow.
    """
    try:
        from PIL import Image
    except ImportError:
        error("Pillow not found. Install it with: pip install Pillow")

    if not ICON_PNG.exists():
        error(f"Source icon not found: {ICON_PNG}")

    img = Image.open(ICON_PNG)
    sizes = [(256, 256), (48, 48), (32, 32), (16, 16)]
    ICON_ICO.parent.mkdir(parents=True, exist_ok=True)
    img.save(str(ICON_ICO), format="ICO", sizes=sizes)
    log(f"Generated {ICON_ICO} from {ICON_PNG}")


def stage_runtime_files() -> None:
    """Run stage_runtime.py and copy output to the installer staging directory."""
    run([sys.executable, "scripts/stage_runtime.py"])

    runtime_source = BUNDLE_DIR / "runtime"
    if not runtime_source.exists():
        error(f"Runtime staging directory not found: {runtime_source}")

    if RUNTIME_STAGING_DIR.exists():
        shutil.rmtree(RUNTIME_STAGING_DIR)
    shutil.copytree(runtime_source, RUNTIME_STAGING_DIR)
    log(f"Staged runtime to {RUNTIME_STAGING_DIR}")


def stage_cli_binary() -> None:
    """Build hmcs.exe and stage it for the installer."""
    run(["cargo", "build", "-p", "homunculus_cli", "--profile", "dist", "--locked"])

    if BIN_STAGING_DIR.exists():
        shutil.rmtree(BIN_STAGING_DIR)
    BIN_STAGING_DIR.mkdir(parents=True)

    hmcs_source = DIST_DIR / "hmcs.exe"
    if not hmcs_source.exists():
        error(f"hmcs.exe not found at {hmcs_source}")
    shutil.copy2(hmcs_source, BIN_STAGING_DIR / "hmcs.exe")
    log(f"Staged hmcs.exe to {BIN_STAGING_DIR}")


def cleanup_runtime_staging() -> None:
    """Remove the runtime and bin staging directories after the build."""
    for d in [RUNTIME_STAGING_DIR, BIN_STAGING_DIR]:
        if d.exists():
            shutil.rmtree(d)
            log(f"Cleaned up {d}")


def release_windows() -> None:
    # 1. Validate prerequisites
    if not command_exists("dotnet"):
        error("dotnet CLI not found. Install .NET SDK first.")

    skip_credits = os.environ.get("SKIP_GEN_CREDITS", "")

    if not skip_credits and not command_exists("cargo-about"):
        error("cargo-about not found. Run 'make setup' first.")

    # 2. Build engine + CLI
    run(["cargo", "build", "--profile", "dist", "--locked"])

    # 3. Generate ICO from source PNG for WiX installer
    generate_icon_for_installer()

    # 4. Generate credits (skip in CI where committed credits are used)
    if skip_credits:
        if not RUST_LICENSES_OUTPUT.exists():
            error(f"SKIP_GEN_CREDITS set but {RUST_LICENSES_OUTPUT} not found. Commit credits first.")
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

    # 5. Stage CEF runtime files for installer
    stage_cef_files()
    verify_cef_staging()

    # 6. Stage Node.js/pnpm/tsx runtime
    stage_runtime_files()

    # 7. Stage hmcs CLI binary
    stage_cli_binary()

    # 8. Get versions
    full_version, msi_version = get_versions()
    log(f"Full version: {full_version}, MSI version: {msi_version}")

    # 9. Build MSI (pass 3-part numeric version via MSBuild property)
    run([
        "dotnet",
        "build",
        str(INSTALLER_PROJECT),
        "-c",
        "Release",
        f"-p:Version={msi_version}",
    ])

    # 10. Copy MSI to target/bundle/ with unified naming
    BUNDLE_DIR.mkdir(parents=True, exist_ok=True)
    msi_source = INSTALLER_PROJECT.parent / "bin" / "Release" / "en-US" / "installer.msi"
    msi_dest = BUNDLE_DIR / f"desktop-homunculus-{full_version}-x64.msi"
    shutil.copy2(msi_source, msi_dest)
    log(f"Done: {msi_dest}")

    # 11. Clean up staging directories
    cleanup_cef_staging()
    cleanup_runtime_staging()


if __name__ == "__main__":
    release_windows()
