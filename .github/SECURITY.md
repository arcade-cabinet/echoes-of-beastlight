# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability within this project, please follow these steps:

1. **DO NOT** create a public GitHub issue for the vulnerability.
2. Send a description of the vulnerability to the maintainers via GitHub Security Advisories.
3. Include steps to reproduce the vulnerability if possible.

### What to Include in Your Report

- Type of vulnerability (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the vulnerability
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### Response Timeline

- We will acknowledge receipt of your vulnerability report within 48 hours
- We will provide a more detailed response within 7 days
- We will work on fixing the vulnerability and will keep you informed of our progress
- Once the vulnerability is fixed, we will publicly disclose the security issue

## Security Best Practices for Contributors

When contributing to this project, please follow these security best practices:

### Dependencies
- Keep dependencies up to date
- Review Dependabot and Renovate alerts promptly
- Audit new dependencies before adding them

### API Keys and Secrets
- Never commit API keys or secrets to the repository
- Use environment variables for sensitive configuration
- Document required environment variables in `.env.example`

### Code Review
- All code must be reviewed before merging
- Pay special attention to:
  - Input validation
  - File system operations
  - Network requests
  - Cryptographic operations

### Rust-Specific Security
- Use `#![forbid(unsafe_code)]` where possible
- When `unsafe` is necessary, document why and ensure it's reviewed
- Prefer strong typing over string parsing
- Use `Result` types for error handling

## Security Features

This project implements several security features:

1. **Input Validation**: All user inputs are validated and sanitized
2. **Safe File Operations**: File paths are sanitized to prevent directory traversal
3. **API Key Protection**: OpenAI API keys are never logged or exposed
4. **Dependency Scanning**: Automated scanning via Dependabot and cargo-audit
5. **Type Safety**: Rust's type system prevents many common vulnerabilities

## Contact

For security concerns, please use GitHub Security Advisories or contact the maintainers directly through GitHub.