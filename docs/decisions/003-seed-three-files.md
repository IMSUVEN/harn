# 003: Seed Produces Exactly Three Files

**Date**: 2026-04-08

## Decision

The seed (`anima init`) produces three files and nothing else:

- `AGENTS.md` — agent entry point with cultivation protocol
- `docs/ARCHITECTURE.md` — empty shell for future architecture
- `docs/decisions/README.md` — convention for recording decisions

## Why

Each file maps to a specific philosophy requirement: AGENTS.md provides the agent entry point and growth mechanisms (§4); the decisions directory provides the knowledge sedimentation skeleton (§4); ARCHITECTURE.md provides an exploration destination (§4). Everything else was excluded because it would either prescribe a technology stack, add placeholder content, or create empty infrastructure with no current value.

## Alternatives considered

- **More files** (README.md, .anima/ directory, CONTRIBUTING.md, growth log). Each was evaluated and excluded: README is project-specific (seed doesn't know what the project is); .anima/ would be empty infrastructure; others are premature.
- **Fewer files** (just AGENTS.md). Misses the structural gravity that empty directories provide — without `docs/decisions/`, there's no natural place to record decisions.
