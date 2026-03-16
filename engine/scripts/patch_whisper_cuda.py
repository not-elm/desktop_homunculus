"""Patch whisper-rs-sys CUDA sources for Blackwell (SM 12.0) compatibility.

Blackwell GPUs report a sharedMemPerBlockOptin value that the CUDA driver
rejects in cudaFuncSetAttribute. The fix is to pass the actual required
shared memory size (nbytes_shared) instead of the device maximum (smpbo).

This script is idempotent — it skips files already patched (detected via
a marker comment). When patches are applied, it runs `cargo clean -p
whisper-rs-sys` so the next build picks up the changes.
"""

import glob
import re
import subprocess
import sys
from pathlib import Path

from utils import error, log

MARKER = "// [patched:blackwell-shmem]"

# Cargo registry location (platform-independent)
CARGO_REGISTRY = Path.home() / ".cargo" / "registry" / "src"


def find_whisper_sys_dir() -> Path | None:
    """Find the whisper-rs-sys source directory in the cargo registry."""
    pattern = str(CARGO_REGISTRY / "*" / "whisper-rs-sys-*")
    candidates = sorted(glob.glob(pattern), reverse=True)
    if not candidates:
        return None
    # Use the newest version
    return Path(candidates[0])


def is_patched(file: Path) -> bool:
    return MARKER in file.read_text(encoding="utf-8")


def patch_common_cuh(file: Path) -> bool:
    """Patch CUDA_SET_SHARED_MEMORY_LIMIT macro: bool guard → size_t max-tracking."""
    if is_patched(file):
        return False

    text = file.read_text(encoding="utf-8")
    original = text

    # Lines are inside a #define macro with trailing `\` continuations and padding.
    # Use regex to handle variable whitespace.
    text = re.sub(
        r"static bool shared_memory_limit_raised\[GGML_CUDA_MAX_DEVICES\]\s*=\s*\{\s*false\s*\};",
        f"static size_t shared_memory_limit_set[GGML_CUDA_MAX_DEVICES] = {{ 0 }}; {MARKER}",
        text,
    )
    text = re.sub(
        r"if\s*\(\s*!shared_memory_limit_raised\[id\]\s*\)",
        "if (shared_memory_limit_set[id] < (size_t)(nbytes))",
        text,
    )
    text = re.sub(
        r"shared_memory_limit_raised\[id\]\s*=\s*true;",
        "shared_memory_limit_set[id] = (size_t)(nbytes);",
        text,
    )

    if text == original:
        log(f"  WARNING: Could not find bool guard pattern in {file.name}, skipping")
        return False

    file.write_text(text, encoding="utf-8")
    log(f"  Patched {file.name}: bool guard → size_t max-tracking")
    return True


def patch_replace_smpbo(file: Path, description: str) -> bool:
    """Replace smpbo → nbytes_shared in CUDA_SET_SHARED_MEMORY_LIMIT calls."""
    if is_patched(file):
        return False

    text = file.read_text(encoding="utf-8")
    original = text

    # Only replace smpbo when it appears as the last argument to CUDA_SET_SHARED_MEMORY_LIMIT
    # Use .+ (greedy within single line) to handle template args with commas
    text = re.sub(
        r"(CUDA_SET_SHARED_MEMORY_LIMIT\(.+),\s*smpbo\s*\)",
        r"\1, nbytes_shared)",
        text,
    )

    if text == original:
        log(f"  WARNING: No smpbo replacements found in {file.name}, skipping")
        return False

    # Add marker at the top (after any existing comments/includes)
    text = text + f"\n{MARKER}\n"

    file.write_text(text, encoding="utf-8")
    log(f"  Patched {file.name}: {description}")
    return True


def patch_mmid_cu(file: Path) -> bool:
    """Patch mmid.cu: move CUDA_SET_SHARED_MEMORY_LIMIT after nbytes_shared definition."""
    if is_patched(file):
        return False

    text = file.read_text(encoding="utf-8")
    lines = text.split("\n")
    patched = False
    new_lines = []
    deferred_macro_line = None

    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Detect CUDA_SET_SHARED_MEMORY_LIMIT with smpbo that needs to be moved
        if "CUDA_SET_SHARED_MEMORY_LIMIT" in stripped and "smpbo" in stripped:
            # Store this line to insert later, after nbytes_shared is defined
            # Replace smpbo with nbytes_shared
            deferred_macro_line = line.replace("smpbo", "nbytes_shared")
            i += 1
            continue

        # If we have a deferred macro line, insert it right before the kernel launch
        # (which comes after nbytes_shared definition and GGML_ASSERT)
        if deferred_macro_line and "<<<" in stripped:
            new_lines.append(deferred_macro_line)
            patched = True
            deferred_macro_line = None

        new_lines.append(line)
        i += 1

    if not patched:
        log(f"  WARNING: Could not reorder CUDA_SET_SHARED_MEMORY_LIMIT in {file.name}, skipping")
        return False

    text = "\n".join(new_lines)
    text = text + f"\n{MARKER}\n"
    file.write_text(text, encoding="utf-8")
    log(f"  Patched {file.name}: moved CUDA_SET_SHARED_MEMORY_LIMIT after nbytes_shared, smpbo → nbytes_shared")
    return True


def main() -> None:
    log("Checking whisper-rs-sys CUDA sources for Blackwell compatibility...")

    whisper_dir = find_whisper_sys_dir()
    if whisper_dir is None:
        log("whisper-rs-sys not found in cargo registry (not yet downloaded). Skipping patch.")
        return

    ggml_cuda = whisper_dir / "whisper-rs" / "sys" / "whisper-cpp" / "ggml" / "src" / "ggml-cuda"
    if not ggml_cuda.exists():
        # Try alternate path structures
        for pattern in [
            "whisper.cpp/ggml/src/ggml-cuda",
            "sys/whisper-cpp/ggml/src/ggml-cuda",
        ]:
            alt = whisper_dir / pattern
            if alt.exists():
                ggml_cuda = alt
                break

    if not ggml_cuda.exists():
        log(f"ggml-cuda directory not found in {whisper_dir}. Skipping patch.")
        return

    log(f"Found ggml-cuda at: {ggml_cuda}")

    any_patched = False

    # 1. Patch common.cuh
    common = ggml_cuda / "common.cuh"
    if common.exists():
        any_patched |= patch_common_cuh(common)
    else:
        log(f"  WARNING: {common} not found")

    # 2. Patch softmax.cu
    softmax = ggml_cuda / "softmax.cu"
    if softmax.exists():
        any_patched |= patch_replace_smpbo(softmax, "smpbo → nbytes_shared in soft_max calls")
    else:
        log(f"  WARNING: {softmax} not found")

    # 3. Patch cross-entropy-loss.cu
    cross_entropy = ggml_cuda / "cross-entropy-loss.cu"
    if cross_entropy.exists():
        any_patched |= patch_replace_smpbo(cross_entropy, "smpbo → nbytes_shared in cross_entropy calls")
    else:
        log(f"  WARNING: {cross_entropy} not found")

    # 4. Patch mmid.cu
    mmid = ggml_cuda / "mmid.cu"
    if mmid.exists():
        any_patched |= patch_mmid_cu(mmid)
    else:
        log(f"  WARNING: {mmid} not found")

    if any_patched:
        log("Patches applied. Cleaning whisper-rs-sys build artifacts...")
        subprocess.run(
            ["cargo", "clean", "-p", "whisper-rs-sys"],
            cwd=Path(__file__).parent.parent,
        )
        log("Done. Next build will use patched CUDA sources.")
    else:
        log("All files already patched or no changes needed.")


if __name__ == "__main__":
    main()
