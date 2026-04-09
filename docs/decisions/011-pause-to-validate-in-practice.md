# Pause feature development to validate in practice

With `anima init`, `anima check`, seed generation versioning, tests, and CI
all in place, the decision is to stop building new features and enter a
real-world validation phase.

## Why

The tool's core loop is complete: plant a seed, observe its growth, detect
evolution. But all validation so far has been short-term and simulated. The
critical question — does the cultivation protocol actually change agent
behavior over the life of a real project? — can only be answered by using
anima in real, sustained development.

Building more features without this answer risks investing in the wrong
direction. The next meaningful feature should be informed by real friction
points, not speculation.

## What this means

- No new commands or seed changes until anima has been used to seed at least
  one real project and observed over multiple development sessions
- The project remains in its "rooting" phase
- Feedback from real usage determines what comes next
- Bug fixes and minor improvements are still fine

## What we're watching for

- Does the agent read and act on the cultivation protocol without human
  prompting?
- Does `anima check` output actually influence agent behavior at session start?
- Does the seed generation upgrade suggestion work when encountered by an
  agent in the wild?
- What needs does the seed fail to address?
- What emerges naturally that the seed didn't anticipate?
