from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path

from stebuild.skills import parse_frontmatter

FIRST_SENTENCE = re.compile(r"^(.+?[.!?])(?:\s|$)")


@dataclass(frozen=True)
class Skill:
    name: str
    summary: str
    folder: Path


@dataclass(frozen=True)
class Target:
    key: str
    label: str
    folder: str


TARGETS = (
    Target("claude-user", "Claude Code, for your user account", "~/.claude/skills"),
    Target("claude-project", "Claude Code, for the current project", ".claude/skills"),
    Target("agents-user", "Codex, Cursor, and other agents, for your user account", "~/.agents/skills"),
    Target("agents-project", "Codex, Cursor, and other agents, for the current project", ".agents/skills"),
)


def find_summary(description: str) -> str:
    found = FIRST_SENTENCE.match(description)
    return found.group(1) if found is not None else description


def list_skills(root: Path) -> list:
    skills = []
    for folder in sorted((root / "skills").iterdir()):
        path = folder / "SKILL.md"
        if not path.is_file():
            continue
        fields = parse_frontmatter(path.read_text(encoding="utf-8"))
        name = fields.get("name", folder.name)
        skills.append(Skill(name, find_summary(fields.get("description", "")), folder))
    return skills


def resolve_folder(folder: str, project: Path) -> Path:
    path = Path(folder).expanduser()
    return path if path.is_absolute() else project / path
