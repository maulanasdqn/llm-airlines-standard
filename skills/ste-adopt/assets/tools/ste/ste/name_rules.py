from __future__ import annotations

from dataclasses import dataclass

from .standard import Standard, Vocabulary
from .findings import Finding
from .names import Name, split_words

NEGATIVE_WORDS = {"not", "no", "non"}
MIN_NAME_CHARS = 2
EVENT_PREFIX = "on"
TRY_PREFIX = "try"
CALLBACK_PREFIX = "handle"


@dataclass
class CodeContext:
    path: str
    language: str
    extension: str
    profile: dict
    standard: Standard
    vocabulary: Vocabulary


def is_exempt(name: Name, context: CodeContext) -> bool:
    profile = context.profile
    is_dunder = name.name.startswith("__") and name.name.endswith("__")
    if is_dunder or name.name in profile["exempt_names"]:
        return True
    return any(name.name.startswith(prefix) for prefix in profile["exempt_prefixes"])


def find_vague_words(name: Name, words: list, context: CodeContext) -> list:
    vocabulary = context.vocabulary
    messages = []
    if name.kind == "function" and words and words[0] in vocabulary.avoid_verbs:
        messages.append(f'"{words[0]}" is a vague verb. Use {vocabulary.avoid_verbs[words[0]]}.')
    for word in words:
        if word in vocabulary.avoid_nouns:
            messages.append(f'"{word}" is a vague word. Use {vocabulary.avoid_nouns[word]}.')
    return [("C1.6", message) for message in messages]


def find_abbreviations(words: list, context: CodeContext) -> list:
    vocabulary = context.vocabulary
    allowed = vocabulary.approved_abbreviations | set(context.profile["allowed_abbreviations"])
    return [
        ("C1.7", f'"{word}" is an abbreviation. Write {vocabulary.avoid_abbreviations[word]}.')
        for word in words
        if word in vocabulary.avoid_abbreviations and word not in allowed
    ]


def find_long_name(name: Name, words: list, context: CodeContext) -> list:
    limit = context.standard.limits["code"]["name_words"]
    is_test = any(name.name.startswith(prefix) for prefix in context.profile["exempt_prefixes"])
    if len(words) <= limit or is_test:
        return []
    return [("C2.1", f"This name has {len(words)} words. The limit is {limit}. Move context into a module or a type.")]


def find_verb_start(words: list) -> str:
    first = words[0]
    if first == TRY_PREFIX and len(words) > 1:
        return words[1]
    return first


def is_approved_start(words: list, context: CodeContext) -> bool:
    vocabulary = context.vocabulary
    first = find_verb_start(words)
    is_callback = first == CALLBACK_PREFIX and context.extension in context.profile["handler_prefix_extensions"]
    approved = vocabulary.verbs | vocabulary.predicates | set(context.profile["verbs"])
    return first == EVENT_PREFIX or is_callback or first in approved or first in vocabulary.avoid_verbs


def find_missing_verb(name: Name, words: list, context: CodeContext) -> list:
    if name.kind != "function" or name.is_noun_allowed or is_approved_start(words, context):
        return []
    first = find_verb_start(words)
    message = (
        f'"{name.name}" does not start with an approved verb. '
        f'Use a verb from the dictionary, or record "{first}" as a technical verb in LEXICON.md.'
    )
    return [("C3.1", message)]


def find_negative_name(words: list, context: CodeContext) -> list:
    predicates = context.vocabulary.predicates
    is_negative = "not" in words or (words[0] == "no" and len(words) > 1)
    if len(words) > 1 and words[0] in predicates and words[1] in NEGATIVE_WORDS:
        is_negative = True
    if not is_negative:
        return []
    return [("C4.4", "This name is negative. Use the positive form, such as isEnabled for isNotDisabled.")]


def find_name_findings(name: Name, line: int, context: CodeContext) -> list:
    words = [word.lower() for word in split_words(name.name)]
    if len(name.name) < MIN_NAME_CHARS or not words or is_exempt(name, context):
        return []
    results = (
        find_vague_words(name, words, context)
        + find_abbreviations(words, context)
        + find_long_name(name, words, context)
        + find_missing_verb(name, words, context)
        + find_negative_name(words, context)
    )
    return [Finding(context.path, line, rule, message) for rule, message in results]
