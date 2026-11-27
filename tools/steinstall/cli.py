from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass
from pathlib import Path

from .catalog import TARGETS, list_skills, resolve_folder
from .copying import copy_skill, find_status, replace_skill
from .selection import Console, SelectionError, ask_choice, ask_confirmation, read_answer

ROOT = Path(__file__).resolve().parents[2]
OTHER_FOLDER = "Another folder (you type the path)"
USAGE_ERROR = 2
DONE_ACTIONS = {"new": "Installed", "changed": "Replaced"}
DRY_ACTIONS = {"new": "install", "changed": "replace"}


@dataclass
class Options:
    is_forced: bool
    is_dry_run: bool
    is_interactive: bool
    console: Console


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="install", description="Install the STE skills for Claude Code, Codex, Cursor, and other agents.")
    parser.add_argument("--list", action="store_true", help="List the skills and the targets, then stop.")
    parser.add_argument("--skills", help='Skill names, separated by commas, or "all".')
    parser.add_argument("--target", action="append", choices=[target.key for target in TARGETS], help="A target folder. You can give it more than once.")
    parser.add_argument("--dir", action="append", help="Another skills folder. You can give it more than once.")
    parser.add_argument("--project", default=".", help="The project folder for the project targets. The default is the current folder.")
    parser.add_argument("--force", action="store_true", help="Replace a skill folder that has local changes.")
    parser.add_argument("--dry-run", action="store_true", help="Show the changes. Change no file.")
    return parser


def format_listing(skills: list) -> str:
    lines = ["Skills:"] + [f"  {skill.name}: {skill.summary}" for skill in skills]
    lines += ["", "Targets:"] + [f"  {target.key}: {target.label} ({target.folder})" for target in TARGETS]
    return "\n".join(lines)


def find_named_skills(text: str, skills: list) -> list:
    by_name = {skill.name: skill for skill in skills}
    names = [name.strip() for name in text.split(",") if name.strip()]
    if names == ["all"]:
        return list(skills)
    unknown = [name for name in names if name not in by_name]
    if unknown or not names:
        listed = ", ".join(unknown) or "no name"
        raise SelectionError(f"Unknown skill: {listed}. Run the installer with --list to see the skill names.")
    return [by_name[name] for name in names]


def select_skills(args: argparse.Namespace, skills: list, console: Console) -> list:
    if args.skills is not None:
        return find_named_skills(args.skills, skills)
    options = [f"{skill.name}: {skill.summary}" for skill in skills]
    indexes = ask_choice(console, "Which skills do you want to install?", options, default=list(range(len(skills))))
    return [skills[index] for index in indexes]


def read_other_folder(console: Console, project: Path) -> Path:
    answer = read_answer(console, "Type the path of the skills folder: ").strip()
    if not answer:
        raise SelectionError("The folder path is empty. Run the installer again and type a path.")
    return resolve_folder(answer, project)


def select_folders(args: argparse.Namespace, console: Console) -> list:
    project = Path(args.project).resolve()
    chosen = set(args.target or [])
    folders = [resolve_folder(target.folder, project) for target in TARGETS if target.key in chosen]
    folders += [resolve_folder(folder, project) for folder in args.dir or []]
    if folders:
        return folders
    options = [f"{target.label} ({target.folder})" for target in TARGETS] + [OTHER_FOLDER]
    indexes = ask_choice(console, "Where do you want to install them?", options, default=[0])
    folders = [resolve_folder(TARGETS[index].folder, project) for index in indexes if index < len(TARGETS)]
    if len(TARGETS) in indexes:
        folders.append(read_other_folder(console, project))
    return folders


def can_replace(destination: Path, options: Options) -> bool:
    if options.is_forced:
        return True
    if not options.is_interactive:
        return False
    return ask_confirmation(options.console, f"{destination} has local changes. Replace it?")


def install_skill(skill: object, folder: Path, options: Options) -> str:
    destination = folder / skill.name
    status = find_status(skill.folder, destination)
    if status == "unchanged":
        return f"Skipped {skill.name}: {destination} is up to date."
    if status == "blocked":
        return f"Skipped {skill.name}: {destination} has no SKILL.md file. Move or delete that folder, then run the installer again."
    if status == "changed" and not can_replace(destination, options):
        return f"Skipped {skill.name}: {destination} has local changes. Run the installer again with --force to replace it."
    if options.is_dry_run:
        return f"Dry run: {DRY_ACTIONS[status]} {skill.name} in {destination}."
    install = copy_skill if status == "new" else replace_skill
    install(skill.folder, destination)
    return f"{DONE_ACTIONS[status]} {skill.name} in {destination}."


def install_all(skills: list, folders: list, options: Options) -> None:
    messages = [install_skill(skill, folder, options) for folder in folders for skill in skills]
    for message in messages:
        options.console.write(message)
    if any(message.startswith(tuple(DONE_ACTIONS.values())) for message in messages):
        options.console.write("Start a new agent session to load the skills.")


def run(argv: list) -> int:
    args = build_parser().parse_args(argv)
    console = Console(read=input, write=print)
    skills = list_skills(ROOT)
    if args.list:
        console.write(format_listing(skills))
        return 0
    is_interactive = args.skills is None or not (args.target or args.dir)
    try:
        selected = select_skills(args, skills, console)
        folders = select_folders(args, console)
        install_all(selected, folders, Options(args.force, args.dry_run, is_interactive, console))
    except SelectionError as error:
        sys.stderr.write(f"install: {error}\n")
        return USAGE_ERROR
    return 0
