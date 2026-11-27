from __future__ import annotations

import re
from dataclasses import dataclass
from typing import Optional

from .findings import Finding
from .markdown import Block
from .sentences import Sentence, is_instruction

BE_FORMS = r"\b(?:am|is|are|was|were|be|been|being)\s+(?:not\s+)?(?:\w+ly\s+)?([a-z]+)\b"
HAVE_FORMS = r"\b(?:has|have|had)\s+(?:not\s+)?(?:\w+ly\s+)?([a-z]+)\b"
PROGRESSIVE_FORMS = r"\b(?:am|is|are|was|were|be|been)\s+(?:not\s+)?([a-z]+ing)\b"
JOINERS = re.compile(r";|—|–| -- ")
MARKER = re.compile(r"^(WARNING|CAUTION|NOTE|Warning|Caution|Note|IMPORTANT|Important|DANGER|Danger|TIP|Tip)\s*:")
STANDARD_MARKERS = {"WARNING", "CAUTION", "NOTE"}
COMMAND_MARKERS = {"WARNING", "CAUTION"}
SHORT_WORD_CHARS = 3


@dataclass
class ProsePatterns:
    prose: dict
    passive: re.Pattern
    perfect: re.Pattern
    progressive: re.Pattern
    avoided: list


@dataclass
class ProseContext:
    path: str
    limits: dict
    patterns: ProsePatterns


def build_patterns(prose: dict) -> ProsePatterns:
    avoided = [
        (re.compile(r"(?<![\w-])" + re.escape(entry["text"]) + r"(?![\w-])", re.IGNORECASE), entry)
        for entry in prose["avoid"]
    ]
    return ProsePatterns(
        prose=prose,
        passive=re.compile(BE_FORMS),
        perfect=re.compile(HAVE_FORMS),
        progressive=re.compile(PROGRESSIVE_FORMS),
        avoided=avoided,
    )


def is_participle(word: str, prose: dict) -> bool:
    return (word.endswith("ed") and len(word) > SHORT_WORD_CHARS) or word in prose["participles"]


def find_length(sentence: Sentence, context: ProseContext) -> list:
    limits = context.limits["prose"]
    if sentence.is_instruction:
        limit, rule, kind = limits["instruction_words"], "W5.1", "This instruction"
    else:
        limit, rule, kind = limits["description_words"], "W6.1", "This sentence"
    if sentence.words <= limit:
        return []
    message = f"{kind} has {sentence.words} words. The limit is {limit}. Split it into shorter sentences."
    return [Finding(context.path, sentence.line, rule, message)]


def find_passive(text: str, context: ProseContext) -> Optional[str]:
    prose = context.patterns.prose
    for found in context.patterns.passive.finditer(text):
        word = found.group(1)
        if is_participle(word, prose) and word not in prose["stative"]:
            return f'Passive voice: "{found.group(0)}". Name the actor and use the active voice.'
    return None


def find_perfect(text: str, context: ProseContext) -> Optional[str]:
    for found in context.patterns.perfect.finditer(text):
        if is_participle(found.group(1), context.patterns.prose):
            return f'Compound verb form: "{found.group(0)}". Use the simple past or the simple present.'
    return None


def find_progressive(text: str, context: ProseContext) -> Optional[str]:
    for found in context.patterns.progressive.finditer(text):
        if found.group(1) not in context.patterns.prose["ing_exceptions"]:
            return f'"-ing" verb form: "{found.group(0)}". Use the simple present or the simple past.'
    return None


def find_verb_forms(sentence: Sentence, context: ProseContext) -> list:
    text = sentence.text.lower()
    checks = (("W3.3", find_passive), ("W3.1", find_perfect), ("W3.2", find_progressive))
    findings = []
    for rule, find_form in checks:
        message = find_form(text, context)
        if message is not None:
            findings.append(Finding(context.path, sentence.line, rule, message))
    return findings


def find_hedges(sentence: Sentence, context: ProseContext) -> list:
    words = re.findall(r"[a-z]+", sentence.text.lower())
    hedges = [word for word in words if word in context.patterns.prose["hedges"]]
    limit = context.limits["prose"]["hedges_per_sentence"]
    if len(hedges) <= limit:
        return []
    message = f"This sentence has {len(hedges)} hedges ({', '.join(hedges)}). Keep {limit} or fewer. State the fact."
    return [Finding(context.path, sentence.line, "W4.3", message)]


def find_avoided_words(sentence: Sentence, context: ProseContext) -> list:
    findings = []
    for pattern, entry in context.patterns.avoided:
        if pattern.search(sentence.text):
            message = f'Do not use "{entry["text"]}". Instead, write: {entry["use"]}.'
            findings.append(Finding(context.path, sentence.line, entry["rule"], message))
    return findings


def find_joiners(sentence: Sentence, context: ProseContext) -> list:
    if JOINERS.search(sentence.text) is None:
        return []
    message = "A semicolon or a dash joins two sentences. Write two sentences."
    return [Finding(context.path, sentence.line, "W8.2", message)]


SENTENCE_RULES = (find_length, find_verb_forms, find_hedges, find_avoided_words, find_joiners)


def find_sentence_findings(sentence: Sentence, context: ProseContext) -> list:
    return [finding for rule in SENTENCE_RULES for finding in rule(sentence, context)]


def find_marker(block: Block, first: Sentence) -> str:
    if block.kind == "alert":
        return block.marker
    found = MARKER.match(first.text)
    return found.group(1) if found is not None else ""


def find_marker_findings(block: Block, sentences: list, context: ProseContext) -> list:
    marker = find_marker(block, sentences[0])
    if not marker:
        return []
    line = sentences[0].line
    if marker not in STANDARD_MARKERS:
        message = f'Use WARNING, CAUTION, or NOTE in capitals. Found "{marker}".'
        return [Finding(context.path, line, "W7.1", message)]
    imperatives = set(context.patterns.prose["imperatives"])
    if marker in COMMAND_MARKERS and not is_instruction(sentences[0].text, imperatives):
        message = f"Start the {marker} with a clear command, such as \"Do not\" or \"Back up\"."
        return [Finding(context.path, line, "W7.2", message)]
    return []


def find_block_findings(block: Block, sentences: list, context: ProseContext) -> list:
    if not sentences:
        return []
    findings = find_marker_findings(block, sentences, context)
    limit = context.limits["prose"]["paragraph_sentences"]
    if len(sentences) > limit:
        message = f"This paragraph has {len(sentences)} sentences. The limit is {limit}. Split it by topic."
        findings.append(Finding(context.path, sentences[0].line, "W6.3", message))
    return findings
