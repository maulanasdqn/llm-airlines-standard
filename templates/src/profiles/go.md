## Go

- Use PascalCase for exported names and camelCase for the other names (C8.4).
- Write an acronym in capitals: `HTTPClient`, `userID` (C8.4).
- A getter has no `Get` prefix: `invoice.Total()`. A constructor starts with `New` (C3.1).
- The names `err`, `ctx`, `ok`, `i`, and one-letter receivers are approved (C1.7).
- Do not write doc comments. This rule overrides the Go convention (C6.2).
- Return an `error` as the last value. Wrap the cause with `%w` (C6.5).
- Start an error string with a lowercase letter, with no period at the end. Still say what failed, why, and what to do (C7.3).
- Run golangci-lint version 2 with `.golangci.yml` from the STE linters (`linters/go`).
