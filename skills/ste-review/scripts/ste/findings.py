from __future__ import annotations

import re
from dataclasses import dataclass, replace

from .standard import Standard

ALL_RULES = "*"
IGNORE_PATTERN = re.compile(r"ste-ignore(?!-file)\b[:\s]*([A-Z]\d+\.\d+(?:[\s,]+[A-Z]\d+\.\d+)*)?")
FILE_IGNORE_PATTERN = re.compile(r"ste-ignore-file\b")
SEVERITY_ORDER = {"error": 0, "warning": 1, "info": 2}


@dataclass(frozen=True)
class Finding:
    path: str
    line: int
    rule: str
    message: str
    severity: str = ""
    column: int = 1


def parse_ignore_ids(text: str) -> set:
    found = IGNORE_PATTERN.search(text)
    if found is None:
        return set()
    if found.group(1) is None:
        return {ALL_RULES}
    return set(re.split(r"[\s,]+", found.group(1).strip()))


def is_file_ignored(text: str) -> bool:
    return FILE_IGNORE_PATTERN.search(text) is not None


def is_suppressed(finding: Finding, suppressions: dict) -> bool:
    rule_ids = suppressions.get(finding.line, set())
    return ALL_RULES in rule_ids or finding.rule in rule_ids


def resolve_severity(finding: Finding, standard: Standard) -> Finding:
    if finding.severity:
        return finding
    rule = standard.rules.get(finding.rule, {})
    return replace(finding, severity=rule.get("severity", "warning"))


def filter_findings(findings: list, standard: Standard, suppressions: dict) -> list:
    kept = []
    for finding in findings:
        if finding.rule in standard.disabled or is_suppressed(finding, suppressions):
            continue
        kept.append(resolve_severity(finding, standard))
    return kept


def sort_findings(findings: list) -> list:
    return sorted(
        findings,
        key=lambda finding: (
            finding.path,
            finding.line,
            SEVERITY_ORDER.get(finding.severity, 3),
            finding.rule,
        ),
    )
