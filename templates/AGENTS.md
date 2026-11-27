# AGENTS.md for templates

This folder holds the drop-in AGENTS.md templates for other repositories.

- Edit the fragments in `src/`. The build joins them into `agents/<profile>/AGENTS.md`.
- Do not edit the files in `agents/`. The build writes them.
- Use a placeholder for each limit: `{{ limits.code.function_lines }}`. Do not write the number.
- Each template must work alone. Do not link to a file in this repository.
- Keep each template short. The skills hold the full rules.
- `LEXICON.md` is the empty project dictionary. Keep its example rows out of the tables, because the checker reads the tables.

The recipes in `tools/stebuild/plan.py` set which fragments each profile uses.
