from __future__ import annotations

import re
from typing import Optional

from .names import Name

OPENERS = {"(": ")", "<": ">", "{": "}"}


def count_code_lines(text: str) -> int:
    return sum(1 for line in text.splitlines() if line.strip())


def find_closing(code_text: str, start: int, opener: str) -> Optional[int]:
    closer = OPENERS[opener]
    depth = 0
    for position in range(start, len(code_text)):
        character = code_text[position]
        if character == opener:
            depth += 1
        elif character == closer:
            depth -= 1
            if depth == 0:
                return position
    return None


def find_generics_end(code_text: str, position: int) -> int:
    stripped = position
    while stripped < len(code_text) and code_text[stripped] in " \t":
        stripped += 1
    if stripped < len(code_text) and code_text[stripped] == "<":
        closing = find_closing(code_text, stripped, "<")
        return position if closing is None else closing + 1
    return position


def find_body_start(code_text: str, name: Name) -> Optional[int]:
    position = find_generics_end(code_text, name.offset + len(name.name))
    parameters = code_text.find("(", position)
    if parameters == -1:
        return None
    closing = find_closing(code_text, parameters, "(")
    if closing is None:
        return None
    body = re.compile(r"[{;]").search(code_text, closing)
    if body is None or body.group(0) == ";":
        return None
    return body.start()


def measure_braced(code_text: str, name: Name) -> int:
    body = find_body_start(code_text, name)
    if body is None:
        return 0
    closing = find_closing(code_text, body, "{")
    if closing is None:
        return 0
    line_start = code_text.rfind("\n", 0, name.offset) + 1
    return count_code_lines(code_text[line_start : closing + 1])


def find_indent(code_text: str, offset: int) -> int:
    line_start = code_text.rfind("\n", 0, offset) + 1
    line = code_text[line_start:offset]
    return len(line) - len(line.lstrip())


def measure_indented(code_text: str, name: Name) -> int:
    indent = find_indent(code_text, name.offset)
    line_start = code_text.rfind("\n", 0, name.offset) + 1
    lines = code_text[line_start:].splitlines()
    length = 1
    for line in lines[1:]:
        if not line.strip():
            continue
        if len(line) - len(line.lstrip()) <= indent and not line.lstrip().startswith(")"):
            break
        length += 1
    return length


def measure_function(code_text: str, name: Name, language: str) -> int:
    if language == "python":
        return measure_indented(code_text, name)
    return measure_braced(code_text, name)
