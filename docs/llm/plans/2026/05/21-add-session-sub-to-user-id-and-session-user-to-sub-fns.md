# Add session_sub_to_user_id_fn and session_user_to_sub_fn (Rust only)

## Overview

Add two new function-typed properties to the Rust `DpsConfig` struct:

1. **`session_sub_to_user_id_fn`** — A function that takes a `&str` (session sub) and returns `Result<i64, Box<dyn std::error::Error + Send + Sync>>`. Default: parses the string as `i64`, returning a boxed parse error on failure.
2. **`session_user_to_sub_fn`** — A function that takes a JSON-like record (`serde_json::Value`) and returns `Result<String, Box<dyn std::error::Error + Send + Sync>>` (the sub). Default: returns the `id` property of the record as a string, or an error if `id` is missing or invalid.

These are Rust-only features. The Bun/TypeScript implementation remains unchanged.

## Implementation Details

### 1. Add fields to `DpsConfig` struct (`src/lib.rs`)

Add two new fields using `Box<dyn Fn...>` to store callable functions. These fields are **non-optional** internally, ensuring they always hold a valid function. The error type for `session_sub_to_user_id_fn` uses `Box<dyn std::error::Error + Send + Sync>` to allow callers to return any error type.

```rust
pub struct DpsConfig {
  // ... existing fields ...

  // Session conversion functions
  session_sub_to_user_id_fn: Box<dyn Fn(&str) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> + Send + Sync>,
  session_user_to_sub_fn: Box<dyn Fn(&serde_json::Value) -> Result<String, Box<dyn std::error::Error + Send + Sync>> + Send + Sync>,
}
```

Note: `Send + Sync` bounds ensure the functions can be used across threads safely.

### 3. Initialize defaults in `DpsConfig::new()`

```rust
Self {
  // ... existing initializations ...

  session_sub_to_user_id_fn: Box::new(|sub: &str| {
    sub.parse::<i64>().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
  }),
  session_user_to_sub_fn: Box::new(|record: &serde_json::Value| {
    record
      .get("id")
      .and_then(|v| {
        if let Some(n) = v.as_u64() {
          Some(Ok(n.to_string()))
        } else {
          v.as_str().map(|s| Ok(s.to_string()))
        }
      })
      .unwrap_or_else(|| Err("missing or invalid 'id' field".into()))
  }),
}
```

### 4. Add getters and setters

```rust
// session_sub_to_user_id_fn
pub fn get_session_sub_to_user_id_fn(&self) -> &dyn Fn(&str) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
  self.session_sub_to_user_id_fn.as_ref()
}

pub fn set_session_sub_to_user_id_fn(
  &mut self,
  f: impl Fn(&str) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> + Send + Sync + 'static,
) {
  self.session_sub_to_user_id_fn = Box::new(f);
}

// session_user_to_sub_fn
pub fn get_session_user_to_sub_fn(&self) -> &dyn Fn(&serde_json::Value) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
  self.session_user_to_sub_fn.as_ref()
}

pub fn set_session_user_to_sub_fn(
  &mut self,
  f: impl Fn(&serde_json::Value) -> Result<String, Box<dyn std::error::Error + Send + Sync>> + Send + Sync + 'static,
) {
  self.session_user_to_sub_fn = Box::new(f);
}
```

### 5. Add dependencies to `Cargo.toml`

```toml
[dependencies]
serde_json = "1"
```

### 6. Add tests

```rust
#[test]
fn test_session_sub_to_user_id_fn_default() {
  let config = DpsConfig::new();
  let to_user_id = config.get_session_sub_to_user_id_fn();
  assert_eq!(to_user_id("12345").unwrap(), 12345);
  assert!(to_user_id("invalid").is_err());
}

#[test]
fn test_session_sub_to_user_id_fn_custom() {
  let mut config = DpsConfig::new();
  config.set_session_sub_to_user_id_fn(|sub| Ok(sub.len() as i64));
  assert_eq!(config.get_session_sub_to_user_id_fn()("hello").unwrap(), 5);
}

#[test]
fn test_session_user_to_sub_fn_default() {
  let config = DpsConfig::new();
  let to_sub = config.get_session_user_to_sub_fn();
  let record = serde_json::json!({ "id": 42, "name": "test" });
  assert_eq!(to_sub(&record).unwrap(), "42");
  assert!(to_sub(&serde_json::json!({})).is_err());
}

#[test]
fn test_session_user_to_sub_fn_custom() {
  let mut config = DpsConfig::new();
  config.set_session_user_to_sub_fn(|record| {
    record.get("sub").and_then(|v| v.as_str()).map(|s| s.to_string()).ok_or("missing 'sub'".into())
  });
  let record = serde_json::json!({ "sub": "user-123", "name": "test" });
  assert_eq!(config.get_session_user_to_sub_fn()(&record).unwrap(), "user-123");
}
```

### 7. Update README.md

Add a new section under Configuration Properties noting these are Rust-only:

```markdown
### Session Conversion Functions (Rust only)

These properties are only available in the Rust implementation. The Bun/TypeScript version does not include them.

| Property | Default | Description |
|----------|---------|-------------|
| `session_sub_to_user_id_fn` | Parses string as i64, returns parse error on failure | Function that converts a session `sub` string to an `i64` user ID. Returns `Result<i64, Box<dyn std::error::Error + Send + Sync>>` to allow custom error types. |
| `session_user_to_sub_fn` | Returns `id` property as string, error on missing/invalid | Function that extracts a `sub` string from a JSON record (`serde_json::Value`). Returns `Result<String, Box<dyn std::error::Error + Send + Sync>>` for consistency. |

Example usage:

```rust
let mut config = DpsConfig::new();

// Use default: parses sub as i64
let to_user_id = config.get_session_sub_to_user_id_fn();
assert_eq!(to_user_id("42").unwrap(), 42);

// Custom converter: use length of sub as user ID
config.set_session_sub_to_user_id_fn(|sub| Ok(sub.len() as i64));

// Custom sub extractor from JSON record
config.set_session_user_to_sub_fn(|record| {
    record.get("sub").and_then(|v| v.as_str()).map(|s| s.to_string()).ok_or("missing 'sub'".into())
});
```

#### Overriding from a consuming crate

Since the functions return `Result<_, Box<dyn std::error::Error>>`, you can return your own error types (e.g., using `thiserror` or `anyhow`) without conversion.

```rust
use dps_config::DpsConfig;

#[derive(thiserror::Error, Debug)]
enum MyUserError {
    #[error("user not found: {0}")]
    NotFound(String),
}

fn configure_with_custom_errors(config: &mut DpsConfig) {
    // Returning a custom thiserror type directly
    config.set_session_sub_to_user_id_fn(|sub| {
        if sub == "invalid" {
            Err(Box::new(MyUserError::NotFound(sub.to_string())))
        } else {
            Ok(42)
        }
    });
}
```
```

## Files to Modify

1. **`Cargo.toml`** — Add `serde_json = "1"` dependency
2. **`src/lib.rs`** — Add struct fields, defaults in `new()`, getters/setters, tests
3. **`README.md`** — Document new properties with Rust-only note and consuming crate example

## Order of Work

1. Add `serde_json` to `Cargo.toml`
2. Add struct fields and defaults in `lib.rs`
3. Add getters and setters
4. Add tests
5. Run `cargo test --quiet` to verify
6. Update README.md
7. Run `cargo clippy --allow-dirty --fix && cargo fmt`
