# AI Game Generator Test Suite

## Overview

This test suite provides comprehensive coverage for the AI Game Generator, including unit tests, integration tests, and property-based tests.

## Test Structure

```
tests/
├── unit/                   # Unit tests for individual modules
│   ├── config_test.rs     # Config parsing and serialization
│   ├── generator_test.rs  # Core generator logic
│   └── templates_test.rs  # Template rendering
├── integration/           # Integration tests
│   ├── openai_api_test.rs # OpenAI API interaction tests
│   └── full_generation_test.rs # End-to-end generation tests
├── fixtures/              # Test data
│   └── test-config.yaml   # Sample configuration for tests
└── README.md             # This file
```

## Running Tests

### Run all tests:

```bash
cargo test
```

### Run specific test modules:

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Specific module
cargo test config::tests
```

### Run with output:

```bash
cargo test -- --nocapture
```

### Run tests in parallel:

```bash
cargo test -- --test-threads=4
```

## Test Categories

### 1. Unit Tests

#### Config Module (`src/config.rs`)

- YAML parsing and deserialization
- Default value handling
- Field validation
- Serialization round-trips

#### Templates Module (`src/templates.rs`)

- Template registration
- Component generation
- System generation
- Tilemap generation
- Edge cases (special characters, empty inputs)

#### Generator Module (`src/generator.rs`)

- Directory setup
- File writing (with and without dry-run)
- Cache key generation
- Token counting
- Progress tracking
- Error handling

#### Git Tracker Module (`src/git_tracker.rs`)

- Repository initialization
- Manifest creation and tracking
- File hash calculation
- Cost estimation
- Serialization/deserialization
- Generation skipping logic

### 2. Integration Tests

#### OpenAI API Tests

- Successful chat completions
- Image generation
- Error handling (500 errors, rate limits)
- Caching behavior
- Token counting validation

#### Full Generation Tests

- Complete generation flow with mocked APIs
- Dry-run mode verification
- Git tracking integration
- Error scenarios (missing/invalid config)

### 3. Property-Based Tests

Using `proptest` for:
- Arbitrary game titles
- Cache key determinism
- File path sanitization
- Input validation

## Mocking Strategy

### OpenAI API Mocking

We use `wiremock` to mock OpenAI API responses:
- Chat completions for text generation
- Image generation endpoints
- Image download URLs
- Rate limiting scenarios

### File System Testing

We use `tempfile` for isolated file system operations:
- Each test gets its own temporary directory
- No interference between tests
- Automatic cleanup

## Test Fixtures

### `test-config.yaml`

A comprehensive test configuration that exercises all features:
- Multiple zones and environments
- Various generation rules
- Complete graphics and audio settings

## Coverage Goals

- **Unit Test Coverage**: >80% for core logic
- **Integration Coverage**: All major workflows
- **Error Coverage**: All error paths tested

## Writing New Tests

### Unit Test Template

```rust
#[test]
fn test_feature_name() {
    // Arrange
    let input = create_test_input();

    // Act
    let result = function_under_test(input);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}
```

### Integration Test Template

```rust
#[tokio::test]
async fn test_integration_scenario() {
    // Setup environment
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    // Configure mocks
    setup_mocks(&mock_server).await;

    // Run test
    let result = run_integration_test().await;

    // Verify
    assert!(result.is_ok());
    verify_side_effects();
}
```

## Continuous Integration

The test suite is designed to run in CI environments:
- No external dependencies required
- All API calls are mocked
- Deterministic results
- Fast execution (<30 seconds)

## Debugging Tests

### Enable logging:

```bash
RUST_LOG=debug cargo test
```

### Run single test:

```bash
cargo test test_name -- --exact
```

### Generate test coverage:

```bash
cargo tarpaulin --out Html
```
