# AGENTS.md

This project follows two writing standards.
Prose follows STE, the Simplified Technical English rules of ASD-STE100.
Code follows STE-C, the port of those rules to code.
This file is the Rust version.

Obey these rules in each file that you write or change.
If a rule in this file and a request from the user disagree, ask the user.

## Prose

These rules apply to docs, commit messages, PR descriptions, error messages, log lines, and prompts for other agents.

- Use one word for one meaning. Do not rotate synonyms (W1.1).
- Use simple words. Write "use", not "utilize" or "leverage" (W1.4).
- Use 3 nouns or fewer in a noun cluster (W2.1).
- Use the active voice and simple verb forms. Do not use "-ing" verb forms (W3.1, W3.2, W3.3).
- Write one idea per sentence. Do not leave out articles or verbs (W4.1, W4.2).
- Use 1 hedge or fewer in each sentence (W4.3).
- Write an instruction in 20 words or fewer, in the imperative (W5.1, W5.3).
- Put a condition before the instruction (W5.4).
- Write a description in 25 words or fewer. Put the most important fact first (W6.1, W6.2).
- Write 6 sentences or fewer in a paragraph (W6.3).
- Start a risk notice with WARNING or CAUTION. Then write the command, then the risk (W7.1, W7.2, W7.3).
- Do not join sentences with a semicolon or a dash (W8.2).
- Keep every fact, number, and condition when you simplify (W9.3).
- Do not change quoted text, or text that already obeys the rules (W9.4, W9.5).

## Code

### Names

- Start a function name with an approved verb (C3.1).
- Keep the contract of each verb (C3.2):
  - `get` reads a value in memory, with no I/O.
  - `find` can return nothing.
  - `fetch` uses the network.
  - `load` and `save` use storage.
  - `create` makes a new persisted entity. `build` makes a value in memory.
  - `validate` returns a result. `assert` throws.
- Start a boolean with is, has, can, or should. Use the positive form: `isEnabled`, not `isNotDisabled` (C1.4, C4.4).
- Do not use vague words, such as handle, process, manage, data, info, util, helper, or manager (C1.6).
- Do not use abbreviations, such as msg, cnt, idx, or usr. The words `id`, `url`, `config`, `ctx`, and `err` are approved (C1.7).
- Use 4 words or fewer in a name (C2.1).
- Use the same verb for the same action everywhere (C1.3).
- Record each domain noun and domain verb in LEXICON.md (C1.5, C9.1).

### Structure

- Write one task per function, in 40 lines or fewer (C5.1).
- Write one topic per file, in 200 lines of code or fewer. Blank lines do not count (C5.7).
- Put guard clauses first (C5.2).
- Keep the nesting depth to 3 or less and the complexity to 10 or less (C5.4).
- Use 3 positional parameters or fewer. For more, use an options object (C5.5).
- Give a name to each intermediate value and constant. Do not use magic numbers or positional booleans (C4.2, C4.3).

### No comments

- Write no comments, no doc comments, and no docstrings (C6.1, C6.2).
- Make the code explain itself with names, constants, and types (C6.4).
- Show each failure mode in the signature or in the error type (C6.5).
- Keep only machine directives, such as a shebang, `# noqa`, `eslint-disable`, or `//go:build` (C6.3).
- Write the reason for a change in the commit message or the PR (C6.6).

### Risk, errors, and logs

- Put the risk in the name of a destructive operation, and give it a safe default (C7.1, C7.2). An example is `deleteAllRows({ isConfirmed: false })`.
- Write an error message that says what failed, why, and what to do next. Include the value (C7.3).
- Write one event per log line, in the simple past: `Deleted 3 expired sessions.` (C7.4).

### Commits

- Write the subject in the imperative, in 50 characters or fewer, with no period (C9.4).
- Put the reason for the change in the body. Make one change per commit (C9.4).

## Rust

- Use snake_case for functions, values, and modules, and PascalCase for types (C8.4).
- Write an acronym as one word: `HttpClient` (C8.4).
- A getter has no `get_` prefix: `invoice.total()` (C3.1).
- Keep the conversion contracts: `as_` borrows, `to_` copies, and `into_` takes ownership (C3.2).
- Do not write `///` or `//!` doc comments (C6.2). Attributes such as `#[expect(...)]` are code.
- Return `Result<T, E>` with a named error type. Do not call `unwrap()` in library code (C6.5).
- Run clippy with `clippy.toml` and the `[lints]` table from the STE linters (`linters/rust`).

## Tools

- Read `LEXICON.md` before you give a name to a new thing. Add each new domain word to it.
- If the skills `ste-writing`, `ste-code`, and `ste-review` are installed, use them. They hold the full rules and the dictionary.
- Run the `ste` checker before you commit:

  ```sh
  ste check prose .
  ste check code .
  ```

- To install `ste` on macOS or Linux, run `curl -fsSL https://raw.githubusercontent.com/maulanasdqn/llm-airlines-standard/main/install.sh | bash -s -- --binary-only`.
- To install `ste` on Windows, run `irm https://raw.githubusercontent.com/maulanasdqn/llm-airlines-standard/main/install.ps1 | iex` in PowerShell.

- To skip a rule on one line, put the directive `ste-ignore: C1.6` in a comment on that line. Give the reason in the commit message.
