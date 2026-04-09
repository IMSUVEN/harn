# Testing and CI pipeline

At the rooting phase, with rapid iteration on seed format and check logic,
the project needs an engineering safety net.

## Decisions

### Unit tests for parsing functions

Tests cover `extract_phase`, `detect_seed_generation`, `count_conventions`,
and seed embed integrity (non-empty, placeholder present, generation marker
matches current). These functions are the sensory core of `anima check` —
regressions here mean the spirit loses its senses.

Tests live in `src/main.rs` as `#[cfg(test)] mod tests`, the idiomatic
Rust approach for a single-binary project.

### CI on three platforms

GitHub Actions runs `cargo test` + `cargo build` on Linux, macOS, and
Windows for every push to `main` and every PR. Cross-platform correctness
matters because anima is a CLI tool used across all three.

### Automated binary releases

Tag-triggered workflow builds release binaries for four targets:
x86_64-unknown-linux-gnu, x86_64-apple-darwin, aarch64-apple-darwin,
x86_64-pc-windows-msvc. Packaged as tar.gz (unix) / zip (windows) and
published to GitHub Releases.

This removes the Rust toolchain requirement for users. "Download and put on
PATH" is the lowest possible installation barrier for a CLI tool.

## Why now

The project is iterating fast on seed format and check logic. Without tests,
a change to the seed that breaks generation detection would go unnoticed.
Without CI, platform-specific issues would surface only in user reports.
Without binary releases, the install friction limits adoption during the
phase when early feedback is most valuable.
