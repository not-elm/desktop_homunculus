"""Setup CEF framework. Equivalent to `make setup-cef`."""

import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from utils import (
    CEF_DEBUG_RENDER_DST,
    CEF_DEBUG_RENDER_SRC,
    CEF_DIR_WINDOWS,
    CEF_EXPORT_DIR_MACOS,
    CEF_FRAMEWORK_DIR,
    CEF_SENTINEL_MACOS,
    CEF_SENTINEL_WINDOWS,
    Platform,
    command_exists,
    current_platform,
    error,
    log,
    run,
)


def setup_cef() -> None:
    plat = current_platform()

    if plat == Platform.UNKNOWN:
        log("WARNING: CEF setup is not yet supported on this platform. Skipping.")
        return

    if not command_exists("export-cef-dir"):
        error("export-cef-dir not found. Run 'make setup' first.")

    if plat == Platform.MACOS:
        _setup_cef_macos()
    elif plat == Platform.WINDOWS:
        _setup_cef_windows()


def _setup_cef_macos() -> None:
    if CEF_SENTINEL_MACOS.exists():
        log("CEF framework already installed. Skipping download.")
    else:
        log("Downloading CEF framework...")
        run(["export-cef-dir", "--force", str(CEF_EXPORT_DIR_MACOS)])

    log("Copying debug render process...")
    CEF_DEBUG_RENDER_DST.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(str(CEF_DEBUG_RENDER_SRC), str(CEF_DEBUG_RENDER_DST))


def _setup_cef_windows() -> None:
    if CEF_SENTINEL_WINDOWS.is_dir():
        log("CEF framework already installed. Skipping download.")
    else:
        log("Downloading CEF framework...")
        run(["export-cef-dir", "--force", str(CEF_DIR_WINDOWS)])


if __name__ == "__main__":
    setup_cef()
