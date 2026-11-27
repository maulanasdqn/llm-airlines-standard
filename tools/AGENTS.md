# AGENTS.md for tools

This folder holds `verify_linters.sh`.
The script runs each installed linter on the fixtures in `tests/fixtures/`. It skips a linter that is not installed.

- Write no comments in the script. A shebang is a machine directive, so it stays.
- Keep the script to 200 lines of code or fewer.
- When you add a linter rule, add a break to the bad fixture. Then add the rule ID to the expected list in the script.
