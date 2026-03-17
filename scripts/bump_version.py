#!/usr/bin/env python3
"""Unified version management for the desktop_homunculus monorepo.

Reads version, targets, and excludes from version.toml (Single Source of Truth).
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    tomllib = None  # type: ignore[assignment]

REPO_ROOT = Path(__file__).resolve().parent.parent

VERSION_RE = re.compile(r"^\d+\.\d+\.\d+(-[a-zA-Z0-9.-]+)?$")

CONFIG_PATH = REPO_ROOT / "version.toml"


def load_config() -> dict[str, object]:
    """Load version, targets, and excludes from version.toml."""
    if not CONFIG_PATH.exists():
        print(f"ERROR: {CONFIG_PATH} not found", file=sys.stderr)
        sys.exit(1)

    if tomllib is not None:
        with open(CONFIG_PATH, "rb") as f:
            return tomllib.load(f)

    # Fallback: regex-based parsing for Python < 3.11
    text = CONFIG_PATH.read_text(encoding="utf-8")

    version_m = re.search(r'^version\s*=\s*"([^"]*)"', text, re.MULTILINE)
    version = version_m.group(1) if version_m else ""

    targets: list[str] = re.findall(
        r'(?<=\n)\s*"([^"]+)"', text[text.find("targets"):text.find("excludes")]
    )
    excludes: list[str] = re.findall(
        r'(?<=\n)\s*"([^"]+)"', text[text.find("excludes"):]
    )

    return {"version": version, "targets": targets, "excludes": excludes}



def resolve_targets(
    targets: list[str], excludes: list[str]
) -> list[Path]:
    """Expand glob patterns in targets and remove excludes."""
    exclude_set = {(REPO_ROOT / e).resolve() for e in excludes}
    paths: list[Path] = []
    for pattern in targets:
        matched = sorted(REPO_ROOT.glob(pattern))
        for p in matched:
            if p.resolve() not in exclude_set:
                paths.append(p)
    if not paths:
        print("ERROR: No target files found", file=sys.stderr)
        sys.exit(1)
    return paths


def validate_version(version: str) -> None:
    if not VERSION_RE.match(version):
        print(
            f"ERROR: Invalid version '{version}'. "
            "Must match X.Y.Z or X.Y.Z-<prerelease> (e.g., 0.2.0, 0.2.0-dev)",
            file=sys.stderr,
        )
        sys.exit(1)


def update_cargo_toml(path: Path, version: str) -> None:
    """Update workspace.package.version in Cargo.toml."""
    text = path.read_text(encoding="utf-8")

    if "[workspace.package]" not in text:
        print(f"ERROR: No [workspace.package] section in {path}", file=sys.stderr)
        sys.exit(1)

    # Replace version only within [workspace.package] section
    in_section = False
    lines = text.splitlines(keepends=True)
    new_lines: list[str] = []
    replaced = False
    for line in lines:
        stripped = line.strip()
        if stripped == "[workspace.package]":
            in_section = True
        elif in_section and stripped.startswith("["):
            in_section = False
        if in_section and not replaced:
            m = re.match(r'^(version\s*=\s*")[^"]*(")', line)
            if m:
                line = f'{m.group(1)}{version}{m.group(2)}\n'
                replaced = True
        new_lines.append(line)

    if not replaced:
        print(
            f"ERROR: Could not find version line in [workspace.package] of {path}",
            file=sys.stderr,
        )
        sys.exit(1)

    path.write_text("".join(new_lines), encoding="utf-8")


def update_package_json(path: Path, version: str) -> None:
    """Update version field in package.json."""
    text = path.read_text(encoding="utf-8")
    data = json.loads(text)
    data["version"] = version
    # Detect indent
    indent = 2
    m = re.match(r'\{\s*\n(\s+)', text)
    if m:
        indent = len(m.group(1))
    path.write_text(
        json.dumps(data, indent=indent, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def update_tmpl(path: Path, version: str) -> None:
    """Update version in package.json.tmpl (replace {{VERSION}} or existing version)."""
    text = path.read_text(encoding="utf-8")
    # Replace {{VERSION}} placeholder
    new_text = text.replace("{{VERSION}}", version)
    if new_text == text:
        # No placeholder found, try replacing existing version string
        new_text = re.sub(
            r'("version"\s*:\s*")[^"]*(")',
            rf"\g<1>{version}\2",
            text,
            count=1,
        )
    path.write_text(new_text, encoding="utf-8")


def refresh_cargo_lock(version: str) -> None:
    """Run cargo update --workspace to refresh Cargo.lock."""
    import subprocess
    import shutil

    cargo = shutil.which("cargo")
    if not cargo:
        print("WARNING: cargo not found, skipping Cargo.lock refresh", file=sys.stderr)
        return

    manifest = REPO_ROOT / "engine" / "Cargo.toml"
    result = subprocess.run(
        [cargo, "update", "--workspace", "--manifest-path", str(manifest)],
        cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        print("WARNING: cargo update --workspace failed", file=sys.stderr)


def bump() -> None:
    """Read version from version.toml and propagate to all target files."""
    config = load_config()
    targets = config.get("targets", [])
    excludes = config.get("excludes", [])

    version = str(config["version"])
    print(f"Using version from version.toml: {version}")

    validate_version(version)
    resolved = resolve_targets(targets, excludes)  # type: ignore[arg-type]

    for path in resolved:
        rel = path.relative_to(REPO_ROOT)
        if path.name == "Cargo.toml":
            update_cargo_toml(path, version)
        elif path.suffix == ".tmpl":
            update_tmpl(path, version)
        else:
            update_package_json(path, version)
        print(f"  Updated: {rel}")

    print(f"\nRefreshing Cargo.lock...")
    refresh_cargo_lock(version)
    print(f"\nAll files updated to {version}")


def read_version_from_cargo_toml(path: Path) -> str:
    """Read workspace.package.version from Cargo.toml."""
    if tomllib is not None:
        with open(path, "rb") as f:
            data = tomllib.load(f)
        return data["workspace"]["package"]["version"]
    # Fallback: regex-based parsing for Python < 3.11
    text = path.read_text(encoding="utf-8")
    in_section = False
    for line in text.splitlines():
        stripped = line.strip()
        if stripped == "[workspace.package]":
            in_section = True
            continue
        if in_section and stripped.startswith("["):
            break
        if in_section:
            m = re.match(r'^version\s*=\s*"([^"]*)"', stripped)
            if m:
                return m.group(1)
    return ""


def read_version_from_json(path: Path) -> str:
    """Read version from package.json."""
    data = json.loads(path.read_text(encoding="utf-8"))
    return data.get("version", "")


def read_version_from_tmpl(path: Path) -> str:
    """Read version from package.json.tmpl."""
    text = path.read_text(encoding="utf-8")
    m = re.search(r'"version"\s*:\s*"([^"]*)"', text)
    return m.group(1) if m else ""


def read_version(path: Path) -> str:
    if path.name == "Cargo.toml":
        return read_version_from_cargo_toml(path)
    elif path.suffix == ".tmpl":
        return read_version_from_tmpl(path)
    else:
        return read_version_from_json(path)


def discover_workspace_packages() -> set[Path]:
    """Discover all package.json files from pnpm-workspace.yaml globs."""
    ws_file = REPO_ROOT / "pnpm-workspace.yaml"
    if not ws_file.exists():
        return set()

    text = ws_file.read_text(encoding="utf-8")
    paths: set[Path] = set()
    in_packages = False
    for line in text.splitlines():
        stripped = line.strip()
        # Detect "packages:" section
        if stripped == "packages:":
            in_packages = True
            continue
        # End of packages section (new top-level key)
        if in_packages and not stripped.startswith("-") and ":" in stripped:
            in_packages = False
            continue
        if not in_packages:
            continue
        if not stripped.startswith("-"):
            continue
        pattern = stripped.lstrip("- ").strip().strip("'\"")
        if not pattern:
            continue
        for p in REPO_ROOT.glob(f"{pattern}/package.json"):
            # Skip dist/ directories
            if "dist" not in p.parts:
                paths.add(p.resolve())
    return paths


def check() -> None:
    """Verify version consistency across the monorepo."""
    config = load_config()
    expected_version = str(config["version"])
    targets_patterns = config.get("targets", [])
    excludes_patterns = config.get("excludes", [])

    targets = resolve_targets(targets_patterns, excludes_patterns)  # type: ignore[arg-type]
    errors: list[str] = []
    warnings: list[str] = []

    # 1. Check each file matches version.toml
    versions: dict[str, str] = {}
    for path in targets:
        rel = str(path.relative_to(REPO_ROOT))
        versions[rel] = read_version(path)

    mismatches = {
        rel: ver
        for rel, ver in versions.items()
        if ver != expected_version and not ver.startswith("{{")
    }

    if mismatches:
        errors.append(f"Version mismatch (expected {expected_version}):")
        for rel, ver in sorted(mismatches.items()):
            errors.append(f"  {rel}: {ver}")

    # 2. Missing package detection
    exclude_set = {(REPO_ROOT / e).resolve() for e in excludes_patterns}  # type: ignore[union-attr]
    target_set = {p.resolve() for p in targets}

    # Check pnpm workspace packages
    ws_packages = discover_workspace_packages()
    for p in ws_packages:
        if p not in target_set and p not in exclude_set:
            warnings.append(
                f"Package not covered by targets or excludes: "
                f"{p.relative_to(REPO_ROOT)}"
            )

    # Check Rust crates
    for cargo in sorted(REPO_ROOT.glob("engine/crates/*/Cargo.toml")):
        text = cargo.read_text(encoding="utf-8")
        if "version.workspace = true" not in text:
            has_own_version = re.search(
                r'^version\s*=\s*"[^"]*"', text, re.MULTILINE
            )
            if has_own_version:
                warnings.append(
                    f"Crate does not use workspace version inheritance: "
                    f"{cargo.relative_to(REPO_ROOT)}"
                )

    # Report
    if warnings:
        print("Warnings:")
        for w in warnings:
            print(f"  ⚠ {w}")
        print()

    if errors:
        print("Errors:")
        for e in errors:
            print(f"  ✗ {e}")
        sys.exit(1)

    print(f"✓ All versions match version.toml: {expected_version}")


def main() -> None:
    if len(sys.argv) < 2:
        bump()
        return

    if sys.argv[1] == "--check":
        check()
    else:
        print(
            f"ERROR: Unknown argument '{sys.argv[1]}'. "
            "Usage: bump_version.py [--check]",
            file=sys.stderr,
        )
        sys.exit(1)


if __name__ == "__main__":
    main()
