# AGENTS.md for src

This folder holds the Rust source of the `ste` binary.

| Module | Contents |
|---|---|
| `spec` | The parser of the rule files, the dictionary, the lexicon, and `.ste.json` |
| `check` | The `ste check` commands: prose, code, commit, and lexicon |
| `build` | The `ste build` command, which writes the generated files of this repository |
| `install` | The `ste install` command, which installs the skills |
| `adopt` | The `ste adopt` command, which sets up a repository |
| `embedded` | The files that `build.rs` puts into the binary |

## Rules

- Obey the Rust profile in `spec/code/profiles/rust.md`.
- Let the module carry the noun, and start the function name with a verb: `markdown::parse`, not `markdown::parse_markdown` (C2.3).
- Give each function 3 parameters or fewer. For more, use a struct (C5.5).
- Write each error message with what failed, why, and what to do next (C7.3). Write each `expect` message as a fact that must be true.
- Do not call `unwrap()`. Clippy reports it.
- Run `cargo fmt`, then `cargo run -- check code src build.rs`.
