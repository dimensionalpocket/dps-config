# dps-config

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

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

Add it to `Cargo.toml`:

```toml
[dependencies]
dps-config = { git = "https://github.com/dimensionalpocket/dps-config-rs" }
```

Or add via cargo:

```bash
cargo add --git https://github.com/dimensionalpocket/dps-config-rs dps-config
```

## Quick Start

Basic usage example:

```rust
use dps_config::DpsConfig;

fn main() {
    let mut config = DpsConfig::new();

    // defaults
    let domain = config.get_domain();
    let api_sub = config.get_api_subdomain();

    // overrides
    config.set_domain("example.com");
    config.set_development_mode(true);

    let api_domain = config.get_api_domain();
    println!("API domain: {}", api_domain);
}
```

## Configuration Properties

The following properties are provided. Properties load from environment variables when present.

### Global

| Property | Environment Variable | Default | Description | Example |
|----------|----------------------|---------|-------------|---------|
| `domain` | `DPS_DOMAIN` | `dps.localhost` | Main domain of the website | `example.com` |
| `api_subdomain` | `DPS_API_SUBDOMAIN` | `api` | Subdomain for all APIs | |
| `development_mode` | `DPS_DEVELOPMENT_MODE` | `false` | Enables development-only features | |

### DpsAuthApi

| Property | Environment Variable | Default | Description | Example |
|----------|----------------------|---------|-------------|---------|
| `auth_api_subdomain` | `DPS_AUTH_API_SUBDOMAIN` | `auth` | Sub-subdomain for DpsAuthApi | |
| `auth_api_port` | `DPS_AUTH_API_PORT` | none | Port for DpsAuthApi (omitted from URL if unset) | `3000` |
| `auth_api_protocol` | `DPS_AUTH_API_PROTOCOL` | `https` | Protocol for DpsAuthApi | `http` for local dev |
| `auth_api_sqlite_file_path` | `DPS_AUTH_API_SQLITE_FILE_PATH` | `data/development.db` | SQLite database file path | `/var/data/auth.db` |
| `auth_api_session_secret` | `DPS_AUTH_API_SESSION_SECRET` | none | 32-byte session secret for encryption | `your-secret-key-here` |

## Computed Getters

Computed getters derive values from base properties and have no setters or environment variables.

- `get_api_domain()` — returns `{api_subdomain}.{domain}` (e.g. `api.dps.localhost`)
- `get_auth_api_url()` — returns `{protocol}://{auth_api_subdomain}.{api_subdomain}.{domain}{:port}` when port is set

```rust
let mut c = DpsConfig::new();
c.set_api_subdomain("api");
c.set_domain("dps.localhost");
assert_eq!(c.get_api_domain(), "api.dps.localhost");
assert_eq!(c.get_auth_api_url(), "https://auth.api.dps.localhost");
```

## Environment Variables

Properties auto-load from environment variables when `DpsConfig::new()` is called.
Boolean true is expressed as `"Y"` in environment variables.

Example (development):

```bash
export DPS_DOMAIN="dps.localhost"
export DPS_API_SUBDOMAIN="api"
export DPS_DEVELOPMENT_MODE="Y"
export DPS_AUTH_API_PROTOCOL="http"
export DPS_AUTH_API_PORT="3000"
export DPS_AUTH_API_SQLITE_FILE_PATH="data/development.db"
export DPS_AUTH_API_SESSION_SECRET="dev-secret-key-32-bytes-long!"
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
        c.set_auth_api_protocol("http");
        c.set_auth_api_port(Some(8080));
        assert_eq!(c.get_auth_api_url(), "http://auth.api.test.local:8080");
    }
}
```

## Project Structure

```
dps-config-rs/
├── src/          # main library code with DpsConfig
├── docs/         # documentation (LLM instructions, plans, drafts, etc.)
├── Cargo.toml
└── README.md
```

## License

[MIT](LICENSE)
