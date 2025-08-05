# List available commands
default:
    @just --list

# ============= DIRECTOR WORKFLOWS =============

# Launch the full director studio for game review and iteration
director:
    @echo "🎬 Launching Director Studio..."
    cd build-tools && cargo run --release --bin studio --features studio

# Review generated game assets in the studio
review:
    @echo "🔍 Opening generated game for review..."
    just director

# Generate a new game and immediately open in studio for review
generate-and-review: generate director
    @echo "✨ Game generated and studio opened!"

# Run the game to test it
play:
    @echo "🎮 Launching game..."
    cargo run --release

# ============= GENERATOR WORKFLOWS =============

# Generate a complete game (headless AI generator)
generate:
    @echo "🤖 Running AI game generator..."
    cd build-tools && cargo run --release --bin ai-gen

# Generate with specific configuration
generate-with CONFIG:
    @echo "🤖 Running AI generator with config: {{CONFIG}}"
    cd build-tools && cargo run --release --bin ai-gen -- --config {{CONFIG}}

# Dry run to see what would be generated
dry-run:
    cd build-tools && cargo run --release --bin ai-gen -- --dry-run

# Generate with caching disabled (fresh generation)
generate-fresh:
    cd build-tools && cargo run --release --bin ai-gen -- --no-cache

# ============= CASCADE WORKFLOWS =============

# Execute the root meta-prompt cascade
cascade:
    @echo "🌊 Running meta-prompt cascade..."
    cd build-tools && cargo run --release --bin cascade -- execute ../game/metaprompts/root.toml -o ../game

# Validate cascade structure
cascade-validate:
    @echo "✅ Validating cascade..."
    cd build-tools && cargo run --release --bin cascade -- validate ../game/metaprompts/root.toml

# Visualize cascade as graph
cascade-viz:
    @echo "📊 Visualizing cascade..."
    cd build-tools && cargo run --release --bin cascade -- visualize ../game/metaprompts/root.toml -o cascade.dot
    dot -Tpng cascade.dot -o cascade.png
    @echo "Graph saved to cascade.png"

# Dry run cascade (no API calls)
cascade-dry:
    @echo "🌊 Dry run cascade..."
    cd build-tools && cargo run --release --bin cascade -- execute ../game/metaprompts/root.toml -o ../game --dry-run

# ============= DEVELOPMENT SETUP =============

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
    cargo install just || true

# Complete development environment setup
setup: install
    @echo "Setting up git hooks..."
    pre-commit install --install-hooks
    @echo "Creating secrets baseline..."
    detect-secrets scan > .secrets.baseline || true
    @echo "Running initial checks..."
    pre-commit run --all-files || true
    @echo "Development environment ready!"

# ============= QUALITY CHECKS =============

# Run all quality checks
check:
    pre-commit run --all-files

# Auto-fix code issues
fix:
    cargo fmt --all
    cd build-tools && cargo fmt --all
    cargo fix --allow-dirty --allow-staged
    cd build-tools && cargo fix --allow-dirty --allow-staged
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

# ============= BUILD & RELEASE =============

# Build all binaries
build:
    @echo "Building game..."
    cargo build --release
    @echo "Building tools..."
    cd build-tools && cargo build --release --all-features

# Build only the game
build-game:
    cargo build --release

# Build only the tools
build-tools:
    cd build-tools && cargo build --release --all-features

# Create optimized release builds
release:
    @echo "Creating release builds..."
    cargo build --release
    cd build-tools && cargo build --release --all-features

# Create distribution packages
dist: release
    mkdir -p dist
    tar -czf dist/echoes-of-beastlight-linux-x64.tar.gz -C target/release echoes-of-beastlight || true
    cd build-tools/target/release && tar -czf ../../../dist/ai-game-generator-linux-x64.tar.gz ai-gen studio
    @echo "Distribution packages created in dist/"

# ============= DOCUMENTATION =============

# Build and open documentation
docs:
    cargo doc --no-deps --open
    cd build-tools && cargo doc --no-deps --all-features

# Open director documentation
docs-director:
    @echo "Opening director documentation..."
    xdg-open docs/director/project-overview.md || open docs/director/project-overview.md

# Open technical documentation
docs-tech:
    @echo "Opening technical documentation..."
    xdg-open docs/technical/ai-agent-context.md || open docs/technical/ai-agent-context.md

# ============= UTILITIES =============

# Clean build artifacts
clean:
    cargo clean
    cd build-tools && cargo clean
    rm -f .secrets.baseline
    rm -rf dist/

# Clean generated game files
clean-generated:
    rm -rf generated-test/
    rm -f GENERATION_SUMMARY.json
    rm -f GENERATION_SUMMARY.md
    rm -f GENERATION_STATUS.md

# Full clean (everything)
clean-all: clean clean-generated
    rm -rf .cache/
    rm -rf .ai-generation/

# Watch for changes and run tests
watch:
    cargo watch -x test

# Run benchmarks
bench:
    cargo bench

# Generate test coverage report
coverage:
    cd build-tools && cargo tarpaulin --out Html --output-dir ../target/coverage
    @echo "Coverage report generated at target/coverage/index.html"

# ============= CI/CD HELPERS =============

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
    cargo install --locked just
    pre-commit autoupdate

# ============= ADVANCED WORKFLOWS =============

# Run a specific pre-commit hook
check-hook HOOK:
    pre-commit run {{HOOK}} --all-files

# Create a new release tag
tag VERSION:
    git tag -a v{{VERSION}} -m "Release v{{VERSION}}"
    git push origin v{{VERSION}}

# Show project statistics
stats:
    @echo "=== Code Statistics ==="
    tokei . --exclude target --exclude .git
    @echo "\n=== Dependency Count ==="
    cargo tree | wc -l
    @echo "\n=== Binary Sizes ==="
    ls -lh target/release/ | grep -E "(ai-gen|studio|echoes)" || echo "No release builds found"

# Run the AI generator with custom prompts directory
generate-custom PROMPTS_DIR:
    cd build-tools && cargo run --release --bin ai-gen -- --prompts-dir {{PROMPTS_DIR}}

# Profile the generator performance
profile-generator:
    cd build-tools && cargo build --release --bin ai-gen
    perf record --call-graph=dwarf tools/target/release/ai-gen
    perf report

# Check the entire workspace
workspace-check:
    cargo check --workspace --all-features

# Test the entire workspace
workspace-test:
    cargo test --workspace --all-features

# Format check with verbose output
fmt-check:
    cargo fmt --all -- --check
    cd build-tools && cargo fmt --all -- --check

# Run clippy with pedantic lints
clippy-pedantic:
    cargo clippy --all-targets --all-features -- -W clippy::pedantic
    cd build-tools && cargo clippy --all-targets --all-features -- -W clippy::pedantic

# ============= DIRECTOR REVIEW WORKFLOWS =============

# Review and approve generated code
review-code:
    @echo "📝 Opening code review interface..."
    just director -- --mode code-review

# Review and modify style guide
review-style:
    @echo "🎨 Opening style guide editor..."
    just director -- --mode style-guide

# Test monster taming mechanics
test-taming:
    @echo "🐾 Testing monster taming system..."
    cargo test --package echoes-of-beastlight --test taming_tests

# Generate world with specific seed
generate-world SEED:
    @echo "🌍 Generating world with seed: {{SEED}}"
    cd build-tools && cargo run --release --bin ai-gen -- --world-seed {{SEED}}
