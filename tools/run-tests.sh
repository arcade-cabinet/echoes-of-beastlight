#!/bin/bash
set -e

echo "🧪 Running AI Game Generator Tests"
echo "=================================="

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in the tools directory
if [ ! -f "Cargo.toml" ]; then
    cd tools 2>/dev/null || { echo -e "${RED}Error: Not in tools directory${NC}"; exit 1; }
fi

# Clean previous test results
echo -e "${YELLOW}Cleaning previous test results...${NC}"
rm -rf target/test-results 2>/dev/null || true
mkdir -p target/test-results

# Run cargo check first
echo -e "${YELLOW}Running cargo check...${NC}"
if cargo check --all-targets; then
    echo -e "${GREEN}✓ Cargo check passed${NC}"
else
    echo -e "${RED}✗ Cargo check failed${NC}"
    exit 1
fi

# Run clippy for linting
echo -e "\n${YELLOW}Running clippy...${NC}"
if cargo clippy --all-targets -- -D warnings 2>&1 | tee target/test-results/clippy.log; then
    echo -e "${GREEN}✓ Clippy passed${NC}"
else
    echo -e "${RED}✗ Clippy found issues${NC}"
fi

# Run unit tests
echo -e "\n${YELLOW}Running unit tests...${NC}"
if cargo test --lib --no-fail-fast -- --test-threads=4 --nocapture 2>&1 | tee target/test-results/unit-tests.log; then
    echo -e "${GREEN}✓ Unit tests passed${NC}"
else
    echo -e "${RED}✗ Unit tests failed${NC}"
    FAILED=1
fi

# Run integration tests
echo -e "\n${YELLOW}Running integration tests...${NC}"
if cargo test --test '*' --no-fail-fast -- --test-threads=2 --nocapture 2>&1 | tee target/test-results/integration-tests.log; then
    echo -e "${GREEN}✓ Integration tests passed${NC}"
else
    echo -e "${RED}✗ Integration tests failed${NC}"
    FAILED=1
fi

# Run doc tests
echo -e "\n${YELLOW}Running doc tests...${NC}"
if cargo test --doc 2>&1 | tee target/test-results/doc-tests.log; then
    echo -e "${GREEN}✓ Doc tests passed${NC}"
else
    echo -e "${RED}✗ Doc tests failed${NC}"
    FAILED=1
fi

# Generate test coverage (if tarpaulin is installed)
if command -v cargo-tarpaulin &> /dev/null; then
    echo -e "\n${YELLOW}Generating test coverage...${NC}"
    cargo tarpaulin --out Html --output-dir target/test-results/coverage || true
    echo -e "${GREEN}Coverage report generated at target/test-results/coverage/index.html${NC}"
fi

# Summary
echo -e "\n${YELLOW}Test Summary${NC}"
echo "============="

if [ -z "$FAILED" ]; then
    echo -e "${GREEN}✅ All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Check the logs in target/test-results/${NC}"
    exit 1
fi