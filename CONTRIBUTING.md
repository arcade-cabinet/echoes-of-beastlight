# Contributing to AI Game Generator

Thank you for your interest in contributing to the AI Game Generator project! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to abide by our code of conduct: be respectful, inclusive, and constructive in all interactions.

## Getting Started

### Prerequisites

1. Rust 1.88.0 or later
2. System dependencies:

   ```bash
   # Ubuntu/Debian
   sudo apt-get install libsdl2-dev pkg-config libssl-dev

   # macOS
   brew install sdl2 pkg-config

   # Windows
   # Install Visual Studio Build Tools
   ```

### Setup

1. Fork the repository
2. Clone your fork:

   ```bash
   git clone https://github.com/YOUR_USERNAME/ai-game-generator.git
   cd ai-game-generator
   ```

3. Add upstream remote:

   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/ai-game-generator.git
   ```

4. Install pre-commit hooks:

   ```bash
   pip install pre-commit
   pre-commit install
   ```

## Development Workflow

### 1. Create a Branch

```bash
# Update your local main branch
git checkout main
git pull upstream main

# Create a feature branch
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Write clean, documented code
- Follow Rust idioms and best practices
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build documentation
cargo doc --no-deps --open
```

### 4. Commit Your Changes

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "feat: add new feature

- Detailed description of what changed
- Why the change was made
- Any breaking changes"
```

#### Commit Message Format

We use conventional commits:

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `test:` Test additions or changes
- `chore:` Maintenance tasks
- `perf:` Performance improvements

### 5. Push and Create PR

```bash
# Push to your fork
git push origin feature/your-feature-name
```

Then create a pull request on GitHub.

## Pull Request Guidelines

### PR Requirements

- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings
- [ ] Documentation is updated
- [ ] Commit messages follow convention
- [ ] PR description is detailed

### PR Description Template

See `.github/pull_request_template.md`

## Code Style

### Rust Style

- Follow standard Rust naming conventions
- Use `rustfmt` for formatting
- Keep functions small and focused
- Prefer explicit over implicit
- Document public APIs

### Examples

```rust
/// Generates a new game component based on the configuration.
///
/// # Arguments
///
/// * `config` - The game configuration
/// * `component_type` - Type of component to generate
///
/// # Returns
///
/// Returns the generated component code as a string
///
/// # Errors
///
/// Returns an error if generation fails
pub async fn generate_component(
    config: &GameConfig,
    component_type: ComponentType,
) -> Result<String> {
    // Implementation
}
```

## Testing

### Test Organization

- Unit tests in the same file as the code
- Integration tests in `tests/` directory
- Use descriptive test names
- Test edge cases

### Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_generation() {
        // Arrange
        let config = create_test_config();

        // Act
        let result = generate_component(&config, ComponentType::Player);

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap().contains("struct Player"));
    }
}
```

## Documentation

### Code Documentation

- Document all public items
- Include examples in doc comments
- Explain "why" not just "what"
- Keep documentation up to date

### Project Documentation

- Update README.md for user-facing changes
- Update technical docs in `docs/technical/`
- Add migration guides for breaking changes

## Security

### Reporting Security Issues

See `.github/SECURITY.md` for security policy.

### Security Best Practices

- Never commit secrets or API keys
- Validate all inputs
- Use safe Rust patterns
- Avoid `unsafe` code unless necessary
- Document any security considerations

## Dependencies

### Adding Dependencies

1. Justify the need for the dependency
2. Check the license is compatible
3. Verify it's actively maintained
4. Consider the dependency size
5. Run security audit:

   ```bash
   cargo audit
   cargo deny check
   ```

### Updating Dependencies

- Test thoroughly after updates
- Check breaking changes
- Update documentation if needed

## Release Process

### Version Numbering

We use semantic versioning (SemVer):
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

### Creating a Release

1. Update version in `Cargo.toml` files
2. Update CHANGELOG.md
3. Create PR with version bump
4. After merge, tag the release:

   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push upstream v0.1.0
   ```

## Getting Help

### Resources

- Read existing code and tests
- Check documentation in `docs/`
- Search existing issues
- Ask in discussions

### Communication

- Be specific about your problem
- Include error messages
- Provide minimal reproducible examples
- Be patient and respectful

## Recognition

Contributors will be recognized in:
- GitHub contributors page
- Release notes
- Project documentation

Thank you for contributing to AI Game Generator!
