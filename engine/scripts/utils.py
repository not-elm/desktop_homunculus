"""Shared utilities for setup scripts."""

import platform
import shutil
import subprocess
import sys
from enum import Enum
from pathlib import Path


class Platform(Enum):
    MACOS = "macos"
    WINDOWS = "windows"
    UNKNOWN = "unknown"


def current_platform() -> Platform:
    system = platform.system()
    if system == "Darwin":
        return Platform.MACOS
    elif system == "Windows":
        return Platform.WINDOWS
    else:
        return Platform.UNKNOWN


def log(msg: str) -> None:
    print(f"==> {msg}")


def error(msg: str) -> None:
    print(f"ERROR: {msg}", file=sys.stderr)
    sys.exit(1)


def run(cmd: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Run a command, exit on failure."""
    resolved = shutil.which(cmd[0])
    if resolved:
        cmd = [resolved] + cmd[1:]
    log(f"Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, **kwargs)
    if result.returncode != 0:
        error(f"Command failed with exit code {result.returncode}: {' '.join(cmd)}")
    return result


def command_exists(name: str) -> bool:
    return shutil.which(name) is not None


# --- CEF paths ---

HOME = Path.home()

# macOS
CEF_FRAMEWORK_DIR = HOME / ".local" / "share" / "Chromium Embedded Framework.framework"
CEF_SENTINEL_MACOS = CEF_FRAMEWORK_DIR / "Chromium Embedded Framework"
CEF_EXPORT_DIR_MACOS = HOME / ".local" / "share"
CEF_DEBUG_RENDER_SRC = HOME / ".cargo" / "bin" / "bevy_cef_debug_render_process"
CEF_DEBUG_RENDER_DST = CEF_FRAMEWORK_DIR / "Libraries" / "bevy_cef_debug_render_process"

# Windows
CEF_DIR_WINDOWS = HOME / ".local" / "share" / "cef"
CEF_SENTINEL_WINDOWS = CEF_DIR_WINDOWS / "Release"
CEF_RENDER_PROCESS_SRC_WINDOWS = HOME / ".cargo" / "bin" / "bevy_cef_render_process.exe"
CEF_RENDER_PROCESS_DST_WINDOWS = CEF_DIR_WINDOWS / "bevy_cef_render_process.exe"
