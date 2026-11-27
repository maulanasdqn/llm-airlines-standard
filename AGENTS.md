# AGENTS.md

This repository holds an airline standard for LLM agents.
It ports ASD-STE100 Simplified Technical English (STE) to agent prose (the W rules) and to source code (STE-C, the C rules).
This repository obeys its own standard.

## Layout

| Folder | Contents | Edit by hand |
|---|---|---|
| `spec/` | The rules, the dictionary, the language profiles, and the examples. This folder is the single source. | Yes, except the generated files that the build lists |
| `skills/` | Four Agent Skills: `ste-writing`, `ste-code`, `ste-review`, and `ste-adopt` | `SKILL.md` files and the checker code only |
| `templates/` | The AGENTS.md templates for other repositories | `templates/src/` and `LEXICON.md` only |
| `linters/` | Configs for Vale, ESLint, ruff, golangci-lint, and clippy | `linters/src/` only |
| `tools/` | The build script, the installer, and the linter check | Yes |
| `tests/` | The unit tests and the fixtures | Yes |
| `.claude-plugin/` | The Claude Code plugin and marketplace manifests, and `icon.png` | Yes. Render `icon.png` again from `docs/images/icon.svg` after you change the SVG. |
| `install.sh` | The bootstrap of the installer, for use with `curl` | Yes |

## How to change the standard

1. Change the source: a rule file in `spec/`, `spec/dictionary.json`, a template fragment, or a linter template.
2. Run the build. It writes the generated files:

   ```sh
   python3 tools/build.py
   ```

3. Run the checks:

   ```sh
   python3 tools/build.py --check
   python3 -m unittest discover -s tests
   python3 skills/ste-review/scripts/ste_check.py prose .
   python3 skills/ste-review/scripts/ste_check.py code .
   ```

4. If you changed a linter template, run `bash tools/verify_linters.sh`.

Do not edit a generated file. The build overwrites it, and `--check` fails on the difference.

## Rules for work in this repository

- Write all prose with the W rules in `spec/writing/rules.md`.
- Write all code with the C rules in `spec/code/rules.md`.
- Write no comments in code. Make the names, the constants, and the types explain the code.
- Keep each source file to 200 lines of code or fewer, and each function to 40 lines or fewer.
- Use only the standard library of Python 3.9 in `tools/` and in the checker.
- Record each new domain word in `LEXICON.md`.

## Legal rule

WARNING: Do not copy text from the ASD-STE100 standard or from its dictionary. ASD owns the copyright and the trademark.

- Paraphrase the intent of a rule in your own words.
- Write your own examples.
- Keep the disclaimer in `NOTICE.md`.

## Commits

- Write the subject in the imperative, in 50 characters or fewer, with no period.
- Put the reason for the change in the body.
- Check the message: `python3 skills/ste-review/scripts/ste_check.py commit .git/COMMIT_EDITMSG`.
