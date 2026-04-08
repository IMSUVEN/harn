# Architecture

anima is currently a documentation-only project with no runtime code. Its
"architecture" is the relationship between its documents and the seed it
produces.

## Document Layers

```
  Philosophy (anima-specific)
       |
       | interprets, does not override
       |
  Spec + Guide (discipline-level)
```

The **Harness Specification** and **Harness Guide** describe harness engineering
as a discipline — the general theory. They adopt a control perspective. The
**Philosophy** reinterprets the same axioms through a cultivation lens. This is
a one-way dependency: the Philosophy builds on the Spec/Guide but the Spec/Guide
are independent of anima.

## The Seed

```
  Philosophy §4 (theory of what a seed should be)
       |
       | realized by
       |
  seed/ (concrete files anima init produces)
```

The `seed/` directory contains the exact files that `anima init` will plant into
a user's project. The only parameterized value is `{project-name}` in
`seed/AGENTS.md`. The seed implements Philosophy §4 and addresses the memory
paradox identified in §6.2 through its cultivation protocol.

## Translations

Every document in `docs/` has a Chinese translation (`*.zh-CN.md`) alongside.
English is written first; Chinese is translated to match (信达雅 standard).
Translations are maintained in parallel — a change to the English version
requires a corresponding change to the Chinese version in the same commit.

## What Does Not Exist Yet

- **CLI tool**: No `anima init` command exists. The seed is designed but not
  deliverable. Technology stack for anima-the-tool has not been chosen.
- **Spirit infrastructure**: No `.anima/` directory, no persistent service, no
  ecosystem signal processing. These belong to the awakening phase (§6.4).
