from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import Optional

from .standard import Standard
from .findings import Finding, filter_findings
from .prose import build_context, find_text_findings
from .prose_rules import build_patterns

CONVENTIONAL_PREFIX = re.compile(r"^[a-z]+(?:\([^)]*\))?!?:\s+")
SCISSORS = "# ------------------------ >8 ------------------------"
TRAILER = re.compile(r"^[A-Za-z][\w-]*: \S")
VAGUE_WORDS = {"stuff", "things", "misc", "various", "wip", "tweaks", "changes", "fixes", "updates", "minor", "cleanup"}
WEAK_WORDS = {"update", "fix", "bug", "bugs", "some", "small", "the", "a"}
PAST_FORMS = {
    "made", "wrote", "ran", "built", "sent", "took", "gave", "found", "kept", "left", "brought", "told",
    "chose", "broke", "froze", "hid", "got", "began", "drew", "threw", "lost", "paid", "said", "spent",
    "led", "met", "bound", "stuck", "did", "went", "undid", "rewrote",
}
COMMIT_VERBS = {
    "fix", "bump", "refactor", "revert", "drop", "document", "improve", "simplify", "support", "upgrade",
    "downgrade", "prepare", "release", "tidy", "clean", "speed", "tune", "pin", "unpin", "port", "extract",
    "inline", "bring", "handle", "process", "change", "rename", "add", "update", "remove", "delete",
}


def read_message(source: str) -> str:
    if source == "-":
        return sys.stdin.read()
    return Path(source).read_text(encoding="utf-8")


def strip_git_comments(text: str) -> list:
    lines = []
    for line in text.splitlines():
        if line.startswith(SCISSORS):
            break
        if not line.startswith("#"):
            lines.append(line.rstrip())
    while lines and not lines[0].strip():
        lines.pop(0)
    return lines


def find_inflection(word: str, verbs: set) -> Optional[str]:
    if word in verbs:
        return None
    if word in PAST_FORMS or (word.endswith("ed") and {word[:-1], word[:-2], word[:-3], word[:-3] + "y"} & verbs):
        return "past"
    if word.endswith("ing") and {word[:-3], word[:-3] + "e", word[:-4]} & verbs:
        return "-ing"
    if word.endswith("s") and {word[:-1], word[:-2], word[:-3] + "y"} & verbs:
        return "third-person"
    return None


def find_subject_wording(subject: str, verbs: set) -> list:
    words = re.findall(r"[a-z']+", subject.lower())
    if not words:
        return []
    findings = []
    form = find_inflection(words[0], verbs)
    if form is not None:
        message = f'"{words[0]}" is a {form} form. Write the subject in the imperative, such as "Fix" or "Add".'
        findings.append(("error", message))
    is_vague = bool(VAGUE_WORDS & set(words)) or set(words) <= WEAK_WORDS | VAGUE_WORDS
    if is_vague:
        findings.append(("warning", "The subject is vague. Say what changed and where."))
    return findings


def find_subject_findings(subject: str, context: dict) -> list:
    limits = context["limits"]
    results = []
    if len(subject) > limits["subject_hard_chars"]:
        results.append(("error", f"The subject has {len(subject)} characters. The limit is {limits['subject_hard_chars']}."))
    elif len(subject) > limits["subject_chars"]:
        results.append(("warning", f"The subject has {len(subject)} characters. Keep it to {limits['subject_chars']} or fewer."))
    if subject.endswith("."):
        results.append(("error", "Do not end the subject with a period."))
    results += find_subject_wording(CONVENTIONAL_PREFIX.sub("", subject), context["verbs"])
    return [Finding(context["path"], 1, "C9.4", message, severity=level) for level, message in results]


def find_body_layout(lines: list, context: dict) -> list:
    findings = []
    if len(lines) > 1 and lines[1].strip():
        findings.append(Finding(context["path"], 2, "C9.4", "Put a blank line after the subject.", severity="error"))
    limit = context["limits"]["body_line_chars"]
    for number, line in enumerate(lines[2:], start=3):
        if len(line) > limit and "://" not in line:
            message = f"This body line has {len(line)} characters. Wrap the body at {limit}."
            findings.append(Finding(context["path"], number, "C9.4", message, severity="warning"))
    return findings


def find_body_prose(lines: list, context: dict, standard: Standard) -> list:
    body = ["", ""] + ["" if TRAILER.match(line) else line for line in lines[2:]]
    prose_context = build_context(context["path"], standard, build_patterns(standard.dictionary["prose"]))
    findings, suppressions = find_text_findings("\n".join(body), prose_context)
    return filter_findings(findings, standard, suppressions)


def build_verbs(standard: Standard) -> set:
    dictionary = standard.dictionary
    verbs = {entry["word"] for entry in dictionary["verbs"]}
    return verbs | set(dictionary["prose"]["imperatives"]) | COMMIT_VERBS


def run_commit(source: str, standard: Standard) -> list:
    label = "<stdin>" if source == "-" else source
    lines = strip_git_comments(read_message(source))
    if not lines:
        return [Finding(label, 1, "C9.4", "The commit message is empty. Write a subject line.", severity="error")]
    context = {"path": label, "limits": standard.limits["commit"], "verbs": build_verbs(standard)}
    findings = find_subject_findings(lines[0], context) + find_body_layout(lines, context)
    findings = filter_findings(findings, standard, {})
    return findings + find_body_prose(lines, context, standard)
