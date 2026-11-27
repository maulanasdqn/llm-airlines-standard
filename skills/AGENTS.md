# AGENTS.md for skills

Each folder here is one Agent Skill in the open Agent Skills format.
Claude Code, Codex, and other agents can load these skills.
The `ste` binary embeds these folders, and `ste install` copies them.

## Format rules

- The `name` in the frontmatter is the folder name. It uses lowercase letters, digits, and hyphens, in 64 characters or fewer.
- The `description` says what the skill does and when to use it, in 1024 characters or fewer.
- `SKILL.md` has fewer than 500 lines. Put detail in `references/`.
- Use relative paths from the skill folder. A user can install one skill without the others.
- A skill must work when the `ste` command is not installed. Then the agent applies the rules by hand.
- The build checks these rules.

## Generated files

The build writes these files. Do not edit them:

- `*/references/*`, except `ste-review/references/finding-format.md`.
- `ste-adopt/assets/` and all its contents.

After a change to a skill, run `cargo run -- build`. Then build the binary again, so that it embeds the new skill.
