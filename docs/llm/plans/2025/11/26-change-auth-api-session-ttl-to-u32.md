# Change auth_api_session_ttl_seconds from u64 to u32

## Overview
Change the data type of `auth_api_session_ttl_seconds` field from `u64` to `u32` to better represent the realistic range of session TTL values.

## Files to Modify

### src/lib.rs
- Line 55: Change field type from `Option<u64>` to `Option<u32>`
- Line 86: Change `load_env_u64` to `load_env_u32` for `DPS_AUTH_API_SESSION_TTL_SECONDS`
- Line 249: Change return type from `u64` to `u32` in `get_auth_api_session_ttl_seconds`
- Line 254: Change parameter type from `Option<u64>` to `Option<u32>` in `set_auth_api_session_ttl_seconds`
- Line 461-471: Update test `test_auth_api_session_ttl_seconds` to use `u32` values

### README.md
- Line 82: Update documentation to reflect the type change (though the table may not explicitly show types)
- Line 148-151: Update example code to use `u32` values if needed

## Implementation Details

### Type Changes
```rust
// Before
auth_api_session_ttl_seconds: Option<u64>,
pub fn get_auth_api_session_ttl_seconds(&self) -> u64
pub fn set_auth_api_session_ttl_seconds(&mut self, value: Option<u64>)

// After  
auth_api_session_ttl_seconds: Option<u32>,
pub fn get_auth_api_session_ttl_seconds(&self) -> u32
pub fn set_auth_api_session_ttl_seconds(&mut self, value: Option<u32>)
```

### Environment Loading
```rust
// Before
auth_api_session_ttl_seconds: load_env_u64("DPS_AUTH_API_SESSION_TTL_SECONDS"),

// After
auth_api_session_ttl_seconds: load_env_u32("DPS_AUTH_API_SESSION_TTL_SECONDS"),
```

### Test Updates
The test values should remain the same since they're within u32 range:
- Default value: 1209600 (14 days in seconds)
- Test values: 3600, 1800

All these values fit comfortably within u32's maximum value of 4,294,967,295.

## Rationale
- Session TTL in seconds realistically won't exceed u32's maximum (~136 years)
- u32 is more memory efficient
- Consistent with other configuration fields that use appropriate-sized types
- The default value of 14 days (1,209,600 seconds) is well within u32 range