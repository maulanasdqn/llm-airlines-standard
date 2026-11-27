---
name: ste-adopt
description: Set up STE (ASD-STE100 Simplified Technical English) and STE-C (its port to code) in a repository. The skill runs ste adopt, which adds the matching AGENTS.md template, a CLAUDE.md shim, a LEXICON.md, the linter configs, and a commit-msg hook. Then the skill drafts the lexicon from the existing names. Use it when the user asks to "adopt STE", "add the airline standard", "set up AGENTS.md for STE", "add STE linting", or to start the standard in a new or an existing project.
license: MIT
---

# Adopt STE and STE-C

This skill installs the standard in a target repository.
The `ste adopt` command writes the files. It never replaces a file that exists.
If the `ste` command is not installed, copy the files from `assets/`, next to this file.

## Assets

| Asset | Target in the repository |
|---|---|
| `assets/agents/<profile>/AGENTS.md` | `AGENTS.md` at the root, or in a subfolder |
| `assets/LEXICON.md` | `LEXICON.md` at the root |
| `assets/linters/typescript/eslint.config.mjs` | `eslint.config.mjs`, or merged into the existing config |
| `assets/linters/python/ruff.toml` | `ruff.toml`, or merged into `pyproject.toml` |
| `assets/linters/go/.golangci.yml` | `.golangci.yml` |
| `assets/linters/rust/clippy.toml` and `lints.toml` | `clippy.toml`, and the `[lints]` table in `Cargo.toml` |
| `assets/linters/vale/` | `.vale.ini` and `styles/STE/` |
| `assets/linters/git/commit-msg` | `.git/hooks/commit-msg` |

The profiles are `base`, `docs`, `typescript`, `python`, `go`, and `rust`.

## Procedure

1. Make sure that the `ste` command works: `ste --version`. If it does not, install it:

   ```sh
   curl -fsSL https://raw.githubusercontent.com/maulanasdqn/llm-airlines-standard/main/install.sh | bash -s -- --binary-only
   ```

2. Choose the profile.
   - For one main language, use its profile. `ste adopt` finds the main language when you give no profile.
   - For a monorepo, put `base` at the root and a language profile in each package folder.
   - For a docs folder, use `docs`.
3. Show the user the changes first:

   ```sh
   ste adopt --linters --hook --dry-run
   ```

4. Write the files:

   ```sh
   ste adopt --linters --hook
   ```

5. Read the output. For each line that starts with "Kept", merge the rules from the matching file in `assets/` into the existing file. Show the user the change.
6. If the existing rules and the standard disagree, ask the user which to keep (C9.2).
7. Write the lexicon draft:

   ```sh
   ste check lexicon src > LEXICON.draft.md
   ```

8. Move the real domain words from the draft into the tables of `LEXICON.md`, with one meaning each. Delete the draft.
9. If the checker must skip some folders, write a `.ste.json` file:

    ```json
    { "exclude": ["vendor/*", "generated/*"] }
    ```

10. Run the checker and report the counts:

    ```sh
    ste check prose .
    ste check code .
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
