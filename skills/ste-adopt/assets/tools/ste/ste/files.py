from __future__ import annotations

import os
from fnmatch import fnmatch
from pathlib import Path

SKIPPED_FOLDERS = {
    ".git",
    ".hg",
    ".venv",
    "venv",
    "node_modules",
    "vendor",
    "target",
    "dist",
    "build",
    "__pycache__",
    ".mypy_cache",
    ".ruff_cache",
    ".pytest_cache",
}


def to_display_path(path: Path) -> str:
    try:
        return path.resolve().relative_to(Path.cwd().resolve()).as_posix()
    except ValueError:
        return path.as_posix()


def is_excluded(path: Path, root: Path, patterns: list) -> bool:
    try:
        relative = path.resolve().relative_to(root.resolve()).as_posix()
    except ValueError:
        relative = path.as_posix()
    return any(fnmatch(relative, pattern) for pattern in patterns)


def list_folder_files(folder: Path) -> list:
    found = []
    for current, folders, names in os.walk(folder):
        folders[:] = sorted(name for name in folders if name not in SKIPPED_FOLDERS)
        found.extend(Path(current) / name for name in sorted(names))
    return found


def list_files(paths: list, extensions: set, *, root: Path, excluded: list) -> list:
    found = []
    for raw_path in paths:
        path = Path(raw_path)
        candidates = list_folder_files(path) if path.is_dir() else [path]
        found.extend(
            candidate
            for candidate in candidates
            if candidate.suffix.lower() in extensions
            and not is_excluded(candidate, root, excluded)
        )
    return found
