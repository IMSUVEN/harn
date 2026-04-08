# Decisions

Record significant technical decisions as files in this directory.

## When to record

A decision is worth recording when:

- Reversing it later would be costly
- The reasoning is non-obvious and future sessions need to understand why
- Multiple alternatives were seriously considered

Not every choice needs a record. "We used TypeScript" might be worth recording
if the alternative was seriously considered. "We named this variable `count`"
is not.

## Format

One file per decision. Name: `NNN-short-title.md` (e.g., `001-use-typescript.md`).

Each record should capture what was decided, why, and what alternatives were
considered. There is no rigid template — write what a future reader needs to
understand the decision. A few sentences often suffice; an essay is rarely
necessary.
