from __future__ import annotations

import argparse
import sys
from pathlib import Path

from .code import run_code, run_lexicon
from .commit import run_commit
from .standard import load_standard
from .output import compute_exit_code, format_summary, write_report
from .prose import run_prose

DESCRIPTIONS = {
    "prose": "Find breaks of the W rules in Markdown and text files.",
    "code": "Find breaks of the C rules in TypeScript, JavaScript, Python, Go, and Rust files.",
    "commit": "Find breaks of rule C9.4 and the W rules in a commit message.",
    "lexicon": "Write a draft LEXICON.md from the names in the code.",
}


def add_common_options(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--config", help="Path to a .ste.json file. The default is the nearest one.")
    parser.add_argument("--format", choices=("text", "json"), default="text", help="Output format.")
    parser.add_argument("--strict", action="store_true", help="Fail on warnings as well as errors.")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="ste_check", description="Check text and code against STE and STE-C.")
    commands = parser.add_subparsers(dest="command", required=True)
    for command, description in DESCRIPTIONS.items():
        subparser = commands.add_parser(command, help=description, description=description)
        add_common_options(subparser)
        if command == "commit":
            subparser.add_argument("source", help='A commit message file, or "-" for standard input.')
        else:
            subparser.add_argument("paths", nargs="*", default=["."], help="Files or folders.")
    return parser


def run_findings(args: argparse.Namespace) -> list:
    standard = load_standard(Path(args.config) if args.config else None)
    if args.command == "prose":
        return run_prose(args.paths, standard)
    if args.command == "code":
        return run_code(args.paths, standard)
    return run_commit(args.source, standard)


def run(argv: list) -> int:
    args = build_parser().parse_args(argv)
    if args.command == "lexicon":
        standard = load_standard(Path(args.config) if args.config else None)
        sys.stdout.write(run_lexicon(args.paths, standard) + "\n")
        return 0
    findings = run_findings(args)
    write_report(findings, sys.stdout, output_format=args.format)
    sys.stderr.write(format_summary(findings) + "\n")
    return compute_exit_code(findings, is_strict=args.strict)
