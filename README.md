# harn

A CLI tool that gives projects the structure and workflows for effective AI-agent-driven development — based on harness engineering principles from [OpenAI](https://openai.com/index/harness-engineering/) and [Anthropic](https://www.anthropic.com/engineering/harness-design-long-running-apps).

## Problem

AI coding agents are context-dependent. Their output quality is bounded by the **environment**, not the model. Setting up the right project structure — discoverable knowledge, architectural constraints, quality criteria, workflow artifacts — is tedious and inconsistent when done manually.

## What harn Does

`harn` is a harness lifecycle tool. It bootstraps and maintains the knowledge and workflow layer that makes agents effective. It does not orchestrate agents — that's your AI coding tool's job (Cursor, Codex, Claude Code, etc.). `harn` gives those tools a well-structured environment to operate in.

```
harn init          # scaffold harness structure (10 files, every one substantive)
harn check         # validate structural integrity
harn status        # show current project state at a glance
harn plan          # manage execution plans
harn sprint        # manage sprint contracts
harn gc            # detect stale docs via git history
harn score         # view and update quality grades
```

## Supported AI Tools

- **Claude Code** — generates `CLAUDE.md` entry point
- **Codex** — generates `AGENTS.md` entry point

Both point to the same `docs/` knowledge structure.

## Key Properties

- **Single binary** — Rust, no runtime dependencies
- **Offline-first** — no network calls for core operations
- **Non-destructive** — never overwrites without confirmation
- **Tool-agnostic** — universal knowledge layer, tool-specific entry points

## Quick Start

```
$ cargo install harn
$ mkdir my-project && cd my-project && git init
$ harn init

Detecting project environment...
  ✓ Git repository
  ✗ No package manager detected (generic project)
  ✗ No AI tool configs detected

AI coding tools [codex, claude-code]: ↵

Creating harness structure...
  ✓ AGENTS.md
  ✓ CLAUDE.md
  ✓ ARCHITECTURE.md
  ✓ .agents/harn/config.toml
  ✓ docs/ (6 files, 4 empty dirs)

Done! Created 10 files.

$ harn check
  ✓ All checks passed (2 warnings: uncustomized templates)

# ... develop with your AI coding tool ...

$ harn plan new "user authentication"
$ harn sprint new "implement login page" --plan user-authentication
# ... work ...
$ harn sprint done
$ harn gc
  ✓ All documentation is current.
```

## Status

**v0.1.0** — All 8 commands implemented and tested (91 tests, <1s full suite). Ready for dogfooding.

## Design Documents

| Document | Contents |
|----------|----------|
| [Product Spec](docs/product-spec.md) | Motivation, scope, target users, UX design |
| [Design](docs/design.md) | Architecture, alternatives, gc design, dependencies |
| [Commands](docs/commands.md) | Full command reference with examples |
| [Generated Content](docs/generated-content.md) | What each template file contains (the core value spec) |
| [HARNESS-SPEC.md](docs/HARNESS-SPEC.md) | The underlying harness engineering specification |
| [HARNESS-GUIDE.md](docs/HARNESS-GUIDE.md) | Companion guide: reasoning, examples, decision frameworks |

## Configuration

Config lives at `.agents/harn/config.toml`, following the community `.agents/` convention.
