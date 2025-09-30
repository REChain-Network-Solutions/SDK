# Contributing

Thanks for your interest in contributing!

- Use conventional commits (e.g., `feat:`, `fix:`)
- Open an issue before large changes
- Add tests when possible
- Update docs when behavior changes
- Ensure CI passes before requesting review

## Development

- Rust toolchain: stable
- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace --all-targets -- -D warnings`
- Test: `cargo test --workspace --locked`