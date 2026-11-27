# AGENTS.md for skills

Each folder here is one Agent Skill in the open Agent Skills format.
Claude Code, Codex, and other agents can load these skills.

## Format rules

- The `name` in the frontmatter is the folder name. It uses lowercase letters, digits, and hyphens, in 64 characters or fewer.
- The `description` says what the skill does and when to use it, in 1024 characters or fewer.
- `SKILL.md` has fewer than 500 lines. Put detail in `references/`.
- Use relative paths from the skill folder. A user can install one skill without the others.
- The build checks these rules.

## Generated files

The build writes these files. Do not edit them:

- `*/references/*`, except `ste-review/references/finding-format.md`.
- `ste-review/scripts/ste_data.json`.
- `ste-adopt/assets/` and all its contents.

## The checker

The checker code is in `ste-review/scripts/ste/`.

- It uses only the standard library of Python 3.9.
- It has no comments and no docstrings. Each file has 200 lines of code or fewer.
- Each function name starts with a verb from the dictionary.
- When you add a check, add a bad case and a good case to `tests/`.
- After a change, run the build. The build copies the checker into `ste-adopt/assets/tools/ste/`.
