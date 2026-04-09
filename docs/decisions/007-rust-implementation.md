# 007: Implement anima in Rust

**Date**: 2026-04-08

## Decision

Use Rust as the implementation language for the anima CLI tool.

## Why

anima's primary user is the AI coding agent, not the human. This inverts the
usual CLI design priorities:

- **Startup speed matters.** Agents may call `anima check` at the start of
  every session. Node.js/Python add 100-200ms of runtime startup overhead.
  A Rust binary starts in microseconds. Negligible for humans; meaningful
  friction for agent workflows.

- **Single binary distribution.** No runtime dependencies — no Node version
  conflicts, no Python virtualenvs, no npx timeouts. One file, works
  everywhere. This is critical for agent environments where toolchain setup
  should be zero-friction.

- **Narrow technical scope.** anima reads files, parses markdown, reads git
  history, and outputs text. This is the sweet spot for Rust CLI tools
  (similar to ripgrep, tokei). No web servers, no complex async, no heavy
  frameworks.

- **Safety for autonomous execution.** Agents run anima without human
  supervision. Rust's memory safety and lack of runtime crashes reduce the
  risk of silent failures in an unsupervised context.

## Alternatives considered

- **Node.js / TypeScript.** Fastest to develop. `npx anima init` has zero
  install friction. But runtime startup overhead is significant for
  agent-called tools, and Node version management adds environment complexity.

- **Python.** Familiar to the ML/data science audience. But same startup
  overhead as Node, plus virtualenv management adds friction in agent
  environments.

- **Go.** Single binary like Rust. Faster to develop. But Rust's ecosystem
  for CLI tools (clap, git2/gitoxide, pulldown-cmark) is more mature, and
  Rust's type system better fits a tool that parses structured documents.
