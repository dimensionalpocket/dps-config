# Add session_sub_to_user_id_fn and session_user_to_sub_fn (Rust only)

## Overview

Add two new function-typed properties to the Rust `DpsConfig` struct:

1. **`session_sub_to_user_id_fn`** — A function that takes a `&str` (session sub) and returns `u64` (user ID). Default: identity function that parses the string as `u64`.
2. **`session_user_to_sub_fn`** — A function that takes a JSON-like record (`serde_json::Value`) and returns a `String` (the sub). Default: returns the `id` property of the record as a string.

These are Rust-only features. The Bun/TypeScript implementation remains unchanged.

## Implementation Details

### 1. Add fields to `DpsConfig` struct (`src/lib.rs`)

Add two new fields using `Box<dyn Fn...>` to store callable functions. These fields are **non-optional** internally, ensuring they always hold a valid function.

```rust
pub struct DpsConfig {
  // ... existing fields ...

  // Session conversion functions
  session_sub_to_user_id_fn: Box<dyn Fn(&str) -> u64 + Send + Sync>,
  session_user_to_sub_fn: Box<dyn Fn(&serde_json::Value) -> String + Send + Sync>,
}
```

Note: `Send + Sync` bounds ensure the functions can be used across threads safely.

### 2. Initialize defaults in `DpsConfig::new()`

```rust
Self {
  // ... existing initializations ...

  session_sub_to_user_id_fn: Box::new(|sub: &str| {
    sub.parse::<u64>().unwrap_or(0)
  }),
  session_user_to_sub_fn: Box::new(|record: &serde_json::Value| {
    record
      .get("id")
      .and_then(|v| {
        if let Some(n) = v.as_u64() {
          Some(n.to_string())
        } else {
          v.as_str().map(|s| s.to_string())
        }
      })
      .unwrap_or_default()
  }),
}
```

### 3. Add getters and setters

```rust
// session_sub_to_user_id_fn
pub fn get_session_sub_to_user_id_fn(&self) -> &dyn Fn(&str) -> u64 {
  self.session_sub_to_user_id_fn.as_ref()
}

pub fn set_session_sub_to_user_id_fn(&mut self, f: impl Fn(&str) -> u64 + Send + Sync + 'static) {
  self.session_sub_to_user_id_fn = Box::new(f);
}

// session_user_to_sub_fn
pub fn get_session_user_to_sub_fn(&self) -> &dyn Fn(&serde_json::Value) -> String {
  self.session_user_to_sub_fn.as_ref()
}

pub fn set_session_user_to_sub_fn(
  &mut self,
  f: impl Fn(&serde_json::Value) -> String + Send + Sync + 'static,
) {
  self.session_user_to_sub_fn = Box::new(f);
}
```

### 4. Add `serde_json` dependency to `Cargo.toml`

```toml
[dependencies]
serde_json = "1"
```

### 5. Add tests

```rust
#[test]
fn test_session_sub_to_user_id_fn_default() {
  let config = DpsConfig::new();
  let to_user_id = config.get_session_sub_to_user_id_fn();
  assert_eq!(to_user_id("12345"), 12345);
  assert_eq!(to_user_id("invalid"), 0);
}

#[test]
fn test_session_sub_to_user_id_fn_custom() {
  let mut config = DpsConfig::new();
  config.set_session_sub_to_user_id_fn(|sub| {
    sub.len() as u64
  });
  assert_eq!(config.get_session_sub_to_user_id_fn()("hello"), 5);
}

#[test]
fn test_session_user_to_sub_fn_default() {
  let config = DpsConfig::new();
  let to_sub = config.get_session_user_to_sub_fn();
  let record = serde_json::json!({ "id": 42, "name": "test" });
  assert_eq!(to_sub(&record), "42");
}

#[test]
fn test_session_user_to_sub_fn_custom() {
  let mut config = DpsConfig::new();
  config.set_session_user_to_sub_fn(|record| {
    record.get("sub").and_then(|v| v.as_str()).unwrap_or("").to_string()
  });
  let record = serde_json::json!({ "sub": "user-123", "name": "test" });
  assert_eq!(config.get_session_user_to_sub_fn()(&record), "user-123");
}
```

### 6. Update README.md

Add a new section under Configuration Properties noting these are Rust-only:

```markdown
### Session Conversion Functions (Rust only)

These properties are only available in the Rust implementation. The Bun/TypeScript version does not include them.

| Property | Default | Description |
|----------|---------|-------------|
| `session_sub_to_user_id_fn` | Parses string as u64, returns 0 on failure | Function that converts a session `sub` string to a `u64` user ID |
| `session_user_to_sub_fn` | Returns `id` property as string | Function that extracts a `sub` string from a JSON record (`serde_json::Value`) |

Example usage:

```rust
let mut config = DpsConfig::new();

// Use default: parses sub as u64
let to_user_id = config.get_session_sub_to_user_id_fn();
assert_eq!(to_user_id("42"), 42);

// Custom converter: use length of sub as user ID
config.set_session_sub_to_user_id_fn(|sub| sub.len() as u64);

// Custom sub extractor from JSON record
config.set_session_user_to_sub_fn(|record| {
    record.get("sub").and_then(|v| v.as_str()).unwrap_or("").to_string()
});
```
```

## Files to Modify

1. **`Cargo.toml`** — Add `serde_json = "1"` dependency
2. **`src/lib.rs`** — Add struct fields, defaults in `new()`, getters/setters, tests
3. **`README.md`** — Document new properties with Rust-only note

## Order of Work

1. Add `serde_json` to `Cargo.toml`
2. Add struct fields and defaults in `lib.rs`
3. Add getters and setters
4. Add tests
5. Run `cargo test --quiet` to verify
6. Update README.md
7. Run `cargo clippy --allow-dirty --fix && cargo fmt`
