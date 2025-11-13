# README Plan for dps-config-rs

## Overview

This plan outlines the structure and content for the comprehensive README.md file for the `dps-config-rs` repository. The README will serve as the primary documentation for the DpsConfig struct, a configuration management component used across the DPS ecosystem.

For more information about the DPS ecosystem, see the [DPS README](https://github.com/dimensionalpocket/dps-readme).

## Target Audience

- Rust developers new to the DPS ecosystem
- Developers working on DPS components (DpsAuthApi, DpsLogsApi, DpsMetricsApi, etc.)
- Users new to Rust (as mentioned in instructions.md)

## README Structure

### 1. Header Section

**Content:**
- Project title: `# dps-config-rs`
- Brief one-line description
- Badges (if applicable): build status, crates.io version, license, etc.

**Example:**
```markdown
# dps-config-rs

Configuration management for the DPS ecosystem

> Part of the [DPS ecosystem](https://github.com/dimensionalpocket/dps-readme)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
```

### 2. Overview

**Content:**
- What is `dps-config-rs`
- Its role in the DPS ecosystem
- Key design principles (no validation, optional values, default values in getters)

**Example content:**
```markdown
## Overview

`dps-config-rs` provides the `DpsConfig` struct, a centralized configuration management 
solution for Rust-based components in the DPS ecosystem. It handles configuration for 
multiple services including:

- **APIs**: DpsAuthApi, DpsLogsApi, DpsMetricsApi (Rust GraphQL APIs)
- **Web Apps**: DpsAuthWeb, DpsMonitorWeb (Vue frontend apps)

### Design Philosophy

- **No Validation**: All configuration values are optional. Validation is delegated to 
  the consuming crate.
- **Smart Defaults**: Most properties have sensible hardcoded defaults suitable for 
  development.
- **Environment-First**: Configuration values are automatically loaded from environment 
  variables.
- **Computed Properties**: Additional getters compute derived values (e.g., full URLs) 
  from base properties.
```

### 3. Installation

**Content:**
- How to add the crate to `Cargo.toml` from GitHub
- Minimum Rust version (if applicable)

**Example:**
```markdown
## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dps-config = { git = "https://github.com/dimensionalpocket/dps-config-rs" }
```

Or using cargo:

```bash
cargo add --git https://github.com/dimensionalpocket/dps-config-rs dps-config
```
```

### 4. Quick Start

**Content:**
- Simple example showing basic usage
- Creating a config instance
- Reading configuration values
- Setting configuration values

**Example code sample:**
```rust
use dps_config::DpsConfig;

fn main() {
    // Create a new config instance
    let mut config = DpsConfig::new();
    
    // Get configuration values (with defaults)
    let domain = config.get_domain(); // Returns "dps.localhost"
    let api_subdomain = config.get_api_subdomain(); // Returns "api"
    
    // Set custom values
    config.set_domain("example.com");
    config.set_development_mode(true);
    
    // Use computed getters
    let api_domain = config.get_api_domain(); // Returns "api.example.com"
    
    println!("API Domain: {}", api_domain);
}
```

### 5. Configuration Properties

**Content:**
- Complete reference table of all configuration properties
- Organized by category (Global, DpsAuthApi-specific, etc.)
- Each property should list:
  - Property name
  - Environment variable
  - Default value
  - Description
  - Example value

**Example structure:**

#### Global Configuration

| Property | Environment Variable | Default | Description | Example |
|----------|---------------------|---------|-------------|---------|
| `domain` | `DPS_DOMAIN` | `"dps.localhost"` | Main domain of the website | `"example.com"` |
| `api_subdomain` | `DPS_API_SUBDOMAIN` | `"api"` | Subdomain for all APIs | `"api"` |
| `development_mode` | `DPS_DEVELOPMENT_MODE` | `false` | Enables development-only features | `true` (set as `"Y"`) |

#### DpsAuthApi Configuration

| Property | Environment Variable | Default | Description | Example |
|----------|---------------------|---------|-------------|---------|
| `auth_api_subdomain` | `DPS_AUTH_API_SUBDOMAIN` | `"auth"` | Sub-subdomain for DpsAuthApi | `"auth"` |
| `auth_api_port` | `DPS_AUTH_API_PORT` | `None` | Port for DpsAuthApi (omitted if not set) | `3000` |
| `auth_api_protocol` | `DPS_AUTH_API_PROTOCOL` | `"https"` | Protocol for DpsAuthApi | `"http"` |
| `auth_api_sqlite_file_path` | `DPS_AUTH_API_SQLITE_FILE_PATH` | `"data/development.db"` | SQLite database file path | `"/var/data/auth.db"` |
| `auth_api_session_secret` | `DPS_AUTH_API_SESSION_SECRET` | `None` | 32-byte session secret for encryption | `"your-secret-key-here"` |

### 6. Computed Getters

**Content:**
- Explanation of computed properties
- List of all computed getters with examples
- How they derive values from base properties

**Example:**
```markdown
## Computed Getters

Computed getters provide derived values based on multiple configuration properties. 
They do not have corresponding setters or environment variables.

### Available Computed Getters

#### `get_api_domain()`

Returns the global API domain without protocol and port, combining `api_subdomain` and `domain`.

**Example:**
```rust
let mut config = DpsConfig::new();
config.set_api_subdomain("api");
config.set_domain("dps.localhost");

let api_domain = config.get_api_domain();
// Returns: "api.dps.localhost"
```

**Usage:** This is commonly used as the cookies domain setting so all APIs can access cookies.

#### `get_auth_api_url()`

Returns the complete URL for the DpsAuthApi service, combining `auth_api_protocol`, 
`auth_api_subdomain`, `api_subdomain`, `domain`, and `auth_api_port` (if set).

**Example:**
```rust
let mut config = DpsConfig::new();
config.set_auth_api_protocol("http");
config.set_auth_api_subdomain("auth");
config.set_api_subdomain("api");
config.set_domain("dps.localhost");
config.set_auth_api_port(Some(3000));

let auth_api_url = config.get_auth_api_url();
// Returns: "http://auth.api.dps.localhost:3000"
```
```

### 7. Environment Variables

**Content:**
- Explanation of automatic environment variable loading
- Boolean environment variable convention (`"Y"` for truthy)
- Example of setting environment variables

**Example:**
```markdown
## Environment Variables

The `DpsConfig` struct automatically loads configuration values from environment variables 
when instantiated. This makes it easy to configure applications in different environments 
without code changes.

### Boolean Values

Boolean configuration properties use the string `"Y"` to represent `true`:

```bash
export DPS_DEVELOPMENT_MODE="Y"  # Enables development mode
```

### Example Configuration

Development environment:
```bash
export DPS_DOMAIN="dps.localhost"
export DPS_API_SUBDOMAIN="api"
export DPS_DEVELOPMENT_MODE="Y"
export DPS_AUTH_API_PROTOCOL="http"
export DPS_AUTH_API_PORT="3000"
export DPS_AUTH_API_SQLITE_FILE_PATH="data/development.db"
export DPS_AUTH_API_SESSION_SECRET="dev-secret-key-32-bytes-long!"
```

Production environment:
```bash
export DPS_DOMAIN="example.com"
export DPS_API_SUBDOMAIN="api"
export DPS_AUTH_API_PROTOCOL="https"
export DPS_AUTH_API_SQLITE_FILE_PATH="/var/lib/dps/auth.db"
export DPS_AUTH_API_SESSION_SECRET="prod-secret-key-32-bytes-long!"
```
```

### 8. Usage Examples

**Content:**
- Common usage patterns
- Integration with different DPS components
- Real-world scenarios

**Example:**
```markdown
## Usage Examples

### Example 1: Using in DpsAuthApi

```rust
use dps_config::DpsConfig;

fn setup_auth_api() {
    let config = DpsConfig::new();
    
    // Get the full Auth API URL
    let api_url = config.get_auth_api_url();
    println!("Starting DpsAuthApi at: {}", api_url);
    
    // Get database configuration
    let db_path = config.get_auth_api_sqlite_file_path();
    println!("Database location: {}", db_path);
    
    // Check if in development mode
    if config.get_development_mode() {
        println!("Running in DEVELOPMENT mode");
    }
}
```

### Example 2: Custom Configuration for Testing

```rust
use dps_config::DpsConfig;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_api_url_construction() {
        let mut config = DpsConfig::new();
        
        // Override defaults for testing
        config.set_domain("test.local");
        config.set_auth_api_protocol("http");
        config.set_auth_api_port(Some(8080));
        
        let url = config.get_auth_api_url();
        assert_eq!(url, "http://auth.api.test.local:8080");
    }
}
```

### Example 3: Different Environments

```rust
use dps_config::DpsConfig;

fn get_environment_specific_config() -> DpsConfig {
    let mut config = DpsConfig::new();
    
    // Environment variables are loaded automatically
    // but you can override for specific use cases
    
    if config.get_development_mode() {
        // Development-specific overrides
        config.set_auth_api_protocol("http");
        config.set_auth_api_port(Some(3000));
    }
    
    config
}
```
```

### 9. API Reference

**Content:**
- Brief overview stating full API docs are in code documentation
- Link to docs.rs (once published)
- List of all available methods grouped by category

**Example:**
```markdown
## API Reference

For detailed API documentation, run `cargo doc --open` in the project directory.

### Method Categories

**Constructors:**
- `new()` - Create a new DpsConfig instance with environment variables loaded

**Global Configuration:**
- `get_domain()` / `set_domain(value: &str)`
- `get_api_subdomain()` / `set_api_subdomain(value: &str)`
- `get_development_mode()` / `set_development_mode(value: bool)`

**DpsAuthApi Configuration:**
- `get_auth_api_subdomain()` / `set_auth_api_subdomain(value: &str)`
- `get_auth_api_port()` / `set_auth_api_port(value: Option<u16>)`
- `get_auth_api_protocol()` / `set_auth_api_protocol(value: &str)`
- `get_auth_api_sqlite_file_path()` / `set_auth_api_sqlite_file_path(value: &str)`
- `get_auth_api_session_secret()` / `set_auth_api_session_secret(value: Option<&str>)`

**Computed Getters:**
- `get_api_domain()` - Returns combined API domain
- `get_auth_api_url()` - Returns full Auth API URL
```

### 10. Development

**Content:**
- How to build the project
- Running tests
- Contributing guidelines (if applicable)
- Using `mise exec --` for Rust commands (per instructions.md)

**Example:**
```markdown
## Development

### Building

This project uses `mise` for environment management. Always prepend Rust commands with 
`mise exec --`:

```bash
# Build the project
mise exec -- cargo build

# Run tests
mise exec -- cargo test

# Check formatting
mise exec -- cargo fmt --check

# Run clippy
mise exec -- cargo clippy
```

### Testing

```bash
mise exec -- cargo test
```

### Project Structure

```
dps-config-rs/
├── src/
│   └── lib.rs          # Main library code with DpsConfig struct
├── docs/               # Documentation (LLM instructions, plans, drafts, etc.)
├── Cargo.toml          # Project manifest
└── README.md           # This file
```
```

### 11. License

**Content:**
- License type
- Link to LICENSE file

**Example:**
```markdown
## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```

## Additional Recommendations

### Optional Sections to Consider

1. **Troubleshooting** - Common issues and solutions
2. **FAQ** - Frequently asked questions
3. **Changelog** - Link to CHANGELOG.md
4. **Roadmap** - Future planned features
5. **Architecture Diagram** - Visual representation of how DpsConfig fits in the ecosystem

### Documentation Best Practices

1. **Code Examples**: All code examples should be complete, runnable, and tested
2. **Consistency**: Use consistent terminology throughout
3. **Beginner-Friendly**: Remember the user is new to Rust - explain concepts clearly
4. **Keep Updated**: Update README whenever API changes

## Implementation Notes

### Files to Create/Modify

- **README.md** (root) - The main README file
- No other files needed for this plan

### Code Samples for Review

Since this is a documentation-only task, the "code samples" are the markdown examples 
shown above in each section. The actual Rust code examples within the README demonstrate 
how the future `DpsConfig` implementation will be used.

### Estimated Sections Count

The final README will have approximately 11 main sections covering:
- Project overview and philosophy
- Installation and quick start
- Complete configuration reference
- Usage examples for common scenarios
- API reference
- Development guidelines

## Summary

This plan creates a comprehensive, beginner-friendly README that:
- ✅ Clearly explains the purpose and design philosophy
- ✅ Provides complete configuration reference tables
- ✅ Includes practical, runnable code examples
- ✅ Documents all properties and computed getters
- ✅ Explains environment variable conventions
- ✅ Shows real-world usage patterns
- ✅ Considers the user's Rust experience level
- ✅ Follows the project's documentation standards
- ✅ Links to DPS ecosystem documentation
- ✅ Uses correct GitHub-based installation instructions