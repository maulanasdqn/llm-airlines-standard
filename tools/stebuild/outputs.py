from __future__ import annotations

import stat
from pathlib import Path

from .plan import EXECUTABLE_NAMES, INSTRUCTION_NAMES, OWNED_FOLDERS

TOP_LEVEL_PARTS = 2


def is_owned(relative: str) -> bool:
    parts = relative.split("/")
    is_source = relative.startswith("linters/src/")
    is_instruction = parts[-1] in INSTRUCTION_NAMES and len(parts) == TOP_LEVEL_PARTS
    is_cache = "__pycache__" in parts
    return any(relative.startswith(folder + "/") for folder in OWNED_FOLDERS) and not (is_source or is_instruction or is_cache)


def list_stale(root: Path, outputs: dict) -> list:
    stale = []
    for folder in OWNED_FOLDERS:
        for path in (root / folder).rglob("*"):
            relative = path.relative_to(root).as_posix()
            if path.is_file() and is_owned(relative) and relative not in outputs:
                stale.append(relative)
    return sorted(stale)


def find_drift(root: Path, outputs: dict) -> list:
    drift = []
    for relative, content in sorted(outputs.items()):
        path = root / relative
        if not path.is_file():
            drift.append(f"missing: {relative}")
        elif path.read_bytes() != content:
            drift.append(f"changed: {relative}")
    drift.extend(f"stale: {relative}" for relative in list_stale(root, outputs))
    return drift


def write_outputs(root: Path, outputs: dict) -> list:
    written = []
    for relative, content in sorted(outputs.items()):
        path = root / relative
        if path.is_file() and path.read_bytes() == content:
            continue
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(content)
        if path.name in EXECUTABLE_NAMES:
            path.chmod(path.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
        written.append(relative)
    for relative in list_stale(root, outputs):
        (root / relative).unlink()
        written.append(f"deleted {relative}")
    return written
