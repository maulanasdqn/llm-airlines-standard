# AGENTS.md

This repository holds an airline standard for LLM agents.
It ports ASD-STE100 Simplified Technical English (STE) to agent prose (the W rules) and to source code (STE-C, the C rules).
One Rust binary, `ste`, checks text and code, installs the skills, sets up a repository, and generates the files of this repository.
This repository obeys its own standard.

## Layout

| Folder | Contents | Edit by hand |
|---|---|---|
| `spec/` | The rules, the dictionary, the language profiles, and the examples. This folder is the single source. | Yes, except the generated files that the build lists |
| `src/` | The Rust source of the `ste` binary | Yes |
| `skills/` | Four Agent Skills: `ste-writing`, `ste-code`, `ste-review`, and `ste-adopt` | `SKILL.md` files and `ste-review/references/finding-format.md` only |
| `templates/` | The AGENTS.md templates for other repositories | `templates/src/` and `LEXICON.md` only |
| `linters/` | Configs for Vale, ESLint, ruff, golangci-lint, and clippy | `linters/src/` only |
| `tools/` | The linter check script | Yes |
| `tests/` | The Rust integration tests and the fixtures | Yes |
| `.claude-plugin/` | The Claude Code plugin and marketplace manifests, and `icon.png` | Yes. Render `icon.png` again from `docs/images/icon.svg` after you change the SVG. |
| `install.sh`, `install.ps1` | The installers for macOS and Linux, and for Windows | Yes. Write no comments in them. |

## How to change the standard

1. Change the source: a rule file in `spec/`, `spec/dictionary.json`, a template fragment, or a linter template.
2. Run the build. It writes the generated files:

   ```sh
   cargo run -- build
   ```

3. Run the checks:

   ```sh
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   cargo run -- build --check
   cargo run -- check prose .
   cargo run -- check code .
   ```

4. If you changed a linter template, run `bash tools/verify_linters.sh`.

Do not edit a generated file. The build overwrites it, and `--check` fails on the difference.
The binary embeds the rules, the dictionary, the skills, the templates, and the linter configs. Build it again after you change them.

## Rules for work in this repository

- Write all prose with the W rules in `spec/writing/rules.md`.
- Write all code with the C rules in `spec/code/rules.md` and the Rust profile in `spec/code/profiles/rust.md`.
- Write no comments in code. Make the names, the constants, and the types explain the code.
- Keep each source file to 200 lines of code or fewer, and each function to 40 lines or fewer.
- Add a crate only when the standard library has no equivalent. The binary must have no runtime dependency.
- Record each new domain word in `LEXICON.md`.

## Legal rule

WARNING: Do not copy text from the ASD-STE100 standard or from its dictionary. ASD owns the copyright and the trademark.

- Paraphrase the intent of a rule in your own words.
- Write your own examples.
- Keep the disclaimer in `NOTICE.md`.

## Commits and releases

- Write the subject in the imperative, in 50 characters or fewer, with no period. Use the Conventional Commits prefixes.
- Put the reason for the change in the body.
- Check the message: `cargo run -- check commit .git/COMMIT_EDITMSG`.
- To publish a release, change the version in `Cargo.toml`, then push a tag such as `v0.3.0`. The release workflow builds `ste` for each platform.
