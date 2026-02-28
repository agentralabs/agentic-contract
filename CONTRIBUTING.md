# Contributing to AgenticContract

Thank you for your interest in contributing to AgenticContract! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/agentic-contract.git`
3. Create a feature branch: `git checkout -b my-feature`
4. Make your changes
5. Run the tests (see below)
6. Commit and push
7. Open a pull request

## Development Setup

This is a Cargo workspace monorepo. All Rust crates are under `crates/`.

### Rust Workspace

```bash
# Build everything (core + MCP server + CLI + FFI)
cargo build --workspace

# Run all tests (core + MCP + CLI + stress)
cargo test --workspace

# Core library only
cargo test -p agentic-contract

# MCP server only
cargo test -p agentic-contract-mcp

# CLI integration tests
cargo test -p agentic-contract-cli --test cli_integration

# Run the CLI
cargo run -p agentic-contract-cli -- stats

# Run the MCP server
cargo run -p agentic-contract-mcp -- serve
```

### Python SDK

```bash
cd python/
python3 -m venv .venv
source .venv/bin/activate
pip install -e ".[dev]"
pytest tests/ -v
```

### Guardrails

Before pushing, always run the canonical guardrails:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
bash scripts/check-canonical-sister.sh
```

## Ways to Contribute

### Report Bugs

File an issue with:
- Steps to reproduce
- Expected behavior
- Actual behavior
- System info (OS, Rust version)

### Add an MCP Tool

1. Add the tool definition in `crates/agentic-contract-mcp/src/tools.rs`
2. Add the handler in the `handle_tool_call` match block
3. Add tests in `crates/agentic-contract-mcp/tests/`
4. Update `docs/public/command-surface.md`

### Add a CLI Subcommand

1. Create a new module in `crates/agentic-contract-cli/src/commands/`
2. Register it in `crates/agentic-contract-cli/src/main.rs`
3. Add integration tests in `crates/agentic-contract-cli/tests/`
4. Update `docs/public/cli-reference.md`

### Improve Documentation

All docs are in `docs/`. Fix typos, add examples, clarify explanations -- all welcome.

## Code Guidelines

- **Rust**: Follow standard Rust conventions. Run `cargo clippy` and `cargo fmt`.
- **Python**: Follow PEP 8. Use type hints.
- **Tests**: Every feature needs tests. We maintain 288 tests across the stack (33 core, 55 engine inventions, 80 MCP stress, 30 server stress, 41 CLI integration, 29 edge cases, 14 stress/edge, 6 lib unit).
- **Documentation**: Update docs when changing public APIs.
- **MCP Quality Standard**: Tool descriptions must be verb-first imperative, no trailing periods. Tool errors use `isError: true`, protocol errors use JSON-RPC error codes. Unknown tool returns `-32803`.

## Commit Messages

Use conventional commit prefixes:
- `feat: add obligation deadline notifications`
- `fix: risk limit check off-by-one`
- `chore: update dependencies`
- `docs: add approval workflow guide`

## Pull Request Guidelines

- Keep PRs focused -- one feature or fix per PR
- Include tests for new functionality
- Update documentation if needed
- Ensure all tests pass before submitting
- Ensure guardrails pass: `bash scripts/check-canonical-sister.sh`
- Write a clear PR description

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
