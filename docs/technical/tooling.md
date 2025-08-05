# Consolidated Tooling Strategy

## Overview

This project uses a consolidated tooling approach centered around `pre-commit` and `just`, providing a single interface for all development tasks while maintaining the separation between AI generator (headless) and director studio (GUI).

## Why This Approach?

1. **Single Source of Truth**: All quality checks are defined in `.pre-commit-config.yaml`
2. **Consistent Interface**: `just` provides simple commands for both generator and director workflows
3. **No Scattered Scripts**: Everything runs through two well-documented tools
4. **CI/CD Alignment**: Local development mirrors CI pipeline exactly
5. **Clear Separation**: Generator vs Director workflows are clearly delineated

## Tool Stack

### Core Tools

1. **`just`** - Command runner (like make, but better for Rust)
   - Simple, readable syntax
   - Native parameter support
   - Cross-platform
   - Perfect for AI automation

2. **`pre-commit`** - Git hook framework
   - Runs all quality checks
   - Language agnostic
   - Extensive plugin ecosystem
   - Can be run manually or automatically

### Quality Tools (via pre-commit)

- **rustfmt** - Code formatting
- **clippy** - Linting
- **cargo-audit** - Security vulnerabilities
- **cargo-deny** - License compliance
- **cargo-outdated** - Dependency updates
- **cargo-machete** - Unused dependencies
- **typos** - Spell checking
- **markdownlint** - Markdown formatting
- **shellcheck** - Shell script linting
- **detect-secrets** - Secret scanning

## Installation

```bash
# One command to set up everything
just setup
```

This installs:
- All cargo tools
- pre-commit hooks
- Git commit hooks
- Creates security baseline

## Daily Workflows

### For AI Generators (You)

1. **Generate game assets via build script**:
   ```bash
   just generate         # Run cascade via game build script
   just dry-run         # Test without API calls
   just generate-fresh  # Ignore cache
   ```

2. **Run tests and checks**:
   ```bash
   just test    # Run all tests
   just check   # Run all checks
   ```

### For Directors (Human Users)

1. **Review and iterate on generated assets**:
   ```bash
   just director              # Launch studio for asset inspection
   just generate-and-review   # Generate then review
   just play                  # Test the game
   ```

2. **Development workflow**:
   ```bash
   just watch   # Auto-run checks on file changes
   just fix     # Auto-fix issues
   ```

### For Developers

```bash
# Run ALL checks (what CI runs)
just check

# Auto-fix what can be fixed
just fix

# Run specific checks
just format
just lint
just test
just security

# Before committing
just precommit
```

## Pre-commit Integration

### Running Manually

```bash
# Run all hooks
pre-commit run --all-files

# Run specific hook
pre-commit run rust-fmt --all-files

# Update hooks
pre-commit autoupdate
```

### Automatic Execution

Pre-commit runs automatically on:
- `git commit` (format, lint)
- `git push` (tests)
- Can be customized per hook

### Adding New Checks

Edit `.pre-commit-config.yaml`:

```yaml
- id: my-new-check
  name: My New Check
  entry: my-command
  language: system
  types: [rust]
```

## CI/CD Integration

### Local CI Simulation

```bash
# Run exactly what CI runs
just ci
```

### GitHub Actions

All workflows use the same pre-commit hooks:
- `rust-tests.yml` → `just test`
- `code-quality.yml` → `just lint`
- `security-audit.yml` → `just security`

## Advanced Usage

### Custom Workflows

```bash
# Generate with custom prompts
just generate-custom ./my-prompts

# Generate world with specific seed
just generate-world 12345

# Profile performance
just profile-generator
```

### Workspace Commands

```bash
# Check entire workspace
just workspace-check

# Test entire workspace
just workspace-test
```

### Statistics and Analysis

```bash
# Show project stats
just stats

# Generate coverage report
just coverage
```

## Benefits

1. **Simplicity**: One command to run everything
2. **Consistency**: Same tools locally and in CI
3. **Flexibility**: Easy to add new checks
4. **Performance**: Parallel execution where possible
5. **Reliability**: Well-tested, mature tools

## Troubleshooting

### Pre-commit Issues

```bash
# Reset pre-commit
pre-commit clean
pre-commit install --install-hooks

# Skip hooks temporarily
git commit --no-verify
```

### Just Issues

```bash
# List all commands
just --list

# Show what would run
just --dry-run <command>

# Run with verbose output
just --verbose <command>
```

## Best Practices

1. **Always run `just check` before pushing**
2. **Use `just fix` to auto-correct issues**
3. **Keep hooks fast (< 10 seconds total)**
4. **Document new workflows in justfile**
5. **Mirror CI checks locally**

## For AI Agents

The `just` command structure is designed for easy parsing:

```bash
# Pattern: just <verb>[-<object>] [ARGS]
just generate
just review-code
just test-taming
```

This makes it simple for AI agents to:
1. Discover available commands
2. Execute appropriate workflows
3. Parse structured output
4. Chain operations together

## Build Integration

The AI generation cascade is now integrated directly into the game's build process:

- Generation runs automatically during release builds
- Can be triggered manually with `ECHOES_GENERATE=1`
- Dry run mode available with `ECHOES_DRY_RUN=1`
- Cache is stored in `game/.cascade-cache`

The studio remains a separate binary for director-level asset review and inspection with bevy-inspector-egui integration.
