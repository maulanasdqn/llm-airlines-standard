from __future__ import annotations

import json
from collections import Counter
from dataclasses import asdict
from typing import TextIO

from .findings import sort_findings


def format_text(findings: list) -> str:
    lines = [
        f"{finding.path}:{finding.line}:{finding.column}: {finding.severity} {finding.rule}: {finding.message}"
        for finding in sort_findings(findings)
    ]
    return "\n".join(lines)


def format_json(findings: list) -> str:
    return json.dumps([asdict(finding) for finding in sort_findings(findings)], indent=2)


def format_summary(findings: list) -> str:
    counts = Counter(finding.severity for finding in findings)
    parts = [f"{counts.get(level, 0)} {level}s" for level in ("error", "warning", "info")]
    return "ste_check: " + ", ".join(parts)


def compute_exit_code(findings: list, *, is_strict: bool) -> int:
    failing = {"error", "warning"} if is_strict else {"error"}
    has_failure = any(finding.severity in failing for finding in findings)
    return 1 if has_failure else 0


def write_report(findings: list, stream: TextIO, *, output_format: str) -> None:
    report = format_json(findings) if output_format == "json" else format_text(findings)
    if report:
        stream.write(report + "\n")
