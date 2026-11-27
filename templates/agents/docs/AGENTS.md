# AGENTS.md

This project follows two writing standards.
Prose follows STE, the Simplified Technical English rules of ASD-STE100.
Code follows STE-C, the port of those rules to code.
This file is the documentation version.

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

## Documentation

These rules add to the prose rules for the files in this folder.

- Write a procedure as numbered steps. Put one instruction in each step (W5.2, W5.5).
- Put a WARNING or a CAUTION before the step that it applies to (W7.4).
- Put a NOTE after the step, not inside it (W5.6).
- Use headings, lists, and tables to show structure (W6.4).
- Define each technical noun and each abbreviation the first time that you use it (W1.5, W1.8).
- Write for a reader who has no context. Name the subject in each sentence (W9.2).
- Put code in a code block. The STE checker does not read code blocks.

## Tools

- Read `LEXICON.md` before you give a name to a new thing. Add each new domain word to it.
- If the skills `ste-writing`, `ste-code`, and `ste-review` are installed, use them. They hold the full rules and the dictionary.
- Run the `ste` checker before you commit:

  ```sh
  ste check prose .
  ste check code .
  ```

- To install `ste`, run `curl -fsSL https://raw.githubusercontent.com/maulanasdqn/llm-airlines-standard/main/install.sh | bash -s -- --binary-only`.

- To skip a rule on one line, put the directive `ste-ignore: C1.6` in a comment on that line. Give the reason in the commit message.
