# 002: Distinguish Coding Loop from Cultivation Loop

**Date**: 2026-04-08

## Decision

Identify two distinct loops in a project's development: the coding loop (human + AI tool, within sessions, execution-oriented) and the cultivation loop (cross-session, observation-oriented, knowledge-sedimentation). anima does not provide the coding loop. The spirit lives in the cultivation loop, which no current tool provides.

## Why

Without this distinction, anima's scope was unclear. Early discussion conflated anima with agent execution platforms like openclaw/pi-mono, which solve coding loop reliability. anima solves a different problem: ensuring the coding loop's outputs compound into project wisdom rather than evaporating between sessions. The distinction clarifies that anima is complementary to, not competitive with, existing AI coding tools.

## Alternatives considered

- **Treat both loops as one.** Muddles anima's scope — it would seem like anima needs to be an execution engine.
- **Ignore the cultivation loop and focus only on the seed.** Honest for the seed phase, but leaves the memory paradox without a resolution path.
