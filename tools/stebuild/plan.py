from __future__ import annotations

import json
from pathlib import Path

from .render import (
    build_checker_bundle,
    render_dictionary,
    render_json,
    render_mapping,
    render_words,
)
from .rules import build_limits, load_rules
from .templating import build_context, render_template

PROFILES = ("typescript", "python", "go", "rust")
RECIPES = {
    "base": ("header", "prose", "code", "footer"),
    "docs": ("header", "prose", "docs", "footer"),
    **{profile: ("header", "prose", "code", f"profiles/{profile}", "footer") for profile in PROFILES},
}
COPIES = {
    "spec/writing/rules.md": (
        "skills/ste-writing/references/rules.md",
        "skills/ste-code/references/writing-rules.md",
        "skills/ste-review/references/writing-rules.md",
    ),
    "spec/writing/examples.md": ("skills/ste-writing/references/examples.md",),
    "spec/writing/words.md": ("skills/ste-writing/references/words.md", "skills/ste-review/references/words.md"),
    "spec/code/rules.md": ("skills/ste-code/references/rules.md", "skills/ste-review/references/code-rules.md"),
    "spec/code/examples.md": ("skills/ste-code/references/examples.md",),
    "spec/code/dictionary.md": ("skills/ste-code/references/dictionary.md", "skills/ste-review/references/dictionary.md"),
    "spec/code/mapping.md": ("skills/ste-code/references/mapping.md", "skills/ste-review/references/mapping.md"),
    "templates/LEXICON.md": ("skills/ste-adopt/assets/LEXICON.md",),
    **{f"spec/code/profiles/{profile}.md": (f"skills/ste-code/references/profiles/{profile}.md",) for profile in PROFILES},
}
MIRRORS = {
    "templates/agents": ("skills/ste-adopt/assets/agents", False),
    "linters": ("skills/ste-adopt/assets/linters", False),
    "skills/ste-review/scripts": ("skills/ste-adopt/assets/tools/ste", True),
}
INSTRUCTION_NAMES = {"AGENTS.md", "CLAUDE.md"}
OWNED_FOLDERS = ("templates/agents", "skills/ste-adopt/assets", "linters")
EXECUTABLE_NAMES = {"commit-msg", "ste_check.py"}


def read_text(root: Path, relative: str) -> str:
    return (root / relative).read_text(encoding="utf-8")


def build_spec_outputs(root: Path) -> tuple:
    rules = load_rules(root / "spec/writing/rules.md") + load_rules(root / "spec/code/rules.md")
    limits = build_limits(rules)
    dictionary = json.loads(read_text(root, "spec/dictionary.json"))
    outputs = {
        "spec/limits.json": render_json(limits),
        "spec/code/dictionary.md": render_dictionary(dictionary),
        "spec/writing/words.md": render_words(dictionary),
        "spec/code/mapping.md": render_mapping(rules),
        "skills/ste-review/scripts/ste_data.json": render_json(build_checker_bundle(rules, limits, dictionary)),
    }
    return outputs, build_context(limits, dictionary), dictionary


def build_agent_outputs(root: Path, context: dict, dictionary: dict) -> dict:
    outputs = {}
    titles = {"base": "any language", "docs": "documentation"}
    titles.update({profile: dictionary["profiles"][profile]["title"] for profile in PROFILES})
    for recipe, parts in RECIPES.items():
        recipe_context = {**context, "profile": {"key": recipe, "title": titles[recipe]}}
        text = "\n".join(read_text(root, f"templates/src/{part}.md").rstrip("\n") + "\n" for part in parts)
        outputs[f"templates/agents/{recipe}/AGENTS.md"] = render_template(text, recipe_context)
    return outputs


def build_linter_outputs(root: Path, context: dict) -> dict:
    outputs = {}
    source = root / "linters/src"
    for path in sorted(source.rglob("*")):
        if path.is_file():
            relative = path.relative_to(source).as_posix().removesuffix(".tmpl")
            outputs[f"linters/{relative}"] = render_template(path.read_text(encoding="utf-8"), context)
    return outputs


def build_fixture_outputs(root: Path, outputs: dict) -> dict:
    template = read_text(root, "tests/fixtures/code/rust/Cargo.toml.tmpl")
    fixtures = {}
    for kind in ("good", "bad"):
        context = {"crate": f"ste-fixture-{kind}", "lints": outputs["linters/rust/lints.toml"].strip()}
        fixtures[f"tests/fixtures/code/rust/{kind}/Cargo.toml"] = render_template(template, context)
    return fixtures


def find_source(root: Path, outputs: dict, relative: str) -> bytes:
    if relative in outputs:
        value = outputs[relative]
        return value.encode("utf-8") if isinstance(value, str) else value
    return (root / relative).read_bytes()


def is_mirrored(relative: str, folder: str) -> bool:
    parts = relative.split("/")
    is_folder_instruction = relative in {f"{folder}/{name}" for name in INSTRUCTION_NAMES}
    return "__pycache__" not in parts and "src" not in parts[1:2] and not is_folder_instruction


def list_mirror_sources(root: Path, outputs: dict, folder: str) -> list:
    is_on_disk = MIRRORS[folder][1]
    on_disk = set()
    if is_on_disk:
        on_disk = {path.relative_to(root).as_posix() for path in (root / folder).rglob("*") if path.is_file()}
    generated = {path for path in outputs if path.startswith(folder + "/")}
    return sorted(path for path in on_disk | generated if is_mirrored(path, folder))


def build_copy_outputs(root: Path, outputs: dict) -> dict:
    copies = {}
    for source, targets in COPIES.items():
        for target in targets:
            copies[target] = find_source(root, outputs, source)
    for folder, (target_folder, _) in MIRRORS.items():
        for source in list_mirror_sources(root, outputs, folder):
            copies[target_folder + source[len(folder) :]] = find_source(root, outputs, source)
    return copies


def build_outputs(root: Path) -> dict:
    outputs, context, dictionary = build_spec_outputs(root)
    outputs.update(build_agent_outputs(root, context, dictionary))
    outputs.update(build_linter_outputs(root, context))
    outputs.update(build_fixture_outputs(root, outputs))
    outputs.update(build_copy_outputs(root, outputs))
    return {path: value.encode("utf-8") if isinstance(value, str) else value for path, value in outputs.items()}
