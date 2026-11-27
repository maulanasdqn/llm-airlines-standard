from __future__ import annotations

from collections import Counter, defaultdict
from dataclasses import dataclass

from .standard import Vocabulary
from .findings import Finding

TOP_NOUNS = 25
MIN_ROTATION_VERBS = 2


@dataclass(frozen=True)
class NameUse:
    name: str
    words: tuple
    path: str
    line: int
    language: str = ""


def build_rotation_index(uses: list) -> dict:
    index = defaultdict(lambda: defaultdict(list))
    for use in uses:
        if len(use.words) > 1:
            index[(use.language, " ".join(use.words[1:]))][use.words[0]].append(use)
    return index


def find_group_rotation(verbs: dict, group: list, vocabulary: Vocabulary) -> list:
    used = [verb for verb in group if verb in verbs]
    if len(used) < MIN_ROTATION_VERBS:
        return []
    severity = "info" if all(verb in vocabulary.verbs for verb in used) else "warning"
    names = ", ".join(sorted({verbs[verb][0].name for verb in used}))
    message = f"These names use {len(used)} verbs for one object: {names}. Use one verb, or keep each verb contract (C3.2)."
    return [
        Finding(use.path, use.line, "C1.3", message, severity=severity)
        for verb in used[1:]
        for use in verbs[verb]
    ]


def find_rotations(uses: list, vocabulary: Vocabulary, groups: list) -> list:
    findings = []
    for verbs in build_rotation_index(uses).values():
        for group in groups:
            findings.extend(find_group_rotation(verbs, group, vocabulary))
    return findings


def render_table(title: str, rows: list) -> list:
    lines = [f"## {title}", "", "| Word | Meaning | Uses |", "|---|---|---|"]
    lines.extend(f"| `{word}` | (write one meaning) | {count} |" for word, count in rows)
    return lines + [""]


def render_findings(uses: list, vocabulary: Vocabulary) -> list:
    avoided = Counter(
        word
        for use in uses
        for word in use.words
        if word in vocabulary.avoid_verbs or word in vocabulary.avoid_nouns
    )
    lines = ["## Findings", "", "These words break C1.6. Replace them in the code.", ""]
    for word, count in avoided.most_common():
        replacement = vocabulary.avoid_verbs.get(word) or vocabulary.avoid_nouns.get(word)
        lines.append(f"- `{word}` ({count} uses): use {replacement}.")
    return lines + [""]


def render_lexicon(uses: list, vocabulary: Vocabulary) -> str:
    known = vocabulary.verbs | vocabulary.predicates | set(vocabulary.avoid_verbs)
    verbs = Counter(use.words[0] for use in uses if use.words and use.words[0] not in known)
    nouns = Counter(word for use in uses for word in use.words[1:] if word not in vocabulary.avoid_nouns)
    lines = [
        "# Lexicon draft",
        "",
        "Review each row. Write one meaning for each word, or delete the row.",
        "",
    ]
    lines += render_table("Technical verbs", sorted(verbs.items()))
    lines += render_table("Technical nouns", nouns.most_common(TOP_NOUNS))
    lines += render_findings(uses, vocabulary)
    return "\n".join(lines)
