.PHONY: build test clippy fmt check install clean

build:
	cargo build --workspace

test:
	cargo test --workspace

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

fmt:
	cargo fmt --all

check: fmt clippy test
	@echo "All checks passed."

install:
	cargo install --path crates/agentic-contract-cli
	cargo install --path crates/agentic-contract-mcp

clean:
	cargo clean

release:
	cargo build --workspace --release

guardrails:
	bash scripts/check-canonical-sister.sh
	bash scripts/check-install-commands.sh
	bash scripts/check-runtime-hardening.sh
	bash scripts/test-primary-problems.sh
