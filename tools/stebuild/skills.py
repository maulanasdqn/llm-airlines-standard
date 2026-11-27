from __future__ import annotations

import re
from pathlib import Path

NAME_PATTERN = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
MAX_NAME_CHARS = 64
MAX_DESCRIPTION_CHARS = 1024
MAX_SKILL_LINES = 500


def parse_frontmatter(text: str) -> dict:
    lines = text.splitlines()
    if not lines or lines[0].strip() != "---":
        return {}
    fields = {}
    for line in lines[1:]:
        if line.strip() == "---":
            return fields
        key, _, value = line.partition(":")
        if value:
            fields[key.strip()] = value.strip().strip("'\"")
    return {}


def validate_skill(folder: Path) -> list:
    path = folder / "SKILL.md"
    if not path.is_file():
        return [f"{folder}: the folder has no SKILL.md file."]
    text = path.read_text(encoding="utf-8")
    fields = parse_frontmatter(text)
    name = fields.get("name", "")
    description = fields.get("description", "")
    problems = []
    if name != folder.name:
        problems.append(f'{path}: the name "{name}" must be the folder name "{folder.name}".')
    if not NAME_PATTERN.match(name) or len(name) > MAX_NAME_CHARS:
        problems.append(f"{path}: the name must use lowercase letters, digits, and hyphens, in {MAX_NAME_CHARS} characters or fewer.")
    if not description or len(description) > MAX_DESCRIPTION_CHARS:
        problems.append(f"{path}: the description must have 1 to {MAX_DESCRIPTION_CHARS} characters.")
    if len(text.splitlines()) >= MAX_SKILL_LINES:
        problems.append(f"{path}: SKILL.md must have fewer than {MAX_SKILL_LINES} lines. Move detail to references.")
    return problems


def validate_skills(root: Path) -> list:
    folders = sorted(path for path in (root / "skills").iterdir() if path.is_dir())
    return [problem for folder in folders for problem in validate_skill(folder)]
