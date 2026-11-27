# STE-C code rules (C)

STE-C ("STE for Code") ports the intent of ASD-STE100 from manuals to source code.
A name is a word. A statement is a sentence. A function is a procedure. A file is a topic.

The 9 sections follow the 9 sections of ASD-STE100.
Section N of the code rules maps to section N of the writing rules.
The file `mapping.md` shows this map and the tools that check each rule.

The code has no comments.
Names, types, and structure explain the code.
The reason for a change goes in the commit message, the PR, or the docs.

A language profile can override a rule for an idiom of that language.
The profiles are in the `profiles` folder.

## C1 Identifiers (STE: Words)

### C1.1 Use words from the dictionary or the project lexicon in names

- Severity: warning
- Checks: `review`
- Why: A reader who knows the dictionary knows what each name does.
- Do: `loadConfig`, `Invoice`, `isExpired`
- Do not: `slurpConfig`, `InvoiceThingy`, `expiredness`

### C1.2 Give a word one meaning in the whole codebase

- Severity: warning
- Checks: `review`
- Why: If `user` is sometimes a record and sometimes an ID, every use needs a second look.
- Do: `user` for the record and `userId` for the ID
- Do not: `user` for both

### C1.3 Use one word for one meaning

- Severity: warning
- Checks: `ste_check`
- Why: Two verbs for one action look like two actions. Do not rotate synonyms.
- Do: `getUser`, `getOrder`, `getInvoice`
- Do not: `getUser`, `retrieveOrder`, `obtainInvoice`

### C1.4 Use a verb for a function, a noun for a value, and a predicate for a boolean

- Severity: warning
- Checks: `review`, `eslint:@typescript-eslint/naming-convention`
- Why: A function does an action, so its name is a verb. A value or a type is a thing, so its name is a noun. A boolean answers a question, so its name starts with is, has, can, or should.
- Do: `sendEmail()`, `EmailQueue`, `isSent`
- Do not: `email()` that sends an email, `sent` for a boolean

### C1.5 Record every technical noun and technical verb in LEXICON.md

- Severity: warning
- Checks: `review`, `ste_check`
- Why: A domain word is clear only when the project defines it once.
- Do: add `reconcile` to the technical verbs in LEXICON.md, with one meaning
- Do not: use `reconcile`, `settle`, and `match` for the same ledger action

### C1.6 Do not use vague words in names

- Severity: warning
- Checks: `ste_check`, `eslint:id-denylist`, `clippy:disallowed_names`
- Why: A vague word fits every name, so it tells the reader nothing.
- Do: `parseInvoice`, `invoiceRows`, `dateFormat`
- Do not: `handleInvoice`, `invoiceData`, `dateUtils`

### C1.7 Use only approved abbreviations

- Severity: warning
- Checks: `ste_check`, `revive:var-naming`
- Why: An abbreviation saves the writer a few keys and costs every reader a guess.
- Do: `message`, `count`, `index`, `userId`
- Do not: `msg`, `cnt`, `idx`, `usr`

### C1.8 Put the unit in the name when the type does not carry it

- Severity: warning
- Checks: `review`
- Why: A number with no unit is a guess.
- Do: `timeoutMs`, `sizeBytes`, `Duration`
- Do not: `timeout`, `size`

## C2 Compound names (STE: Noun clusters)

### C2.1 Use 4 words or fewer in a name

- Severity: warning
- Checks: `ste_check`
- Limit: code.name_words = 4
- Why: A name has one verb or one qualifier, and 3 nouns or fewer. A longer name is a noun cluster.
- Do: `retryDelayMs`, `sendInvoiceEmail`
- Do not: `userAccountSettingsUpdateRequestFactory`

### C2.2 Use a module, a type, or an options object instead of a longer name

- Severity: warning
- Checks: `review`
- Why: Structure carries context so that the name does not have to.
- Do: `billing.Invoice`, `send(email, { retry: true })`
- Do not: `BillingInvoiceModel`, `sendEmailWithRetry`

### C2.3 Do not repeat the context in a name

- Severity: warning
- Checks: `review`, `clippy:module_name_repetitions`
- Why: The reader already knows the context. A repeated word is noise.
- Do: `user.name`, `invoice.total`
- Do not: `user.userName`, `invoice.invoiceTotal`

## C3 Functions (STE: Verbs)

### C3.1 Start a function name with an approved verb

- Severity: warning
- Checks: `ste_check`
- Why: The verb tells the reader what the function does before the reader opens it. A profile can allow a noun name for a getter, a property, a constructor, or a component.
- Do: `parseDate`, `fetchInvoice`, `isValid`
- Do not: `dateParser`, `invoice`, `validity`

### C3.2 Keep the contract of the verb

- Severity: error
- Checks: `review`, `clippy:wrong_self_convention`
- Why: Each verb in the dictionary has one contract. `get` does no I/O. `fetch` uses the network. `find` can return nothing.
- Do: `fetchUser` for a network call, `getUser` for a value in memory
- Do not: `getUser` that sends an HTTP request

### C3.3 Use the imperative for a command and the simple past for an event

- Severity: warning
- Checks: `review`
- Why: A command tells the code to act. An event tells the code that something occurred. Use "-ing" only for a state.
- Do: `cancelOrder()`, `OrderCanceled`, `onOrderCanceled`, `isLoading`
- Do not: `orderCancel()`, `OrderCancel` for an event, `cancelingOrder()`

### C3.4 Put every effect of a function in its name

- Severity: error
- Checks: `review`
- Why: A hidden effect is a trap for the next reader.
- Do: `saveInvoice()` and then `sendInvoiceEmail()`
- Do not: `saveInvoice()` that also sends an email

## C4 Statements and expressions (STE: Sentences)

### C4.1 Write one action per statement

- Severity: warning
- Checks: `review`, `eslint:no-cond-assign`, `eslint:no-sequences`
- Why: A statement with two actions is a sentence with two ideas.
- Do: `const user = await fetchUser(id);` then `if (user === null) {`
- Do not: `if ((user = await fetchUser(id)) === null) {`

### C4.2 Give a name to an intermediate result

- Severity: warning
- Checks: `review`, `eslint:no-nested-ternary`
- Why: A name explains the value without a comment. Do not nest conditional expressions.
- Do: `const isOverdue = dueDate < today;` then `if (isOverdue && !isPaid) {`
- Do not: `if (dueDate < today && !(paidAt ?? false)) {`

### C4.3 Do not leave out words

- Severity: warning
- Checks: `review`, `eslint:@typescript-eslint/no-magic-numbers`, `eslint:no-implicit-coercion`, `eslint:@typescript-eslint/explicit-module-boundary-types`, `ruff:PLR2004`, `ruff:FBT001`, `ruff:FBT003`, `ruff:ANN001`, `ruff:ANN201`, `golangci:mnd`, `revive:flag-parameter`, `clippy:fn_params_excessive_bools`
- Why: A magic number, an implicit coercion, a positional boolean, or a missing public type is a missing word.
- Do: `const MAX_RETRIES = 3;`, `Number(input)`, `send(email, { isUrgent: true })`
- Do not: `if (count > 3)`, `+input`, `send(email, true)`

### C4.4 Use positive names and conditions

- Severity: warning
- Checks: `ste_check`, `eslint:no-negated-condition`, `ruff:SIM208`
- Why: A negative name in a negative condition makes the reader do logic.
- Do: `isEnabled`, `if (isEnabled)`
- Do not: `isNotDisabled`, `if (!isNotDisabled)`

### C4.5 Put more than 3 arguments or conditions on separate lines, or give them names

- Severity: warning
- Checks: `review`
- Limit: code.list_items = 3
- Why: A vertical list shows the count and the sequence.
- Do: one condition on each line, or one named boolean for each condition
- Do not: five conditions on one line

## C5 Functions, files, and control flow (STE: Procedures)

### C5.1 Write one task per function, in 40 lines or fewer

- Severity: warning
- Checks: `ste_check`, `eslint:max-lines-per-function`, `revive:function-length`, `clippy:too_many_lines`, `ruff:PLR0915`
- Limit: code.function_lines = 40
- Why: A function is one procedure. A reader must see all of it at one time. Blank lines do not count.
- Do: `validateInvoice()`, `saveInvoice()`, `sendInvoiceEmail()`
- Do not: one 150-line `submitInvoice()`

### C5.2 Put the condition first

- Severity: warning
- Checks: `review`, `eslint:no-else-return`, `ruff:RET505`, `revive:early-return`, `revive:indent-error-flow`
- Why: A guard clause states the condition before the action, like "If the build fails, read the log."
- Do: `if (user === null) { return; }` at the top of the function
- Do not: the main path inside an `else` block at the end

### C5.3 Write the steps in execution order

- Severity: warning
- Checks: `review`
- Why: The reader reads from the top down. Declare each value near its first use.
- Do: read the input, then validate it, then save it
- Do not: a value that you declare 30 lines before you use it

### C5.4 Keep the nesting depth to 3 or less and the complexity to 10 or less

- Severity: warning
- Checks: `eslint:max-depth`, `eslint:complexity`, `ruff:C901`, `ruff:PLR0912`, `ruff:PLR1702`, `golangci:gocognit`, `revive:max-control-nesting`, `clippy:excessive_nesting`, `clippy:cognitive_complexity`
- Limit: code.nesting_depth = 3
- Limit: code.complexity = 10
- Why: Deep nesting is a sentence with too many clauses.
- Do: a guard clause and a named function for the inner loop
- Do not: an `if` in a `for` in an `if` in a `try`

### C5.5 Use 3 positional parameters or fewer

- Severity: warning
- Checks: `eslint:max-params`, `ruff:PLR0917`, `revive:argument-limit`, `clippy:too_many_arguments`
- Limit: code.positional_params = 3
- Why: The reader cannot remember the meaning of a fourth position. Use an options object or keyword arguments.
- Do: `createUser({ name, email, role, team })`
- Do not: `createUser(name, email, role, team)`

### C5.6 Make a script stop at the first error

- Severity: error
- Checks: `review`
- Why: A script that continues after an error does the next step on bad input.
- Do: `set -euo pipefail` at the top of a shell script
- Do not: a shell script that ignores the exit code of each command

### C5.7 Write one topic per source file, in 200 lines or fewer

- Severity: error
- Checks: `ste_check`, `eslint:max-lines`, `revive:file-length-limit`
- Limit: code.file_lines = 200
- Why: A file is a topic. A reader must be able to read one topic at one time. Blank lines do not count.
- Do: `invoice/parse.ts`, `invoice/validate.ts`, `invoice/save.ts`
- Do not: one 900-line `invoice.ts`

## C6 Self-explaining code (STE: Descriptive writing)

### C6.1 Write no comments

- Severity: error
- Checks: `ste_check`, `eslint:no-inline-comments`, `eslint:no-warning-comments`, `ruff:ERA001`
- Why: A comment is a second copy of the code in a different language. The two copies drift apart. Make the code explain itself.
- Do: `const isTrialExpired = trialEndsAt < now;`
- Do not: `// check if the trial is over` above `if (t < n)`

### C6.2 Write no doc comments and no docstrings

- Severity: error
- Checks: `ste_check`
- Why: The name, the parameters, and the types describe a function. Put usage guides in Markdown docs that obey the W rules.
- Do: `function parseInvoice(csv: string): Invoice`
- Do not: a JSDoc block, a Python docstring, a Go doc comment, or a Rust `///` line

### C6.3 Keep only machine directives

- Severity: error
- Checks: `ste_check`
- Why: A machine directive is an instruction to a tool, not prose for a reader. A shebang, a license identifier, and a tool pragma are machine directives.
- Do: `#!/usr/bin/env python3`, `# noqa: E501`, `//go:build linux`, `// eslint-disable-next-line no-console`
- Do not: `// HACK: this is temporary`

### C6.4 Replace a comment with code

- Severity: warning
- Checks: `review`
- Why: Each comment has a code form. Use a function name, a constant, a named boolean, or a type.
- Do: `const SESSION_TIMEOUT_MS = 30 * 60 * 1000;`
- Do not: `1800000 // 30 minutes`

### C6.5 Show every failure mode in the signature or in the error type

- Severity: warning
- Checks: `review`, `clippy:unwrap_used`
- Why: With no doc comments, the type must tell the reader how the function fails.
- Do: `fn parse(text: &str) -> Result<Invoice, ParseError>`, `findUser(): User | null`
- Do not: a function that panics on bad input and says so only in a comment

### C6.6 Write the reason for a change in the commit, the PR, or the docs

- Severity: warning
- Checks: `review`
- Why: The code says what it does. The history says why. A decision record says why for a large design.
- Do: a commit body that says why the retry limit is 3
- Do not: a comment that says why the retry limit is 3

## C7 Risk, errors, and logs (STE: Safety instructions)

### C7.1 Put the risk in the name of a destructive or unsafe operation

- Severity: error
- Checks: `review`
- Why: With no comments, the name is the warning. The reader must see the risk at the call site.
- Do: `deleteAllInvoices()`, `dangerouslySetHtml()`, `unsafeSkipVerify`
- Do not: `cleanup()` that deletes every invoice

### C7.2 Give a destructive operation a safe default and an explicit opt-in

- Severity: error
- Checks: `review`
- Why: The default must protect the reader who did not read the code.
- Do: `deleteBucket(name, { force: false })`, `isDryRun = true`
- Do not: a `deleteBucket(name)` that deletes every object with no confirmation

### C7.3 Write an error message that says what failed, why, and what to do next

- Severity: warning
- Checks: `ste_check`, `revive:error-strings`
- Why: The error message is the only text that the user of the code reads. Include the value that failed. Obey the W rules.
- Do: `Cannot read config file "app.toml": permission denied. Make the file readable by the current user.`
- Do not: `Error!`, `Something went wrong`, `Invalid input`

### C7.4 Write one event per log line, in the simple past

- Severity: warning
- Checks: `review`
- Why: A log line is a fact about the past. Use field names from LEXICON.md.
- Do: `Deleted expired sessions. count=42`
- Do not: `Deleting sessions...`, `Session cleanup in progress`

## C8 Formatting and counts (STE: Punctuation and word counts)

### C8.1 Let the formatter decide the layout

- Severity: warning
- Checks: `review`
- Why: Layout is not a choice for the writer. Use Prettier, ruff format, gofmt, or rustfmt.
- Do: run the formatter before each commit
- Do not: align code by hand

### C8.2 Count the words in a name at case changes and underscores

- Severity: warning
- Checks: `ste_check`
- Why: Every tool and every reader must count the same way. An acronym is one word, and a number is one word.
- Do: count `parseHTTPResponse` as 3 words, and `MAX_RETRY_COUNT` as 3 words
- Do not: count `HTTP` as 4 words

### C8.3 Take every limit from the standard

- Severity: warning
- Checks: `review`
- Why: One limit in one place cannot drift. The build script copies each limit into the linter configs.
- Do: change a limit in the rule file, then run the build script
- Do not: change a limit in one linter config only

### C8.4 Write acronyms in the case that the profile sets

- Severity: warning
- Checks: `review`, `revive:var-naming`
- Why: Each language has one convention. Use it everywhere.
- Do: `HttpClient` in TypeScript and Rust, `HTTPClient` in Go
- Do not: `HttpClient` and `HTTPClient` in one codebase

## C9 Consistency and change (STE: Writing practices)

### C9.1 Keep a LEXICON.md file

- Severity: warning
- Checks: `review`
- Why: The lexicon is the project dictionary. It holds the technical nouns, the technical verbs, and the local words to avoid.
- Do: one row for each domain word, with one meaning
- Do not: domain words that only the original writer knows

### C9.2 Follow the existing convention of the project

- Severity: warning
- Checks: `review`
- Why: Consistency is more important than preference. Record each project convention in LEXICON.md.
- Do: use `fetch` if the codebase uses `fetch` for network calls
- Do not: add `retrieve` next to 200 uses of `fetch`

### C9.3 When a meaning changes, rename the name everywhere

- Severity: warning
- Checks: `review`
- Why: A name that no longer matches its meaning is a wrong instruction.
- Do: rename `user` to `account` in every file when the concept changes
- Do not: keep `user` for a value that is now an account

### C9.4 Write a commit subject in the imperative, in 50 characters or fewer

- Severity: error
- Checks: `ste_check`
- Limit: commit.subject_chars = 50
- Limit: commit.subject_hard_chars = 72
- Limit: commit.body_line_chars = 72
- Why: The subject completes the phrase "If you apply this commit, it will". Do not end it with a period. Keep each line to 72 characters or fewer. Put the reason in the body, and make one change per commit.
- Do: `Add retry to the invoice sender`
- Do not: `Fixed stuff.`

### C9.5 Write PR descriptions and agent handoffs with the W rules

- Severity: warning
- Checks: `review`
- Why: A PR description and an agent prompt are instructions to a reader with no context.
- Do: a PR description with a list of changes and the reason for each
- Do not: a PR description that says "see commits"
