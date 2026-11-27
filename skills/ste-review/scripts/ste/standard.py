from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

BUNDLE_PATH = Path(__file__).resolve().parent.parent / "ste_data.json"
CONFIG_NAME = ".ste.json"
LEXICON_KEYS = {
    "technical verbs": "verbs",
    "technical nouns": "nouns",
    "approved abbreviations": "abbreviations",
    "words to avoid": "avoid",
}


@dataclass
class Standard:
    rules: dict
    limits: dict
    dictionary: dict
    root: Path
    disabled: set = field(default_factory=set)
    excluded: list = field(default_factory=list)
    lexicon: dict = field(default_factory=dict)


@dataclass
class Vocabulary:
    verbs: set
    predicates: set
    avoid_verbs: dict
    avoid_nouns: dict
    avoid_abbreviations: dict
    approved_abbreviations: set


def find_config(start: Path) -> Optional[Path]:
    for folder in [start, *start.parents]:
        candidate = folder / CONFIG_NAME
        if candidate.is_file():
            return candidate
    return None


def merge_limits(base: dict, overrides: dict) -> dict:
    merged = {group: dict(values) for group, values in base.items()}
    for group, values in overrides.items():
        merged.setdefault(group, {}).update(values)
    return merged


def split_table_row(line: str) -> list:
    return [cell.strip().strip("`").strip() for cell in line.strip().strip("|").split("|")]


def is_table_body(line: str) -> bool:
    stripped = line.strip()
    is_row = stripped.startswith("|")
    is_rule = set(stripped) <= set("|-: ")
    return is_row and not is_rule


def parse_lexicon(text: str) -> dict:
    lexicon = {key: {} for key in LEXICON_KEYS.values()}
    section = None
    rows_seen = 0
    for line in text.splitlines():
        if line.startswith("## "):
            section = LEXICON_KEYS.get(line[3:].strip().lower())
            rows_seen = 0
            continue
        if section is None or not is_table_body(line):
            continue
        rows_seen += 1
        cells = split_table_row(line)
        if rows_seen > 1 and cells[0]:
            lexicon[section][cells[0].lower()] = cells[1] if len(cells) > 1 else ""
    return lexicon


def load_lexicon(path: Optional[Path]) -> dict:
    if path is None or not path.is_file():
        return parse_lexicon("")
    return parse_lexicon(path.read_text(encoding="utf-8"))


def load_standard(config_path: Optional[Path] = None) -> Standard:
    bundle = json.loads(BUNDLE_PATH.read_text(encoding="utf-8"))
    config_path = config_path or find_config(Path.cwd())
    config = {}
    root = Path.cwd()
    if config_path is not None:
        config = json.loads(config_path.read_text(encoding="utf-8"))
        root = config_path.resolve().parent
    lexicon_path = root / config.get("lexicon", "LEXICON.md")
    return Standard(
        rules=bundle["rules"],
        limits=merge_limits(bundle["limits"], config.get("limits", {})),
        dictionary=bundle["dictionary"],
        root=root,
        disabled=set(config.get("disable", [])),
        excluded=list(config.get("exclude", [])),
        lexicon=load_lexicon(lexicon_path),
    )


def list_words(entries: list) -> set:
    return {entry["word"].lower() for entry in entries}


def map_words(entries: list, allowed: set) -> dict:
    return {
        entry["word"].lower(): entry["use"]
        for entry in entries
        if entry["word"].lower() not in allowed
    }


def build_vocabulary(standard: Standard) -> Vocabulary:
    words = standard.dictionary
    lexicon = standard.lexicon
    allowed = set(lexicon["nouns"]) | set(lexicon["verbs"]) | set(lexicon["abbreviations"])
    avoid_nouns = map_words(words["avoid_nouns"], allowed)
    avoid_nouns.update(lexicon["avoid"])
    return Vocabulary(
        verbs=list_words(words["verbs"]) | set(lexicon["verbs"]),
        predicates=list_words(words["predicates"]),
        avoid_verbs=map_words(words["avoid_verbs"], allowed),
        avoid_nouns=avoid_nouns,
        avoid_abbreviations=map_words(words["abbreviations"]["avoid"], allowed),
        approved_abbreviations=set(words["abbreviations"]["approved"]) | set(lexicon["abbreviations"]),
    )
