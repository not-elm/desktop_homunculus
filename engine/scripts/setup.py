"""Full development setup. Equivalent to `make setup`."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from setup_cef import setup_cef
from utils import Platform, current_platform, error, run


def setup() -> None:
    plat = current_platform()

    if plat == Platform.UNKNOWN:
        error("Unsupported platform. Only macOS and Windows are supported.")

    # cargo install (platform-specific packages)
    cargo_packages = ["export-cef-dir@145.6.1+145.0.28", "cargo-about"]
    if plat == Platform.MACOS:
        cargo_packages.extend([
            "bevy_cef_debug_render_process@0.4.1",
            "bevy_cef_render_process@0.4.1",
            "bevy_cef_bundle_app@0.4.1",
        ])
    elif plat == Platform.WINDOWS:
        cargo_packages.append("bevy_cef_render_process@0.4.1")

    run(["cargo", "install"] + cargo_packages)

    # npm global install
    run(["npm", "i", "-g", "@redocly/cli"])

    # CEF setup
    setup_cef()


if __name__ == "__main__":
    setup()
