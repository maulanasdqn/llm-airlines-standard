from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INSTALLER = ROOT / "tools/install.py"
ALL_SKILLS = {"ste-adopt", "ste-code", "ste-review", "ste-writing"}
USAGE_ERROR = 2

sys.path.insert(0, str(ROOT / "tools"))

from steinstall.selection import SelectionError, parse_selection  # noqa: E402


def run_installer(arguments: list, *, stdin: str = "") -> subprocess.CompletedProcess:
    command = [sys.executable, str(INSTALLER), *arguments]
    return subprocess.run(command, capture_output=True, text=True, input=stdin, check=False)


def list_installed(folder: Path) -> set:
    return {path.name for path in folder.iterdir() if (path / "SKILL.md").is_file()}


class SelectionTest(unittest.TestCase):
    def test_parse_numbers(self) -> None:
        self.assertEqual(parse_selection("1 3", 4, default=[0]), [0, 2])
        self.assertEqual(parse_selection("3,1,3", 4, default=[0]), [2, 0])

    def test_parse_defaults_and_all(self) -> None:
        self.assertEqual(parse_selection("", 4, default=[1]), [1])
        self.assertEqual(parse_selection("all", 3, default=[0]), [0, 1, 2])

    def test_parse_rejects_bad_numbers(self) -> None:
        for answer in ("0", "5", "two"):
            with self.subTest(answer=answer), self.assertRaises(SelectionError):
                parse_selection(answer, 4, default=[0])


class InstallerTest(unittest.TestCase):
    def test_list_names_every_skill(self) -> None:
        output = run_installer(["--list"]).stdout
        for name in ALL_SKILLS:
            self.assertIn(name, output)

    def test_install_only_the_selected_skills(self) -> None:
        with tempfile.TemporaryDirectory() as folder:
            completed = run_installer(["--skills", "ste-writing,ste-review", "--dir", folder])
            self.assertEqual(completed.returncode, 0, completed.stderr)
            self.assertEqual(list_installed(Path(folder)), {"ste-writing", "ste-review"})
            self.assertFalse(list(Path(folder).rglob("__pycache__")))

    def test_interactive_answers(self) -> None:
        with tempfile.TemporaryDirectory() as folder:
            completed = run_installer([], stdin=f"2 4\n5\n{folder}\n")
            self.assertEqual(completed.returncode, 0, completed.stderr)
            self.assertEqual(list_installed(Path(folder)), {"ste-code", "ste-writing"})

    def test_second_install_changes_nothing(self) -> None:
        with tempfile.TemporaryDirectory() as folder:
            run_installer(["--skills", "all", "--dir", folder])
            output = run_installer(["--skills", "all", "--dir", folder]).stdout
        self.assertEqual(output.count("is up to date"), len(ALL_SKILLS))
        self.assertNotIn("Start a new agent session", output)

    def test_local_change_needs_force(self) -> None:
        with tempfile.TemporaryDirectory() as folder:
            skill_file = Path(folder) / "ste-code/SKILL.md"
            run_installer(["--skills", "ste-code", "--dir", folder])
            skill_file.write_text("local edit\n", encoding="utf-8")
            run_installer(["--skills", "ste-code", "--dir", folder])
            self.assertEqual(skill_file.read_text(encoding="utf-8"), "local edit\n")
            run_installer(["--skills", "ste-code", "--dir", folder, "--force"])
            self.assertNotEqual(skill_file.read_text(encoding="utf-8"), "local edit\n")

    def test_dry_run_writes_nothing(self) -> None:
        with tempfile.TemporaryDirectory() as folder:
            output = run_installer(["--skills", "all", "--dir", folder, "--dry-run"]).stdout
            self.assertEqual(list(Path(folder).iterdir()), [])
        self.assertEqual(output.count("Dry run: install"), len(ALL_SKILLS))

    def test_folder_without_skill_file_stays(self) -> None:
        with tempfile.TemporaryDirectory() as folder:
            other = Path(folder) / "ste-review"
            other.mkdir()
            (other / "notes.txt").write_text("keep\n", encoding="utf-8")
            output = run_installer(["--skills", "ste-review", "--dir", folder, "--force"]).stdout
            self.assertTrue((other / "notes.txt").is_file())
        self.assertIn("has no SKILL.md file", output)

    def test_unknown_skill_fails(self) -> None:
        completed = run_installer(["--skills", "ste-nothing", "--dir", "."])
        self.assertEqual(completed.returncode, USAGE_ERROR)
        self.assertIn("Unknown skill: ste-nothing", completed.stderr)

    def test_no_answer_fails(self) -> None:
        completed = run_installer([], stdin="")
        self.assertEqual(completed.returncode, USAGE_ERROR)
        self.assertIn("got no answer", completed.stderr)

    def test_install_script_uses_the_local_copy(self) -> None:
        completed = subprocess.run(["bash", str(ROOT / "install.sh"), "--list"], capture_output=True, text=True, check=False)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertIn("ste-adopt", completed.stdout)


if __name__ == "__main__":
    unittest.main()
