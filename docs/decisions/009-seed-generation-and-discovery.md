# Seed generation versioning and agent discovery

Two related problems required a joint decision:

1. **Agent discovery**: The seed's `AGENTS.md` did not mention `anima check`,
   so agents working in seeded projects had no way to know the command existed.
   anima's commands (except `init`) are designed to be agent-facing, but the
   agent's only entry point is `AGENTS.md`.

2. **Seed evolution**: When the seed gains new capabilities, projects already
   seeded with an older version have no upgrade path.

## Decisions

### Discovery: Add `anima check` to the Cultivation section

A single line at the top of the Cultivation section:

    Run `anima check` to see which areas of the harness need attention.

This is the natural position — the agent reads Cultivation to learn how to
behave, and "sense the project's state" is a behavior. It precedes the four
directives, creating a flow: sense first, then act.

### Evolution: Seed generations + `anima check` as upgrade advisor

- Seeds carry a generation marker: `<!-- anima:seed:N -->` at the end of
  `AGENTS.md`. No marker = generation 1.
- `anima check` detects the planted generation and, if behind current, outputs
  specific suggestions for what to add.
- The agent reads the suggestions and applies them. No `anima upgrade` command.

## Why not `anima upgrade`

The seed files are living documents — agents and humans modify them after
planting. An automated upgrade risks overwriting growth. Agents are fully
capable of applying textual suggestions to a file. This approach is also
consistent with the cultivation philosophy: observe and suggest, don't force.

## Why generation integers, not semver

Seed changes happen at the granularity of "one meaningful capability addition."
There is no concept of breaking vs. non-breaking for a seed. A simple integer
(v1, v2, v3...) suffices.

## Alternatives considered

- **`anima upgrade` command**: Rejected for v2 — risks overwriting growth,
  adds complexity. Can be reconsidered if the observation-based approach
  proves insufficient.
- **Heuristic detection** (checking if AGENTS.md mentions "anima check"):
  Fragile and doesn't scale. Explicit markers are more reliable.
- **No versioning**: Would leave existing projects permanently behind with
  no feedback mechanism.
