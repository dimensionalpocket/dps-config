# @dimensionalpocket/dps-config

[![Rust Tests](https://github.com/dimensionalpocket/dps-config/actions/workflows/test.yml/badge.svg)](https://github.com/dimensionalpocket/dps-config/actions/workflows/test.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Configuration management for the [DPS ecosystem](https://github.com/dimensionalpocket/dps-readme).

## Overview

This repo provides the `DpsConfig` struct, a lightweight configuration container used by Rust components in the DPS ecosystem.
It focuses on optional values, sensible defaults, environment variable loading, and computed getters.

Key principles:

- No validation in the struct; consuming crates perform validation.
- Most getters provide hardcoded defaults suitable for development.
- Environment variables are used to populate properties automatically.
- Computed getters derive combined values (URLs, domains) from base properties.

## Installation

### Rust

Add it to `Cargo.toml`:

```toml
[dependencies]
dps-config = { git = "https://github.com/dimensionalpocket/dps-config" }
```

Or add via cargo:

```bash
cargo add --git https://github.com/dimensionalpocket/dps-config dps-config
```

### Bun

```bash
bun add @dimensionalpocket/dps-config
```

## Quick Start

### Rust

```rust
use dps_config::DpsConfig;

fn main() {
    let mut config = DpsConfig::new();

    // defaults
    let domain = config.get_domain();
    let api_path = config.get_api_path();

    // overrides
    config.set_domain("example.com");
    config.set_api_path("v1");
    config.set_development_mode(true);

    let auth_api_url = config.get_auth_api_url();
    println!("Auth API URL: {}", auth_api_url);
}
```

### Bun / TypeScript

```typescript
import { DpsConfig } from "@dimensionalpocket/dps-config";

const config = new DpsConfig();

// defaults
const domain = config.getDomain();
const apiPath = config.getApiPath();

// overrides
config.setDomain("example.com");
config.setApiPath("v1");
config.setDevelopmentMode(true);

const authApiUrl = config.getAuthApiUrl();
console.log(`Auth API URL: ${authApiUrl}`);

// Vite support (loads environment variables with VITE_ prefix)
const viteConfig = new DpsConfig("VITE_");
```

## Configuration Properties

The following properties are provided. Properties load from environment variables when present.  
Each property has a getter (`get_<property_name>()`) and a setter (`set_<property_name>(value)`).

### Global

| Property | Environment Variable | Default | Description |
|----------|----------------------|---------|-------------|
| `domain` | `DPS_DOMAIN` | `dps.localhost` | Main domain of the website |
| `api_path` | `DPS_API_PATH` | `api` | Path (without leading slash) for API endpoints |
| `development_mode` | `DPS_DEVELOPMENT_MODE` | `false` | Enables development-only features |

### DpsAuthApi

| Property | Environment Variable | Default | Description |
|----------|----------------------|---------|-------------|
| `auth_api_subdomain` | `DPS_AUTH_API_SUBDOMAIN` | `auth` | Sub-subdomain for DpsAuthApi |
| `auth_api_port` | `DPS_AUTH_API_PORT` | none | Port for DpsAuthApi (omitted from URL if unset) |
| `auth_api_protocol` | `DPS_AUTH_API_PROTOCOL` | `https` | Protocol for DpsAuthApi |
| `auth_api_insecure_cookie` | `DPS_AUTH_API_INSECURE_COOKIE` | `false` | Allow insecure cookies (HTTP) |
| `auth_api_sqlite_main_file_path` | `DPS_AUTH_API_SQLITE_MAIN_FILE_PATH` | `data/main-development.db` | SQLite main database file path |
| `auth_api_sqlite_main_pool_size` | `DPS_AUTH_API_SQLITE_MAIN_POOL_SIZE` | `1` | SQLite main database connection pool size |
| `auth_api_sqlite_collection_file_path` | `DPS_AUTH_API_SQLITE_COLLECTION_FILE_PATH` | `data/collection-development.db` | SQLite collection database file path |
| `auth_api_sqlite_collection_pool_size` | `DPS_AUTH_API_SQLITE_COLLECTION_POOL_SIZE` | `1` | SQLite collection database connection pool size |
| `auth_api_sqlite_session_file_path` | `DPS_AUTH_API_SQLITE_SESSION_FILE_PATH` | `data/session-development.db` | SQLite session database file path |
| `auth_api_sqlite_session_pool_size` | `DPS_AUTH_API_SQLITE_SESSION_POOL_SIZE` | `1` | SQLite session database connection pool size |
| `auth_api_session_secret` | `DPS_AUTH_API_SESSION_SECRET` | none | 32-byte session secret for encryption |
| `auth_api_session_ttl_seconds` | `DPS_AUTH_API_SESSION_TTL_SECONDS` | `1209600` (14 days) | Session TTL in seconds |

## Computed Getters

Computed getters derive values from base properties and have no setters or environment variables.

- `get_auth_api_url()` — returns `{protocol}://{auth_api_subdomain}.{domain}/{api_path}` (with `:{port}` appended after domain when port is set)
- `get_auth_api_session_secret_bytes()` — returns session secret as `Vec<u8>` for encryption libraries

```rust
let mut c = DpsConfig::new();
c.set_api_path("v1");
c.set_domain("dps.localhost");
assert_eq!(c.get_auth_api_url(), "https://auth.dps.localhost/v1");

// Session secret as bytes (convenient for encryption libraries)
c.set_auth_api_session_secret(Some("my-32-byte-secret-key-here!!!"));
if let Some(secret_bytes) = c.get_auth_api_session_secret_bytes() {
    assert_eq!(secret_bytes.len(), 32);
}
```

## Environment Variables

Properties auto-load from environment variables when `DpsConfig::new()` is called.
Boolean true is expressed as `"Y"` in environment variables.

Example (development):

```bash
export DPS_DOMAIN="dps.localhost"
export DPS_API_PATH="api"
export DPS_DEVELOPMENT_MODE="Y"
export DPS_AUTH_API_PROTOCOL="http"
export DPS_AUTH_API_PORT="3000"
export DPS_AUTH_API_INSECURE_COOKIE="Y"
export DPS_AUTH_API_SQLITE_MAIN_FILE_PATH="data/main-development.db"
export DPS_AUTH_API_SQLITE_MAIN_POOL_SIZE="4"
export DPS_AUTH_API_SQLITE_COLLECTION_FILE_PATH="data/collection-development.db"
export DPS_AUTH_API_SQLITE_COLLECTION_POOL_SIZE="4"
export DPS_AUTH_API_SQLITE_SESSION_FILE_PATH="data/session-development.db"
export DPS_AUTH_API_SQLITE_SESSION_POOL_SIZE="4"
export DPS_AUTH_API_SESSION_SECRET="dev-secret-key-32-bytes-long!"
export DPS_AUTH_API_SESSION_TTL_SECONDS="1209600"
```

## Usage Examples

```rust
use dps_config::DpsConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_url_builds() {
        let mut c = DpsConfig::new();
        c.set_domain("test.local");
        c.set_api_path("v1");
        c.set_auth_api_protocol("http");
        c.set_auth_api_port(Some(8080));
        assert_eq!(c.get_auth_api_url(), "http://auth.test.local:8080/v1");
    }

    #[test]
    fn auth_session_ttl_examples() {
        let mut c = DpsConfig::new();

        // default is 14 days in seconds
        assert_eq!(c.get_auth_api_session_ttl_seconds(), 1209600);

        c.set_auth_api_session_ttl_seconds(Some(3600));
        assert_eq!(c.get_auth_api_session_ttl_seconds(), 3600);
    }
}
```

## Project Structure

```
dps-config/
├── src/
│   ├── index.ts  # Bun / TypeScript implementation
│   └── lib.rs    # Rust implementation
├── docs/         # documentation (LLM instructions, plans, drafts, etc.)
├── Cargo.toml    # Rust package manifest
├── package.json  # Bun package manifest
└── README.md
```

## License

[MIT](LICENSE)
