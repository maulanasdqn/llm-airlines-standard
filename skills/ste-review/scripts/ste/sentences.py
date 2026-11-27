from __future__ import annotations

import re
from dataclasses import dataclass

SENTENCE_END = re.compile(r"[.!?]+[\"')\]]*(?=\s+[A-Z0-9\"'(\[]|\s*$)")
WORD = re.compile(r"[A-Za-z0-9][\w'’./:+-]*")
CONDITION = re.compile(
    r"^(?:if|when|before|after|unless|while|once|until)\b[^,]*,\s*([a-z']+)",
    re.IGNORECASE,
)
MARKER_PREFIX = re.compile(r"^[A-Za-z]+:\s*")


@dataclass(frozen=True)
class Sentence:
    text: str
    line: int
    words: int
    is_instruction: bool


def count_words(text: str) -> int:
    return len(WORD.findall(text))


def find_first_word(text: str) -> str:
    found = WORD.search(text)
    if found is None:
        return ""
    return found.group(0).lower().rstrip(".:,")


def is_instruction(text: str, imperatives: set) -> bool:
    stripped = MARKER_PREFIX.sub("", text.strip())
    if find_first_word(stripped) in imperatives:
        return True
    condition = CONDITION.match(stripped)
    return condition is not None and condition.group(1).lower() in imperatives


def is_abbreviation_end(text: str, dot: int, abbreviations: set) -> bool:
    word_start = max(text.rfind(" ", 0, dot), text.rfind("\n", 0, dot)) + 1
    return text[word_start : dot + 1].lower() in abbreviations


def build_sentence(text: str, span: tuple, line_numbers: list) -> Sentence:
    start, end = span
    segment = text[start:end]
    leading = len(segment) - len(segment.lstrip())
    row = text.count("\n", 0, start + leading)
    return Sentence(
        text=segment.strip(),
        line=line_numbers[min(row, len(line_numbers) - 1)],
        words=count_words(segment),
        is_instruction=False,
    )


def list_spans(text: str, abbreviations: set) -> list:
    spans = []
    start = 0
    for ending in SENTENCE_END.finditer(text):
        if is_abbreviation_end(text, ending.start(), abbreviations):
            continue
        spans.append((start, ending.end()))
        start = ending.end()
    if text[start:].strip():
        spans.append((start, len(text)))
    return spans


def split_sentences(text: str, line_numbers: list, prose: dict) -> list:
    abbreviations = set(prose["period_abbreviations"])
    imperatives = set(prose["imperatives"])
    sentences = []
    for span in list_spans(text, abbreviations):
        sentence = build_sentence(text, span, line_numbers)
        if sentence.words == 0:
            continue
        sentences.append(
            Sentence(
                text=sentence.text,
                line=sentence.line,
                words=sentence.words,
                is_instruction=is_instruction(sentence.text, imperatives),
            )
        )
    return sentences
