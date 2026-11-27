from __future__ import annotations

import re
from dataclasses import dataclass
from typing import Callable

from .source import build_line_index, compute_depths, find_line

WORD_PARTS = re.compile(r"[A-Z]+(?=[A-Z][a-z])|[A-Z]?[a-z]+|[A-Z]+|\d+")
SELF_PARAMETERS = {"self", "&self", "&mut self", "mut self"}
SCRIPT_KEYWORDS = {"if", "for", "while", "switch", "catch", "function", "return", "constructor", "with", "else"}
FINDERS = {}


@dataclass(frozen=True)
class Name:
    name: str
    kind: str
    offset: int
    is_noun_allowed: bool = False


def split_words(name: str) -> list:
    words = []
    for part in name.strip("_$#").split("_"):
        words.extend(WORD_PARTS.findall(part))
    return words


def register_finder(language: str) -> Callable[[Callable], Callable]:
    def register(finder: Callable) -> Callable:
        FINDERS[language] = finder
        return finder

    return register


def find_names(code_text: str, language: str) -> list:
    return FINDERS[language](code_text)


def find_name_line(code_text: str, name: Name) -> int:
    return find_line(build_line_index(code_text), name.offset)


def is_property(code_text: str, offset: int) -> bool:
    line_start = code_text.rfind("\n", 0, offset)
    previous = code_text[:line_start].rstrip().rsplit("\n", 1)[-1].strip()
    return re.match(r"@(property|cached_property|\w+\.(setter|getter|deleter))\b", previous) is not None


@register_finder("python")
def find_python_names(code_text: str) -> list:
    names = []
    for found in re.finditer(r"^[ \t]*(?:async\s+)?def\s+(\w+)", code_text, re.MULTILINE):
        name = found.group(1)
        is_dunder = name.startswith("__") and name.endswith("__")
        is_noun = is_dunder or is_property(code_text, found.start())
        names.append(Name(name, "function", found.start(1), is_noun))
    for found in re.finditer(r"^[ \t]*class\s+(\w+)", code_text, re.MULTILINE):
        names.append(Name(found.group(1), "type", found.start(1)))
    assignments = list(re.finditer(r"^[ \t]*([A-Za-z_]\w*)\s*(?::[^=\n]+)?=(?!=)", code_text, re.MULTILINE))
    depths = compute_depths(code_text, [found.start(1) for found in assignments])
    for found in assignments:
        if depths.get(found.start(1), 0) == 0:
            names.append(Name(found.group(1), "variable", found.start(1)))
    return names


SCRIPT_FUNCTION = re.compile(r"\bfunction\s*\*?\s*([A-Za-z_$][\w$]*)\s*[<(]")
SCRIPT_ARROW = re.compile(
    r"\b(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*(?::[^=\n]+)?=\s*(?:async\s+)?"
    r"(?:function\b|\([^)]*\)\s*(?::[^=\n]+)?=>|[A-Za-z_$][\w$]*\s*=>)"
)
SCRIPT_VARIABLE = re.compile(r"\b(?:const|let|var)\s+([A-Za-z_$][\w$]*)")
SCRIPT_TYPE = re.compile(r"\b(?:class|interface|type|enum)\s+([A-Za-z_$][\w$]*)")
SCRIPT_METHOD = re.compile(
    r"^[ \t]*(?:(?:public|private|protected|static|async|override|readonly|abstract)\s+)*"
    r"(get\s+|set\s+)?(#?[A-Za-z_$][\w$]*)\s*(?:<[^>\n]*>)?\s*\([^;\n]*\)\s*(?::\s*[^={;\n]+)?\{\s*$",
    re.MULTILINE,
)


def is_pascal_case(name: str) -> bool:
    return name[:1].isupper() and not name.isupper()


@register_finder("typescript")
def find_script_names(code_text: str) -> list:
    names = []
    functions = {}
    for pattern in (SCRIPT_FUNCTION, SCRIPT_ARROW):
        for found in pattern.finditer(code_text):
            functions[found.start(1)] = found.group(1)
    for offset, name in functions.items():
        names.append(Name(name, "function", offset, is_pascal_case(name)))
    for found in SCRIPT_METHOD.finditer(code_text):
        name = found.group(2)
        if name not in SCRIPT_KEYWORDS:
            names.append(Name(name, "function", found.start(2), found.group(1) is not None))
    for found in SCRIPT_VARIABLE.finditer(code_text):
        if found.start(1) not in functions:
            names.append(Name(found.group(1), "variable", found.start(1)))
    for found in SCRIPT_TYPE.finditer(code_text):
        names.append(Name(found.group(1), "type", found.start(1)))
    return names


@register_finder("go")
def find_go_names(code_text: str) -> list:
    names = []
    pattern = r"^func\s*(\([^)]*\))?\s*([A-Za-z_]\w*)\s*(?:\[[^\]]*\])?\s*\(([^)]*)\)"
    for found in re.finditer(pattern, code_text, re.MULTILINE):
        is_getter = found.group(1) is not None and not found.group(3).strip()
        names.append(Name(found.group(2), "function", found.start(2), is_getter))
    for found in re.finditer(r"^\s*type\s+([A-Za-z_]\w*)", code_text, re.MULTILINE):
        names.append(Name(found.group(1), "type", found.start(1)))
    for found in re.finditer(r"^\s*((?:[A-Za-z_]\w*\s*,\s*)*[A-Za-z_]\w*)\s*:=", code_text, re.MULTILINE):
        for part in re.finditer(r"[A-Za-z_]\w*", found.group(1)):
            names.append(Name(part.group(0), "variable", found.start(1) + part.start()))
    for found in re.finditer(r"^\s*(?:var|const)\s+([A-Za-z_]\w*)", code_text, re.MULTILINE):
        names.append(Name(found.group(1), "variable", found.start(1)))
    return [name for name in names if name.name != "_"]


@register_finder("rust")
def find_rust_names(code_text: str) -> list:
    names = []
    for found in re.finditer(r"\bfn\s+([A-Za-z_]\w*)\s*(?:<[^>{]*>)?\s*\(([^)]*)\)", code_text):
        is_getter = found.group(2).strip().rstrip(",") in SELF_PARAMETERS
        names.append(Name(found.group(1), "function", found.start(1), is_getter))
    for found in re.finditer(r"\b(?:struct|enum|trait|type|union)\s+([A-Za-z_]\w*)", code_text):
        names.append(Name(found.group(1), "type", found.start(1)))
    for found in re.finditer(r"\blet\s+(?:mut\s+)?([A-Za-z_]\w*)", code_text):
        names.append(Name(found.group(1), "variable", found.start(1)))
    for found in re.finditer(r"\b(?:const|static)\s+(?:mut\s+)?([A-Za-z_]\w*)\s*:", code_text):
        names.append(Name(found.group(1), "variable", found.start(1)))
    return [name for name in names if name.name != "_"]
