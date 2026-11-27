from __future__ import annotations

import re
from dataclasses import dataclass
from typing import Callable

ALL_WORDS = {"all", "a", "*"}
YES_WORDS = {"y", "yes"}


class SelectionError(ValueError):
    pass


@dataclass
class Console:
    read: Callable[[str], str]
    write: Callable[[str], None]


def parse_selection(text: str, count: int, *, default: list) -> list:
    words = [word for word in re.split(r"[\s,]+", text.strip().lower()) if word]
    if not words:
        return list(default)
    if set(words) & ALL_WORDS:
        return list(range(count))
    indexes = []
    for word in words:
        if not word.isdigit() or not 1 <= int(word) <= count:
            raise SelectionError(f'"{word}" is not a number from 1 to {count}. Type numbers such as "1 3", or "all".')
        if int(word) - 1 not in indexes:
            indexes.append(int(word) - 1)
    return indexes


def read_answer(console: Console, prompt: str) -> str:
    try:
        return console.read(prompt)
    except EOFError as error:
        message = "The installer got no answer. Use --skills and --target or --dir to install with no questions."
        raise SelectionError(message) from error


def ask_choice(console: Console, question: str, options: list, *, default: list) -> list:
    console.write(question)
    for number, option in enumerate(options, start=1):
        console.write(f"  {number}) {option}")
    default_text = ", ".join(str(index + 1) for index in default)
    while True:
        answer = read_answer(console, f"Type numbers, or press Enter for {default_text}: ")
        try:
            return parse_selection(answer, len(options), default=default)
        except SelectionError as error:
            console.write(str(error))


def ask_confirmation(console: Console, question: str) -> bool:
    return read_answer(console, f"{question} [y/N]: ").strip().lower() in YES_WORDS
