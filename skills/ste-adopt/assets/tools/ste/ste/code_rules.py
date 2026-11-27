from __future__ import annotations

import re
from dataclasses import dataclass

from .blocks import count_code_lines, measure_function
from .findings import Finding, is_file_ignored, parse_ignore_ids
from .name_rules import CodeContext
from .source import Token, find_line, is_doc_comment, to_comment_body

ERROR_CALL = re.compile(
    r"(?:new\s+\w*Error|raise\s+[\w.]+|errors\.New|fmt\.Errorf|panic!?|bail!|anyhow!|\.expect)\s*\(\s*$"
)
COMMENT_MESSAGE = (
    "Remove this comment. Put the meaning in a name, a constant, or a type. "
    "Put the reason in the commit message."
)
DOC_MESSAGE = "Remove this doc comment. The name, the parameters, and the types describe the code."
DOCSTRING_MESSAGE = "Remove this docstring. The name, the parameters, and the types describe the code."
MIN_MESSAGE_WORDS = 3
GO_DECLARATION = re.compile(r"\s*(?:func|type|var|const|package)\b")


@dataclass
class SourceView:
    text: str
    code_text: str
    tokens: list
    docstrings: list
    line_index: list


def is_directive(token: Token, directives: list) -> bool:
    if token.text.startswith("#!"):
        return token.start == 0
    return any(directive.match(to_comment_body(token)) for directive in directives)


def is_go_doc(view: SourceView, token: Token) -> bool:
    return GO_DECLARATION.match(view.code_text, token.end) is not None


def is_doc(view: SourceView, token: Token, language: str) -> bool:
    if language == "go":
        return is_go_doc(view, token)
    return is_doc_comment(token, language)


def find_comment_findings(view: SourceView, context: CodeContext) -> list:
    directives = [re.compile(pattern) for pattern in context.standard.dictionary["directives"]]
    findings = []
    for token in view.tokens:
        if token.kind != "comment" or is_directive(token, directives):
            continue
        is_doc_token = is_doc(view, token, context.language)
        rule, message = ("C6.2", DOC_MESSAGE) if is_doc_token else ("C6.1", COMMENT_MESSAGE)
        findings.append(Finding(context.path, find_line(view.line_index, token.start), rule, message))
    for token in view.docstrings:
        line = find_line(view.line_index, token.start)
        findings.append(Finding(context.path, line, "C6.2", DOCSTRING_MESSAGE))
    return findings


def find_file_length(view: SourceView, context: CodeContext) -> list:
    limit = context.standard.limits["code"]["file_lines"]
    count = count_code_lines(view.text)
    if count <= limit:
        return []
    message = f"This file has {count} lines of code. The limit is {limit}. Split it by topic."
    return [Finding(context.path, 1, "C5.7", message)]


def find_function_lengths(view: SourceView, names: list, context: CodeContext) -> list:
    limit = context.standard.limits["code"]["function_lines"]
    findings = []
    for name in names:
        if name.kind != "function":
            continue
        length = measure_function(view.code_text, name, context.language)
        if length > limit:
            message = f'"{name.name}" has {length} lines. The limit is {limit}. Split it into named steps.'
            findings.append(Finding(context.path, find_line(view.line_index, name.offset), "C5.1", message))
    return findings


def strip_quotes(text: str) -> str:
    return text.lstrip("rRbBuUfF").strip("#").strip("\"'`")


def is_vague_message(message: str, vague: set) -> bool:
    words = re.findall(r"[A-Za-z]+", message)
    return len(words) < MIN_MESSAGE_WORDS or message.strip().rstrip(".!").lower() in vague


def find_error_messages(view: SourceView, context: CodeContext) -> list:
    vague = set(context.standard.dictionary["vague_messages"])
    findings = []
    for token in view.tokens:
        before = view.code_text[max(0, token.start - 80) : token.start]
        if token.kind != "string" or ERROR_CALL.search(before) is None:
            continue
        message = strip_quotes(token.text)
        if is_vague_message(message, vague):
            line = find_line(view.line_index, token.start)
            text = f'The error message "{message}" is vague. Say what failed, why, and what to do next.'
            findings.append(Finding(context.path, line, "C7.3", text))
    return findings


def find_suppressions(view: SourceView) -> dict:
    suppressions = {}
    for token in view.tokens:
        rule_ids = parse_ignore_ids(token.text) if token.kind == "comment" else set()
        if not rule_ids:
            continue
        line = find_line(view.line_index, token.start)
        line_start = view.text.rfind("\n", 0, token.start) + 1
        is_alone = not view.text[line_start : token.start].strip()
        suppressions.setdefault(line + 1 if is_alone else line, set()).update(rule_ids)
    return suppressions


def is_view_ignored(view: SourceView) -> bool:
    return any(token.kind == "comment" and is_file_ignored(token.text) for token in view.tokens)
