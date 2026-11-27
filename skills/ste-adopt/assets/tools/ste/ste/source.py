from __future__ import annotations

import re
from bisect import bisect_right
from dataclasses import dataclass

C_COMMENTS = r"(?P<comment>//[^\n]*|/\*[\s\S]*?\*/)"
QUOTED = r"\"(?:\\.|[^\"\\\n])*\""
PATTERNS = {
    "python": re.compile(
        r"(?P<comment>#[^\n]*)|(?P<string>(?<![\w])[rRbBuUfF]{0,2}"
        r"(?:\"\"\"[\s\S]*?\"\"\"|'''[\s\S]*?'''|" + QUOTED + r"|'(?:\\.|[^'\\\n])*'))"
    ),
    "typescript": re.compile(
        C_COMMENTS + r"|(?P<string>" + QUOTED + r"|'(?:\\.|[^'\\\n])*'|`(?:\\.|[^`\\])*`)"
    ),
    "go": re.compile(C_COMMENTS + r"|(?P<string>" + QUOTED + r"|`[^`]*`|'(?:\\.|[^'\\\n])+')"),
    "rust": re.compile(
        C_COMMENTS + r"|(?P<string>(?<![\w])b?r(?P<hashes>#*)\"[\s\S]*?\"(?P=hashes)"
        r"|b?\"(?:\\.|[^\"\\])*\"|b?'(?:\\u\{[0-9a-fA-F]+\}|\\.|[^'\\\n])')"
    ),
}
DOC_PREFIXES = {
    "typescript": ("/**",),
    "rust": ("///", "//!", "/**", "/*!"),
    "go": (),
    "python": (),
}


@dataclass(frozen=True)
class Token:
    kind: str
    start: int
    end: int
    text: str


def list_tokens(text: str, language: str) -> list:
    tokens = []
    for found in PATTERNS[language].finditer(text):
        kind = "comment" if found.group("comment") else "string"
        tokens.append(Token(kind, found.start(), found.end(), found.group(0)))
    return tokens


def build_code_text(text: str, tokens: list) -> str:
    parts = []
    position = 0
    for token in tokens:
        parts.append(text[position : token.start])
        parts.append(re.sub(r"[^\n]", " ", token.text))
        position = token.end
    parts.append(text[position:])
    return "".join(parts)


def to_comment_body(token: Token) -> str:
    if token.text.startswith("#"):
        return token.text[1:]
    if token.text.startswith("//"):
        return token.text[2:]
    return token.text[2:-2]


def is_doc_comment(token: Token, language: str) -> bool:
    prefixes = DOC_PREFIXES[language]
    is_empty_block = token.text.startswith("/**/")
    is_slashes = token.text.startswith("////")
    return token.text.startswith(prefixes) and not is_empty_block and not is_slashes


def build_line_index(text: str) -> list:
    return [0] + [found.end() for found in re.finditer(r"\n", text)]


def find_line(index: list, offset: int) -> int:
    return bisect_right(index, offset)


def compute_depths(code_text: str, offsets: list) -> dict:
    depths = {}
    depth = 0
    pending = sorted(offsets)
    cursor = 0
    for position, character in enumerate(code_text):
        while cursor < len(pending) and pending[cursor] == position:
            depths[position] = depth
            cursor += 1
        if character in "([{":
            depth += 1
        elif character in ")]}":
            depth = max(depth - 1, 0)
    return depths


def is_bare_statement(text: str, code_text: str, token: Token) -> bool:
    line_start = text.rfind("\n", 0, token.start) + 1
    line_end = text.find("\n", token.end)
    line_end = len(text) if line_end == -1 else line_end
    prefix = text[line_start : token.start]
    suffix = code_text[token.end : line_end]
    return not prefix.strip() and suffix.strip() in ("", ";")


def find_docstrings(text: str, tokens: list, code_text: str) -> list:
    strings = [token for token in tokens if token.kind == "string"]
    depths = compute_depths(code_text, [token.start for token in strings])
    return [
        token
        for token in strings
        if depths.get(token.start, 0) == 0 and is_bare_statement(text, code_text, token)
    ]
