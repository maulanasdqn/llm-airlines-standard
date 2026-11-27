# AGENTS.md for linters

This folder holds the linter configs of the standard.

- Edit the templates in `src/`. The build writes the configs in the other folders.
- Do not edit a generated config. Change the template or the limit in `spec/`, then run the build.
- Use a placeholder for each limit and word list, such as `{{ limits.code.nesting_depth }}`.
- Write no comments in a config file.

CAUTION: Check each rule ID and each option against the current documentation of the linter. Do not guess a name. A wrong name can turn off a rule with no error.

After a change, run the linters on the fixtures:

```sh
bash tools/verify_linters.sh
```
