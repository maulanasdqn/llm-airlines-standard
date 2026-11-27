# The standard

This folder holds two rule sets and their data.

| File | Contents |
|---|---|
| `writing/rules.md` | The W rules for prose |
| `code/rules.md` | The C rules for code (STE-C) |
| `dictionary.json` | The approved verbs, the words to avoid, the abbreviations, and the profile data |
| `code/profiles/` | The idioms of TypeScript, Python, Go, and Rust |
| `writing/examples.md`, `code/examples.md` | Before and after examples |
| `code/mapping.md` | The map from each STE section to the rules and the tools (generated) |
| `code/dictionary.md`, `writing/words.md`, `limits.json` | Generated views of the data |

## Where the rules come from

ASD-STE100 is the controlled language that aerospace manuals use.
Issue 9 of the standard is from January 2025.
It has 53 writing rules in 9 sections, and a dictionary of approved words.
Each approved word has one meaning and one part of speech.

The W rules paraphrase the intent of those 9 sections for the prose that agents write.
The C rules port the same 9 sections to source code:

| Section | Prose (W) | Code (C) |
|---|---|---|
| 1 | Words | Identifiers |
| 2 | Noun clusters | Compound names |
| 3 | Verbs | Functions |
| 4 | Sentences | Statements and expressions |
| 5 | Procedures | Functions, files, and control flow |
| 6 | Descriptive writing | Self-explaining code |
| 7 | Safety instructions | Risk, errors, and logs |
| 8 | Punctuation and word counts | Formatting and counts |
| 9 | Writing practices | Consistency and change |

The rule numbers are our own. They do not match the numbers in ASD-STE100.
To get the exact text of the standard, go to https://www.asd-ste100.org/.

## Two decisions in STE-C

- **No comments.** ASD-STE100 makes a manual clear with controlled words. STE-C makes code clear with controlled names. A comment is a second copy of the code that drifts, so STE-C forbids comments and docstrings.
- **Small units.** An STE instruction has 20 words or fewer, and a paragraph holds one topic. In STE-C, a function has 40 lines or fewer, and a file holds one topic in 200 lines of code or fewer.
