# Implementation Plan for dps-config

## Overview

This plan outlines the complete implementation of the `dps-config` Rust crate from scratch. The crate provides a lightweight configuration container (`DpsConfig`) for the DPS ecosystem with support for optional values, sensible defaults, environment variable loading, and computed getters.

## Project Requirements Summary

Based on the README.md:

- **No validation**: All configuration values are optional; validation is delegated to consuming crates
- **Smart defaults**: Most properties have hardcoded defaults suitable for development
- **Environment-first**: Configuration values auto-load from environment variables
- **Computed properties**: Additional getters derive combined values (URLs, domains)
- **Boolean convention**: Use `"Y"` string for true in environment variables

## Architecture

### Core Structure

```
DpsConfig
├── Global Properties
│   ├── domain
│   ├── api_subdomain
│   └── development_mode
│
├── DpsAuthApi Properties
│   ├── auth_api_subdomain
│   ├── auth_api_port
│   ├── auth_api_protocol
│   ├── auth_api_sqlite_file_path
│   └── auth_api_session_secret
│
└── Computed Getters
    ├── get_api_domain()
    ├── get_auth_api_url()
    └── get_auth_api_session_secret_bytes()
```

## Files to Create

### 1. `Cargo.toml`

**Purpose**: Project manifest with metadata and dependencies

**Content**:
```toml
[package]
name = "dps-config"
version = "0.1.0"
edition = "2021"
authors = ["Dimensional Pocket"]
license = "MIT"
description = "Configuration management for the DPS ecosystem"
repository = "https://github.com/dimensionalpocket/dps-config"
keywords = ["config", "configuration", "dps"]
categories = ["config"]

[dependencies]
# No external dependencies needed for basic functionality

[dev-dependencies]
# For testing environment variable loading
```

**Notes**:
- No external dependencies required (uses std::env)
- Minimal, lightweight implementation
- Edition 2021 for modern Rust features

### 2. `src/lib.rs`

**Purpose**: Main library file containing the `DpsConfig` struct and all implementation

**Structure**:
```rust
//! Configuration management for the DPS ecosystem
//!
//! This crate provides [`DpsConfig`], a lightweight configuration container
//! used by Rust components in the DPS ecosystem.

/// Main configuration struct for DPS ecosystem components
pub struct DpsConfig {
  // Global properties
  domain: Option<String>,
  api_subdomain: Option<String>,
  development_mode: Option<bool>,
  
  // DpsAuthApi properties
  auth_api_subdomain: Option<String>,
  auth_api_port: Option<u16>,
  auth_api_protocol: Option<String>,
  auth_api_sqlite_file_path: Option<String>,
  auth_api_session_secret: Option<String>,
}

impl DpsConfig {
  /// Creates a new DpsConfig instance with values loaded from environment variables
  pub fn new() -> Self { /* ... */ }
  
  // Getters and setters for each property
  // Computed getters
}

impl Default for DpsConfig {
  fn default() -> Self {
    Self::new()
  }
}

// Helper functions for environment variable loading
fn load_env_string(key: &str) -> Option<String> { /* ... */ }
fn load_env_bool(key: &str) -> Option<bool> { /* ... */ }
fn load_env_u16(key: &str) -> Option<u16> { /* ... */ }

#[cfg(test)]
mod tests {
  // Comprehensive unit tests
}
```

## Implementation Details

### Property Implementation Pattern

Each property follows this pattern:

```rust
// Getter with default
pub fn get_domain(&self) -> String {
  self.domain.clone().unwrap_or_else(|| "dps.localhost".to_string())
}

// Setter
pub fn set_domain(&mut self, value: &str) {
  self.domain = Some(value.to_string());
}
```

### Properties Table

| Property | Type | Environment Variable | Default | Notes |
|----------|------|---------------------|---------|-------|
| `domain` | `String` | `DPS_DOMAIN` | `"dps.localhost"` | Main domain |
| `api_subdomain` | `String` | `DPS_API_SUBDOMAIN` | `"api"` | API subdomain |
| `development_mode` | `bool` | `DPS_DEVELOPMENT_MODE` | `false` | Dev mode flag |
| `auth_api_subdomain` | `String` | `DPS_AUTH_API_SUBDOMAIN` | `"auth"` | Auth sub-subdomain |
| `auth_api_port` | `Option<u16>` | `DPS_AUTH_API_PORT` | `None` | Auth API port |
| `auth_api_protocol` | `String` | `DPS_AUTH_API_PROTOCOL` | `"https"` | Auth API protocol |
| `auth_api_sqlite_file_path` | `String` | `DPS_AUTH_API_SQLITE_FILE_PATH` | `"data/development.db"` | SQLite path |
| `auth_api_session_secret` | `Option<String>` | `DPS_AUTH_API_SESSION_SECRET` | `None` | Session secret (use bytes getter) |

### Computed Getters Implementation

#### `get_api_domain()`

```rust
/// Returns the global API domain (e.g., "api.dps.localhost")
///
/// Combines `api_subdomain` and `domain`.
///
/// # Examples
///
/// ```
/// use dps_config::DpsConfig;
///
/// let mut config = DpsConfig::new();
/// config.set_api_subdomain("api");
/// config.set_domain("dps.localhost");
/// assert_eq!(config.get_api_domain(), "api.dps.localhost");
/// ```
pub fn get_api_domain(&self) -> String {
  format!("{}.{}", self.get_api_subdomain(), self.get_domain())
}
```

#### `get_auth_api_url()`

```rust
/// Returns the complete Auth API URL
///
/// Combines `auth_api_protocol`, `auth_api_subdomain`, `api_subdomain`,
/// `domain`, and optionally `auth_api_port`.
///
/// # Examples
///
/// ```
/// use dps_config::DpsConfig;
///
/// let mut config = DpsConfig::new();
/// config.set_auth_api_protocol("http");
/// config.set_auth_api_port(Some(3000));
/// assert_eq!(config.get_auth_api_url(), "http://auth.api.dps.localhost:3000");
/// ```
pub fn get_auth_api_url(&self) -> String {
  let protocol = self.get_auth_api_protocol();
  let subdomain = self.get_auth_api_subdomain();
  let api_domain = self.get_api_domain();
  
  if let Some(port) = self.auth_api_port {
    format!("{}://{}.{}:{}", protocol, subdomain, api_domain, port)
  } else {
    format!("{}://{}.{}", protocol, subdomain, api_domain)
  }
}

/// Returns the Auth API session secret as bytes (Vec<u8>)
///
/// This is a convenience getter that converts the session secret string
/// into bytes, which is the format typically needed for encryption libraries.
///
/// # Examples
///
/// ```
/// use dps_config::DpsConfig;
///
/// let mut config = DpsConfig::new();
/// config.set_auth_api_session_secret(Some("my-secret-key"));
///
/// if let Some(secret_bytes) = config.get_auth_api_session_secret_bytes() {
///   assert_eq!(secret_bytes, b"my-secret-key");
/// }
/// ```
pub fn get_auth_api_session_secret_bytes(&self) -> Option<Vec<u8>> {
  self.auth_api_session_secret.as_ref().map(|s| s.as_bytes().to_vec())
}
```

#### `get_auth_api_session_secret_bytes()`

```rust
/// Returns the Auth API session secret as bytes (Vec<u8>)
///
/// This is a convenience getter that converts the session secret string
/// into bytes, which is the format typically needed for encryption libraries.
///
/// # Examples
///
/// ```
/// use dps_config::DpsConfig;
///
/// let mut config = DpsConfig::new();
/// config.set_auth_api_session_secret(Some("my-secret-key"));
///
/// if let Some(secret_bytes) = config.get_auth_api_session_secret_bytes() {
///   assert_eq!(secret_bytes, b"my-secret-key");
/// }
/// ```
pub fn get_auth_api_session_secret_bytes(&self) -> Option<Vec<u8>> {
  self.auth_api_session_secret.as_ref().map(|s| s.as_bytes().to_vec())
}
```

### Environment Variable Loading

```rust
impl DpsConfig {
  pub fn new() -> Self {
    Self {
      domain: load_env_string("DPS_DOMAIN"),
      api_subdomain: load_env_string("DPS_API_SUBDOMAIN"),
      development_mode: load_env_bool("DPS_DEVELOPMENT_MODE"),
      auth_api_subdomain: load_env_string("DPS_AUTH_API_SUBDOMAIN"),
      auth_api_port: load_env_u16("DPS_AUTH_API_PORT"),
      auth_api_protocol: load_env_string("DPS_AUTH_API_PROTOCOL"),
      auth_api_sqlite_file_path: load_env_string("DPS_AUTH_API_SQLITE_FILE_PATH"),
      auth_api_session_secret: load_env_string("DPS_AUTH_API_SESSION_SECRET"),
    }
  }
}

fn load_env_string(key: &str) -> Option<String> {
  std::env::var(key).ok()
}

fn load_env_bool(key: &str) -> Option<bool> {
  std::env::var(key).ok().map(|v| v == "Y")
}

fn load_env_u16(key: &str) -> Option<u16> {
  std::env::var(key).ok().and_then(|v| v.parse().ok())
}
```

## Testing Strategy

### Unit Tests

Create comprehensive tests in `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_values() {
    let config = DpsConfig::new();
    assert_eq!(config.get_domain(), "dps.localhost");
    assert_eq!(config.get_api_subdomain(), "api");
    assert_eq!(config.get_development_mode(), false);
    assert_eq!(config.get_auth_api_subdomain(), "auth");
    assert_eq!(config.get_auth_api_protocol(), "https");
    assert_eq!(config.get_auth_api_sqlite_file_path(), "data/development.db");
    assert!(config.auth_api_port.is_none());
    assert!(config.auth_api_session_secret.is_none());
  }

  #[test]
  fn test_setters() {
    let mut config = DpsConfig::new();
    config.set_domain("example.com");
    config.set_development_mode(true);
    config.set_auth_api_port(Some(3000));
    
    assert_eq!(config.get_domain(), "example.com");
    assert_eq!(config.get_development_mode(), true);
    assert_eq!(config.auth_api_port, Some(3000));
  }

  #[test]
  fn test_api_domain_computed() {
    let mut config = DpsConfig::new();
    config.set_api_subdomain("api");
    config.set_domain("dps.localhost");
    assert_eq!(config.get_api_domain(), "api.dps.localhost");
  }

  #[test]
  fn test_auth_api_url_without_port() {
    let mut config = DpsConfig::new();
    config.set_auth_api_protocol("https");
    config.set_auth_api_subdomain("auth");
    config.set_api_subdomain("api");
    config.set_domain("dps.localhost");
    assert_eq!(config.get_auth_api_url(), "https://auth.api.dps.localhost");
  }

  #[test]
  fn test_auth_api_url_with_port() {
    let mut config = DpsConfig::new();
    config.set_auth_api_protocol("http");
    config.set_auth_api_port(Some(3000));
    assert_eq!(config.get_auth_api_url(), "http://auth.api.dps.localhost:3000");
  }

  #[test]
  fn test_readme_example() {
    // Test example from README
    let mut config = DpsConfig::new();
    config.set_domain("test.local");
    config.set_auth_api_protocol("http");
    config.set_auth_api_port(Some(8080));
    assert_eq!(config.get_auth_api_url(), "http://auth.api.test.local:8080");
  }

  #[test]
  fn test_auth_api_session_secret_bytes() {
    let mut config = DpsConfig::new();
    config.set_auth_api_session_secret(Some("my-secret-key"));
    
    let secret_bytes = config.get_auth_api_session_secret_bytes();
    assert!(secret_bytes.is_some());
    assert_eq!(secret_bytes.unwrap(), b"my-secret-key");
  }

  #[test]
  fn test_auth_api_session_secret_bytes_none() {
    let config = DpsConfig::new();
    assert!(config.get_auth_api_session_secret_bytes().is_none());
  }
}
```

### Environment Variable Tests

```rust
#[cfg(test)]
mod env_tests {
  use super::*;

  #[test]
  fn test_env_bool_loading() {
    std::env::set_var("DPS_DEVELOPMENT_MODE", "Y");
    let config = DpsConfig::new();
    assert_eq!(config.get_development_mode(), true);
    std::env::remove_var("DPS_DEVELOPMENT_MODE");
  }

  #[test]
  fn test_env_string_loading() {
    std::env::set_var("DPS_DOMAIN", "test.example.com");
    let config = DpsConfig::new();
    assert_eq!(config.get_domain(), "test.example.com");
    std::env::remove_var("DPS_DOMAIN");
  }

  #[test]
  fn test_env_port_loading() {
    std::env::set_var("DPS_AUTH_API_PORT", "8080");
    let config = DpsConfig::new();
    assert_eq!(config.auth_api_port, Some(8080));
    std::env::remove_var("DPS_AUTH_API_PORT");
  }
}
```

## Documentation Requirements

### Crate-level Documentation

```rust
//! # dps-config
//!
//! Configuration management for the DPS ecosystem.
//!
//! ## Overview
//!
//! This crate provides [`DpsConfig`], a lightweight configuration container
//! used by Rust components in the DPS ecosystem. It focuses on optional values,
//! sensible defaults, environment variable loading, and computed getters.
//!
//! ## Key Principles
//!
//! - **No validation**: Consuming crates perform validation
//! - **Smart defaults**: Hardcoded defaults suitable for development
//! - **Environment-first**: Auto-loads from environment variables
//! - **Computed properties**: Derives combined values (URLs, domains)
//!
//! ## Quick Start
//!
//! ```rust
//! use dps_config::DpsConfig;
//!
//! let mut config = DpsConfig::new();
//! let domain = config.get_domain();
//! config.set_domain("example.com");
//! let api_domain = config.get_api_domain();
//! ```
```

### Doc Comments for All Public APIs

- Every `pub` function must have doc comments
- Include examples for complex functionality
- Document defaults, environment variables, and return values

## Implementation Checklist

- [ ] Create `Cargo.toml` with project metadata
- [ ] Create `src/lib.rs` with crate documentation
- [ ] Implement `DpsConfig` struct with all fields
- [ ] Implement `new()` constructor with env loading
- [ ] Implement helper functions: `load_env_string`, `load_env_bool`, `load_env_u16`
- [ ] Implement getters for all 8 properties (with defaults)
- [ ] Implement setters for all 8 properties
- [ ] Implement `get_api_domain()` computed getter
- [ ] Implement `get_auth_api_url()` computed getter
- [ ] Implement `get_auth_api_session_secret_bytes()` computed getter
- [ ] Implement `Default` trait
- [ ] Add comprehensive doc comments to all public APIs
- [ ] Create unit tests for default values
- [ ] Create unit tests for setters
- [ ] Create unit tests for computed getters
- [ ] Create unit tests for README examples
- [ ] Create environment variable loading tests
- [ ] Run `mise exec -- cargo test` to verify all tests pass
- [ ] Run `mise exec -- cargo fmt` to format code
- [ ] Run `mise exec -- cargo clippy` to check for issues
- [ ] Run `mise exec -- cargo doc --open` to verify documentation

## Verification Steps

After implementation:

1. **Build verification**:
   ```bash
   mise exec -- cargo build
   ```

2. **Test verification**:
   ```bash
   mise exec -- cargo test
   ```

3. **Documentation verification**:
   ```bash
   mise exec -- cargo doc --open
   ```

4. **Code quality**:
   ```bash
   mise exec -- cargo fmt --check
   mise exec -- cargo clippy
   ```

5. **README examples**: Manually verify all code examples in README.md compile and run correctly

## Edge Cases to Handle

1. **Port handling**: Ensure URL correctly includes/omits port
2. **Empty strings**: Should be treated as None for optional fields
3. **Invalid port numbers**: Parse errors should result in None
4. **Boolean parsing**: Only "Y" should be true, everything else false
5. **String allocation**: Minimize cloning, use references where possible

## Performance Considerations

- All fields are `Option<T>` to minimize memory when not set
- String cloning only happens when getters are called
- No external dependencies keeps compilation fast
- Inline hints for frequently called getters

## Future Extensibility

The design allows for easy addition of:
- New configuration properties (add field + getter/setter + env loading)
- New computed getters (add method using existing properties)
- Additional DPS services (follow same pattern as DpsAuthApi)

## Additional Updates Required

### README.md Updates

The README.md file needs to be updated to document the new `get_auth_api_session_secret_bytes()` getter:

1. **Computed Getters Section**: Add the new getter to the list:
   - `get_api_domain()` — returns `{api_subdomain}.{domain}`
   - `get_auth_api_url()` — returns full Auth API URL with protocol and optional port
   - `get_auth_api_session_secret_bytes()` — returns session secret as `Vec<u8>` for encryption libraries

2. **Add example in Computed Getters section**:
   ```rust
   let mut c = DpsConfig::new();
   c.set_auth_api_session_secret(Some("my-32-byte-secret-key-here!!!"));
   
   // Get as string
   let secret_str = c.get_auth_api_session_secret();
   
   // Get as bytes (convenient for encryption libraries)
   if let Some(secret_bytes) = c.get_auth_api_session_secret_bytes() {
       // Use secret_bytes with encryption library
       assert_eq!(secret_bytes.len(), 32);
   }
   ```

3. **API Reference Section**: Update the computed getters list to include:
   - `get_auth_api_session_secret_bytes()` - Returns session secret as Vec<u8>

## Summary

This implementation plan creates a minimal, efficient configuration crate that:
- ✅ Loads configuration from environment variables automatically
- ✅ Provides sensible defaults for development
- ✅ Offers computed getters for derived values
- ✅ Has zero external dependencies
- ✅ Includes comprehensive tests
- ✅ Has complete documentation
- ✅ Follows Rust best practices
- ✅ Matches all README examples exactly
- ✅ Uses 2-space indentation per `rustfmt.toml`