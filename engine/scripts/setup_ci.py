"""CI release build setup. Equivalent to `make setup-ci`."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from setup_cef import setup_cef
from utils import Platform, current_platform, run


def setup_ci() -> None:
    packages = ["export-cef-dir@144.4.0+144.0.13"]

    plat = current_platform()
    if plat == Platform.MACOS:
        packages.extend([
            "bevy_cef_debug_render_process@0.4.0",
            "bevy_cef_render_process@0.4.0",
            "bevy_cef_bundle_app@0.4.0",
        ])
    elif plat == Platform.WINDOWS:
        packages.append("bevy_cef_render_process@0.4.0")

    run(["cargo", "binstall", "--no-confirm"] + packages)

    setup_cef()


if __name__ == "__main__":
    setup_ci()
