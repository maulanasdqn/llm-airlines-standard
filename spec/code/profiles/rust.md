# Profile: Rust

This profile applies to `.rs` files.
It overrides the core STE-C rules only where this page says so.

## Names

| Kind | Case | Example |
|---|---|---|
| Function, method, variable, module | snake_case | `fetch_invoice`, `retry_delay` |
| Boolean | is, has, can, or should | `is_paid` |
| Type, trait, enum variant | PascalCase | `InvoiceStore` |
| Constant, static | UPPER_CASE | `MAX_RETRIES` |
| Acronym | one word, as the Rust API Guidelines say (C8.4) | `HttpClient`, `parse_json` |

## Idioms that override the core

- A getter has no `get_` prefix, as the Rust API Guidelines say: `invoice.total()`. A method that takes only `self` can be a noun (C3.1).
- Conversion verbs keep their API Guidelines contracts (C3.2): `as_` is free and borrows, `to_` is expensive, and `into_` takes ownership.
- `new`, `from`, `with`, `as`, and `into` are approved verbs. `try_` takes an approved verb after it: `try_parse`.
- Trait methods keep the names that their traits need: `fmt`, `drop`, `next`, `clone`, and `default`.
- `buf`, `fmt`, `len`, and `addr` are approved.

## No comments

- Do not write `///`, `//!`, or `/** */` doc comments (C6.2). The `missing_docs` lint stays off.
- Attributes are code, not comments: `#[allow(clippy::too_many_lines)]` is allowed. Add a reason with `#[expect(lint, reason = "...")]`.

## Errors and failure modes

- Return `Result<T, E>` with a named error type for each failure that a caller can handle (C6.5).
- Do not call `unwrap()` in library code. Use `expect("...")` only when the failure is a bug, and write the message as a fact that must be true.
- The `Display` text of an error obeys C7.3: `cannot parse row 4: the total column is empty. Add a total.`

## Linter

Use `linters/rust/clippy.toml` and the `[lints]` table in `linters/rust/lints.toml`.
Copy the table into your `Cargo.toml`.
