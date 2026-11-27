from __future__ import annotations

from pathlib import Path

from .code_rules import (
    SourceView,
    find_comment_findings,
    find_error_messages,
    find_file_length,
    find_function_lengths,
    find_suppressions,
    is_view_ignored,
)
from .standard import Standard, Vocabulary, build_vocabulary
from .files import list_files, to_display_path
from .findings import filter_findings
from .lexicon import NameUse, find_rotations, render_lexicon
from .name_rules import CodeContext, find_name_findings, is_exempt
from .names import find_names, split_words
from .source import build_code_text, build_line_index, find_docstrings, find_line, list_tokens


def build_extension_map(dictionary: dict) -> dict:
    return {
        extension: language
        for language, profile in dictionary["profiles"].items()
        for extension in profile["extensions"]
    }


def read_view(path: Path, language: str) -> SourceView:
    text = path.read_text(encoding="utf-8")
    tokens = list_tokens(text, language)
    code_text = build_code_text(text, tokens)
    docstrings = find_docstrings(text, tokens, code_text) if language == "python" else []
    return SourceView(text, code_text, tokens, docstrings, build_line_index(text))


def build_context(path: Path, standard: Standard, vocabulary: Vocabulary) -> CodeContext:
    language = build_extension_map(standard.dictionary)[path.suffix.lower()]
    return CodeContext(
        path=to_display_path(path),
        language=language,
        extension=path.suffix.lower(),
        profile=standard.dictionary["profiles"][language],
        standard=standard,
        vocabulary=vocabulary,
    )


def list_name_uses(view: SourceView, names: list, context: CodeContext) -> list:
    return [
        NameUse(
            name=name.name,
            words=tuple(word.lower() for word in split_words(name.name)),
            path=context.path,
            line=find_line(view.line_index, name.offset),
            language=context.language,
        )
        for name in names
        if name.kind == "function" and not name.is_noun_allowed and not is_exempt(name, context)
    ]


def find_file_findings(path: Path, standard: Standard, vocabulary: Vocabulary) -> tuple:
    context = build_context(path, standard, vocabulary)
    view = read_view(path, context.language)
    if is_view_ignored(view):
        return [], []
    names = find_names(view.code_text, context.language)
    findings = find_comment_findings(view, context) + find_file_length(view, context)
    findings += find_function_lengths(view, names, context) + find_error_messages(view, context)
    for name in names:
        findings += find_name_findings(name, find_line(view.line_index, name.offset), context)
    kept = filter_findings(findings, standard, find_suppressions(view))
    return kept, list_name_uses(view, names, context)


def list_code_files(paths: list, standard: Standard) -> list:
    extensions = set(build_extension_map(standard.dictionary))
    return list_files(paths, extensions, root=standard.root, excluded=standard.excluded)


def run_code(paths: list, standard: Standard) -> list:
    vocabulary = build_vocabulary(standard)
    findings = []
    uses = []
    for path in list_code_files(paths, standard):
        file_findings, file_uses = find_file_findings(path, standard, vocabulary)
        findings += file_findings
        uses += file_uses
    rotations = find_rotations(uses, vocabulary, standard.dictionary["synonym_groups"])
    return findings + filter_findings(rotations, standard, {})


def run_lexicon(paths: list, standard: Standard) -> str:
    vocabulary = build_vocabulary(standard)
    uses = []
    for path in list_code_files(paths, standard):
        uses += find_file_findings(path, standard, vocabulary)[1]
    return render_lexicon(uses, vocabulary)
