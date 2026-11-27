# Profile: Go

This profile applies to `.go` files.
It overrides the core STE-C rules only where this page says so.

## Names

| Kind | Case | Example |
|---|---|---|
| Exported function, type, constant | PascalCase | `FetchInvoice`, `InvoiceStore` |
| Unexported name | camelCase | `retryDelay` |
| Boolean | is, has, can, or should | `isPaid` |
| Acronym | all capitals, as Go does (C8.4) | `HTTPClient`, `userID`, `ParseJSON` |

## Idioms that override the core

- A getter method has no `Get` prefix, as Effective Go says: `invoice.Total()`. A method with a receiver and no parameters can be a noun (C3.1).
- A constructor starts with `New`: `NewInvoiceStore`. A function that panics on failure starts with `Must`.
- `err`, `ctx`, `ok`, `i`, and one-letter receivers (`s *Server`) are approved.
- `buf`, `addr`, `req`, and `len` are approved, because the standard library uses them.
- `Test*`, `Benchmark*`, `Example*`, and `Fuzz*` functions keep the names that `go test` needs.
- `String()`, `Error()`, `Unwrap()`, and `ServeHTTP()` keep the names that their interfaces need.

## No comments

- Do not write doc comments (C6.2). This rule overrides the Go convention of a doc comment on each exported name. The `revive` rule `exported` is off in the linter config.
- Directives stay: `//go:build`, `//go:generate`, `//go:embed`, and `//nolint:rule`.

## Errors and failure modes

- Return an `error` as the last value (C6.5). Name an error type `XxxError` and a sentinel error `ErrXxx`.
- An error string starts with a lowercase letter and has no period at the end, as Go does. It still says what failed, why, and what to do (C7.3): `fmt.Errorf("parse row %d: total column is empty: add a total", row)`.
- Wrap an error with `%w` to keep the cause.

## Linter

Use `linters/go/.golangci.yml` with golangci-lint version 2.
