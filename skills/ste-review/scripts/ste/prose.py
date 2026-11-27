from __future__ import annotations

from pathlib import Path

from .standard import Standard
from .files import list_files, to_display_path
from .findings import filter_findings
from .markdown import parse_markdown, strip_inline
from .prose_rules import (
    ProseContext,
    ProsePatterns,
    build_patterns,
    find_block_findings,
    find_sentence_findings,
)
from .sentences import split_sentences

PROSE_EXTENSIONS = {".md", ".markdown", ".mdx", ".txt"}


def find_text_findings(text: str, context: ProseContext) -> tuple:
    state = parse_markdown(text)
    if state.is_ignored:
        return [], {}
    prose = context.patterns.prose
    findings = []
    for block in state.blocks:
        cleaned = "\n".join(strip_inline(line) for _, line in block.lines)
        numbers = [number for number, _ in block.lines]
        sentences = split_sentences(cleaned, numbers, prose)
        findings.extend(find_block_findings(block, sentences, context))
        for sentence in sentences:
            findings.extend(find_sentence_findings(sentence, context))
    return findings, state.suppressions


def build_context(label: str, standard: Standard, patterns: ProsePatterns) -> ProseContext:
    return ProseContext(path=label, limits=standard.limits, patterns=patterns)


def find_file_findings(path: Path, standard: Standard, patterns: ProsePatterns) -> list:
    context = build_context(to_display_path(path), standard, patterns)
    findings, suppressions = find_text_findings(path.read_text(encoding="utf-8"), context)
    return filter_findings(findings, standard, suppressions)


def run_prose(paths: list, standard: Standard) -> list:
    patterns = build_patterns(standard.dictionary["prose"])
    files = list_files(paths, PROSE_EXTENSIONS, root=standard.root, excluded=standard.excluded)
    findings = []
    for path in files:
        findings.extend(find_file_findings(path, standard, patterns))
    return findings
