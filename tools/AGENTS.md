# AGENTS.md for tools

`build.py` starts the build. The code is in the `stebuild/` package.
`install.py` starts the installer of the skills. The code is in the `steinstall/` package.

- Use only the standard library of Python 3.9.
- Write no comments and no docstrings.
- Keep each file to 200 lines of code or fewer, and each function to 40 lines or fewer.
- Start each function name with a verb from `spec/code/dictionary.md`.
- `verify_linters.sh` runs each installed linter on the fixtures. It skips a linter that is not installed.
- The installer must never delete a folder that has no `SKILL.md` file. It replaces a changed skill only after `--force` or a "y" answer.
- If you add a target folder to `steinstall/catalog.py`, cite the documentation of that agent in the README.
