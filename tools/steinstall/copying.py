from __future__ import annotations

import shutil
from pathlib import Path

IGNORED_NAMES = ("__pycache__", "*.pyc", ".DS_Store")
SKILL_FILE = "SKILL.md"


def is_ignored(path: Path) -> bool:
    return "__pycache__" in path.parts or path.suffix == ".pyc" or path.name == ".DS_Store"


def list_tree(folder: Path) -> dict:
    return {
        path.relative_to(folder).as_posix(): path.read_bytes()
        for path in folder.rglob("*")
        if path.is_file() and not is_ignored(path)
    }


def find_status(source: Path, destination: Path) -> str:
    if not destination.exists():
        return "new"
    if not (destination / SKILL_FILE).is_file():
        return "blocked"
    return "unchanged" if list_tree(source) == list_tree(destination) else "changed"


def copy_skill(source: Path, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(source, destination, ignore=shutil.ignore_patterns(*IGNORED_NAMES))


def replace_skill(source: Path, destination: Path) -> None:
    if not (destination / SKILL_FILE).is_file():
        raise FileExistsError(f"Cannot replace {destination}: the folder has no {SKILL_FILE} file.")
    shutil.rmtree(destination)
    copy_skill(source, destination)
