# List available commands
default:
    @just --list

# Install all development dependencies
install:
    @echo "Installing pre-commit..."
    pip install --user pre-commit
    pre-commit install
    pre-commit install --hook-type commit-msg
    @echo "Installing cargo extensions..."
    cargo install cargo-audit || true
    cargo install cargo-deny || true
    cargo install cargo-outdated || true
    cargo install cargo-machete || true
    cargo install cargo-tarpaulin || true

# Complete development environment setup
setup: install
    @echo "Setting up git hooks..."
    pre-commit install --install-hooks
    @echo "Creating secrets baseline..."
    detect-secrets scan > .secrets.baseline || true
    @echo "Running initial checks..."
    pre-commit run --all-files || true
    @echo "Development environment ready!"

# Run all quality checks
check:
    pre-commit run --all-files

# Auto-fix code issues
fix:
    cargo fmt --all
    cd tools && cargo fmt --all
    cargo fix --allow-dirty --allow-staged
    cd tools && cargo fix --allow-dirty --allow-staged
    pre-commit run --all-files markdownlint || true
    pre-commit run --all-files prettier || true

# Check code formatting
format:
    pre-commit run rust-fmt --all-files

# Run clippy linting
lint:
    pre-commit run rust-clippy --all-files

# Run all tests
test:
    pre-commit run rust-test --all-files

# Run security checks
security:
    pre-commit run rust-audit --all-files
    pre-commit run rust-deny --all-files
    pre-commit run detect-secrets --all-files

# Check for outdated dependencies
outdated:
    pre-commit run rust-outdated --all-files

# Build all binaries
build:
    @echo "Building game..."
    cargo build --release
    @echo "Building tools..."
    cd tools && cargo build --release --all-features

# Run the AI generator with optional arguments
run *ARGS:
    cd tools && cargo run --release --bin ai-gen -- {{ARGS}}

# Run the Bevy studio
studio:
    cd tools && cargo run --release --bin studio --features studio

# Build and open documentation
docs:
    cargo doc --no-deps --open
    cd tools && cargo doc --no-deps --all-features

# Clean build artifacts
clean:
    cargo clean
    cd tools && cargo clean
    rm -f .secrets.baseline

# Create optimized release builds
release:
    @echo "Creating release builds..."
    cargo build --release
    cd tools && cargo build --release --all-features

# Create distribution packages
dist: release
    mkdir -p dist
    tar -czf dist/echoes-of-beastlight-linux-x64.tar.gz -C target/release echoes-of-beastlight || true
    cd tools/target/release && tar -czf ../../../dist/ai-game-generator-linux-x64.tar.gz ai-gen
    @echo "Distribution packages created in dist/"

# Simulate CI pipeline locally
ci: check test security
    @echo "CI simulation complete!"

# Quick checks before committing
precommit: format lint
    @echo "Pre-commit checks passed!"

# Update all development tools
update-tools:
    rustup update
    cargo install --locked cargo-audit
    cargo install --locked cargo-deny
    cargo install --locked cargo-outdated
    cargo install --locked cargo-machete
    cargo install --locked cargo-tarpaulin
    pre-commit autoupdate

# Run a specific pre-commit hook
check-hook HOOK:
    pre-commit run {{HOOK}} --all-files

# Watch for changes and run tests
watch:
    cargo watch -x test

# Run benchmarks
bench:
    cargo bench

# Generate test coverage report
coverage:
    cd tools && cargo tarpaulin --out Html --output-dir ../target/coverage

# Check the entire workspace
workspace-check:
    cargo check --workspace --all-features

# Test the entire workspace
workspace-test:
    cargo test --workspace --all-features

# Format check with verbose output
fmt-check:
    cargo fmt --all -- --check
    cd tools && cargo fmt --all -- --check

# Run clippy with pedantic lints
clippy-pedantic:
    cargo clippy --all-targets --all-features -- -W clippy::pedantic
    cd tools && cargo clippy --all-targets --all-features -- -W clippy::pedantic

# Create a new release tag
tag VERSION:
    git tag -a v{{VERSION}} -m "Release v{{VERSION}}"
    git push origin v{{VERSION}}

# Show project statistics
stats:
    @echo "=== Code Statistics ==="
    tokei
    @echo "\n=== Dependency Count ==="
    cargo tree | wc -l
    @echo "\n=== Binary Sizes ==="
    ls -lh target/release/ | grep -E "(ai-gen|studio|echoes)" || echo "No release builds found"