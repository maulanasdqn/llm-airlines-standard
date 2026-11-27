# Profile: Python

This profile applies to `.py` and `.pyi` files.
It overrides the core STE-C rules only where this page says so.

## Names

| Kind | Case | Example |
|---|---|---|
| Function, method, variable | snake_case | `fetch_invoice`, `retry_delay_ms` |
| Boolean | snake_case with is, has, can, or should | `is_paid`, `has_items` |
| Class, exception | PascalCase | `InvoiceStore`, `InvoiceParseError` |
| Constant at module level | UPPER_CASE | `MAX_RETRIES` |
| Acronym | PascalCase classes write it in capitals, as PEP 8 says (C8.4) | `HTTPClient`, `parse_json` |

## Idioms that override the core

- A `@property` is a noun: `invoice.total`.
- A dunder method (`__eq__`) keeps its Python name.
- The test framework sets some names: `setUp`, `tearDown`, and `test_*` functions.
- A test name can be longer than 4 words, because it states one behavior.
- `self`, `cls`, `args`, `kwargs`, and `argv` are approved.
- Use keyword-only parameters (`def create_user(name, *, email, role)`) to keep 3 positional parameters or fewer (C5.5).

## No comments

- Do not write docstrings (C6.2). A docstring is a comment in a string.
- Directives stay: `#!/usr/bin/env python3`, `# noqa: E501`, `# type: ignore[arg-type]`, `# pragma: no cover`, and `# fmt: off`.
- Use type hints on every public function (C4.3). The hints are the documentation.

## Errors and failure modes

- Raise an exception class with a name that ends in `Error`.
- For an expected failure, return `None` or a result type, and show it in the hint (C6.5): `def find_invoice(invoice_id: str) -> Invoice | None`.
- The message obeys C7.3: `raise InvoiceParseError(f'Cannot parse row {row}: the "total" column is empty. Add a total.')`.

## Linter

Use `linters/python/ruff.toml` with ruff.
The config does not turn on the pydocstyle (`D`) rules, because this standard forbids docstrings.
The STE checker finds docstrings and comments.
