# AGENTS.md for tests

Run the tests with `python3 -m unittest discover -s tests`.

CAUTION: Do not correct a bad fixture. Each file named `bad` breaks rules on purpose. The tests expect the breaks.

- A `good` fixture must pass the checker and every linter with no findings.
- `.ste.json` at the root of the repository excludes `tests/fixtures/` from the checks of the repository.
- When you add a rule to the checker, add a break to a bad fixture and a test for it.
- The build writes `fixtures/code/rust/*/Cargo.toml` from `Cargo.toml.tmpl`. Do not edit the generated files.
