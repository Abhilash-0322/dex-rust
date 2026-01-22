#!/bin/bash

# Comprehensive Test Runner for Crypto Tracker Backend
# This script runs all tests with proper logging and coverage

set -e

echo "🧪 Crypto Tracker - Comprehensive Test Suite"
echo "============================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check if MongoDB is running (for integration tests)
check_mongodb() {
    echo "Checking MongoDB connection..."
    if mongosh --eval "db.version()" > /dev/null 2>&1 || mongo --eval "db.version()" > /dev/null 2>&1; then
        print_status "MongoDB is running"
        return 0
    else
        print_warning "MongoDB is not running. Some integration tests may fail."
        print_warning "Start MongoDB with: docker-compose up -d"
        return 1
    fi
}

# Run all tests
run_tests() {
    echo ""
    echo "📋 Running all tests..."
    echo "----------------------"
    
    if cargo test --all-features 2>&1 | tee test_output.log; then
        print_status "All tests passed!"
    else
        print_error "Some tests failed. Check test_output.log for details."
        exit 1
    fi
}

# Run tests with verbose output
run_tests_verbose() {
    echo ""
    echo "📋 Running tests (verbose mode)..."
    echo "-----------------------------------"
    
    cargo test --all-features -- --nocapture --test-threads=1
}

# Run specific test suite
run_test_suite() {
    local suite=$1
    echo ""
    echo "📋 Running $suite tests..."
    echo "------------------------"
    
    cargo test --test "$suite" -- --nocapture
}

# Run unit tests only
run_unit_tests() {
    echo ""
    echo "📋 Running unit tests..."
    echo "------------------------"
    
    cargo test --lib
}

# Run integration tests only
run_integration_tests() {
    echo ""
    echo "📋 Running integration tests..."
    echo "-------------------------------"
    
    cargo test --tests
}

# Run tests with coverage (requires cargo-tarpaulin)
run_coverage() {
    echo ""
    echo "📊 Running tests with coverage..."
    echo "---------------------------------"
    
    if command -v cargo-tarpaulin &> /dev/null; then
        cargo tarpaulin --out Html --output-dir coverage --all-features
        print_status "Coverage report generated in coverage/index.html"
    else
        print_error "cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin"
        exit 1
    fi
}

# Benchmark tests (requires nightly Rust)
run_benchmarks() {
    echo ""
    echo "⚡ Running benchmarks..."
    echo "-----------------------"
    
    cargo +nightly bench || print_warning "Benchmarks require nightly Rust"
}

# Clean test artifacts
clean_tests() {
    echo ""
    echo "🧹 Cleaning test artifacts..."
    echo "-----------------------------"
    
    cargo clean
    rm -f test_output.log
    rm -rf coverage/
    print_status "Test artifacts cleaned"
}

# Display test summary
show_summary() {
    echo ""
    echo "📊 Test Summary"
    echo "==============="
    
    # Count test files
    unit_tests=$(find tests/ -name "*.rs" -type f 2>/dev/null | wc -l || echo "0")
    
    echo "Unit tests: $unit_tests test files"
    echo ""
    echo "Test Suites:"
    echo "  • integration_test.rs    - API endpoint integration tests"
    echo "  • crypto_service_test.rs - CoinGecko API service tests"
    echo "  • rate_limiting_test.rs  - Rate limiting logic tests"
    echo "  • db_test.rs             - Database operation tests"
    echo "  • property_test.rs       - Property-based tests"
    echo "  • models unit tests      - Model serialization tests"
}

# Main menu
show_menu() {
    echo ""
    echo "Available Commands:"
    echo "  all         - Run all tests"
    echo "  verbose     - Run all tests with verbose output"
    echo "  unit        - Run unit tests only"
    echo "  integration - Run integration tests only"
    echo "  coverage    - Run tests with coverage report"
    echo "  benchmark   - Run benchmarks (requires nightly)"
    echo "  clean       - Clean test artifacts"
    echo "  summary     - Show test summary"
    echo "  help        - Show this help message"
    echo ""
}

# Parse command line arguments
if [ $# -eq 0 ]; then
    check_mongodb
    run_tests
    show_summary
else
    case "$1" in
        all)
            check_mongodb
            run_tests
            show_summary
            ;;
        verbose)
            check_mongodb
            run_tests_verbose
            ;;
        unit)
            run_unit_tests
            ;;
        integration)
            check_mongodb
            run_integration_tests
            ;;
        suite)
            if [ -z "$2" ]; then
                print_error "Please specify a test suite name"
                echo "Available suites: integration_test, crypto_service_test, rate_limiting_test, db_test, property_test"
                exit 1
            fi
            check_mongodb
            run_test_suite "$2"
            ;;
        coverage)
            check_mongodb
            run_coverage
            ;;
        benchmark)
            run_benchmarks
            ;;
        clean)
            clean_tests
            ;;
        summary)
            show_summary
            ;;
        help|--help|-h)
            show_menu
            ;;
        *)
            print_error "Unknown command: $1"
            show_menu
            exit 1
            ;;
    esac
fi

echo ""
print_status "Test run complete!"
