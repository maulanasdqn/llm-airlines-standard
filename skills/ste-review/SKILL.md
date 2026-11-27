---
name: ste-review
description: Audit prose or code against STE (the ASD-STE100 writing rules) and STE-C (the code port), and report each break with its rule ID. The skill runs the ste checker and the configured linters first. Then it reviews the rules that tools cannot find, such as a word with two meanings or a verb that breaks its contract. Use it to review a diff, a PR, a file, a doc, or a commit message. Also use it when the user asks to "check STE", "lint the prose", "review against the standard", or "find comments in the code".
license: MIT
---

# STE review

This skill finds breaks of the W rules (prose) and the C rules (code).
It reports each break. It does not change a file unless the user asks for a fix.

## Procedure

1. Find the targets.
   - For a diff, list the changed files: `git diff --name-only main...HEAD`.
   - For a PR, read the files and the description.
   - For a commit message, save it to a file or pipe it to the checker.
2. Run the `ste` checker:

   ```sh
   ste check prose README.md docs
   ste check code src
   ste check commit .git/COMMIT_EDITMSG
   ```

   Add `--format json` to get the findings as data. The exit code is 1 when there is an error.
   If the `ste` command is not installed, tell the user how to install it. Then do the review by hand with the rules in `references/`:

   ```sh
   curl -fsSL https://raw.githubusercontent.com/maulanasdqn/llm-airlines-standard/main/install.sh | bash -s -- --binary-only
   ```

   On Windows, use PowerShell:

   ```powershell
   & ([scriptblock]::Create((irm https://raw.githubusercontent.com/maulanasdqn/llm-airlines-standard/main/install.ps1))) -BinaryOnly -AddToPath
   ```
3. If the project has linter configs from the standard, run them. Look for `eslint.config.mjs`, `ruff.toml`, `.golangci.yml`, `clippy.toml`, and `.vale.ini`.
4. Review the rules that tools cannot find. Use the list in the next section.
5. Report the findings in the format of `references/finding-format.md`.
6. If the user asks for fixes, apply the `ste-writing` and `ste-code` skills. Then run the checker again.

## Rules that only a review can find

Prose:

- W1.1 and W1.2: one word for one meaning, and one meaning for each word.
- W1.5: a technical noun or verb that the text does not define.
- W4.1 and W5.2: two ideas or two instructions in one sentence.
- W4.2: a missing subject, verb, or article.
- W5.4: an instruction that comes before its condition.
- W9.3: a rewrite that dropped a fact, a number, or a condition.

Code:

- C1.2: one word with two meanings in the codebase.
- C3.2: a verb that breaks its contract, such as a `get` that uses the network.
- C3.4: a function with an effect that its name does not state.
- C6.5: a failure mode that the signature does not show.
- C7.1 and C7.2: a destructive operation with a safe-looking name or an unsafe default.
- C9.2: a new convention next to an existing one.

## Severity

| Severity | Meaning |
|---|---|
| error | The text or the code can cause a wrong action. Fix it before the merge. |
| warning | The text or the code is harder to read than necessary. Fix it in this change if you can. |
| info | A possible problem. A person must decide. |

## What not to report

- Text in quotes, in block quotes, or in code blocks (W9.4).
- Text that already obeys the rules (W9.5).
- A machine directive, such as `# noqa` or `//go:build` (C6.3).
- An idiom that the language profile approves.

## References

- `references/finding-format.md`: the report format.
- `references/writing-rules.md` and `references/code-rules.md`: every rule.
- `references/words.md` and `references/dictionary.md`: the word lists.
- `references/mapping.md`: which tool checks which rule.
