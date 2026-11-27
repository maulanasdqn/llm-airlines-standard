from __future__ import annotations

import re
from dataclasses import dataclass, field
from typing import Optional

from .findings import is_file_ignored, parse_ignore_ids

FENCE = re.compile(r"^\s*(```|~~~)")
LIST_ITEM = re.compile(r"^(\s*)([-*+]|\d+[.)])\s+(.*)$")
ORDERED_ITEM = re.compile(r"^\s*\d+[.)]\s")
ALERT = re.compile(r"^>\s*\[!(\w+)\]\s*$")
HEADING = re.compile(r"^\s{0,3}#{1,6}\s")
COMMENT_START = "<!--"
COMMENT_END = "-->"
INLINE_COMMENT = re.compile(r"<!--(.*?)-->")
INLINE_RULES = [
    (re.compile(r"`[^`]*`"), "CODE"),
    (re.compile(r"\"[^\"]*\"|“[^”]*”"), "QUOTE"),
    (re.compile(r"!\[[^\]]*\]\([^)]*\)"), ""),
    (re.compile(r"\[([^\]]*)\]\([^)]*\)"), r"\1"),
    (re.compile(r"<https?://[^>]+>|https?://\S+"), "LINK"),
    (re.compile(r"<[^>]+>"), ""),
    (re.compile(r"\*\*|__|(?<!\w)\*(?=\S)|(?<=\S)\*(?!\w)"), ""),
    (re.compile(r"[ \t]+"), " "),
]


@dataclass
class Block:
    kind: str
    lines: list = field(default_factory=list)
    is_step: bool = False
    marker: str = ""
    ignored: set = field(default_factory=set)


@dataclass
class MarkdownState:
    blocks: list = field(default_factory=list)
    suppressions: dict = field(default_factory=dict)
    current: Optional[Block] = None
    fence: str = ""
    is_in_comment: bool = False
    pending_ignore: set = field(default_factory=set)
    is_ignored: bool = False


def strip_inline(text: str) -> str:
    for pattern, replacement in INLINE_RULES:
        text = pattern.sub(replacement, text)
    return text.strip()


def find_content_start(lines: list) -> int:
    if not lines or lines[0].strip() != "---":
        return 0
    for index, line in enumerate(lines[1:], start=1):
        if line.strip() == "---":
            return index + 1
    return 0


def add_suppression(state: MarkdownState, line: int, rule_ids: set) -> None:
    if rule_ids:
        state.suppressions.setdefault(line, set()).update(rule_ids)


def start_block(state: MarkdownState, block: Block) -> None:
    close_block(state)
    block.ignored = state.pending_ignore
    state.pending_ignore = set()
    state.current = block


def close_block(state: MarkdownState) -> None:
    block = state.current
    state.current = None
    if block is None or not block.lines:
        return
    state.blocks.append(block)
    for number, _ in block.lines:
        add_suppression(state, number, block.ignored)


def read_comment_line(state: MarkdownState, line: str) -> None:
    state.pending_ignore |= parse_ignore_ids(line)
    state.is_ignored = state.is_ignored or is_file_ignored(line)
    state.is_in_comment = COMMENT_END not in line


def read_quote_line(state: MarkdownState, number: int, line: str) -> None:
    alert = ALERT.match(line.strip())
    if alert is not None:
        start_block(state, Block("alert", marker=alert.group(1).upper()))
        return
    if state.current is not None and state.current.kind == "alert":
        state.current.lines.append((number, line.lstrip()[1:].strip()))
        return
    close_block(state)


def read_text_line(state: MarkdownState, number: int, line: str) -> None:
    for comment in INLINE_COMMENT.findall(line):
        add_suppression(state, number, parse_ignore_ids(comment))
    item = LIST_ITEM.match(line)
    if item is not None:
        start_block(state, Block("item", is_step=ORDERED_ITEM.match(line) is not None))
        state.current.lines.append((number, item.group(3)))
        return
    if state.current is None or state.current.kind == "alert":
        start_block(state, Block("paragraph"))
    state.current.lines.append((number, line.strip()))


def is_structure_line(line: str) -> bool:
    stripped = line.lstrip()
    return HEADING.match(line) is not None or stripped.startswith(("|", "<"))


def read_open_line(state: MarkdownState, number: int, line: str) -> None:
    fence = FENCE.match(line)
    if fence is not None:
        close_block(state)
        state.fence = fence.group(1)
    elif line.lstrip().startswith(COMMENT_START):
        close_block(state)
        read_comment_line(state, line)
    elif line.lstrip().startswith(">"):
        read_quote_line(state, number, line)
    elif not line.strip() or is_structure_line(line):
        close_block(state)
    else:
        read_text_line(state, number, line)


def read_line(state: MarkdownState, number: int, line: str) -> None:
    if state.fence:
        if line.strip().startswith(state.fence):
            state.fence = ""
        return
    if state.is_in_comment:
        read_comment_line(state, line)
        return
    read_open_line(state, number, line)


def parse_markdown(text: str) -> MarkdownState:
    state = MarkdownState()
    lines = text.splitlines()
    first = find_content_start(lines)
    for number, line in enumerate(lines[first:], start=first + 1):
        read_line(state, number, line)
    close_block(state)
    return state
