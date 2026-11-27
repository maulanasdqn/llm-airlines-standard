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
- Use {{ limits.code.name_words }} words or fewer in a name (C2.1).
- Use the same verb for the same action everywhere (C1.3).
- Record each domain noun and domain verb in LEXICON.md (C1.5, C9.1).

### Structure

- Write one task per function, in {{ limits.code.function_lines }} lines or fewer (C5.1).
- Write one topic per file, in {{ limits.code.file_lines }} lines of code or fewer. Blank lines do not count (C5.7).
- Put guard clauses first (C5.2).
- Keep the nesting depth to {{ limits.code.nesting_depth }} or less and the complexity to {{ limits.code.complexity }} or less (C5.4).
- Use {{ limits.code.positional_params }} positional parameters or fewer. For more, use an options object (C5.5).
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

- Write the subject in the imperative, in {{ limits.commit.subject_chars }} characters or fewer, with no period (C9.4).
- Put the reason for the change in the body. Make one change per commit (C9.4).
