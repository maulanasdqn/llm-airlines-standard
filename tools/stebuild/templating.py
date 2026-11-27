from __future__ import annotations

import json
import re

PLACEHOLDER = re.compile(r"\{\{\s*([\w.]+)\s*\}\}")


class TemplateError(KeyError):
    pass


def find_value(context: dict, dotted: str) -> object:
    value = context
    for part in dotted.split("."):
        if not isinstance(value, dict) or part not in value:
            raise TemplateError(f'The template uses "{{{{ {dotted} }}}}", but the build has no value for it.')
        value = value[part]
    return value


def format_value(value: object) -> str:
    if isinstance(value, (list, dict)):
        return json.dumps(value, ensure_ascii=False)
    return str(value)


def render_template(text: str, context: dict) -> str:
    return PLACEHOLDER.sub(lambda found: format_value(find_value(context, found.group(1))), text)


def escape_yaml(text: str) -> str:
    return "'" + text.replace("'", "''") + "'"


def build_be_exceptions(words: list, forms: list) -> list:
    return [f"{form} {word}" for form in forms for word in words]


def build_vale_lists(dictionary: dict) -> dict:
    prose = dictionary["prose"]
    be_forms = ["am", "is", "are", "was", "were", "be", "been", "being"]
    swap = [f"  {escape_yaml(re.escape(entry['text']))}: {escape_yaml(entry['use'])}" for entry in prose["avoid"]]
    participles = "|".join(prose["participles"])
    return {
        "swap": "\n".join(swap),
        "hedges": "|".join(prose["hedges"]),
        "passive": f"(?:{'|'.join(be_forms)})\\s+(?:\\w+ly\\s+)?(?:\\w+ed|{participles})",
        "perfect": f"(?:has|have|had)\\s+(?:\\w+ly\\s+)?(?:\\w+ed|{participles})",
        "progressive": f"(?:{'|'.join(be_forms[:7])})\\s+\\w+ing",
        "passive_exceptions": build_be_exceptions(prose["stative"], be_forms),
        "progressive_exceptions": build_be_exceptions(prose["ing_exceptions"], be_forms[:7]),
    }


def build_denied_names(dictionary: dict) -> dict:
    nouns = [entry["word"] for entry in dictionary["avoid_nouns"]]
    abbreviations = [entry["word"] for entry in dictionary["abbreviations"]["avoid"]]
    return {
        language: sorted(set(nouns + abbreviations) - set(profile["allowed_abbreviations"]))
        for language, profile in dictionary["profiles"].items()
    }


def build_context(limits: dict, dictionary: dict) -> dict:
    return {
        "limits": limits,
        "lists": {
            "denied_names": build_denied_names(dictionary),
            "vale": build_vale_lists(dictionary),
        },
    }
