from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / "skills/ste-review/scripts"
CHECKER = SCRIPTS / "ste_check.py"
FIXTURES = ROOT / "tests/fixtures"
FIXTURE_CONFIG = FIXTURES / "ste.json"
CODE_RULES = {"C1.3", "C1.6", "C1.7", "C2.1", "C3.1", "C5.1", "C6.1", "C6.2", "C7.3"}
BAD_CODE = {
    "code/typescript/bad.ts": CODE_RULES | {"C4.4"},
    "code/python/bad.py": CODE_RULES | {"C4.4"},
    "code/go/bad": CODE_RULES - {"C6.1"},
    "code/rust/bad": CODE_RULES,
}
GOOD_CODE = ("code/typescript/good.ts", "code/python/good.py", "code/go/good", "code/rust/good")
BAD_PROSE = {"W1.4", "W3.1", "W3.2", "W3.3", "W4.3", "W6.1", "W6.3", "W7.1", "W7.2", "W8.2"}

sys.path.insert(0, str(SCRIPTS))

from ste.names import split_words  # noqa: E402


def run_checker(arguments: list, *, config: Path = FIXTURE_CONFIG, stdin: str = "") -> tuple:
    command = [sys.executable, str(CHECKER), *arguments, "--format", "json", "--config", str(config)]
    completed = subprocess.run(command, capture_output=True, text=True, input=stdin, cwd=ROOT, check=False)
    return completed.returncode, json.loads(completed.stdout or "[]")


def list_rules(findings: list) -> set:
    return {finding["rule"] for finding in findings}


def write_project(folder: Path, files: dict) -> Path:
    for name, text in files.items():
        (folder / name).write_text(text, encoding="utf-8")
    return folder / ".ste.json"


class CodeFixtureTest(unittest.TestCase):
    def test_bad_fixtures_break_expected_rules(self) -> None:
        for fixture, expected in BAD_CODE.items():
            with self.subTest(fixture=fixture):
                code, findings = run_checker(["code", str(FIXTURES / fixture)])
                self.assertEqual(code, 1)
                self.assertLessEqual(expected, list_rules(findings))

    def test_good_fixtures_have_no_findings(self) -> None:
        for fixture in GOOD_CODE:
            with self.subTest(fixture=fixture):
                code, findings = run_checker(["code", str(FIXTURES / fixture)])
                self.assertEqual((code, findings), (0, []))


class ProseFixtureTest(unittest.TestCase):
    def test_bad_prose_breaks_expected_rules(self) -> None:
        code, findings = run_checker(["prose", str(FIXTURES / "prose/bad.md")])
        self.assertEqual(code, 1)
        self.assertLessEqual(BAD_PROSE, list_rules(findings))

    def test_good_prose_has_no_findings(self) -> None:
        self.assertEqual(run_checker(["prose", str(FIXTURES / "prose/good.md")]), (0, []))


class CommitTest(unittest.TestCase):
    def test_bad_commit_from_stdin(self) -> None:
        code, findings = run_checker(["commit", "-"], stdin="Fixed stuff.\n")
        self.assertEqual(code, 1)
        self.assertEqual(len(findings), 3)
        self.assertEqual(list_rules(findings), {"C9.4"})

    def test_good_commit_has_no_findings(self) -> None:
        self.assertEqual(run_checker(["commit", str(FIXTURES / "commit/good.txt")]), (0, []))

    def test_subject_without_blank_line(self) -> None:
        _, findings = run_checker(["commit", "-"], stdin="Add the retry limit\nThe body starts here.\n")
        self.assertIn(2, [finding["line"] for finding in findings])


class ProjectTest(unittest.TestCase):
    def test_file_length_limit(self) -> None:
        with tempfile.TemporaryDirectory() as folder:
            config = write_project(Path(folder), {".ste.json": "{}", "long.py": "total = 1\n" * 201})
            _, findings = run_checker(["code", folder], config=config)
        self.assertEqual(list_rules(findings), {"C5.7"})

    def test_code_suppression(self) -> None:
        source = "# ste-ignore: C1.6\ndef process_order() -> None:\n    return None\n"
        with tempfile.TemporaryDirectory() as folder:
            config = write_project(Path(folder), {".ste.json": "{}", "order.py": source})
            self.assertEqual(run_checker(["code", folder], config=config), (0, []))

    def test_markdown_suppression(self) -> None:
        text = "<!-- ste-ignore W1.4 -->\nLeverage the cache.\n"
        with tempfile.TemporaryDirectory() as folder:
            config = write_project(Path(folder), {".ste.json": "{}", "notes.md": text})
            self.assertEqual(run_checker(["prose", folder], config=config), (0, []))

    def test_lexicon_adds_technical_verbs(self) -> None:
        source = "def reconcile_ledger() -> None:\n    return None\n"
        lexicon = "## Technical verbs\n\n| Word | Meaning |\n|---|---|\n| `reconcile` | Match two records. |\n"
        with tempfile.TemporaryDirectory() as folder:
            config = write_project(Path(folder), {".ste.json": "{}", "ledger.py": source})
            _, without = run_checker(["code", folder], config=config)
            write_project(Path(folder), {"LEXICON.md": lexicon})
            _, with_lexicon = run_checker(["code", folder], config=config)
        self.assertEqual(list_rules(without), {"C3.1"})
        self.assertEqual(with_lexicon, [])

    def test_directives_are_allowed(self) -> None:
        source = "#!/usr/bin/env python3\nTOTAL = 1  # noqa: E501\nNAMES = []  # type: ignore[var-annotated]\n"
        with tempfile.TemporaryDirectory() as folder:
            config = write_project(Path(folder), {".ste.json": "{}", "tool.py": source})
            self.assertEqual(run_checker(["code", folder], config=config), (0, []))


class WordCountTest(unittest.TestCase):
    def test_split_words(self) -> None:
        self.assertEqual(split_words("parseHTTPResponse"), ["parse", "HTTP", "Response"])
        self.assertEqual(split_words("MAX_RETRY_COUNT"), ["MAX", "RETRY", "COUNT"])
        self.assertEqual(split_words("_load_config"), ["load", "config"])
        self.assertEqual(split_words("userID"), ["user", "ID"])


if __name__ == "__main__":
    unittest.main()
