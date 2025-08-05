# AI Game Generator Testing Strategy

## Overview

This document outlines the comprehensive testing strategy for the AI Game Generator project. The approach emphasizes test-driven development, thorough coverage, and maintainable test code.

## Testing Philosophy

1. **Test First**: Write tests before implementation when possible
2. **Mock External Dependencies**: All OpenAI API calls are mocked
3. **Isolated Tests**: Each test runs in its own environment
4. **Fast Feedback**: Tests should complete in under 30 seconds
5. **Comprehensive Coverage**: Target >80% code coverage

## Test Layers

### 1. Unit Tests
Located in module-level `#[cfg(test)]` blocks within source files.

**Purpose**: Test individual functions and methods in isolation

**Coverage Areas**:
- Configuration parsing and validation
- Template rendering
- File operations
- Token counting
- Cache key generation
- Git tracking logic

**Example**:
```rust
#[test]
fn test_cache_key_generation() {
    let key = generate_cache_key("system", "user");
    assert_eq!(key, expected_hash);
}
```

### 2. Integration Tests
Located in `tests/integration/` directory.

**Purpose**: Test interactions between modules and external systems

**Coverage Areas**:
- Full generation workflow
- OpenAI API integration (mocked)
- File system operations
- Git repository operations
- Cache behavior

**Example**:
```rust
#[tokio::test]
async fn test_full_generation_with_mocked_api() {
    let mock_server = MockServer::start().await;
    setup_mocks(&mock_server).await;
    
    let mut generator = AIGameGenerator::new();
    let result = generator.generate_game().await;
    
    assert!(result.is_ok());
    verify_all_files_created();
}
```

### 3. Property-Based Tests
Using `proptest` for generative testing.

**Purpose**: Test with randomly generated inputs to find edge cases

**Coverage Areas**:
- Input sanitization
- Path handling
- Configuration validation
- Cache key determinism

**Example**:
```rust
proptest! {
    #[test]
    fn test_sanitize_path(input in ".*") {
        let sanitized = sanitize_path(&input);
        prop_assert!(!sanitized.contains('/'));
        prop_assert!(!sanitized.contains('\0'));
    }
}
```

## Mocking Strategy

### OpenAI API Mocking
Using `wiremock` for HTTP mocking:

```rust
Mock::given(method("POST"))
    .and(path("/v1/chat/completions"))
    .respond_with(ResponseTemplate::new(200)
        .set_body_json(mock_response))
    .mount(&mock_server)
    .await;
```

**Mocked Scenarios**:
- Successful completions
- Rate limiting (429 responses)
- Server errors (500 responses)
- Network timeouts
- Invalid responses

### File System Mocking
Using `tempfile` for isolated file operations:

```rust
let temp_dir = TempDir::new().unwrap();
std::env::set_current_dir(&temp_dir).unwrap();
```

## Test Data Management

### Fixtures
Located in `tests/fixtures/`:
- `test-config.yaml`: Complete game configuration
- Mock response templates
- Sample generated content

### Test Builders
Helper functions to create test data:

```rust
fn create_test_config() -> GameConfig {
    GameConfig {
        game: GameInfo {
            title: "Test Game".to_string(),
            // ...
        },
        // ...
    }
}
```

## Error Testing

### Expected Failures
Test error conditions explicitly:

```rust
#[test]
fn test_missing_config_file() {
    let result = load_config("nonexistent.yaml");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}
```

### Error Categories Tested
1. Missing configuration files
2. Invalid YAML syntax
3. API failures
4. File system permissions
5. Git repository errors
6. Token limit exceeded

## Performance Testing

### Benchmarks
Using Rust's built-in benchmark framework:

```rust
#[bench]
fn bench_token_counting(b: &mut Bencher) {
    let text = "Sample text for tokenization";
    b.iter(|| {
        tokenizer.encode(text)
    });
}
```

### Performance Targets
- Token counting: <1ms for typical prompts
- Cache lookup: <10ms
- File writing: <50ms per file
- Full generation: <5 minutes (with real API)

## Test Execution

### Local Development
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run with coverage
cargo tarpaulin --out Html
```

### Continuous Integration
GitHub Actions workflow runs on every push:
1. Format checking (`cargo fmt`)
2. Linting (`cargo clippy`)
3. Unit tests
4. Integration tests
5. Coverage reporting

## Test Maintenance

### Best Practices
1. **Keep tests focused**: One assertion per test when possible
2. **Use descriptive names**: `test_config_parsing_handles_missing_fields`
3. **Avoid test interdependence**: Each test should be runnable in isolation
4. **Mock at boundaries**: Mock external services, not internal modules
5. **Test public APIs**: Focus on public interfaces, not implementation details

### Test Review Checklist
- [ ] Does the test have a clear purpose?
- [ ] Is the test name descriptive?
- [ ] Are assertions specific and meaningful?
- [ ] Is the test isolated from others?
- [ ] Does it run quickly (<100ms)?
- [ ] Is it deterministic?

## Coverage Goals

### Target Coverage by Module
- `config.rs`: 90%+ (critical parsing logic)
- `generator.rs`: 85%+ (core functionality)
- `templates.rs`: 95%+ (simple, testable)
- `git_tracker.rs`: 80%+ (complex Git operations)
- Integration: 100% of major workflows

### Coverage Reporting
- Local: `cargo tarpaulin --out Html`
- CI: Automated upload to Codecov
- Review: Coverage required for PR approval

## Future Improvements

1. **Mutation Testing**: Use `cargo mutants` to verify test quality
2. **Fuzz Testing**: Add fuzzing for parser components
3. **Load Testing**: Simulate concurrent generations
4. **Contract Testing**: Verify OpenAI API compatibility
5. **Visual Regression**: Test generated sprite quality