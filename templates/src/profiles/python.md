## Python

- Use snake_case for functions and values, and PascalCase for classes (C8.4).
- Write an acronym in capitals in a class name, as PEP 8 says: `HTTPClient` (C8.4).
- A `@property` is a noun. A test function can have a name with more than 4 words (C2.1, C3.1).
- Use keyword-only parameters (`*`) to keep the positional parameters to {{ limits.code.positional_params }} or fewer (C5.5).
- Do not write docstrings (C6.2). Put type hints on each public function (C4.3).
- Return `None` or a result type for an expected failure, and show it in the type hint (C6.5).
- Run ruff with `ruff.toml` from the STE linters (`linters/python`).
