"""Download and stage Node.js, pnpm, and tsx for bundling into the installer.

Reads pinned versions from ``runtime-versions.toml`` and stages the runtime
components into ``target/bundle/runtime/``.

Output layout::

    target/bundle/runtime/
    ├── node/bin/node(.exe)
    ├── pnpm/bin/pnpm.cjs
    └── tsx/node_modules/tsx/...
"""

import io
import json
import os
import platform
import shutil
import subprocess
import sys
import tarfile
import tempfile
import zipfile
from pathlib import Path
from urllib.request import urlopen

sys.path.insert(0, str(Path(__file__).parent))

from utils import error, log

SCRIPT_DIR = Path(__file__).parent
ENGINE_DIR = SCRIPT_DIR.parent
VERSIONS_FILE = ENGINE_DIR / "runtime-versions.toml"
STAGING_DIR = ENGINE_DIR / "target" / "bundle" / "runtime"
CACHE_DIR = ENGINE_DIR / "target" / "bundle" / "runtime-cache"

# On Windows, npm is a .cmd shim; subprocess.run requires the full filename
# because CreateProcess only resolves .exe by default.
NPM = "npm.cmd" if platform.system() == "Windows" else "npm"


def load_versions() -> dict[str, str]:
    """Parse runtime-versions.toml (minimal TOML parser for [runtime] section)."""
    versions: dict[str, str] = {}
    in_runtime = False
    for line in VERSIONS_FILE.read_text().splitlines():
        stripped = line.strip()
        if stripped == "[runtime]":
            in_runtime = True
            continue
        if stripped.startswith("["):
            in_runtime = False
            continue
        if in_runtime and "=" in stripped:
            key, _, value = stripped.partition("=")
            versions[key.strip()] = value.strip().strip('"')
    return versions


def detect_platform() -> tuple[str, str]:
    """Detect (os, arch) for Node.js download naming.

    Honors the ``TARGET_ARCH`` environment variable (``arm64`` or ``x64``)
    so that cross-architecture builds (e.g. building x86 on Apple Silicon)
    bundle the correct Node.js binary.
    """
    system = platform.system()
    target_arch = os.environ.get("TARGET_ARCH", "").lower()

    if system == "Darwin":
        node_os = "darwin"
        if target_arch in ("arm64", "x64"):
            node_arch = target_arch
        else:
            machine = platform.machine().lower()
            node_arch = "arm64" if machine == "arm64" else "x64"
    elif system == "Windows":
        node_os = "win"
        node_arch = target_arch if target_arch in ("x64",) else "x64"
    else:
        error(f"Unsupported platform: {system}")

    return node_os, node_arch


def download_file(url: str, dest: Path) -> None:
    """Download a URL to a local file, with basic progress."""
    log(f"Downloading {url}")
    with urlopen(url) as response:
        data = response.read()
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_bytes(data)
    size_mb = len(data) / (1024 * 1024)
    log(f"  Downloaded {size_mb:.1f} MB -> {dest}")


def cached_download(url: str, filename: str) -> Path:
    """Download with caching — skip if already cached."""
    cached = CACHE_DIR / filename
    if cached.exists():
        log(f"Using cached {cached}")
        return cached
    download_file(url, cached)
    return cached


def stage_node(version: str, node_os: str, node_arch: str) -> None:
    """Download Node.js and extract only the node binary."""
    node_dir = STAGING_DIR / "node"
    if node_dir.exists():
        shutil.rmtree(node_dir)

    if node_os == "win":
        filename = f"node-v{version}-{node_os}-{node_arch}.zip"
        url = f"https://nodejs.org/dist/v{version}/{filename}"
        archive = cached_download(url, filename)

        with zipfile.ZipFile(archive) as zf:
            prefix = f"node-v{version}-{node_os}-{node_arch}/"
            node_exe_name = f"{prefix}node.exe"
            dest = node_dir / "node.exe"
            dest.parent.mkdir(parents=True, exist_ok=True)
            with zf.open(node_exe_name) as src, open(dest, "wb") as dst:
                shutil.copyfileobj(src, dst)
    else:
        filename = f"node-v{version}-{node_os}-{node_arch}.tar.gz"
        url = f"https://nodejs.org/dist/v{version}/{filename}"
        archive = cached_download(url, filename)

        with tarfile.open(archive, "r:gz") as tf:
            prefix = f"node-v{version}-{node_os}-{node_arch}/"
            node_bin_name = f"{prefix}bin/node"
            member = tf.getmember(node_bin_name)
            dest_dir = node_dir / "bin"
            dest_dir.mkdir(parents=True, exist_ok=True)
            dest = dest_dir / "node"
            with tf.extractfile(member) as src:
                dest.write_bytes(src.read())
            dest.chmod(0o755)

    log(f"Staged Node.js v{version} -> {node_dir}")


def stage_pnpm(version: str) -> None:
    """Download pnpm and extract the bin entry point."""
    pnpm_dir = STAGING_DIR / "pnpm"
    if pnpm_dir.exists():
        shutil.rmtree(pnpm_dir)

    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        log(f"Packing pnpm@{version}")
        result = subprocess.run(
            [NPM, "pack", f"pnpm@{version}", "--pack-destination", str(tmp_path)],
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            error(f"npm pack pnpm failed: {result.stderr}")

        tgz_files = list(tmp_path.glob("pnpm-*.tgz"))
        if not tgz_files:
            error("npm pack produced no .tgz file")

        with tarfile.open(tgz_files[0], "r:gz") as tf:
            bin_dir = pnpm_dir / "bin"
            bin_dir.mkdir(parents=True, exist_ok=True)

            # Extract bin/pnpm.cjs and supporting dist/ files
            for member in tf.getmembers():
                # Strip "package/" prefix
                rel = member.name.removeprefix("package/")
                if rel.startswith("bin/") or rel.startswith("dist/"):
                    dest = pnpm_dir / rel
                    if member.isdir():
                        dest.mkdir(parents=True, exist_ok=True)
                    else:
                        dest.parent.mkdir(parents=True, exist_ok=True)
                        with tf.extractfile(member) as src:
                            dest.write_bytes(src.read())

    log(f"Staged pnpm v{version} -> {pnpm_dir}")


def stage_tsx(version: str) -> None:
    """Install tsx and its dependencies into a self-contained directory."""
    tsx_dir = STAGING_DIR / "tsx"
    if tsx_dir.exists():
        shutil.rmtree(tsx_dir)

    tsx_dir.mkdir(parents=True, exist_ok=True)

    # Create a minimal package.json, then npm install tsx into it
    pkg_json = tsx_dir / "package.json"
    pkg_json.write_text(json.dumps({"private": True}))

    log(f"Installing tsx@{version}")
    result = subprocess.run(
        [NPM, "install", "--no-audit", "--no-fund", f"tsx@{version}"],
        cwd=tsx_dir,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        error(f"npm install tsx failed: {result.stderr}")

    # Remove package.json and package-lock.json (not needed at runtime)
    pkg_json.unlink(missing_ok=True)
    (tsx_dir / "package-lock.json").unlink(missing_ok=True)

    # Verify the ESM entry point exists
    esm_entry = tsx_dir / "node_modules" / "tsx" / "dist" / "esm" / "index.mjs"
    if not esm_entry.exists():
        error(f"tsx ESM entry point not found: {esm_entry}")

    log(f"Staged tsx v{version} -> {tsx_dir}")


def main() -> None:
    if not VERSIONS_FILE.exists():
        error(f"runtime-versions.toml not found: {VERSIONS_FILE}")

    versions = load_versions()
    node_version = versions.get("node")
    pnpm_version = versions.get("pnpm")
    tsx_version = versions.get("tsx")

    if not all([node_version, pnpm_version, tsx_version]):
        error(f"Missing version(s) in runtime-versions.toml: {versions}")

    node_os, node_arch = detect_platform()
    log(f"Platform: {node_os}-{node_arch}")
    log(f"Versions: node={node_version}, pnpm={pnpm_version}, tsx={tsx_version}")

    # Clean staging dir
    if STAGING_DIR.exists():
        shutil.rmtree(STAGING_DIR)
    STAGING_DIR.mkdir(parents=True, exist_ok=True)

    stage_node(node_version, node_os, node_arch)
    stage_pnpm(pnpm_version)
    stage_tsx(tsx_version)

    log(f"Runtime staged successfully at {STAGING_DIR}")

    # Print size summary
    total = sum(f.stat().st_size for f in STAGING_DIR.rglob("*") if f.is_file())
    log(f"Total size: {total / (1024 * 1024):.1f} MB")


if __name__ == "__main__":
    main()
