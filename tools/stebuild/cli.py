from __future__ import annotations

import argparse
import sys
from pathlib import Path
from typing import TextIO

from .outputs import find_drift, write_outputs
from .plan import build_outputs
from .rules import RuleFormatError
from .skills import validate_skills
from .templating import TemplateError

ROOT = Path(__file__).resolve().parents[2]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="build", description="Generate the skills, templates, and linter configs from spec/.")
    parser.add_argument("--check", action="store_true", help="Report generated files that are out of date. Change no file.")
    return parser


def write_lines(lines: list, stream: TextIO) -> None:
    for line in lines:
        stream.write(line + "\n")


def run(argv: list) -> int:
    args = build_parser().parse_args(argv)
    try:
        outputs = build_outputs(ROOT)
    except (RuleFormatError, TemplateError, FileNotFoundError) as error:
        sys.stderr.write(f"build: {error}\n")
        return 1
    problems = validate_skills(ROOT)
    write_lines(problems, sys.stderr)
    if args.check:
        drift = find_drift(ROOT, outputs)
        write_lines(drift, sys.stderr)
        if drift:
            sys.stderr.write("build: generated files are out of date. Run python3 tools/build.py.\n")
        return 1 if drift or problems else 0
    write_lines(write_outputs(ROOT, outputs), sys.stdout)
    return 1 if problems else 0
