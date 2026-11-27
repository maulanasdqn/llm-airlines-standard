## Rust

- Use snake_case for functions, values, and modules, and PascalCase for types (C8.4).
- Write an acronym as one word: `HttpClient` (C8.4).
- A getter has no `get_` prefix: `invoice.total()` (C3.1).
- Keep the conversion contracts: `as_` borrows, `to_` copies, and `into_` takes ownership (C3.2).
- Do not write `///` or `//!` doc comments (C6.2). Attributes such as `#[expect(...)]` are code.
- Return `Result<T, E>` with a named error type. Do not call `unwrap()` in library code (C6.5).
- Run clippy with `clippy.toml` and the `[lints]` table from the STE linters (`linters/rust`).
