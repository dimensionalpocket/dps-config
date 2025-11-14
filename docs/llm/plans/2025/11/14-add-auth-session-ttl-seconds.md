# Add auth_session_ttl_seconds to DpsConfig

## Summary

Add an optional property `auth_session_ttl_seconds` (u64) with getter/setter, env var support, tests, and README update. The property is optional (Option<u64>) and parsed from DPS_AUTH_SESSION_TTL_SECONDS. The default value will be 14 days (1209600 seconds); getters will return the default when unset.

## Files to create/modify

- [`src/lib.rs`](src/lib.rs:1)
- [`README.md`](README.md:1)
- [`docs/llm/plans/2025/11/14-add-auth-session-ttl-seconds.md`](docs/llm/plans/2025/11/14-add-auth-session-ttl-seconds.md:1)

## Design decisions

- Type: u64 to allow large TTLs and avoid negative values.
- Default: 1209600 (14 days in seconds). When the field is unset the getter will return this default.
- Field/storage: store as Option<u64> for consistency with other optional fields.
- Getter signature: get_auth_session_ttl_seconds(&self) -> u64 (returns default when unset).
- Setter signature: set_auth_session_ttl_seconds(&mut self, value: Option<u64>) (use None to unset).
- Env var: DPS_AUTH_SESSION_TTL_SECONDS parsed as u64; invalid parse => treated as unset.
- Helper: add load_env_u64(key: &str) -> Option<u64>

## Implementation steps (high level)

1. Add field to struct in [`src/lib.rs`](src/lib.rs:1).
2. Load from env in DpsConfig::new() using load_env_u64.
3. Implement helper load_env_u64.
4. Implement getter and setter.
5. Add unit tests to tests module in [`src/lib.rs`](src/lib.rs:1).
6. Update README table and example.

## Code samples

Field addition:

```rust
auth_session_ttl_seconds: Option<u64>,
```

DpsConfig::new() addition:

```rust
auth_session_ttl_seconds: load_env_u64("DPS_AUTH_SESSION_TTL_SECONDS"),
```

Helper:

```rust
fn load_env_u64(key: &str) -> Option<u64> {
  env::var(key).ok().and_then(|v| v.parse::<u64>().ok())
}
```

Getter and setter:

```rust
pub fn get_auth_session_ttl_seconds(&self) -> u64 {
  self.auth_session_ttl_seconds.unwrap_or(1209600)
}

pub fn set_auth_session_ttl_seconds(&mut self, value: Option<u64>) {
  self.auth_session_ttl_seconds = value;
}
```

Tests (add to existing tests module):

```rust
#[test]
fn test_auth_session_ttl_seconds_default_and_set() {
  let mut c = DpsConfig::new();
  assert_eq!(c.get_auth_session_ttl_seconds(), 1209600);
  c.set_auth_session_ttl_seconds(Some(3600));
  assert_eq!(c.get_auth_session_ttl_seconds(), 3600);
}

#[test]
fn test_auth_session_ttl_seconds_from_env() {
  std::env::set_var("DPS_AUTH_SESSION_TTL_SECONDS", "1800");
  let c = DpsConfig::new();
  assert_eq!(c.get_auth_session_ttl_seconds(), 1800);
  std::env::remove_var("DPS_AUTH_SESSION_TTL_SECONDS");
}
```

README update:

- Add table row under DpsAuthApi:

| `auth_session_ttl_seconds` | `DPS_AUTH_SESSION_TTL_SECONDS` | 1209600 | Session TTL in seconds (optional; defaults to 14 days) |

- Add a short example showing set/get:

```rust
let mut c = DpsConfig::new();
c.set_auth_session_ttl_seconds(Some(3600));
assert_eq!(c.get_auth_session_ttl_seconds(), 3600);
```

## Test command

Use the project test command:

```
mise exec -- cargo test
```

## Notes

- Keep behavior consistent with existing numeric option `auth_api_port`.
- Treat parse failures as unset to avoid panics.

End.