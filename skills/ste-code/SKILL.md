---
name: ste-code
description: Write, refactor, or review source code with STE-C, the port of ASD-STE100 Simplified Technical English to code. STE-C sets approved verbs with one contract each and words to avoid in names. It forbids comments and docstrings, so the code must explain itself. It limits functions to 40 lines and files to 200 lines of code, and it sets rules for guard clauses, destructive operations, error messages, and logs. Use it when you write or change TypeScript, JavaScript, Python, Go, or Rust in a project with an STE AGENTS.md or LEXICON.md. Also use it when the user asks for "STE-C", "self-explaining code", "no comments", or "clear names".
license: MIT
---

# STE-C: STE for code

STE-C applies the intent of ASD-STE100 to source code.
A name is a word. A statement is a sentence. A function is a procedure. A file is a topic.
The code has no comments, so the names, the types, and the structure must explain it.

## Before you write code

1. Read `LEXICON.md` at the root of the project, if it exists. It holds the domain words.
2. Find the language profile in `references/profiles/`. A profile can override a core rule for an idiom of that language.
3. Read the code near the change. Use the verbs and the nouns that it uses (C9.2).

## How to choose a name

1. Find the concept in `LEXICON.md`. If it is there, use that word.
2. For a function, start with a verb from `references/dictionary.md`. Make sure the verb contract fits (C3.1, C3.2).
3. Add 3 nouns or fewer. A name has 4 words or fewer in total (C2.1).
4. For a boolean, start with is, has, can, or should. Use the positive form (C1.4, C4.4).
5. Look for the same object with a different verb elsewhere in the code. Do not add a synonym (C1.3).
6. If you need a new domain word, add it to `LEXICON.md` with one meaning (C1.5).

The verbs with the most use and their contracts:

| Verb | Contract |
|---|---|
| `get` | Return a value in memory. No I/O. |
| `find` | Search. Return nothing when there is no match. |
| `list` | Return a collection. It can be empty. |
| `fetch` | Get a value over the network. |
| `load` / `save` | Read from storage / write to storage. |
| `create` / `build` | Make a new persisted entity / construct a value in memory. |
| `update` / `delete` | Change an existing entity / remove it permanently. |
| `parse` / `format` | Text to a structure / a structure to text for people. |
| `validate` / `assert` | Return a result / throw when false. |
| `ensure` | Make a condition true. Do nothing when it is already true. |

Do not use these words in names: handle, process, manage, do, perform, data, info, util, helper, manager, tmp, obj (C1.6).
Do not use these abbreviations: msg, cnt, idx, usr, cfg, cb, req, resp, prev, cur (C1.7). A profile can approve some of them.

## Structure

| Limit | Value | Rule |
|---|---|---|
| Words in a name | 4 or fewer | C2.1 |
| Lines in a function | 40 or fewer | C5.1 |
| Lines of code in a file, not counting blank lines | 200 or fewer | C5.7 |
| Nesting depth | 3 or less | C5.4 |
| Complexity | 10 or less | C5.4 |
| Positional parameters | 3 or fewer | C5.5 |

- Put guard clauses and early returns first (C5.2).
- Write one action per statement. Give a name to each intermediate result (C4.1, C4.2).
- Do not use magic numbers, implicit coercion, or positional booleans (C4.3).
- Write the steps in execution order. Declare each value near its first use (C5.3).
- Split a file by topic when it grows past the limit. Do not split it at random lines (C5.7).

## No comments

Write no comments, no doc comments, and no docstrings (C6.1, C6.2).
When you want to write a comment, change the code instead (C6.4):

| You want to say | Write this |
|---|---|
| What a block does | Move the block into a function with a verb name |
| What a number means | A named constant: `SESSION_TIMEOUT_MS` |
| What a condition means | A named boolean: `const isTrialExpired = ...` |
| What a value can be | A type, an enum, or a result type |
| How a function fails | The return type or the error type (C6.5) |
| Why the code is like this | The commit message or the PR description (C6.6) |
| A danger | A name with the risk in it, and a safe default (C7.1, C7.2) |

Machine directives stay, because a tool reads them (C6.3).
Examples are a shebang, `# noqa`, `# type: ignore`, `// eslint-disable-next-line`, `// @ts-expect-error`, `//go:build`, and `ste-ignore`.

## Risk, errors, and logs

- Put the risk in the name of a destructive operation: `deleteAllObjects`, `dangerouslySetHtml` (C7.1).
- Give it a safe default and an explicit opt-in: `{ isConfirmed: false }`, `isDryRun = true` (C7.2).
- Write an error message that says what failed, why, and what to do next. Include the value. Obey the W rules (C7.3).
- Write one event per log line, in the simple past, with field names from the lexicon (C7.4).

Example error message:

```text
Cannot read config file "app.toml": permission denied. Make the file readable by the current user.
```

## Self-check

Before you finish, answer these questions for each changed file:

- Does each function name start with an approved verb, and does the verb contract match the code?
- Does a name use a vague word, an abbreviation, or more than 4 words?
- Is there a comment, a doc comment, or a docstring?
- Does a function have more than 40 lines, or the file more than 200 lines of code?
- Does a destructive operation have the risk in its name and a safe default?
- Does each error message say what failed, why, and what to do?

## Tool

If the `ste` command is installed, run it on the changed files:

```sh
ste check code src
```

The checker finds comments, docstrings, long files, long functions, vague words, abbreviations, long names, and names with no verb.
It cannot find a verb that breaks its contract. You must find that.

## References

- `references/rules.md`: all C rules, with the reason and an example for each.
- `references/dictionary.md`: approved verbs, words to avoid, and abbreviations.
- `references/profiles/`: TypeScript, Python, Go, and Rust idioms.
- `references/examples.md`: before and after examples.
- `references/writing-rules.md`: the W rules for error messages, logs, and commits.
- `references/mapping.md`: the map from each STE section to the code rules and the linters.
