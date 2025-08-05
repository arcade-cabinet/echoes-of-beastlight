.PHONY: help install check fix test build clean release docs setup-dev

# Default target
help:
	@echo "AI Game Generator - Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make install      - Install all development dependencies"
	@echo "  make setup-dev    - Complete development environment setup"
	@echo ""
	@echo "Quality Checks (using pre-commit):"
	@echo "  make check        - Run all checks (format, lint, test, security)"
	@echo "  make fix          - Auto-fix issues where possible"
	@echo "  make format       - Check code formatting"
	@echo "  make lint         - Run clippy linting"
	@echo "  make test         - Run tests"
	@echo "  make security     - Run security checks"
	@echo "  make outdated     - Check for outdated dependencies"
	@echo ""
	@echo "Development:"
	@echo "  make build        - Build all binaries"
	@echo "  make run          - Run the AI generator"
	@echo "  make studio       - Run the Bevy studio"
	@echo "  make docs         - Build and open documentation"
	@echo "  make clean        - Clean build artifacts"
	@echo ""
	@echo "Release:"
	@echo "  make release      - Create a release build"
	@echo "  make dist         - Create distribution packages"

# Install all dependencies
install:
	@echo "Installing Rust dependencies..."
	cargo fetch
	cd tools && cargo fetch
	@echo "Installing pre-commit..."
	pip install --user pre-commit
	pre-commit install
	pre-commit install --hook-type commit-msg
	@echo "Installing cargo tools..."
	cargo install cargo-audit || true
	cargo install cargo-deny || true
	cargo install cargo-outdated || true
	cargo install cargo-machete || true
	cargo install cargo-tarpaulin || true
	@echo "Installation complete!"

# Complete development setup
setup-dev: install
	@echo "Setting up git hooks..."
	pre-commit install --install-hooks
	@echo "Creating secrets baseline..."
	detect-secrets scan > .secrets.baseline || true
	@echo "Running initial checks..."
	pre-commit run --all-files || true
	@echo "Development environment ready!"

# Run all checks using pre-commit
check:
	@echo "Running all quality checks..."
	pre-commit run --all-files

# Auto-fix issues where possible
fix:
	@echo "Auto-fixing code issues..."
	cargo fmt --all
	cd tools && cargo fmt --all
	cargo fix --allow-dirty --allow-staged
	cd tools && cargo fix --allow-dirty --allow-staged
	pre-commit run --all-files markdownlint || true
	pre-commit run --all-files prettier || true

# Format check only
format:
	pre-commit run rust-fmt --all-files

# Lint check only
lint:
	pre-commit run rust-clippy --all-files

# Run tests only
test:
	pre-commit run rust-test --all-files

# Security checks
security:
	pre-commit run rust-audit --all-files
	pre-commit run rust-deny --all-files
	pre-commit run detect-secrets --all-files

# Check outdated dependencies
outdated:
	pre-commit run rust-outdated --all-files

# Build all binaries
build:
	@echo "Building game..."
	cargo build --release
	@echo "Building tools..."
	cd tools && cargo build --release --all-features

# Run the AI generator
run:
	cd tools && cargo run --release --bin ai-gen -- $(ARGS)

# Run the studio
studio:
	cd tools && cargo run --release --bin studio --features studio

# Build and open documentation
docs:
	@echo "Building documentation..."
	cargo doc --no-deps --open
	cd tools && cargo doc --no-deps --all-features

# Clean build artifacts
clean:
	cargo clean
	cd tools && cargo clean
	rm -rf target/
	rm -rf tools/target/
	rm -f .secrets.baseline

# Create release build
release:
	@echo "Creating release build..."
	cargo build --release
	cd tools && cargo build --release --all-features
	@echo "Release builds created in:"
	@echo "  - target/release/"
	@echo "  - tools/target/release/"

# Create distribution packages
dist: release
	@echo "Creating distribution packages..."
	mkdir -p dist
	# Linux
	tar -czf dist/echoes-of-beastlight-linux-x64.tar.gz -C target/release echoes-of-beastlight
	tar -czf dist/ai-game-generator-linux-x64.tar.gz -C tools/target/release ai-gen studio
	@echo "Distribution packages created in dist/"

# Specific pre-commit hooks
.PHONY: check-% fix-%

check-%:
	pre-commit run $* --all-files

fix-%:
	pre-commit run $* --all-files || true

# CI simulation - run what CI would run
ci:
	@echo "Simulating CI pipeline..."
	make check
	make test
	make security
	@echo "CI simulation complete!"

# Quick check before committing
precommit: format lint
	@echo "Pre-commit checks passed!"

# Update all tools
update-tools:
	@echo "Updating Rust..."
	rustup update
	@echo "Updating cargo tools..."
	cargo install --locked cargo-audit
	cargo install --locked cargo-deny
	cargo install --locked cargo-outdated
	cargo install --locked cargo-machete
	cargo install --locked cargo-tarpaulin
	@echo "Updating pre-commit..."
	pre-commit autoupdate
	@echo "Tools updated!"