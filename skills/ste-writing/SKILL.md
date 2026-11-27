---
name: ste-writing
description: Write or rewrite prose with the ASD-STE100 Simplified Technical English rules, so that no reader can misread it. Use it for text that a person or an agent acts on, such as docs, READMEs, commit messages, PR descriptions, error messages, log lines, runbooks, release notes, and prompts or handoffs to other agents. Also use it when the user says "simplify this", "make this clear", "STE", "Simplified Technical English", or "ASD-STE100". Do not use it for marketing copy, persuasive text, or quoted text.
license: MIT
---

# STE writing

ASD-STE100 is the controlled language of aircraft maintenance manuals.
It removes the two main causes of a misread instruction.
These causes are a word with more than one meaning, and a sentence with more than one possible structure.

Agent prose has the same problems.
It has long sentences, stacked hedges, passive verbs, and synonyms that make one action look like three.
The reader has no author to ask.

Apply the rules while you write.
Do not write a long draft and correct it later.

## When to use this skill

Use it for text that someone acts on:

- Docs, READMEs, runbooks, and release notes.
- Commit messages and PR descriptions.
- Error messages, log lines, and the text of a user interface.
- Prompts, plans, and handoffs to other agents.

Do not use it for marketing copy or persuasive text.
Do not change quoted text. Quote a log, a user, or a document exactly (W9.4).

## The rules

| Rule | Do | Do not |
|---|---|---|
| W1.1 One word for one meaning | Use one verb for one action, every time | "verify", then "confirm", then "check" for one action |
| W1.4 Simple words | "use", "help", "before" | "leverage", "facilitate", "prior to" |
| W2.1 Noun clusters | 3 nouns or fewer: "the retry delay" | "the queue worker retry delay setting" |
| W3.1 Simple verb forms | "The job failed." | "The job has been failing." |
| W3.3 Active voice | "The worker deletes the file." | "The file is deleted." |
| W4.1 One idea | "Open the file. Read line 3." | "Open the file and read line 3, then compare it." |
| W4.2 No missing words | "The importer skips each row that has no match." | "Rows not matched skipped." |
| W4.3 Hedges | 1 hedge or fewer in each sentence | "It may possibly be..." |
| W5.1 Instructions | 20 words or fewer, in the imperative | A 35-word instruction |
| W5.4 Condition first | "If the build fails, read the log." | "Read the log if the build fails." |
| W6.1 Descriptions | 25 words or fewer, with the main fact first | A 40-word sentence |
| W6.3 Paragraphs | One topic, 6 sentences or fewer | Three topics in one paragraph |
| W7.1 Risk markers | "WARNING: Back up the database. The migration deletes the old table." | "Important: this might be risky." |
| W8.2 Joiners | Two sentences | A semicolon or a dash between two ideas |
| W9.3 Precision | Keep each fact, number, and condition | A short sentence that drops a condition |

The full rules are in `references/rules.md`.
The list of words to avoid is in `references/words.md`.
The worked examples are in `references/examples.md`.

## How to rewrite a text

1. Read the full text once. Find what it must still say after the rewrite.
2. Go through the text one sentence at a time. Name the rule that each sentence breaks.
3. Rewrite the sentence to obey that rule. Keep each fact, number, condition, and qualifier.
4. Read the result against the rules again. A rewrite can make a new long sentence.
5. If a shorter sentence drops a fact, keep the longer sentence. Say why in one line.

If the text already obeys the rules, say so. Do not change it (W9.5).

## Formats for common texts

- **Commit message.** Write the subject in the imperative, in 50 characters or fewer, with no period. Put a blank line next. Then say why in the body.
- **Error message.** Say what failed, why, and what to do next. Include the value that failed.
- **Log line.** Write one event in the simple past: "Deleted 3 expired sessions."
- **Agent handoff.** Write the goal first. Then write numbered steps, the limits, and the expected output.

## Self-check

Before you send the text, read it once and answer these questions:

- Does an instruction have more than 20 words, or a description more than 25?
- Does a sentence hold two instructions or two ideas?
- Does one action have two different verbs?
- Can a passive sentence name its actor?
- Does a sentence have two or more hedges?
- Can a reader with no context know what to do next?

## Tool

If the `ste` command is installed, run the checker on Markdown files:

```sh
ste check prose README.md docs
```

The checker finds length, voice, hedges, and words to avoid. It cannot find a word with two meanings. You must find that.
