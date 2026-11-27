from __future__ import annotations

import re
from pathlib import Path

SECTION = re.compile(r"^## ([WC])(\d) (.+)$")
RULE = re.compile(r"^### ([WC]\d\.\d+) (.+)$")
FIELD = re.compile(r"^- (Severity|Checks|Limit|Why|Do|Do not): (.+)$")
LIMIT = re.compile(r"^([a-z]+)\.([a-z_]+) = (\d+)$")
SEVERITIES = {"error", "warning", "info"}
TOOLS = {"review", "ste_check", "vale", "eslint", "ruff", "golangci", "revive", "clippy"}
REQUIRED_FIELDS = ("Severity", "Checks", "Why", "Do", "Do not")


class RuleFormatError(ValueError):
    pass


def start_rule(found: re.Match, section: dict) -> dict:
    return {
        "id": found.group(1),
        "title": found.group(2),
        "section": section["number"],
        "section_title": section["title"],
        "fields": {},
        "limits": {},
    }


def read_field(rule: dict, found: re.Match, path: Path) -> None:
    key, value = found.group(1), found.group(2)
    if key != "Limit":
        rule["fields"][key] = value
        return
    limit = LIMIT.match(value)
    if limit is None:
        raise RuleFormatError(f'{path}: rule {rule["id"]} has a bad limit "{value}". Write "group.key = number".')
    rule["limits"][(limit.group(1), limit.group(2))] = int(limit.group(3))


def parse_rule_lines(lines: list, path: Path) -> list:
    rules = []
    section = {"number": 0, "title": ""}
    for line in lines:
        section_found = SECTION.match(line)
        rule_found = RULE.match(line)
        field_found = FIELD.match(line)
        if section_found is not None:
            section = {"number": int(section_found.group(2)), "title": section_found.group(3)}
        elif rule_found is not None:
            rules.append(start_rule(rule_found, section))
        elif field_found is not None and rules:
            read_field(rules[-1], field_found, path)
    return rules


def validate_checks(rule: dict, path: Path) -> None:
    for check in rule["checks"]:
        tool = check.split(":", 1)[0]
        if tool not in TOOLS:
            raise RuleFormatError(f'{path}: rule {rule["id"]} names the unknown tool "{tool}".')


def validate_rule(rule: dict, path: Path) -> None:
    missing = [key for key in REQUIRED_FIELDS if key not in rule["fields"]]
    if missing:
        raise RuleFormatError(f'{path}: rule {rule["id"]} has no {", ".join(missing)} field.')
    if rule["fields"]["Severity"] not in SEVERITIES:
        raise RuleFormatError(f'{path}: rule {rule["id"]} has a bad severity. Use error, warning, or info.')
    if not rule["id"].startswith(f'{rule["id"][0]}{rule["section"]}.'):
        raise RuleFormatError(f'{path}: rule {rule["id"]} is not in section {rule["section"]}.')
    text = rule["title"] + " " + rule["fields"]["Why"]
    for (group, key), value in rule["limits"].items():
        if str(value) not in text:
            raise RuleFormatError(f"{path}: rule {rule['id']} sets {group}.{key} = {value}, but its text does not say {value}.")
    validate_checks(rule, path)


def normalize_rule(rule: dict, path: Path) -> dict:
    rule["severity"] = rule["fields"].get("Severity", "")
    checks = rule["fields"].get("Checks", "").split(",")
    rule["checks"] = [check.strip().strip("`") for check in checks if check.strip()]
    validate_rule(rule, path)
    return rule


def load_rules(path: Path) -> list:
    rules = [normalize_rule(rule, path) for rule in parse_rule_lines(path.read_text(encoding="utf-8").splitlines(), path)]
    identifiers = [rule["id"] for rule in rules]
    duplicates = sorted({identifier for identifier in identifiers if identifiers.count(identifier) > 1})
    if duplicates:
        raise RuleFormatError(f"{path}: these rule IDs occur more than once: {', '.join(duplicates)}.")
    return rules


def build_limits(rules: list) -> dict:
    limits = {}
    for rule in rules:
        for (group, key), value in rule["limits"].items():
            limits.setdefault(group, {})[key] = value
    return limits
