# Calibration Examples: API Ergonomics

These examples illustrate what grade A, C, and F CLI interfaces look like for the "API Ergonomics" criterion. Use them to calibrate your grading.

All examples implement the same concept: a command that completes an execution plan.

---

## Grade A — Clear feedback, actionable errors, discoverability

From `harn plan complete` (actual harn behavior):

```
$ harn plan complete "auth feature"

Plan "auth-feature" completed. Moved to docs/exec-plans/completed/2026-04-03-auth-feature.md
```

On error:

```
$ harn plan complete "nonexistent"
Error: No plan found matching "nonexistent" in docs/exec-plans/active
Run `harn plan list` to see available plans.
```

With ambiguous input:

```
$ harn plan complete "auth"
Error: Multiple plans match "auth": 2026-04-03-auth-feature.md, 2026-04-01-auth-refactor.md
Be more specific.
```

**Why A**: Output confirms what happened and where. Error messages name the problem, show the user's input, and tell them exactly what to do next. Ambiguous input gets a specific disambiguation message rather than silently picking the first match.

---

## Grade C — Works but requires documentation

```
$ mytools plan-complete auth-feature

Done.
```

On error:

```
$ mytools plan-complete nonexistent
Error: plan not found
```

**Why C**: The success message doesn't say what was moved or where. The error says what failed but not how to recover. The user has to guess the right plan name or read documentation to find the list command.

---

## Grade F — Unusable without reading source code

```
$ mytools pc af
Error: ENOENT
```

**Why F**: Abbreviated command names (`pc`) are not discoverable. Abbreviated arguments (`af`) suggest the tool expects users to memorize short codes. The error is a raw system error with no context. No `--help`, no suggestions, no indication of valid inputs.
