## Tools

- Read `LEXICON.md` before you give a name to a new thing. Add each new domain word to it.
- If the skills `ste-writing`, `ste-code`, and `ste-review` are installed, use them. They hold the full rules and the dictionary.
- Run the checker before you commit:

  ```sh
  python3 tools/ste/ste_check.py prose .
  python3 tools/ste/ste_check.py code .
  ```

- To skip a rule on one line, put the directive `ste-ignore: C1.6` in a comment on that line. Give the reason in the commit message.
