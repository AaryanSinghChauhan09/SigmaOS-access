# SigmaOS-access Component Wiki

## Overview
SigmaOS-access is a SigmaOS component providing access control and permission primitives for the SigmaOS operating system. It demonstrates the security foundation and permission management system within the SigmaOS ecosystem.

## Principles
- Single Responsibility
- Access Control Primitives
- Fail-Fast on Permission Errors
- Deterministic Permission Initialization
- Clear Abstraction Over Permission APIs
- Composition over Inheritance for permission modules

## Architecture
SigmaOS-access follows an access control primitives pattern:
- **Permission Core**: Provides fundamental permission checking and management
- **Role Management**: Handles role-based access control (RBAC) structures
- **Resource Access**: Manages read/write/execute permissions on system resources
- **Audit Logging**: Tracks permission checks and access decisions

## API
The component provides the following public APIs:
- `permission::check(resource, action)` - Check if a permission is granted
- `permission::grant(resource, role, action)` - Grant a permission
- `permission::revoke(resource, action)` - Revoke a permission
- `permission::roles()` - List all defined roles
- Error types for permission denied or invalid operations

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific permission configuration
2. **Act**: Execute the code under test (permission checks, role management)
3. **Assert**: Verify the expected permission results

Example:
```rust
#[test]
fn test_permission_check_aaa() {
    // Arrange
    let resource = "file:/etc/passwd";
    let action = "read";
    
    // Act
    let result = permission::check(resource, action);
    
    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_granted());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Permission initialization is best-effort; gracefully degrades on conflicting policies
- Comprehensive error types for all permission-related failure scenarios
- Unit tests provided for permission checks and role management
- Integration tests for full access control stack with policy engine

## TODO
- Add support for RBAC role hierarchy management
- Expand resource permission enumeration capabilities
- Add audit logging integration APIs
- Create integration tests with actual permission policy engine
- Implement permission inheritance and delegation