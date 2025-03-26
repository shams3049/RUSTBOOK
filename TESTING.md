# Testing Guidelines

This document outlines the testing strategies and procedures for the RUSTBOOK project.

## Types of Tests

### Unit Tests

Unit tests verify individual components in isolation. Write unit tests for:
- Service functions
- Data models
- Utility functions

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com"));
        assert!(!validate_email("invalid-email"));
    }
}
```

### Integration Tests

Integration tests verify that components work together correctly:
- API endpoint tests
- Database interaction tests

Example:
```rust
#[actix_rt::test]
async fn test_create_user() {
    let app = test::init_service(
        App::new()
            .service(create_user)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&json!({
            "username": "testuser",
            "email": "test@example.com",
            "password": "password123"
        }))
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::CREATED);
}
```

### End-to-End Tests

End-to-end tests verify the entire system works correctly:
- Complete user flows
- API and database interactions

## Running Tests

### Running Unit Tests
```
cargo test
```

### Running Integration Tests
```
cargo test --test integration_tests
```

### Running with Code Coverage
```
cargo tarpaulin --out Html
```

## Test Environment

- Create a `.env.test` file for test-specific configurations
- Use a separate test database
- Mock external services

## Continuous Integration

Tests are automatically run in GitHub Actions on:
- Every push to main/master branches
- Every pull request to main/master branches

See `.github/workflows/ci.yml` for details.
