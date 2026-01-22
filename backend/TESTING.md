# Testing Documentation

## Overview

This project includes comprehensive tests for production-readiness:

- ✅ **Unit Tests** - Test individual functions and modules
- ✅ **Integration Tests** - Test API endpoints and workflows  
- ✅ **Property-Based Tests** - Test mathematical properties and invariants
- ✅ **Database Tests** - Test MongoDB operations
- ✅ **Service Tests** - Test external API integration
- ✅ **Rate Limiting Tests** - Test rate limit logic

## Test Structure

```
backend/
├── src/
│   ├── models.rs          # Contains unit tests for models
│   ├── handlers.rs        # Handler logic
│   ├── db.rs              # Database operations
│   └── crypto_service.rs  # API service
│
└── tests/
    ├── common/
    │   └── mod.rs         # Shared test utilities
    ├── integration_test.rs      # API endpoint tests
    ├── crypto_service_test.rs   # Service layer tests
    ├── rate_limiting_test.rs    # Rate limit tests
    ├── db_test.rs               # Database tests
    └── property_test.rs         # Property-based tests
```

## Running Tests

### Quick Start

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run specific test file
cargo test --test integration_test
```

### Using Test Runner Script

```bash
# Make executable
chmod +x run_tests.sh

# Run all tests
./run_tests.sh

# Run with verbose output
./run_tests.sh verbose

# Run unit tests only
./run_tests.sh unit

# Run integration tests only
./run_tests.sh integration

# Run with coverage
./run_tests.sh coverage

# Clean test artifacts
./run_tests.sh clean
```

## Test Coverage

### Unit Tests

**Models (`src/models.rs`)**
- Token serialization/deserialization
- Optional field handling
- Data validation
- JSON roundtrip testing

**Database (`tests/db_test.rs`)**
- Connection establishment
- CRUD operations (Create, Read, Update, Delete)
- Bulk operations
- Upsert operations
- Query filtering
- Index usage

### Integration Tests

**API Endpoints (`tests/integration_test.rs`)**
- `GET /api/tokens` - Get all tokens
- `GET /api/tokens/{id}` - Get single token
- `POST /api/tokens/favorite` - Toggle favorite
- `GET /api/favorites` - Get favorites
- `GET /api/search` - Search tokens
- Error handling (404, 400, 500)

### Service Tests

**CoinGecko API (`tests/crypto_service_test.rs`)**
- Successful API responses
- API error handling (429, 500)
- Timeout handling
- Null value handling
- Invalid JSON responses
- Network failures

### Rate Limiting Tests

**Rate Limit Logic (`tests/rate_limiting_test.rs`)**
- Minimum interval enforcement (2 seconds)
- Backoff on 429 errors (60 seconds)
- Concurrent request handling
- State transitions
- Expiry calculations

### Property-Based Tests

**Mathematical Properties (`tests/property_test.rs`)**
- Prices always positive
- High >= Low invariant
- Market cap formula validation
- Timestamp ordering
- Serialization roundtrip
- Price change bounds

## Test Data

### Mock Data

Test utilities in `tests/common/mod.rs` provide:
- `create_test_token()` - Generate single token
- `create_test_tokens(n)` - Generate multiple tokens
- `create_test_price_history()` - Generate price history
- `setup_test_db()` - Create test database
- `cleanup_test_db()` - Clean up after tests

### Example Usage

```rust
use common::mock_data;

#[tokio::test]
async fn test_example() {
    let token = mock_data::create_test_token("bitcoin");
    assert_eq!(token.token_id, "bitcoin");
    
    let tokens = mock_data::create_test_tokens(10);
    assert_eq!(tokens.len(), 10);
}
```

## Prerequisites for Tests

### Required
- Rust 1.75+
- Cargo

### Optional (for full test suite)
- MongoDB (for integration tests)
  ```bash
  docker-compose up -d
  # or
  docker run -d -p 27017:27017 mongo:7.0
  ```

### Environment Variables

Create `.env.test` for test configuration:
```env
MONGODB_TEST_URI=mongodb://localhost:27017
RUST_LOG=debug
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      mongodb:
        image: mongo:7.0
        ports:
          - 27017:27017
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test --all-features
      
      - name: Run tests with coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

## Test Best Practices

### 1. Serial Tests
Tests that modify shared state use `#[serial]`:
```rust
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_rate_limiting() {
    // Test that modifies global rate limit state
}
```

### 2. Database Cleanup
Always clean up test databases:
```rust
#[tokio::test]
async fn test_database() {
    let db = setup_test_db().await;
    
    // ... test code ...
    
    cleanup_test_db(&db).await;
}
```

### 3. Async Tests
Use `#[tokio::test]` or `#[actix_rt::test]` for async:
```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### 4. Test Isolation
Each test should be independent:
- Create fresh test data
- Don't rely on test execution order
- Clean up after completion

## Debugging Tests

### Enable Logging
```bash
RUST_LOG=debug cargo test -- --nocapture
```

### Run Single Test
```bash
cargo test test_name -- --nocapture --test-threads=1
```

### Show All Output
```bash
cargo test -- --nocapture --show-output
```

## Coverage Reports

### Generate HTML Coverage
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate report
cargo tarpaulin --out Html --output-dir coverage

# Open report
open coverage/index.html
```

### Expected Coverage
- **Models**: 95%+
- **Handlers**: 85%+
- **Services**: 90%+
- **Database**: 90%+
- **Overall**: 85%+

## Performance Testing

### Benchmark Tests
```bash
# Requires nightly Rust
cargo +nightly bench
```

### Load Testing
Use `cargo-criterion` for detailed benchmarks:
```bash
cargo install cargo-criterion
cargo criterion
```

## Common Test Scenarios

### Testing API Endpoints
```rust
#[actix_rt::test]
async fn test_api_endpoint() {
    let app = test::init_service(
        App::new().route("/api/test", web::get().to(handler))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/test")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
```

### Testing Database Operations
```rust
#[tokio::test]
async fn test_db_operation() {
    let db = setup_test_db().await;
    let collection = db.collection::<Token>("tokens");
    
    let token = create_test_token("test");
    collection.insert_one(&token, None).await.unwrap();
    
    let found = collection
        .find_one(doc! { "token_id": "test" }, None)
        .await
        .unwrap();
    
    assert!(found.is_some());
    cleanup_test_db(&db).await;
}
```

### Testing Error Handling
```rust
#[tokio::test]
async fn test_error_handling() {
    let result = function_that_might_fail().await;
    
    match result {
        Ok(_) => panic!("Should have failed"),
        Err(e) => assert!(e.to_string().contains("expected error")),
    }
}
```

## Troubleshooting

### MongoDB Connection Issues
```bash
# Check if MongoDB is running
mongosh --eval "db.version()"

# Start MongoDB with Docker
docker-compose up -d

# Check logs
docker logs mongo
```

### Test Timeout Issues
Increase timeout in test:
```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn long_running_test() {
    tokio::time::timeout(
        Duration::from_secs(30),
        my_function()
    ).await.unwrap();
}
```

### Flaky Tests
- Use `#[serial]` for tests with shared state
- Add delays for async operations
- Check for race conditions
- Ensure proper cleanup

## Contributing

When adding new features:
1. Write tests first (TDD)
2. Ensure all tests pass
3. Add integration tests for new endpoints
4. Update this documentation
5. Maintain >85% coverage

## Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Actix Web Testing](https://actix.rs/docs/testing/)
- [Tokio Testing](https://tokio.rs/tokio/topics/testing)
- [Property Testing with Proptest](https://proptest-rs.github.io/proptest/intro.html)
- [MongoDB Testing](https://www.mongodb.com/docs/drivers/rust/)
