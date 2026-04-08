# anima

> Plant seeds, not templates. Cultivate agents, don't configure them.

## Project State

**Phase: Germination.** The intellectual foundation is established. No implementation exists.

anima will be a tool that plants growth-capable seeds into new projects — minimal structures from which a harness grows through practice, not templates that prescribe one. The guiding documents are written and stable. The next step is determining what `anima init` concretely produces.

## Documents

The Philosophy defines anima's identity. The Spec and Guide provide the underlying discipline theory — written from a control perspective that the Philosophy deliberately reinterprets through a cultivation lens.

| File | Role |
|---|---|
| [PHILOSOPHY.md](docs/PHILOSOPHY.md) | anima's product philosophy: cultivation over control |
| [HARNESS-SPEC.md](docs/HARNESS-SPEC.md) | Prescriptive spec for harness engineering (discipline-level) |
| [HARNESS-GUIDE.md](docs/HARNESS-GUIDE.md) | Reasoning guide for harness design (discipline-level) |

Each document has a Chinese translation (`*.zh-CN.md`) alongside.

## Structure

```
AGENTS.md              # you are here
README.md              # public-facing description
docs/
  PHILOSOPHY.md        # product philosophy (anima-specific)
  HARNESS-SPEC.md      # prescriptive spec (discipline-level)
  HARNESS-GUIDE.md     # reasoning guide (discipline-level)
  *.zh-CN.md           # Chinese translations
```

## Conventions

- English is the primary language; Chinese translations maintained for all documents
- English written first; Chinese translated to match (信达雅 standard)
- All project knowledge lives in the repository — if it's not in the repo, it doesn't exist
- Guiding documents live in `docs/`
- The Philosophy interprets the Spec/Guide but does not override them
