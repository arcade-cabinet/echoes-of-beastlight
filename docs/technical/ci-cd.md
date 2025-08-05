# CI/CD and Dependency Management

## Overview

This project uses a comprehensive CI/CD pipeline with automated dependency management, security scanning, and quality checks.

## GitHub Actions Workflows

### 1. Rust Tests (`rust-tests.yml`)

**Trigger**: Push to main/develop, PRs to main
**Purpose**: Run all tests and generate coverage reports

**Jobs**:
- Format checking
- Clippy linting
- Unit and integration tests
- Documentation build
- Code coverage with Tarpaulin

### 2. Code Quality (`code-quality.yml`)

**Trigger**: Push to main/develop, PRs to main
**Purpose**: Ensure code quality standards

**Jobs**:
- Format check (rustfmt)
- Lint check (clippy with warnings as errors)
- Documentation check
- MSRV (Minimum Supported Rust Version) check
- Unused dependency detection

### 3. Security Audit (`security-audit.yml`)

**Trigger**: Push/PR with Cargo changes, weekly schedule
**Purpose**: Scan for security vulnerabilities

**Jobs**:
- cargo-audit for known vulnerabilities
- cargo-deny for license compliance
- Dependency review for PRs
- Outdated dependency check

### 4. Release (`release.yml`)

**Trigger**: Push of version tags (v*)
**Purpose**: Build and publish releases

**Jobs**:
- Cross-platform builds (Linux, macOS, Windows)
- GitHub release creation
- Optional crates.io publishing

## Dependency Management

### Renovate Bot

Configuration: `.github/renovate.json`

**Features**:
- Weekly dependency updates
- Automatic merging of minor/patch updates
- Grouped updates for related packages
- Security vulnerability auto-merge
- Dependency dashboard

**Package Groups**:
- Rust dependencies
- Bevy ecosystem
- Testing tools

### Dependabot

Configuration: `.github/dependabot.yml`

**Monitors**:
- Cargo dependencies (root and tools)
- GitHub Actions
- Weekly update schedule
- Grouped updates for related packages

### cargo-deny

Configuration: `deny.toml`

**Checks**:
- License compliance (allows MIT, Apache-2.0, etc.)
- Security advisories
- Duplicate dependencies
- Source repository verification

## Security Policies

### Vulnerability Reporting

See `.github/SECURITY.md` for:
- Supported versions
- Reporting process
- Response timeline
- Security best practices

### Automated Security

- Weekly security audits
- Immediate alerts for critical vulnerabilities
- Automated PRs for security fixes
- License compliance checking

## Development Workflow

### 1. Feature Development

```bash
# Create feature branch
git checkout -b feature/your-feature

# Make changes and test locally
cargo test
cargo fmt
cargo clippy

# Push and create PR
git push origin feature/your-feature
```

### 2. PR Requirements

All PRs must pass:
- [ ] Format check
- [ ] Clippy (no warnings)
- [ ] All tests
- [ ] Security audit
- [ ] Documentation build

### 3. Merge Strategy

- Squash and merge for features
- Regular merge for dependency updates
- Require up-to-date branches
- Delete head branches after merge

## Release Process

### 1. Version Bump

```bash
# Update version in Cargo.toml files
# Update CHANGELOG.md
git commit -m "chore: bump version to X.Y.Z"
```

### 2. Create Release

```bash
# Create and push tag
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin vX.Y.Z
```

### 3. Automated Release

The release workflow will:
1. Build binaries for all platforms
2. Create GitHub release with changelog
3. Upload binary artifacts
4. Optionally publish to crates.io

## Local Development

### Required Tools

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default 1.88.0

# Install development tools
cargo install cargo-audit
cargo install cargo-deny
cargo install cargo-outdated
cargo install cargo-machete
cargo install cargo-tarpaulin

# Install system dependencies (Ubuntu/Debian)
sudo apt-get install libsdl2-dev pkg-config libssl-dev
```

### Pre-commit Checks

Run before committing:

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test

# Check for security issues
cargo audit
cargo deny check
```

## Monitoring and Alerts

### Dependency Updates

- Check Renovate dashboard: `/dependency-dashboard`
- Review Dependabot PRs weekly
- Monitor security advisories

### Build Status

- GitHub Actions tab shows all workflow runs
- Failed builds trigger notifications
- Coverage reports available in PR checks

## Best Practices

### 1. Dependencies

- Review all dependency updates
- Group related updates
- Test thoroughly after updates
- Keep dependencies minimal

### 2. Security

- Never commit secrets
- Review security advisories promptly
- Update vulnerable dependencies immediately
- Use `cargo audit` locally

### 3. Releases

- Follow semantic versioning
- Update changelog for each release
- Test release builds locally first
- Create detailed release notes

## Troubleshooting

### Common Issues

**1. CI Failures**
- Check workflow logs in GitHub Actions
- Run failing commands locally
- Ensure all dependencies are installed

**2. Dependency Conflicts**
- Run `cargo update` to refresh lock file
- Check for duplicate dependencies
- Use `cargo tree` to debug

**3. Security Warnings**
- Check advisory details
- Update affected dependencies
- If can't update, document in `deny.toml`

### Getting Help

1. Check workflow logs
2. Review this documentation
3. Search existing issues
4. Create new issue with details
