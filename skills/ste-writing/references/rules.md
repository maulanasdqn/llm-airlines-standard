# STE writing rules (W)

These rules apply to prose that a person or an agent must act on.
That prose includes docs, commit messages, PR descriptions, error messages, log lines, and agent prompts.

The 9 sections follow the 9 sections of ASD-STE100.
The rule numbers are the numbers of this project.
They do not match the rule numbers in ASD-STE100.
Each rule paraphrases the intent of the standard, and every example is our own.
For the exact text, get the standard from asd-ste100.org.

Each rule has these fields:

- Severity: what the checker reports when it finds a break.
- Checks: `the tools that find a break. "review" means the review skill`, `which is a person or an agent.`
- Limit: a number that tools read. The build script copies it to the linter configs.

## W1 Words

### W1.1 Use one word for one meaning

- Severity: warning
- Checks: `review`
- Why: When a reader sees two words, the reader thinks that there are two things.
- Do: "Check the flag. Then check the rollout."
- Do not: "Verify the flag. Then confirm the rollout."

### W1.2 Give a word only one meaning

- Severity: warning
- Checks: `review`
- Why: A word with two meanings makes the reader stop and guess.
- Do: "Select the box. Then save the file."
- Do not: "Check the box. Then check the file."

### W1.3 Use a word as one part of speech only

- Severity: warning
- Checks: `review`
- Why: A noun that acts as a verb hides the action.
- Do: "Put a lock on the branch."
- Do not: "Lock-protect the branch."

### W1.4 Use simple words, and do not use the words in the words-to-avoid list

- Severity: warning
- Checks: `ste_check`, `vale:STE.WordsToAvoid`
- Why: A long word or a filler word adds work for the reader and gives no information.
- Do: "Use the cache."
- Do not: "Leverage the cache."

### W1.5 Define a technical noun or a technical verb before you use it

- Severity: warning
- Checks: `review`
- Why: The reader cannot ask you what a project term means.
- Do: "A shard is one part of the index. Each shard holds 1 GB."
- Do not: "Rebalance the shards."

### W1.6 Do not use idioms or slang

- Severity: warning
- Checks: `ste_check`, `vale:STE.WordsToAvoid`
- Why: An idiom has a meaning that its words do not show. Many readers do not know it.
- Do: "The tool works with no setup."
- Do not: "The tool works out of the box."

### W1.7 Use one spelling convention

- Severity: warning
- Checks: `review`
- Why: Two spellings of one word look like two words. This project uses American English.
- Do: "color", "license", "canceled"
- Do not: "colour" and "color" in one document

### W1.8 Write an abbreviation in full the first time you use it

- Severity: warning
- Checks: `review`
- Why: An abbreviation that the reader does not know is a word the reader cannot read.
- Do: "Time to live (TTL) sets how long the cache keeps an entry."
- Do not: "Set the TTL."

## W2 Noun clusters

### W2.1 Use 3 nouns or fewer in a noun cluster

- Severity: warning
- Checks: `review`
- Limit: prose.noun_cluster_words = 3
- Why: In a long noun cluster, the reader cannot see which noun is the subject.
- Do: "the retry delay"
- Do not: "the queue worker retry delay setting"

### W2.2 Break a long noun cluster with a preposition or a clause

- Severity: warning
- Checks: `review`
- Why: A preposition shows how the nouns relate.
- Do: "the setting that controls the retry delay of the queue worker"
- Do not: "the queue worker retry delay setting"

### W2.3 Use a hyphen to join words that act as one modifier

- Severity: warning
- Checks: `review`
- Why: The hyphen shows which words go together.
- Do: "a read-only file"
- Do not: "a read only file"

## W3 Verbs

### W3.1 Use only simple verb forms

- Severity: warning
- Checks: `ste_check`, `vale:STE.PresentPerfect`
- Why: The imperative, the simple present, the simple past, and the simple future are easy to read. Compound forms are not.
- Do: "The job failed at 10:00."
- Do not: "The job has been failing since 10:00."

### W3.2 Do not use "-ing" verb forms

- Severity: warning
- Checks: `ste_check`, `vale:STE.Progressive`
- Why: An "-ing" form can be a verb, a noun, or an adjective. The reader must guess which.
- Do: "The server sends the report."
- Do not: "The server is sending the report."

### W3.3 Use the active voice

- Severity: warning
- Checks: `ste_check`, `vale:STE.Passive`
- Why: The active voice names the actor. Use the passive only in a description, and only when the actor is unknown or does not matter.
- Do: "The worker deletes the file."
- Do not: "The file is deleted."

### W3.4 Use a verb for an action, not a noun phrase

- Severity: warning
- Checks: `ste_check`, `vale:STE.WordsToAvoid`
- Why: A noun phrase hides the action and adds words.
- Do: "Install the package."
- Do not: "Perform an installation of the package."

## W4 Sentences

### W4.1 Write one idea per sentence

- Severity: warning
- Checks: `review`
- Why: A reader can act on one idea at a time.
- Do: "Open the file. Read line 3."
- Do not: "Open the file and read line 3, then compare it with the log."

### W4.2 Do not leave out words to make a sentence shorter

- Severity: warning
- Checks: `review`
- Why: A missing subject, verb, or article makes the sentence ambiguous. Short is not always clear.
- Do: "The worker drops each session that the sampler did not select."
- Do not: "Sessions not sampled dropped."

### W4.3 Use 1 hedge or fewer in each sentence

- Severity: warning
- Checks: `ste_check`, `vale:STE.Hedges`
- Limit: prose.hedges_per_sentence = 1
- Why: Stacked hedges hide the fact. State a fact as a fact, and state doubt once.
- Do: "The client is probably out of date."
- Do not: "It may possibly be that the client could be out of date."

### W4.4 Use a vertical list for 3 or more steps, items, or conditions

- Severity: warning
- Checks: `review`
- Why: A list shows the sequence and the count. A long sentence hides them.
- Do: a numbered list with one step on each line
- Do not: "Install it, then configure it, then start it, and then test it."

### W4.5 Write positive statements

- Severity: warning
- Checks: `review`
- Why: A double negative makes the reader do logic before the reader can act.
- Do: "Keep the lock."
- Do not: "Do not fail to keep the lock."

## W5 Procedures

### W5.1 Write an instruction in 20 words or fewer

- Severity: error
- Checks: `ste_check`
- Limit: prose.instruction_words = 20
- Why: A reader who does a task reads one instruction, acts, and then reads the next one.
- Do: "Stop the service before you change the config file."
- Do not: an instruction with three clauses and a list of exceptions

### W5.2 Write one instruction per sentence

- Severity: warning
- Checks: `review`
- Why: Two instructions in one sentence make the reader miss the second one. Combine two actions only when they occur at the same time.
- Do: "Open the valve. Then start the pump."
- Do not: "Open the valve and start the pump and then read the gauge."

### W5.3 Use the imperative form for an instruction

- Severity: warning
- Checks: `review`
- Why: The imperative tells the reader who acts: the reader.
- Do: "Run the tests."
- Do not: "The tests should be run."

### W5.4 Put a condition before the instruction

- Severity: warning
- Checks: `review`
- Why: The reader must know the condition before the reader acts.
- Do: "If the build fails, read the log."
- Do not: "Read the log if the build fails."

### W5.5 Write the steps in the sequence the reader does them, and number them

- Severity: warning
- Checks: `review`
- Why: A reader who follows steps does not go back.
- Do: "1. Stop the service. 2. Back up the database. 3. Run the migration."
- Do not: "Run the migration, but first back up the database."

### W5.6 Keep notes out of the steps

- Severity: warning
- Checks: `review`
- Why: A note inside a step looks like an instruction.
- Do: a step, then a line that starts with "NOTE:"
- Do not: a step with a note in parentheses in the middle

## W6 Descriptive writing

### W6.1 Write a descriptive sentence in 25 words or fewer

- Severity: error
- Checks: `ste_check`, `vale:STE.SentenceLength`
- Limit: prose.description_words = 25
- Why: A long sentence makes the reader hold too many facts at the same time.
- Do: "The cache keeps each entry for 10 minutes. After that, the next read gets a new value."
- Do not: a 40-word sentence with three clauses

### W6.2 Put the most important information first

- Severity: warning
- Checks: `review`
- Why: Many readers stop after the first sentence.
- Do: "The import failed. The CSV file has no header row."
- Do not: "After we parsed the CSV file and looked at row 1, we found that the import failed."

### W6.3 Write one topic per paragraph, in 6 sentences or fewer

- Severity: warning
- Checks: `ste_check`, `vale:STE.ParagraphLength`
- Limit: prose.paragraph_sentences = 6
- Why: A paragraph is one unit of information. A reader can skip it or read it.
- Do: a paragraph about the cache, then a new paragraph about the queue
- Do not: one paragraph about the cache, the queue, and the deploy

### W6.4 Use headings, lists, and tables to show structure

- Severity: warning
- Checks: `review`
- Why: Structure lets the reader find information without reading every word.
- Do: a table that compares three options
- Do not: three paragraphs that compare three options

## W7 Safety instructions

### W7.1 Use the standard markers for risk

- Severity: warning
- Checks: `ste_check`, `vale:STE.Markers`
- Why: A reader must know the type of risk before reading the detail. Use WARNING for a risk to people, data, security, or production. Use CAUTION for a risk to a system or to work. Use NOTE for information only.
- Do: "WARNING: Back up the database before you run this migration."
- Do not: "Important: this migration might be risky."

### W7.2 Start a warning or a caution with a clear command

- Severity: warning
- Checks: `ste_check`
- Why: The command is the part that prevents the damage.
- Do: "CAUTION: Do not run two deploys at the same time."
- Do not: "CAUTION: It is not a good idea to run two deploys at the same time."

### W7.3 Give the risk after the command

- Severity: warning
- Checks: `review`
- Why: The reader who knows the risk obeys the command.
- Do: "WARNING: Do not delete this bucket. It holds the only copy of the audit log."
- Do not: "WARNING: Do not delete this bucket."

### W7.4 Put a warning or a caution before the step that it applies to

- Severity: warning
- Checks: `review`
- Why: A warning after the step comes too late.
- Do: the warning, then the step
- Do not: the step, then the warning

## W8 Punctuation and word counts

### W8.1 Count words the same way every time

- Severity: warning
- Checks: `ste_check`
- Why: A limit is useful only when everyone counts the same way. A number, a code span, a file name, and a hyphenated word each count as one word.
- Do: count "Run `make test` on the read-only volume" as 6 words
- Do not: count the code span as two words

### W8.2 Do not join sentences with a semicolon or a dash

- Severity: warning
- Checks: `ste_check`, `vale:STE.Joiners`
- Why: A semicolon or a dash puts two ideas in one sentence.
- Do: "The job failed. The disk is full."
- Do not: "The job failed; the disk is full."

### W8.3 End each sentence with a period, and use a colon before a list

- Severity: warning
- Checks: `review`
- Why: Regular punctuation shows where each sentence stops.
- Do: "Install these packages:" followed by a list
- Do not: "Install these packages" followed by a list

## W9 Writing practices

### W9.1 Use the same term, format, and structure for the same thing

- Severity: warning
- Checks: `review`
- Why: Consistency lets the reader learn a pattern once.
- Do: every error message in the form "what failed, why, what to do"
- Do not: a different form in each error message

### W9.2 Write for a reader who has no context

- Severity: warning
- Checks: `review`
- Why: The reader joins in the middle and has no author to ask.
- Do: "The nightly backup job failed on 12 May."
- Do not: "It failed again."

### W9.3 Keep every fact, number, condition, and qualifier when you simplify

- Severity: error
- Checks: `review`
- Why: Precision is more important than brevity. If a shorter sentence drops a fact, keep the longer sentence.
- Do: "Delete the row only after the export finishes."
- Do not: "Delete the row."

### W9.4 Do not change quoted text

- Severity: warning
- Checks: `ste_check`
- Why: A quote must show what the source said. The checker skips text in quotes and in block quotes.
- Do: quote a log line exactly
- Do not: rewrite a log line inside quotes

### W9.5 Do not change text that already obeys the rules

- Severity: warning
- Checks: `review`
- Why: A change with no reason costs review time and can add errors.
- Do: report "No break found" and stop
- Do not: rewrite a correct sentence in a different style
