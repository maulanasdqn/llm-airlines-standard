# Profile: TypeScript and JavaScript

This profile applies to `.ts`, `.tsx`, `.mts`, `.cts`, `.js`, `.jsx`, `.mjs`, and `.cjs` files.
It overrides the core STE-C rules only where this table says so.

## Names

| Kind | Case | Example |
|---|---|---|
| Function, method, variable | camelCase | `fetchInvoice`, `retryDelayMs` |
| Boolean | camelCase with is, has, can, or should | `isPaid`, `hasItems` |
| Class, interface, type, enum | PascalCase | `InvoiceStore` |
| React component | PascalCase noun | `InvoiceTable` |
| Constant at module level | UPPER_CASE | `MAX_RETRIES` |
| Acronym | one word (C8.4) | `HttpClient`, `userId`, `parseJson` |

## Idioms that override the core

- A React component is a noun, because it describes a thing on the screen (C3.1).
- A React hook starts with `use`: `useInvoice`. The word `use` is an approved verb in this profile.
- A prop for an event callback starts with `on`: `onSubmit`.
- In `.tsx` and `.jsx` files, a local event callback can start with `handle`: `handleSubmit`. Use `handle` nowhere else (C1.6).
- A getter (`get total()`) is a noun.
- `props`, `ref`, and `req` are approved abbreviations.

## No comments

- Do not write JSDoc or TSDoc blocks (C6.2). The types are the documentation.
- Directives stay: `// eslint-disable-next-line rule -- reason`, `// @ts-expect-error reason`, `/// <reference types="..." />`, and `/* @jsxImportSource ... */`.
- Prefer a type to a runtime comment: `type Milliseconds = number`.

## Errors and failure modes

- Throw an `Error` subclass with a name that ends in `Error`: `InvoiceParseError`.
- For an expected failure, return a result type or `null`, and show it in the signature (C6.5): `findInvoice(id): Invoice | null`.
- The message obeys C7.3: `throw new InvoiceParseError(\`Cannot parse row ${row}: the "total" column is empty. Add a total.\`)`.

## Linter

Use `linters/typescript/eslint.config.mjs`.
It needs the `eslint`, `typescript-eslint`, and `typescript` packages.
Rules that need type information (the boolean prefix rule) read your `tsconfig.json`.
