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
