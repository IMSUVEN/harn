# harn

This project uses a structured harness for agent-driven development.

All project context — architecture, workflow, constraints, evaluation criteria, and documentation map — lives in [AGENTS.md](AGENTS.md). Read it first.

## Claude Code-Specific Notes

- Use `cargo test` as the primary feedback loop — full suite runs in <1 second.
- Run `cargo clippy -- -D warnings` before declaring any task complete. Zero warnings policy.
- The project dogfoods itself: `.agents/harn/config.toml` tracks harness state. Run `harn check` to validate.
