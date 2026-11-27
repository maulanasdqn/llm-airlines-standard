# Writing examples

We wrote every example on this page to show a rule.
No example comes from a real report, ticket, or conversation.

## Single sentences

| Rule | Before | After |
|---|---|---|
| W1.1 | "Verify the token, then confirm the scope, then check the expiry." | "Check the token, the scope, and the expiry." |
| W1.4 | "Leverage the cache in order to reduce latency." | "Use the cache to reduce latency." |
| W3.1 | "We have received 40 reports since Monday." | "We received 40 reports since Monday." |
| W3.3 | "The stale entry is removed on the next write." | "The next write removes the stale entry." |
| W3.4 | "Perform a restart of the worker." | "Restart the worker." |
| W4.2 | "Rows not matched skipped." | "The importer skips each row that has no match." |
| W4.3 | "This might possibly be caused by a bad token." | "A bad token is the probable cause." |
| W5.4 | "Read the log if the deploy fails." | "If the deploy fails, read the log." |

## Error message

Before:

```text
An error may have occurred while the server was processing your request, which could possibly be due to a malformed payload.
```

Breaks: W3.1 ("may have occurred"), W3.2 ("was processing"), W4.1 (three ideas in 21 words), W4.3 (three hedges).

After:

```text
The server rejected the request. The JSON body is not valid. Correct the syntax at line 4.
```

## Instruction to an agent

Before:

```text
Once the migration has completed and assuming no errors were raised, you should go ahead and run the backfill, though it's worth keeping in mind that it can time out on large tables.
```

Breaks: W3.1 ("has completed"), W3.3 ("were raised"), W5.1 (33 words), W5.2 (two instructions).

After:

> 1. Wait for the migration to finish with no errors.
> 2. Run the backfill.
>
> CAUTION: Run the backfill on one table at a time. It can time out on a table with more than 10 million rows.

## Commit message

Before:

```text
Fixed stuff in the parser.
```

Breaks: C9.4 (past tense, a period at the end, a vague word).

After:

```text
Reject a CSV row with an empty total

The importer saved a total of 0 for an empty cell. Finance reports
then showed paid invoices as free. Now the parser returns an error
for the row, and the importer skips it.
```

## What not to simplify

Keep the longer sentence when the shorter one loses a fact (W9.3):

- A condition: "Delete the backup only after the restore test passes."
- A scope: "This applies only to accounts on the legacy plan."
- A number, a date, or a unit.

If you keep a long sentence, say why in one line.
