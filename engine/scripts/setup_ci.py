"""CI release build setup. Equivalent to `make setup-ci`."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from setup_cef import setup_cef
from utils import run


def setup_ci() -> None:
    run([
        "cargo", "binstall", "--no-confirm",
        "export-cef-dir@144.4.0+144.0.13",
        "bevy_cef_debug_render_process@0.2.0",
        "bevy_cef_render_process@0.2.0",
        "bevy_cef_bundle_app@0.2.0",
    ])

    setup_cef()


if __name__ == "__main__":
    setup_ci()
