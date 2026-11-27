# AGENTS.md for spec

This folder is the single source of the standard.
The build script reads it and writes the skills, the templates, and the linter configs.

## Files

| File | Source or generated |
|---|---|
| `writing/rules.md`, `code/rules.md` | Source. The build reads the rule IDs, the severities, the checks, and the limits from these files. |
| `dictionary.json` | Source. The verbs, the words to avoid, the abbreviations, and the profile data. |
| `code/profiles/*.md`, `*/examples.md`, `README.md` | Source |
| `limits.json`, `code/dictionary.md`, `code/mapping.md`, `writing/words.md` | Generated. Do not edit them. |

## Rule format

Each rule has a heading and a list of fields:

```markdown
### C5.7 Write one topic per source file, in 200 lines or fewer

- Severity: error
- Checks: `ste`, `eslint:max-lines`
- Limit: code.file_lines = 200
- Why: ...
- Do: ...
- Do not: ...
```

- Do not renumber a rule, and do not use an old ID again. Add a new rule at the end of its section.
- Write each limit value in the title or in the "Why" field. The build stops if the text does not show the value.
- Name each tool in "Checks" with the form `tool:rule-id`, in backticks. Use `review` for a rule that only a person or an agent can check.
- Put each example in quotes or in backticks. The checker does not read quoted text.

## Dictionary

- Give each verb one meaning and a "not for" entry that names the verb to use instead.
- Do not put a word in the approved list and in a words-to-avoid list.

WARNING: Do not copy text from ASD-STE100 or its dictionary. Paraphrase the intent, and write your own examples.
