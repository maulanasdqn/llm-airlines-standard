from __future__ import annotations

import subprocess
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SECTIONS = set(range(1, 10))

sys.path.insert(0, str(ROOT / "tools"))

from stebuild.rules import build_limits, load_rules  # noqa: E402
from stebuild.skills import validate_skills  # noqa: E402


def load_all_rules() -> list:
    return load_rules(ROOT / "spec/writing/rules.md") + load_rules(ROOT / "spec/code/rules.md")


class BuildTest(unittest.TestCase):
    def test_generated_files_are_current(self) -> None:
        command = [sys.executable, str(ROOT / "tools/build.py"), "--check"]
        completed = subprocess.run(command, capture_output=True, text=True, check=False)
        self.assertEqual(completed.returncode, 0, completed.stderr)

    def test_both_rule_sets_cover_nine_sections(self) -> None:
        rules = load_all_rules()
        for prefix in ("W", "C"):
            sections = {rule["section"] for rule in rules if rule["id"].startswith(prefix)}
            self.assertEqual(sections, SECTIONS)

    def test_limits_from_the_rules(self) -> None:
        limits = build_limits(load_all_rules())
        self.assertEqual(limits["code"]["file_lines"], 200)
        self.assertEqual(limits["code"]["function_lines"], 40)
        self.assertEqual(limits["prose"]["instruction_words"], 20)

    def test_skills_are_valid(self) -> None:
        self.assertEqual(validate_skills(ROOT), [])

    def test_generated_files_have_no_placeholders(self) -> None:
        folders = [ROOT / "templates/agents", ROOT / "linters"]
        for folder in folders:
            for path in folder.rglob("*"):
                is_instruction = path.parent == folder and path.name in {"AGENTS.md", "CLAUDE.md"}
                if path.is_file() and not is_instruction and "src" not in path.relative_to(ROOT).parts:
                    self.assertNotIn("{{", path.read_text(encoding="utf-8"), str(path))

    def test_linters_take_limits_from_the_rules(self) -> None:
        ruff = (ROOT / "linters/python/ruff.toml").read_text(encoding="utf-8")
        eslint = (ROOT / "linters/typescript/eslint.config.mjs").read_text(encoding="utf-8")
        self.assertIn("max-positional-args = 3", ruff)
        self.assertIn('"max-lines": ["error", { max: 200', eslint)


if __name__ == "__main__":
    unittest.main()
