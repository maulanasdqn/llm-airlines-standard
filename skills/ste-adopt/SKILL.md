---
name: ste-adopt
description: Set up STE (ASD-STE100 Simplified Technical English) and STE-C (its port to code) in a repository. The skill adds the matching AGENTS.md template, a CLAUDE.md shim, a LEXICON.md drafted from the existing names, the linter configs, the checker, and a commit-msg hook. Use it when the user asks to "adopt STE", "add the airline standard", "set up AGENTS.md for STE", "add STE linting", or to start the standard in a new or an existing project.
license: MIT
---

# Adopt STE and STE-C

This skill installs the standard in a target repository.
All the files that it copies are in `assets/`, next to this file.

## Assets

| Asset | Target in the repository |
|---|---|
| `assets/agents/<profile>/AGENTS.md` | `AGENTS.md` at the root, or in a subfolder |
| `assets/LEXICON.md` | `LEXICON.md` at the root |
| `assets/tools/ste/` | `tools/ste/` |
| `assets/linters/typescript/eslint.config.mjs` | `eslint.config.mjs`, or merged into the existing config |
| `assets/linters/python/ruff.toml` | `ruff.toml`, or merged into `pyproject.toml` |
| `assets/linters/go/.golangci.yml` | `.golangci.yml` |
| `assets/linters/rust/clippy.toml` and `lints.toml` | `clippy.toml`, and the `[lints]` table in `Cargo.toml` |
| `assets/linters/vale/` | `.vale.ini` and `styles/STE/` |
| `assets/linters/git/commit-msg` | `.git/hooks/commit-msg` |

The profiles are `base`, `docs`, `typescript`, `python`, `go`, and `rust`.

## Procedure

1. Find the languages of the repository. Count the files for each extension, but skip folders such as `node_modules` and `vendor`.
2. Choose the profile.
   - For one main language, use its profile.
   - For a monorepo, put `base` at the root and a language profile in each package folder.
   - For a docs folder, use `docs`.
3. Look for an existing `AGENTS.md`, `CLAUDE.md`, and linter config.
   - If a file exists, do not overwrite it. Merge the new rules into it, and show the user the change.
   - If the existing rules and the standard disagree, ask the user which to keep (C9.2).
4. Copy the AGENTS.md template.
5. Next to each AGENTS.md, write a `CLAUDE.md` file that contains only the line `@AGENTS.md`.
6. Copy `assets/tools/ste/` to `tools/ste/`.
7. Write the lexicon draft:

   ```sh
   python3 tools/ste/ste_check.py lexicon src > LEXICON.draft.md
   ```

8. Copy `assets/LEXICON.md` to `LEXICON.md`. Move the real domain words from the draft into its tables, with one meaning each. Delete the draft.
9. Copy the linter configs for the languages of the repository. Merge them into the existing configs.
10. If the repository uses git, install the hook: copy `assets/linters/git/commit-msg` to `.git/hooks/commit-msg` and make it executable.
11. If the checker must skip some folders, write a `.ste.json` file:

    ```json
    { "exclude": ["vendor/*", "generated/*"] }
    ```

12. Run the checker and report the counts:

    ```sh
    python3 tools/ste/ste_check.py prose .
    python3 tools/ste/ste_check.py code .
    ```

## In an existing codebase

An existing codebase can have thousands of findings.
Do not fix all of them in the setup change.

- Report the counts for each rule to the user.
- Suggest an order: first the comments (C6.1, C6.2), then the long files (C5.7), and then the names.
- If the user agrees, add the rules that the team fixes later to `"disable"` in `.ste.json`. Remove each one when the team fixes its findings.

## Result

Tell the user these facts:

- The files that you added or changed.
- The finding counts for each severity.
- The next steps, such as a review of `LEXICON.md`.
