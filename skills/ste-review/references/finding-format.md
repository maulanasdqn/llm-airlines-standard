# Finding format

Report each finding on one line in this format:

```text
path:line RULE-ID severity: what breaks the rule. Fix: what to write instead.
```

Examples:

```text
docs/deploy.md:14 W5.1 error: The instruction has 31 words. Fix: Split it into three numbered steps.
src/billing/invoice.ts:42 C3.2 error: getInvoice sends an HTTP request. Fix: Rename it to fetchInvoice.
src/billing/invoice.ts:88 C6.1 error: A comment explains the retry limit. Fix: Name the constant MAX_SEND_RETRIES and put the reason in the commit.
```

## Order

1. Put errors first, then warnings, then info.
2. In each severity, sort by file and then by line.
3. Put the findings of the checker and the findings of the review in one list.

## Summary

After the list, write one line with the count for each severity:

```text
3 errors, 5 warnings, 1 info.
```

If there are no findings, write "No break found." Do not add a list of praise.
