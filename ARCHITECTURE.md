# Architecture

## System Overview

harn is a single-binary Rust CLI. It scaffolds a harness structure for a project, then provides lifecycle commands to maintain that structure over time. It does **not** orchestrate agents — it gives agents a well-structured environment.

## Crate Structure

```
harn/
├── src/
│   ├── main.rs          # Entry point: parse args, dispatch to commands
│   ├── cli.rs           # clap derive definitions, subcommand routing
│   ├── config.rs        # .agents/harn/config.toml read/write (serde + toml)
│   ├── detect.rs        # Project environment detection (git, package managers, AI tools)
│   ├── init/
│   │   ├── mod.rs       # Init orchestration: detect → prompt → render → write
│   │   ├── prompt.rs    # Interactive prompts (dialoguer)
│   │   └── render.rs    # Template rendering (minijinja)
│   ├── plan.rs          # Execution plan management (new, list, complete)
│   ├── sprint.rs        # Sprint contract management (new, status, done)
│   ├── check.rs         # Structural validation (file existence, cross-refs, hashes)
│   ├── score.rs         # Quality score display and update
│   └── gc.rs            # Staleness detection via git history (git2)
├── templates/           # Embedded at compile time via include_dir!
│   ├── AGENTS.md.j2
│   ├── CLAUDE.md.j2
│   ├── ARCHITECTURE.md.j2
│   └── docs/            # Mirrors the generated docs/ tree
└── Cargo.toml
```

## Module Dependency Rules

Dependencies flow **downward only**. No module may import from a module above it.

```
main.rs
  └── cli.rs
        ├── init/       → config, detect
        ├── plan.rs     → config
        ├── sprint.rs   → config
        ├── check.rs    → config
        ├── score.rs    → config
        └── gc.rs       → config, git2
```

- `cli.rs` dispatches to command modules. Command modules depend on `config.rs` and domain-specific crates.
- `config.rs` is a shared dependency for all commands. It owns the `Config` type and all config I/O.
- `detect.rs` is used only by `init/`. No other module should call detection logic.
- `templates/` is a compile-time asset directory, not a runtime module. Accessed via `include_dir!` in `init/render.rs`.
